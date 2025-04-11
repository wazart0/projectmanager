
pub mod models {

    pub const COLUMNS: [&str; 10] = ["id", "wbs", "name", "description", "parent", "begin_month", "end_month", "planned_work_pm", "planned_team_cost_eur", "planned_other_cost_eur"];


    #[derive(bitcode::Encode, bitcode::Decode, 
        serde::Deserialize, serde::Serialize, 
        Clone, PartialEq, Debug)]
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

    #[derive(bitcode::Encode, bitcode::Decode, 
        serde::Deserialize, serde::Serialize, 
        Clone, PartialEq, Debug)]
    pub struct TaskDetails {
        pub id: i64,
        pub wbs: String,
        pub parent: i64,
        pub begin_month: i32,
        pub end_month: i32,
    }

    #[derive(bitcode::Encode, bitcode::Decode, 
        serde::Deserialize, serde::Serialize, 
        Clone, PartialEq, Debug)]
    pub struct Baseline {
        pub id: i64,
        pub name: String,
        pub description: String,
        pub tasks: Vec<TaskDetails>,
    }

    #[derive(bitcode::Encode, bitcode::Decode, 
        serde::Deserialize, serde::Serialize, 
        Clone, PartialEq, Debug)]
    pub struct TeamMember {
        pub user_id: i64,
        pub user_name: String,
        pub user_last_name: String,
        pub position: Option<String>,
        pub comment: Option<String>,
    }



    #[derive(bitcode::Encode, bitcode::Decode, 
        serde::Deserialize, serde::Serialize, 
        Clone, PartialEq, Debug)]
    pub struct Config {
        pub currency: String,
        pub avg_hours_per_month: i32,
    }


    #[derive(bitcode::Encode, bitcode::Decode, 
        serde::Deserialize, serde::Serialize, 
        Clone, PartialEq, Debug)]
    pub struct ProjectInfo {
        pub project_name: String,
        pub project_start: String,
        pub project_finish: String,
        pub config: Config,
    }


}