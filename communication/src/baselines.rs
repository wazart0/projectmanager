use struct_field_names_as_array::FieldNamesAsArray;

#[derive(
    bitcode::Encode,
    bitcode::Decode,
    serde::Deserialize,
    serde::Serialize,
    Clone,
    PartialEq,
    Debug,
    FieldNamesAsArray,
)]
pub struct Baseline {
    pub baseline_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub comment: Option<String>,
    pub is_active: bool,
}

impl Baseline {
    pub fn fields() -> [&'static str; 5] {
        Baseline::FIELD_NAMES_AS_ARRAY
    }
}

#[derive(
    bitcode::Encode,
    bitcode::Decode,
    serde::Deserialize,
    serde::Serialize,
    Clone,
    PartialEq,
    Debug,
    FieldNamesAsArray,
)]
#[cfg_attr(feature = "pmbe", derive(sea_orm::FromQueryResult))]
pub struct ResourceAllocation {
    pub resource_baseline_id: i64,
    pub baseline_id: i64,
    pub resource_id: i64,
    pub task_id: i64,
    pub resource_summary: Option<String>,
    pub task_summary: Option<String>,
    pub capacity_allocated: Option<f64>,
    pub capacity: Option<f64>,
    pub capacity_unit: Option<String>,
}

impl ResourceAllocation {
    pub fn fields() -> [&'static str; 9] {
        ResourceAllocation::FIELD_NAMES_AS_ARRAY
    }
}

// #[cfg(feature = "pmbe")]
// pub mod pmbe_impl {
//     use super::ResourceAllocation;
//     use sea_orm::{DbErr, FromQueryResult, QueryResult};

//     impl FromQueryResult for ResourceAllocation {
//         fn from_query_result(res: &QueryResult, pre: &str) -> Result<Self, DbErr> {
//             Ok(Self {
//                 resource_baseline_id: res.try_get(pre, "resource_baseline_id")?,
//                 baseline_id: res.try_get(pre, "baseline_id")?,
//                 resource_id: res.try_get(pre, "resource_id")?,
//                 task_id: res.try_get(pre, "task_id")?,
//                 capacity_allocated: res.try_get(pre, "capacity_allocated")?,
//                 resource_summary: res.try_get(pre, "resource_summary")?,
//                 task_summary: res.try_get(pre, "task_summary")?,
//                 capacity: res.try_get(pre, "capacity")?,
//                 capacity_unit: res.try_get(pre, "capacity_unit")?,
//             })
//         }
//     }
// }
