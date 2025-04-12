use axum::{routing::get, Router, response::{Response, IntoResponse, Json}, http::StatusCode, extract::State};
use sea_orm::{Database, DatabaseConnection, Statement, ConnectionTrait, EntityTrait};
use migration::{Migrator, MigratorTrait};
use serde_json::{json, Value};
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use tracing::{info, error, warn, debug, trace};
// use dioxus::prelude::*;
use std::fs;
use tower_http::cors::{Any, CorsLayer};


use common::*;
use entity::*;



#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // Read database connection string from environment variable
    let db_connection_string = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| {
            error!("DATABASE_URL environment variable not set, using default");
            std::process::exit(1);
        });

    // Initialize database connection
    let db_connection = Database::connect(db_connection_string)
        .await
        .unwrap_or_else(|e| {
            error!("Failed to create DB connection: {}", e);
            std::process::exit(1);
        });

    // Verify database connection
    match db_connection.execute(Statement::from_string(
        db_connection.get_database_backend(),
        "SELECT 1".to_owned(),
    )).await {
        Ok(_) => info!("Database connection verified"),
        Err(e) => {
            error!("Failed to connect to database: {}", e);
            std::process::exit(1);
        }
    }

    match Migrator::up(&db_connection, None).await {
        Ok(_) => info!("Database migrations completed"),
        Err(e) => {
            error!("Failed to migrate database: {}", e);
            std::process::exit(1);
        }
    }

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3333")
        .await
        .unwrap_or_else(|e| {
            error!("Failed to bind to port 3333: {}", e);
            std::process::exit(1);
        });

    info!("listening on http://0.0.0.0:3333");

    // Configure CORS to allow all origins with a wildcard
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
        
    // Build the application with routes and middleware
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/tasks", get(get_list_of_tasks))
        .route("/project", get(get_project_info))
        .route("/resources", get(get_resources))
        .layer(cors)
        .with_state(db_connection);  // Add database connection to application state
        
    axum::serve(listener, app).await.unwrap_or_else(|e| {
        error!("Server error: {}", e);
        std::process::exit(1);
    });
}

async fn health_check(State(db): State<DatabaseConnection>) -> Json<Value> {
    let start = Instant::now();
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    // Check database connectivity
    let db_status = match db.execute(Statement::from_string(
        db.get_database_backend(),
        "SELECT 1 as connected".to_owned(),
    )).await {
        Ok(_) => {
            let db_ping_us = start.elapsed().as_micros();
            json!({
                "status": "connected",
                "ping_us": db_ping_us
            })
        },
        Err(err) => {
            warn!("Database health check failed: {}", err);
            json!({
                "status": "disconnected",
                "error": err.to_string()
            })
        }
    };
    
    // Overall service status depends on database status
    let service_status = if db_status["status"] == "connected" { "healthy" } else { "degraded" };
    
    // Return JSON with service and database status
    Json(json!({
        "status": service_status,
        "database": db_status,
        "version": env!("CARGO_PKG_VERSION"),
        "timestamp": timestamp
    }))
}





enum MyError {
    FileDoesntExists,
    JsonParserError,
    DatabaseError,
}

