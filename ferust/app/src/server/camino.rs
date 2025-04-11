use std::fs;

// use tokio::io::*;

// const APP_PATH: &str = "/app";

const LIST_OF_TASKS_PATH: &str = "/app/local/tmp/list_of_tasks.json";
const PROJECT_INFO_PATH: &str = "/app/local/tmp/project_info.json";




pub async fn get_list_of_tasks() -> Option<String> {
    let data = fs::read_to_string(LIST_OF_TASKS_PATH).unwrap();
    Some(data)
}

pub async fn get_project_info() -> Option<String> {
    let data = fs::read_to_string(PROJECT_INFO_PATH).unwrap();
    Some(data)
}
