use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "baselines")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub baseline_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub comment: Option<String>,
    #[sea_orm(default_value = true)]
    pub is_active: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
