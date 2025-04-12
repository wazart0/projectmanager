use sea_orm::entity::prelude::*;


#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "resource_types")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "BigInteger", auto_increment = false)]
    pub resource_type_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub comment: Option<String>
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