impl IntoResponse for MyError {
    fn into_response(self) -> Response {
        let body = match self {
            MyError::FileDoesntExists => "Server error: File doesn't exists",
            MyError::JsonParserError => "Server error: Json parser error",
            MyError::DatabaseError => "Server error: Database operation failed",
        };

        error!("{}, {}", StatusCode::INTERNAL_SERVER_ERROR, body);
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}







const LIST_OF_TASKS_PATH: &str = "/app/local/tmp/list_of_tasks.json";
const PROJECT_INFO_PATH: &str = "/app/local/tmp/project_info.json";



async fn get_list_of_tasks() -> Result<Vec<u8>, MyError> {
    let data = match fs::read_to_string(LIST_OF_TASKS_PATH) {
        Ok(data) => data,
        Err(_) => {
            tracing::error!("File doesn't exists");
            return Err(MyError::FileDoesntExists);
        }
    };
    match serde_json::from_str::<Vec<common::models::Task>>(&data) {
        Ok(tasks) => Ok(bitcode::encode(&tasks)),
        Err(_) => {
            tracing::error!("Json parser error");
            return Err(MyError::JsonParserError);
        },
    }
}


// async fn get_list_of_team_members() -> Result<Vec<u8>, MyError> {
//     let data = match fs::read_to_string(LIST_OF_TASKS_PATH) {
//         Ok(data) => data,
//         Err(_) => {
//             tracing::error!("File doesn't exists");
//             return Err(MyError::FileDoesntExists);
//         }
//     };
//     match serde_json::from_str::<Vec<common::models::TeamMember>>(&data) {
//         Ok(tasks) => Ok(bitcode::encode(&tasks)),
//         Err(_) => {
//             tracing::error!("Json parser error");
//             return Err(MyError::JsonParserError);
//         },
//     }
// }



async fn get_project_info() -> Vec<u8> {
    let data = fs::read_to_string(PROJECT_INFO_PATH).unwrap();
    let project_info: common::models::ProjectInfo = serde_json::from_str(&data).unwrap();
    bitcode::encode(&project_info)
}



// async fn app_endpoint() -> Html<String> {
//     // render the rsx! macro to HTML
//     Html(dioxus_ssr::render_element(rsx! { div { "hello world!" } }))
// }

// async fn app_endpoint() -> Html<String> {
//     // create a component that renders a div with the text "hello world"
//     fn app() -> Element {
//         rsx! { div { "hello world" } }
//     }
//     // create a VirtualDom with the app component
//     let mut app = VirtualDom::new(app);
//     // rebuild the VirtualDom before rendering
//     app.rebuild_in_place();

//     // render the VirtualDom to HTML
//     Html(dioxus_ssr::render(&app))
// }



// Define a local trait for frequency conversion
trait IntoModelFrequency {
    fn into_model_frequency(self) -> models::Frequency;
}

// Implement the trait for resources::Frequency
impl IntoModelFrequency for resources::Frequency {
    fn into_model_frequency(self) -> models::Frequency {
        match self {
            resources::Frequency::Monthly => models::Frequency::Monthly,
            resources::Frequency::Weekly => models::Frequency::Weekly,
            resources::Frequency::Daily => models::Frequency::Daily,
            resources::Frequency::Hourly => models::Frequency::Hourly,
            resources::Frequency::Minutely => models::Frequency::Minutely,
            resources::Frequency::Secondly => models::Frequency::Secondly,
            resources::Frequency::Yearly => models::Frequency::Yearly,
        }
    }
}


trait IntoModelResource {
    fn into_model_resource(self) -> common::models::Resource;
}

impl IntoModelResource for resources::Model {
    fn into_model_resource(self) -> common::models::Resource {
        common::models::Resource {
            resource_id: self.resource_id,
            name: self.name,
            resource_type_id: self.resource_type_id,
            description: self.description,
            comment: self.comment,
            cost: self.cost,
            cost_currency: self.cost_currency,
            billing_frequency: self.billing_frequency.map(|f| f.into_model_frequency()),
            billing_interval: self.billing_interval,
            availability: self.availability,
            capacity: self.capacity,
            capacity_unit: self.capacity_unit,
            is_active: self.is_active,
        }
    }
}

trait IntoModelResourceType {
    fn into_model_resource_type(self) -> common::models::ResourceType;
}

impl IntoModelResourceType for resource_types::Model {
    fn into_model_resource_type(self) -> common::models::ResourceType {
        common::models::ResourceType {
            resource_type_id: self.resource_type_id,
            name: self.name,
            description: self.description,
            comment: self.comment,
        }
    }
}

// New function to get team members from the database using SeaORM
async fn get_resources(State(db): State<DatabaseConnection>) -> Result<Vec<u8>, MyError> {
    // Use the SeaORM query builder
    let resources = resources::Entity::find()
        .all(&db)
        .await
        .map_err(|_| MyError::DatabaseError)?;
    let resource_types = resource_types::Entity::find()
        .all(&db)
        .await
        .map_err(|_| MyError::DatabaseError)?;
    
    // Convert from SeaORM model to the application model
    let resources: Vec<common::models::Resource> = resources.into_iter()
        .map(|record| record.into_model_resource())
        .collect();
    let resource_types: Vec<common::models::ResourceType> = resource_types.into_iter()
        .map(|record| record.into_model_resource_type())
        .collect();
    
    Ok(bitcode::encode(&(resources, resource_types)))
}

