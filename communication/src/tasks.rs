use struct_field_names_as_array::FieldNamesAsArray;

#[derive(
    bitcode::Encode, bitcode::Decode, serde::Deserialize, serde::Serialize, Clone, PartialEq, Debug,
)]
pub enum TaskStatus {
    ToDo,
    InProgress,
    Done,
    Cancelled,
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskStatus::ToDo => write!(f, "ToDo"),
            TaskStatus::InProgress => write!(f, "InProgress"),
            TaskStatus::Done => write!(f, "Done"),
            TaskStatus::Cancelled => write!(f, "Cancelled"),
        }
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
pub struct Task {
    pub task_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub comment: Option<String>,
    pub status: TaskStatus,
}

impl Task {
    pub fn fields() -> [&'static str; 5] {
        Task::FIELD_NAMES_AS_ARRAY
    }
}
