use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "task_status")]
pub enum TaskStatus {
    #[sea_orm(string_value = "ToDo")]
    ToDo,
    #[sea_orm(string_value = "InProgress")]
    InProgress,
    #[sea_orm(string_value = "Done")]
    Done,
    #[sea_orm(string_value = "Cancelled")]
    Cancelled,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "tasks")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "BigInteger", auto_increment = false)]
    pub task_id: i64,
    pub summary: String,
    pub description: Option<String>,
    pub comment: Option<String>,
    #[sea_orm(default_value = "ToDo")]
    pub status: TaskStatus,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
