pub use sea_orm_migration::prelude::*;

mod m20250410_000001_init;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250410_000001_init::Migration),
        ]
    }
}
