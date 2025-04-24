use chrono::NaiveDateTime;
use struct_field_names_as_array::FieldNamesAsArray;

#[derive(
    bitcode::Encode,
    bitcode::Decode,
    serde::Deserialize,
    serde::Serialize,
    Clone,
    PartialEq,
    Debug,
    FieldNamesAsArray,
)]
pub struct Baseline {
    pub baseline_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub comment: Option<String>,
    pub is_active: bool,
}

impl Baseline {
    pub fn fields() -> [&'static str; 5] {
        Baseline::FIELD_NAMES_AS_ARRAY
    }
}

#[derive(
    bitcode::Encode,
    bitcode::Decode,
    serde::Deserialize,
    serde::Serialize,
    Clone,
    PartialEq,
    Debug,
    FieldNamesAsArray,
)]
#[cfg_attr(feature = "pmbe", derive(sea_orm::FromQueryResult))]
pub struct ResourceAllocation {
    pub resource_baseline_id: i64,
    pub baseline_id: i64,
    pub resource_id: i64,
    pub task_id: i64,
    pub resource_summary: Option<String>,
    pub task_summary: Option<String>,
    pub capacity_allocated: Option<f64>,
    pub capacity: Option<f64>,
    pub capacity_unit: Option<String>,
}

impl ResourceAllocation {
    pub fn fields() -> [&'static str; 9] {
        ResourceAllocation::FIELD_NAMES_AS_ARRAY
    }
}

#[derive(
    // bitcode::Encode,
    // bitcode::Decode,
    serde::Deserialize,
    serde::Serialize,
    Clone,
    PartialEq,
    Debug,
    FieldNamesAsArray,
)]
#[cfg_attr(feature = "pmbe", derive(sea_orm::FromQueryResult))]
pub struct TaskBaseline {
    pub task_baseline_id: i64,
    pub task_id: i64,
    pub baseline_id: i64,
    pub task_summary: String,
    pub task_description: Option<String>,
    pub task_comment: Option<String>,
    pub wbs: String,
    pub parent: Option<i64>,
    pub start: NaiveDateTime,
    pub start_timezone: String,
    pub finish: NaiveDateTime,
    pub finish_timezone: String,
}

impl TaskBaseline {
    pub fn fields() -> [&'static str; 12] {
        TaskBaseline::FIELD_NAMES_AS_ARRAY
    }
}
