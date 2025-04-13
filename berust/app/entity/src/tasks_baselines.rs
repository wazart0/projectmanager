use sea_orm::entity::prelude::*;

use crate::baselines::Entity as Baseline;
use crate::tasks::Entity as Task;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "tasks_baselines")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "BigInteger", auto_increment = false)]
    pub task_baseline_id: u64,
    #[sea_orm(index)]
    pub task_id: u64,
    #[sea_orm(index)]
    pub baseline_id: Option<u64>,
    pub wbs: String,
    #[sea_orm(index)]
    pub parent: Option<u64>,
    pub start: ChronoDateTime,
    pub start_timezone: String,
    pub finish: ChronoDateTime,
    pub finish_timezone: String,
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
        belongs_to = "super::tasks::Entity",
        from = "Column::TaskId",
        to = "super::tasks::Column::TaskId"
    )]
    Task,
    #[sea_orm(
        belongs_to = "super::tasks::Entity",
        from = "Column::TaskId",
        to = "super::tasks::Column::TaskId"
    )]
    Parent,
}

impl Related<Baseline> for Entity {
    fn to() -> RelationDef {
        Relation::Baseline.def()
    }
}

impl Related<Task> for Entity {
    fn to() -> RelationDef {
        Relation::Task.def()
    }
}

// impl Related<Task> for Entity {
//     fn to() -> RelationDef {
//         Relation::Parent.def()
//     }
// }

// impl Related<Task> for Entity {}

impl ActiveModelBehavior for ActiveModel {}
