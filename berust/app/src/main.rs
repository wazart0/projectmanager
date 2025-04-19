use axum::{
    Router,
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Json, Response},
    routing::get,
};
use migration::{Migrator, MigratorTrait};
use sea_orm::{
    ColumnTrait, ConnectionTrait, Database, DatabaseConnection, EntityTrait, FromQueryResult,
    QueryFilter, QuerySelect, RelationTrait, Statement,
};
use serde_json::{Value, json};
use std::collections::HashMap;
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use tracing::{error, info, warn};
// use dioxus::prelude::*;
use axum::extract::Request;
use axum::middleware::{self, Next};
use serde::{Deserialize, Serialize};
use std::fs;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // Read database connection string from environment variable
    let db_connection_string = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        panic!("DATABASE_URL environment variable not set, using default");
    });

    // Initialize database connection
    let db_connection = Database::connect(db_connection_string)
        .await
        .unwrap_or_else(|e| {
            panic!("Failed to create DB connection: {}", e);
        });

    // Verify database connection
    match db_connection
        .execute(Statement::from_string(
            db_connection.get_database_backend(),
            "SELECT 1".to_owned(),
        ))
        .await
    {
        Ok(_) => info!("Database connection verified"),
        Err(e) => {
            panic!("Failed to connect to database: {}", e);
        }
    }

    match Migrator::up(&db_connection, None).await {
        Ok(_) => info!("Database migrations completed"),
        Err(e) => {
            panic!("Failed to migrate database: {}", e);
        }
    }

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3333")
        .await
        .unwrap_or_else(|e| {
            panic!("Failed to bind to port 3333: {}", e);
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
        .route("/resources", get(get_resources))
        .route("/resources/allocation", get(get_resource_allocation))
        .layer(
            ServiceBuilder::new()
                .layer(middleware::from_fn(log_requests))
                .layer(cors),
        )
        .with_state(db_connection); // Add database connection to application state

    axum::serve(listener, app).await.unwrap_or_else(|e| {
        panic!("Server error: {}", e);
    });
}

// Middleware function to log request details
async fn log_requests(req: Request, next: Next) -> Response {
    let start = Instant::now();
    let method = req.method().clone();
    let uri = req.uri().clone();

    // Process the request
    let response = next.run(req).await;

    let duration = start.elapsed();
    let status = response.status();

    // Log the details
    info!(
        method = %method,
        uri = %uri,
        status = %status,
        duration = ?duration,
        "Processed request"
    );

    response
}

async fn health_check(State(db): State<DatabaseConnection>) -> Json<Value> {
    let start = Instant::now();
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // Check database connectivity
    let db_status = match db
        .execute(Statement::from_string(
            db.get_database_backend(),
            "SELECT 1 as connected".to_owned(),
        ))
        .await
    {
        Ok(_) => {
            let db_ping_us = start.elapsed().as_micros();
            json!({
                "status": "connected",
                "ping_us": db_ping_us
            })
        }
        Err(err) => {
            warn!("Database health check failed: {}", err);
            json!({
                "status": "disconnected",
                "error": err.to_string()
            })
        }
    };

    // Overall service status depends on database status
    let service_status = if db_status["status"] == "connected" {
        "healthy"
    } else {
        "degraded"
    };

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
// const PROJECT_INFO_PATH: &str = "/app/local/tmp/project_info.json";

async fn get_list_of_tasks() -> Result<Vec<u8>, MyError> {
    let data = match fs::read_to_string(LIST_OF_TASKS_PATH) {
        Ok(data) => data,
        Err(_) => {
            tracing::error!("File doesn't exists");
            return Err(MyError::FileDoesntExists);
        }
    };
    match serde_json::from_str::<Vec<communication::models::Task>>(&data) {
        Ok(tasks) => Ok(bitcode::encode(&tasks)),
        Err(_) => {
            tracing::error!("Json parser error");
            Err(MyError::JsonParserError)
        }
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
//     match serde_json::from_str::<Vec<communication::models::TeamMember>>(&data) {
//         Ok(tasks) => Ok(bitcode::encode(&tasks)),
//         Err(_) => {
//             tracing::error!("Json parser error");
//             return Err(MyError::JsonParserError);
//         },
//     }
// }

// async fn get_project_info() -> Vec<u8> {
//     let data = fs::read_to_string(PROJECT_INFO_PATH).unwrap();
//     let project_info: communication::models::ProjectInfo = serde_json::from_str(&data).unwrap();
//     bitcode::encode(&project_info)
// }

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
    fn into_model_frequency(self) -> communication::resources::Frequency;
}

// Implement the trait for resources::Frequency
impl IntoModelFrequency for entity::resources::Frequency {
    fn into_model_frequency(self) -> communication::resources::Frequency {
        match self {
            entity::resources::Frequency::Monthly => communication::resources::Frequency::Monthly,
            entity::resources::Frequency::Weekly => communication::resources::Frequency::Weekly,
            entity::resources::Frequency::Daily => communication::resources::Frequency::Daily,
            entity::resources::Frequency::Hourly => communication::resources::Frequency::Hourly,
            entity::resources::Frequency::Minutely => communication::resources::Frequency::Minutely,
            entity::resources::Frequency::Secondly => communication::resources::Frequency::Secondly,
            entity::resources::Frequency::Yearly => communication::resources::Frequency::Yearly,
        }
    }
}

trait IntoModelResource {
    fn into_model_resource(self) -> communication::resources::Resource;
}

impl IntoModelResource for entity::resources::Model {
    fn into_model_resource(self) -> communication::resources::Resource {
        communication::resources::Resource {
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
    fn into_model_resource_type(self) -> communication::resources::ResourceType;
}

impl IntoModelResourceType for entity::resource_types::Model {
    fn into_model_resource_type(self) -> communication::resources::ResourceType {
        communication::resources::ResourceType {
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
    let resources = entity::resources::Entity::find()
        .all(&db)
        .await
        .map_err(|_| MyError::DatabaseError)?;
    let resource_types = entity::resource_types::Entity::find()
        .all(&db)
        .await
        .map_err(|_| MyError::DatabaseError)?;

    // Convert from SeaORM model to the application model
    let resources: Vec<communication::resources::Resource> = resources
        .into_iter()
        .map(|record| record.into_model_resource())
        .collect();
    let resource_types: Vec<communication::resources::ResourceType> = resource_types
        .into_iter()
        .map(|record| record.into_model_resource_type())
        .collect();

    Ok(bitcode::encode(&(resources, resource_types)))
}

// Define a struct to hold the joined query results
#[derive(FromQueryResult, Debug, Serialize)]
struct ResourceAllocationDetails {
    // Fields from resources_baselines
    baseline_id: i64,
    resource_id: i64,
    task_id: i64,
    capacity_allocated: Option<f64>,
    // Add other necessary fields from resources_baselines here...
    // e.g., allocation_percentage: Option<Decimal>,

    // Fields from joined tables (using aliases defined in column_as)
    resource_name: Option<String>, // Use Option<> because of LEFT JOIN
    task_summary: Option<String>,  // Use Option<> because of LEFT JOIN
}

async fn get_resource_allocation(
    State(db): State<DatabaseConnection>,
    Query(query): Query<HashMap<String, String>>,
) -> Result<Json<Value>, MyError> {
    // Return JSON for better structure
    // Safely parse the baseline_id from the query parameters
    let baseline_id_str = query.get("baseline_id").ok_or_else(|| {
        warn!("Missing 'baseline_id' query parameter");
        // Consider a more specific error type like BadRequest
        MyError::JsonParserError // Reusing for simplicity, ideally a better error
    })?;

    let baseline_id: i64 = baseline_id_str.parse().map_err(|e| {
        warn!("Invalid 'baseline_id' format: {}", e);
        // Consider a more specific error type like BadRequest
        MyError::JsonParserError // Reusing for simplicity
    })?;

    let resource_allocations = entity::resources_baselines::Entity::find()
        .filter(entity::resources_baselines::Column::BaselineId.eq(baseline_id))
        // Select columns from the primary table (resources_baselines)
        .column(entity::resources_baselines::Column::BaselineId)
        .column(entity::resources_baselines::Column::ResourceId)
        .column(entity::resources_baselines::Column::TaskId)
        .column(entity::resources_baselines::Column::CapacityAllocated)
        // Add other columns from resources_baselines as needed
        // .column(entity::resources_baselines::Column::AllocationPercentage)
        // Select and alias columns from joined tables
        .column_as(entity::resources::Column::Name, "resource_name")
        .column_as(entity::tasks::Column::Summary, "task_summary")
        // Perform the joins
        .left_join(entity::resources::Entity)
        .left_join(entity::tasks::Entity)
        // Map the results into our custom struct
        .into_model::<ResourceAllocationDetails>()
        .all(&db)
        .await
        .map_err(|db_err| {
            // Log the actual database error
            error!("Database error fetching resource allocations: {}", db_err);
            MyError::DatabaseError // Return the generic error type
        })?;

    // Return the results as JSON
    Ok(Json(json!(resource_allocations)))
}
