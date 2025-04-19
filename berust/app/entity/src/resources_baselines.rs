use sea_orm::entity::prelude::*;

use crate::baselines::Entity as Baseline;
use crate::resources::Entity as Resource;
use crate::tasks::Entity as Task;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "resources_baselines")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub resource_baseline_id: i64,
    #[sea_orm(index)]
    pub resource_id: i64,
    #[sea_orm(index)]
    pub baseline_id: i64,
    #[sea_orm(index)]
    pub task_id: i64,
    pub capacity_allocated: Option<f64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::baselines::Entity",
        from = "Column::BaselineId",
        to = "super::baselines::Column::BaselineId"
    )]
    Baseline,
    #[sea_orm(
        belongs_to = "super::resources::Entity",
        from = "Column::ResourceId",
        to = "super::resources::Column::ResourceId"
    )]
    Resource,
    #[sea_orm(
        belongs_to = "super::tasks::Entity",
        from = "Column::TaskId",
        to = "super::tasks::Column::TaskId"
    )]
    Task,
}

impl Related<Baseline> for Entity {
    fn to() -> RelationDef {
        Relation::Baseline.def()
    }
}

impl Related<Resource> for Entity {
    fn to() -> RelationDef {
        Relation::Resource.def()
    }
}

impl Related<Task> for Entity {
    fn to() -> RelationDef {
        Relation::Task.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
