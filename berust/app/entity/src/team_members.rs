use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "team_members")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub user_id: i64,
    pub user_name: String,
    pub user_last_name: String,
    pub position: Option<String>,
    pub comment: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
