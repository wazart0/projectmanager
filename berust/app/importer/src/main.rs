mod camino_importer;
use crate::camino_importer::*;
use sea_orm::Database;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let db_connection_string = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        panic!("DATABASE_URL environment variable not set, using default");
    });

    let db = Database::connect(db_connection_string).await.unwrap();
    import_project_plan(db.clone()).await.unwrap();

    todo!("Import reports");
}
