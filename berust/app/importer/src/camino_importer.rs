use chrono::Months;
use polars::prelude::*;
use sea_orm::QueryFilter;
use sea_orm::entity::*;
use sea_orm::{ActiveValue::Set, DatabaseConnection, EntityTrait, TransactionTrait};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use spreadsheet_ods::*;
use std::fs::read_to_string;
use tracing::*;

const APP_PATH: &str = "/app";
const PROJECT_INFO: &str = "/app/local/tmp/project_info.json";
const LIST_OF_TASKS: &str = "/app/local/tmp/list_of_tasks.json";
const RESOURCES: &str = "/app/local/tmp/resources.json";
const ROW_OFFSET: i32 = 8;

// fn ods_cell_to_string(cell: CellContent) -> String {
//     match cell.value() {
//         polars::value::Value::Empty => "".to_string(),
//         polars::value::Value::Text(s) => s.to_string(),
//         polars::value::Value::Number(n) => n.to_string(),
//         polars::value::Value::DateTime(dt) => dt.to_string(),
//         polars::value::Value::Boolean(b) => b.to_string(),
//         _ => panic!("{:?}", cell.value()),
//     }
// }

fn read_json_file<T: DeserializeOwned>(path: &str) -> Result<T, String> {
    read_to_string(path)
        .map_err(|e| format!("Failed to read json file: {e}"))
        .and_then(|data| {
            serde_json::from_str(&data).map_err(|e| format!("Failed to deserialize json file: {e}"))
        })
}

fn read_ods_file() -> Result<WorkBook, String> {
    read_ods(format!("{APP_PATH}/local/tmp/eic2025summary-people.ods"))
        .map_err(|e| format!("Failed to read ODS file: {e}"))
}

fn get_sheet<'a>(wb: &'a WorkBook, name: &str) -> Result<&'a Sheet, String> {
    Ok(wb.sheet(
        wb.sheet_idx(name)
            .ok_or(format!("Cannot find sheet with name {name}"))?,
    ))
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    currency: String,
    avg_hours_per_month: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct ProjectInfo {
    project_name: String,
    project_start: chrono::NaiveDateTime,
    project_finish: chrono::NaiveDateTime,
    timezone: String,
    config: Config,
}

