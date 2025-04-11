use dioxus::prelude::*;


#[cfg(feature = "server")]
mod camino;





#[server]
pub async fn get_list_of_tasks() -> Result<String, ServerFnError> {
    match camino::get_list_of_tasks().await {
        Some(list_of_tasks) => Ok(list_of_tasks),
        None => Err(ServerFnError::ServerError("Error: Failed to get list of tasks".to_string())),
    }
}

/// Echo the user input on the server.
#[server]
pub async fn get_project_info() -> Result<String, ServerFnError> {
    match camino::get_project_info().await {
        Some(project_info) => Ok(project_info),
        None => Err(ServerFnError::ServerError("Error: Failed to get project info".to_string())),
    }
}

