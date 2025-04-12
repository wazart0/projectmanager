use sea_orm_migration::prelude::*;
use sea_orm::{DatabaseBackend, Statement};
use tracing::error;
use std::process::exit;

use entity::*;


#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let schema = sea_orm::Schema::new(manager.get_database_backend());
        let db = manager.get_connection();

        db.execute(
            Statement::from_string(
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
                "#
            )
        ).await?;   

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
                        Alias::new("Secondly")
                    ])
                    .to_owned()
            )
            .await?;

        manager
            .create_table(
                schema.create_table_from_entity(team_members::Entity)
            )
            .await?;

        manager
            .create_table(
                schema.create_table_from_entity(resource_types::Entity)
            )
            .await?;
        for statement in default_id_statement("resource_types", "resource_type_id") {
            db.execute(
                Statement::from_string(
                    db.get_database_backend(),
                    statement
                )
            ).await?;
        }
        db.execute(
            Statement::from_sql_and_values(
                db.get_database_backend(),
                    r#"INSERT INTO resource_types (name) VALUES ($1), ($2), ($3), ($4), ($5)"#,
                    [
                        "Human".into(),
                        "Material".into(),
                        "Equipment".into(),
                        "Service".into(),   
                        "Other".into(),
                    ]
                )
            )
            .await?;

        manager
            .create_table(
                schema.create_table_from_entity(resources::Entity)
            )
            .await?;
        for statement in default_id_statement("resources", "resource_id") {
            db.execute(
                Statement::from_string(
                    db.get_database_backend(),
                    statement
                )
            ).await?;
        }

        

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
            },
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

