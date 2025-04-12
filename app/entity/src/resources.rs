use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(Some(20))")]
pub enum CostCalculationMethod {
    #[sea_orm(string_value = "OneTime")]
    OneTime,
    #[sea_orm(string_value = "Recurring")]
    Recurring,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "resources")]
pub struct Model {
    // ... existing code ...
    pub cost: f64,
    pub cost_currency: String,
    pub cost_calculation_method: CostCalculationMethod,
}

// ... existing code ...