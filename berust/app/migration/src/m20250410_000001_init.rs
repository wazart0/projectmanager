use sea_orm::{DatabaseBackend, EntityTrait, Set, Statement};
use sea_orm_migration::prelude::*;
use std::process::exit;
use tracing::error;

use entity::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let schema = sea_orm::Schema::new(manager.get_database_backend());
        let db = manager.get_connection();

        db.execute(Statement::from_string(
            db.get_database_backend(),
            r#"
CREATE OR REPLACE
FUNCTION pseudo_encrypt(value bigint) returns bigint AS $$
DECLARE
l1 bigint;
l2 bigint;
r1 bigint;
r2 bigint;
i bigint:=0;
BEGIN
l1:= (value >> 16) & 65535;
r1:= value & 65535;
WHILE i < 3 LOOP
l2 := r1;
r2 := l1 # ((((1366 * r1 + 150889) % 714025) / 714025.0) * 32767)::bigint;
l1 := l2;
r1 := r2;
i := i + 1;
END LOOP;
return ((r1 << 16) + l1);
END;
$$ LANGUAGE plpgsql strict immutable;
            "#,
        ))
        .await?;

        // Create the frequency enum type first
        manager
            .create_type(
                extension::postgres::Type::create()
                    .as_enum(Alias::new("frequency"))
                    .values([
                        Alias::new("Yearly"),
                        Alias::new("Monthly"),
                        Alias::new("Weekly"),
                        Alias::new("Daily"),
                        Alias::new("Hourly"),
                        Alias::new("Minutely"),
                        Alias::new("Secondly"),
                    ])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(schema.create_table_from_entity(resource_types::Entity))
            .await?;
        for statement in default_id_statement("resource_types", "resource_type_id") {
            db.execute(Statement::from_string(db.get_database_backend(), statement))
                .await?;
        }
        resource_types::Entity::insert_many([
            resource_types::ActiveModel {
                name: Set("Personnel".to_string()),
                ..Default::default()
            },
            resource_types::ActiveModel {
                name: Set("Material".to_string()),
                ..Default::default()
            },
            resource_types::ActiveModel {
                name: Set("Equipment".to_string()),
                ..Default::default()
            },
            resource_types::ActiveModel {
                name: Set("Service".to_string()),
                ..Default::default()
            },
            resource_types::ActiveModel {
                name: Set("Other".to_string()),
                ..Default::default()
            },
        ])
        .exec(db)
        .await?;

        manager
            .create_table(schema.create_table_from_entity(resources::Entity))
            .await?;
        for statement in default_id_statement("resources", "resource_id") {
            db.execute(Statement::from_string(db.get_database_backend(), statement))
                .await?;
        }

        manager
            .create_table(schema.create_table_from_entity(baselines::Entity))
            .await?;
        for statement in default_id_statement("baselines", "baseline_id") {
            db.execute(Statement::from_string(db.get_database_backend(), statement))
                .await?;
        }

        manager
            .create_type(
                extension::postgres::Type::create()
                    .as_enum(Alias::new("task_status"))
                    .values([
                        Alias::new("ToDo"),
                        Alias::new("InProgress"),
                        Alias::new("Done"),
                        Alias::new("Cancelled"),
                    ])
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(schema.create_table_from_entity(tasks::Entity))
            .await?;
        for statement in default_id_statement("tasks", "task_id") {
            db.execute(Statement::from_string(db.get_database_backend(), statement))
                .await?;
        }

        manager
            .create_table(schema.create_table_from_entity(tasks_baselines::Entity))
            .await?;
        for statement in default_id_statement("tasks_baselines", "task_baseline_id") {
            db.execute(Statement::from_string(db.get_database_backend(), statement))
                .await?;
        }

        manager
            .create_table(schema.create_table_from_entity(config::Entity))
            .await?;
        for statement in default_id_statement("config", "config_id") {
            db.execute(Statement::from_string(db.get_database_backend(), statement))
                .await?;
        }

        baselines::Entity::insert_many([baselines::ActiveModel {
            baseline_id: Set(1),
            name: Set("Current".to_string()),
            ..Default::default()
        }])
        .exec(db)
        .await?;

        config::Entity::insert_many([
            config::ActiveModel {
                config_key: Set("baseline_id_default".to_string()),
                config_value: Set(None),
                ..Default::default()
            },
            config::ActiveModel {
                config_key: Set("baseline_id_current".to_string()),
                config_value: Set(Some("1".to_string())),
                ..Default::default()
            },
            config::ActiveModel {
                config_key: Set("timezone".to_string()),
                config_value: Set(Some("Europe/Warsaw".to_string())),
                ..Default::default()
            },
        ])
        .exec(db)
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        match manager.get_database_backend() {
            DatabaseBackend::Postgres => {
                // Using execute_unprepared instead of exec_stmt
                manager
                    .get_connection()
                    .execute_unprepared("DROP SCHEMA public CASCADE; CREATE SCHEMA public;")
                    .await?;
            }
            _ => {
                error!("Down migration not supported for this database backend");
                exit(1);
            }
        }

        Ok(())
    }
}

fn default_id_statement(table_name: &str, column_name: &str) -> Vec<String> {
    vec![
        format!("CREATE SEQUENCE {column_name}_seq;"),
        format!("ALTER TABLE {table_name} ALTER COLUMN {column_name} SET DEFAULT pseudo_encrypt(nextval('{column_name}_seq'));"),
        format!("ALTER SEQUENCE {column_name}_seq OWNED BY {table_name}.{column_name};"),
    ]
}
