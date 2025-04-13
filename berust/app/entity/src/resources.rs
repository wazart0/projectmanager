use sea_orm::entity::prelude::*;

use crate::resource_types::Entity as ResourceType;

#[derive(Clone, Debug, PartialEq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum")]
pub enum Frequency {
    #[sea_orm(string_value = "Yearly")]
    Yearly,
    #[sea_orm(string_value = "Monthly")]
    Monthly,
    #[sea_orm(string_value = "Weekly")]
    Weekly,
    #[sea_orm(string_value = "Daily")]
    Daily,
    #[sea_orm(string_value = "Hourly")]
    Hourly,
    #[sea_orm(string_value = "Minutely")]
    Minutely,
    #[sea_orm(string_value = "Secondly")]
    Secondly,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "resources")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "BigInteger", auto_increment = false)]
    pub resource_id: u64,
    pub name: String,
    #[sea_orm(column_type = "BigInteger")]
    pub resource_type_id: u64,
    pub description: Option<String>,
    pub comment: Option<String>,
    pub cost: Option<f64>,
    #[sea_orm(default_value = "USD")]
    pub cost_currency: String,
    pub billing_frequency: Option<Frequency>,
    pub billing_interval: Option<i32>,
    pub availability: Option<String>, // TODO: placeholder, modify to reference to calendar
    pub capacity: Option<i32>,
    pub capacity_unit: Option<String>,
    #[sea_orm(default_value = true)]
    pub is_active: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::resource_types::Entity",
        from = "Column::ResourceTypeId",
        to = "super::resource_types::Column::ResourceTypeId"
    )]
    ResourceType,
}

impl Related<ResourceType> for Entity {
    fn to() -> RelationDef {
        Relation::ResourceType.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
