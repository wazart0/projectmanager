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
