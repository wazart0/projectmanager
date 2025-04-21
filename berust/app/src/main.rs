use actix_cors::Cors;
use actix_web::http::StatusCode;
use actix_web::{
    App, HttpResponse, HttpServer, Responder, ResponseError, Result, middleware::Logger, web,
};

use migration::{Migrator, MigratorTrait};
use sea_orm::{
    ColumnTrait, ConnectionTrait, Database, DatabaseConnection, EntityTrait, QueryFilter,
    QuerySelect, Statement,
};
use serde_json::json;
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::fs;
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use tracing::{error, info, warn};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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

    info!("starting server on http://0.0.0.0:3333");
    // Database connection needs to be cloned for each worker
    let db_data = web::Data::new(db_connection);

    // Build the application with routes and middleware using Actix-web
    HttpServer::new(move || {
        // Configure CORS to allow all origins with a wildcard using actix-cors
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .app_data(db_data.clone())
            .wrap(cors)
            .wrap(Logger::default())
            .service(web::resource("/health").route(web::get().to(health_check)))
            .service(web::resource("/tasks").route(web::get().to(get_list_of_tasks)))
            .service(web::resource("/resources").route(web::get().to(get_resources)))
            .service(
                web::resource("/resources/allocation")
                    .route(web::get().to(get_resource_allocation)),
            )
    })
    .bind(("0.0.0.0", 3333))?
    .workers(4)
    .run()
    .await
}

async fn health_check(db: web::Data<DatabaseConnection>) -> impl Responder {
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

    // Return JSON with service and database status using HttpResponse
    HttpResponse::Ok().json(json!({
        "status": service_status,
        "database": db_status,
        "version": env!("CARGO_PKG_VERSION"),
        "timestamp": timestamp
    }))
}

#[derive(Debug)]
enum MyError {
    FileDoesntExists,
    JsonParserError,
    DatabaseError,
}

impl Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            MyError::FileDoesntExists => write!(f, "Server error: File doesn't exists"),
            MyError::JsonParserError => write!(f, "Server error: Json parser error"),
            MyError::DatabaseError => write!(f, "Server error: Database operation failed"),
        }
    }
}

impl ResponseError for MyError {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> HttpResponse {
        error!("{}, {}", self.status_code(), self.to_string());
        HttpResponse::build(self.status_code()).body(self.to_string())
    }
}

const LIST_OF_TASKS_PATH: &str = "/app/local/tmp/list_of_tasks.json";

async fn get_list_of_tasks() -> Result<HttpResponse, MyError> {
    let data = match fs::read_to_string(LIST_OF_TASKS_PATH) {
        Ok(data) => data,
        Err(_) => {
            tracing::error!("File doesn't exists");
            return Err(MyError::FileDoesntExists);
        }
    };
    match serde_json::from_str::<Vec<communication::models::Task>>(&data) {
        Ok(tasks) => {
            let encoded = bitcode::encode(&tasks);
            Ok(HttpResponse::Ok()
                .content_type("application/octet-stream")
                .body(encoded))
        }
        Err(_) => {
            tracing::error!("Json parser error");
            Err(MyError::JsonParserError)
        }
    }
}

trait IntoModelFrequency {
    fn into_model_frequency(self) -> communication::resources::Frequency;
}

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
            name: self.summary,
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

async fn get_resources(db: web::Data<DatabaseConnection>) -> Result<HttpResponse, MyError> {
    let resources = entity::resources::Entity::find()
        .all(db.get_ref())
        .await
        .map_err(|e| {
            error!("Database error fetching resources: {}", e);
            MyError::DatabaseError
        })?;
    let resource_types = entity::resource_types::Entity::find()
        .all(db.get_ref())
        .await
        .map_err(|e| {
            error!("Database error fetching resource types: {}", e);
            MyError::DatabaseError
        })?;

    let resources: Vec<communication::resources::Resource> = resources
        .into_iter()
        .map(|record| record.into_model_resource())
        .collect();
    let resource_types: Vec<communication::resources::ResourceType> = resource_types
        .into_iter()
        .map(|record| record.into_model_resource_type())
        .collect();

    let encoded = bitcode::encode(&(resources, resource_types));
    Ok(HttpResponse::Ok()
        .content_type("application/octet-stream")
        .body(encoded))
}

// #[derive(FromQueryResult, Debug)]
// struct ResourceAllocationORM(communication::baselines::ResourceAllocation);

async fn get_resource_allocation(
    db: web::Data<DatabaseConnection>,
    query: web::Query<HashMap<String, String>>,
) -> Result<HttpResponse, MyError> {
    let baseline_id_str = query.get("baseline_id").ok_or_else(|| {
        warn!("Missing 'baseline_id' query parameter");
        MyError::JsonParserError
    })?;

    let baseline_id: i64 = baseline_id_str.parse().map_err(|e| {
        warn!("Invalid 'baseline_id' format: {}", e);
        MyError::JsonParserError
    })?;

    let resource_allocations = entity::resources_baselines::Entity::find()
        .filter(entity::resources_baselines::Column::BaselineId.eq(baseline_id))
        .column_as(entity::resources::Column::Summary, "resource_summary")
        .column_as(entity::tasks::Column::Summary, "task_summary")
        .column_as(entity::resources::Column::Capacity, "capacity")
        .column_as(entity::resources::Column::CapacityUnit, "capacity_unit")
        .left_join(entity::resources::Entity)
        .left_join(entity::tasks::Entity)
        .into_model::<communication::baselines::ResourceAllocation>()
        .all(db.get_ref())
        .await
        .map_err(|db_err| {
            error!("Database error fetching resource allocations: {}", db_err);
            MyError::DatabaseError
        })?;

    let encoded = bitcode::encode(&resource_allocations);
    Ok(HttpResponse::Ok()
        .content_type("application/octet-stream")
        .body(encoded))
}
