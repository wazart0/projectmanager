pub const COLUMNS: [&str; 10] = [
    "id",
    "wbs",
    "name",
    "description",
    "parent",
    "begin_month",
    "end_month",
    "planned_work_pm",
    "planned_team_cost_eur",
    "planned_other_cost_eur",
];

#[derive(
    bitcode::Encode, bitcode::Decode, serde::Deserialize, serde::Serialize, Clone, PartialEq, Debug,
)]
pub struct Task {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub wbs: String,
    pub parent: Option<i64>,
    pub begin_month: Option<i32>,
    pub end_month: Option<i32>,
    pub planned_work_pm: Option<i32>,
    pub planned_team_cost_eur: Option<f64>,
    pub planned_other_cost_eur: Option<f64>,
}

#[derive(
    bitcode::Encode, bitcode::Decode, serde::Deserialize, serde::Serialize, Clone, PartialEq, Debug,
)]
pub struct TaskDetails {
    pub id: i64,
    pub wbs: String,
    pub parent: i64,
    pub begin_month: i32,
    pub end_month: i32,
}

#[derive(
    bitcode::Encode, bitcode::Decode, serde::Deserialize, serde::Serialize, Clone, PartialEq, Debug,
)]
pub struct Baseline {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub tasks: Vec<TaskDetails>,
}

#[derive(
    bitcode::Encode, bitcode::Decode, serde::Deserialize, serde::Serialize, Clone, PartialEq, Debug,
)]
pub struct TeamMember {
    pub user_id: i64,
    pub user_name: String,
    pub user_last_name: String,
    pub position: Option<String>,
    pub comment: Option<String>,
}

#[derive(
    bitcode::Encode, bitcode::Decode, serde::Deserialize, serde::Serialize, Clone, PartialEq, Debug,
)]
pub struct Config {
    pub currency: String,
    pub avg_hours_per_month: i32,
}

#[derive(
    bitcode::Encode, bitcode::Decode, serde::Deserialize, serde::Serialize, Clone, PartialEq, Debug,
)]
pub struct ProjectInfo {
    pub project_name: String,
    pub project_start: String,
    pub project_finish: String,
    pub config: Config,
}

#[derive(
    bitcode::Encode, bitcode::Decode, serde::Deserialize, serde::Serialize, Clone, PartialEq, Debug,
)]
pub enum Frequency {
    Yearly,
    Monthly,
    Weekly,
    Daily,
    Hourly,
    Minutely,
    Secondly,
}

impl std::fmt::Display for Frequency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Frequency::Yearly => write!(f, "Yearly"),
            Frequency::Monthly => write!(f, "Monthly"),
            Frequency::Weekly => write!(f, "Weekly"),
            Frequency::Daily => write!(f, "Daily"),
            Frequency::Hourly => write!(f, "Hourly"),
            Frequency::Minutely => write!(f, "Minutely"),
            Frequency::Secondly => write!(f, "Secondly"),
        }
    }
}

pub const RESOURCE_COLUMNS: [&str; 13] = [
    "resource_id",
    "name",
    "resource_type_id",
    "description",
    "comment",
    "cost",
    "cost_currency",
    "billing_frequency",
    "billing_interval",
    "availability",
    "capacity",
    "capacity_unit",
    "is_active",
];

#[derive(
    bitcode::Encode, bitcode::Decode, serde::Deserialize, serde::Serialize, Clone, PartialEq, Debug,
)]
pub struct Resource {
    pub resource_id: u64,
    pub name: String,
    pub resource_type_id: u64,
    pub description: Option<String>,
    pub comment: Option<String>,
    pub cost: Option<f64>,
    pub cost_currency: String,
    pub billing_frequency: Option<Frequency>,
    pub billing_interval: Option<i32>,
    pub availability: Option<String>, // TODO: placeholder, modify to reference to calendar
    pub capacity: Option<i32>,
    pub capacity_unit: Option<String>,
    pub is_active: bool,
}

pub const RESOURCE_TYPE_COLUMNS: [&str; 4] = ["resource_type_id", "name", "description", "comment"];

#[derive(
    bitcode::Encode, bitcode::Decode, serde::Deserialize, serde::Serialize, Clone, PartialEq, Debug,
)]
pub struct ResourceType {
    pub resource_type_id: u64,
    pub name: String,
    pub description: Option<String>,
    pub comment: Option<String>,
}
