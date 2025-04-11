


pub mod camino;
// use camino::*;





// pub fn get_team_members() -> Result<serde_json::Value, String> {
//     let mut df = get_team_members_df()?;

//     let json = df_to_str_json(&mut df)?;

//     serde_json::from_str(&json)
//         .map_err(|e| format!("Failed to parse JSON: {e}"))
// }




// pub fn get_team_cost() -> Result<serde_json::Value, String> {
//     let mut df = get_team_cost_df()?;

//     let json = df_to_str_json(&mut df)?;

//     serde_json::from_str(&json)
//         .map_err(|e| format!("Failed to parse JSON: {e}"))
// }





// pub fn get_time_report() -> Result<serde_json::Value, String> {
//     let mut df = get_time_report_df()?;

//     let json = df_to_str_json(&mut df)?;

//     serde_json::from_str(&json)
//         .map_err(|e| format!("Failed to parse JSON: {e}"))
// }   




// pub fn get_other_costs() -> Result<serde_json::Value, String> {
//     let mut df = get_other_costs_df()?
//         .collect()
//         .map_err(|e| format!("Failed to collect dataframe: {e}"))?;

//     // Parse the JSON string to a serde_json::Value
//     serde_json::from_str(&df_to_str_json(&mut df)
//             .map_err(|e| format!("Failed to convert DataFrame to JSON: {e}"))?)
//         .map_err(|e| format!("Failed to parse JSON: {e}"))
// }




// // pub fn get_project_info() -> Result<DataFrame, String> {
// //     read_json_to_df(&PROJECT_INFO)
// // }


// pub fn get_tasks() -> Result<DataFrame, String> {
//     read_json_to_df(&LIST_OF_TASKS)
// }
