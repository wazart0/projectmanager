use sea_orm::entity::prelude::*;




#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "config")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "BigInteger", auto_increment = false)]
    pub config_id: i64,
    #[sea_orm(index)]
    pub config_key: String,
    pub config_value: String,
    pub description: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

