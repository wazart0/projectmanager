use sea_orm_migration::prelude::*;

use entity::*;


#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let schema = sea_orm::Schema::new(manager.get_database_backend());

        manager
            .create_table(
                schema.create_table_from_entity(team_members::Entity)
            )
            .await

    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(team_members::Entity).to_owned())
            .await
    }
}

// #[derive(DeriveIden)]
// enum Post {
//     Table,
//     Id,
//     Title,
//     Text,
// }