#[derive(Debug, Serialize, Deserialize)]
struct TaskInput {
    id: u32,
    name: String,
    wbs: String,
    parent: Option<u32>,
    begin_month: Option<u32>,
    end_month: Option<u32>,
    planned_work_pm: Option<u32>,
    planned_team_cost_eur: Option<u32>,
    planned_other_cost_eur: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ResourceInput {
    task: String,
    name: String,
    description: Option<String>,
    resource_type_id: String,
    cost: f64,
    cost_currency: String,
    billing_frequency: String,
    capacity: Option<f64>,
    capacity_unit: Option<String>,
}

pub async fn import_project_plan(db: DatabaseConnection) -> Result<(), String> {
    let baseline_id = entity::config::Entity::find()
        .filter(entity::config::Column::ConfigKey.eq("baseline_id_default"))
        .one(&db) // Use .one() as we expect a single result for the key
        .await
        .map_err(|e| format!("Database error finding baseline_id: {e}"))? // Handle potential DB error
        .ok_or_else(|| "Baseline ID 'baseline_id_default' not found".to_string())? // Handle case where key doesn't exist
        .config_value
        .ok_or_else(|| "Baseline ID 'baseline_id_default' has a NULL value".to_string())?
        .parse::<i64>()
        .map_err(|e| format!("Failed to parse baseline_id: {e}"))?; // Handle case where value is NULL

    let project_info = read_json_file::<ProjectInfo>(PROJECT_INFO)?;
    let tasks = read_json_file::<Vec<TaskInput>>(LIST_OF_TASKS)?;
    let mut resources = read_json_file::<Vec<ResourceInput>>(RESOURCES)?;

    for resource in resources.iter_mut() {
        resource.description = Some(format!("{} {}", resource.name, resource.task));
    }

    // Start a transaction
    let txn = db
        .begin()
        .await
        .map_err(|e| format!("Failed to start transaction: {e}"))?;

    let tasks_inserted = entity::tasks::Entity::insert_many(
        tasks
            .iter()
            .map(|task| entity::tasks::ActiveModel {
                summary: Set(task.name.clone()),
                ..Default::default()
            })
            .collect::<Vec<entity::tasks::ActiveModel>>(),
    )
    .exec_with_returning_many(&txn) // Use the transaction
    .await
    .unwrap_or_else(|e| panic!("Failed to insert tasks: {e}"));

    entity::tasks_baselines::Entity::insert_many(
        tasks
            .iter()
            .map(|task| entity::tasks_baselines::ActiveModel {
                task_id: Set({
                    tasks_inserted
                        .iter()
                        .find(|t| t.summary == task.name)
                        .map(|t| t.task_id)
                        .unwrap_or_else(|| panic!("Task {} not found", task.name))
                }),
                baseline_id: Set(baseline_id),
                wbs: Set(task.wbs.clone()),
                start_timezone: Set(project_info.timezone.clone()),
                finish_timezone: Set(project_info.timezone.clone()),
                parent: Set({
                    tasks_inserted
                        .iter()
                        .find(|t| t.summary == task.name)
                        .map(|t| t.task_id)
                }),
                start: Set(project_info
                    .project_start
                    .checked_add_months(Months::new(
                        task.begin_month.unwrap_or_else(|| {
                            let mut begin_month = 29;
                            for t in tasks.iter() {
                                if t.parent == Some(task.id)
                                    && t.begin_month.unwrap_or(29) < begin_month
                                {
                                    begin_month = t.begin_month.unwrap_or(29);
                                }
                            }
                            begin_month
                        }) - 1,
                    ))
                    .unwrap()),
                finish: Set(project_info
                    .project_start
                    .checked_add_months(Months::new(task.end_month.unwrap_or_else(|| {
                        let mut end_month = 0;
                        for t in tasks.iter() {
                            if t.parent == Some(task.id) && t.end_month.unwrap_or(0) > end_month {
                                end_month = t.end_month.unwrap_or(0);
                            }
                        }
                        end_month
                    })))
                    .unwrap()),
                ..Default::default()
            })
            .collect::<Vec<entity::tasks_baselines::ActiveModel>>(),
    )
    .exec(&txn) // Use the transaction
    .await
    .unwrap_or_else(|e| panic!("Failed to insert tasks baselines: {e}"));

    let resource_types = entity::resource_types::Entity::find()
        .all(&db)
        .await
        .unwrap()
        .into_iter()
        .map(|resource_type| (resource_type.name, resource_type.resource_type_id))
        .collect::<std::collections::HashMap<String, i64>>();

    debug!("Resource types: {:?}", resource_types);

    let resources_inserted = entity::resources::Entity::insert_many(
        resources
            .iter()
            .map(|resource| entity::resources::ActiveModel {
                summary: Set(resource.name.clone()),
                description: Set(resource.description.clone()),
                resource_type_id: Set(*resource_types.get(&resource.resource_type_id).unwrap()),
                cost: Set(Some(resource.cost)),
                cost_currency: Set(resource.cost_currency.clone()),
                capacity: Set(resource.capacity),
                capacity_unit: Set(resource.capacity_unit.clone()),
                ..Default::default()
            })
            .collect::<Vec<entity::resources::ActiveModel>>(),
    )
    .exec_with_returning_many(&txn) // Use the transaction
    .await
    .unwrap_or_else(|e| panic!("Failed to insert resources: {e}"));

    entity::resources_baselines::Entity::insert_many(
        resources
            .iter()
            .map(|resource| entity::resources_baselines::ActiveModel {
                resource_id: Set(resources_inserted
                    .iter()
                    .find(|r| r.description == resource.description) // TODO: bug as hell fix this
                    .map(|r| r.resource_id)
                    .unwrap_or_else(|| panic!("Resource {} not found", resource.name))),
                baseline_id: Set(baseline_id),
                task_id: Set(tasks_inserted
                    .iter()
                    .find(|t| t.summary == resource.task)
                    .map(|t| t.task_id)
                    .unwrap_or_else(|| panic!("Task {} not found", resource.task))),
                capacity_allocated: Set(resource.capacity),
                ..Default::default()
            })
            .collect::<Vec<entity::resources_baselines::ActiveModel>>(),
    )
    .exec(&txn) // Use the transaction
    .await
    .unwrap_or_else(|e| panic!("Failed to insert resources baselines: {e}"));

    // Commit the transaction
    txn.commit()
        .await
        .map_err(|e| format!("Failed to commit transaction: {e}"))?;

    Ok(())
}
