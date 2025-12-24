use std::time::Duration;

use sqlx::{PgPool, postgres::PgPoolOptions};

// We never create DB Connection per request
pub async fn create_pool(database_url:&str)->PgPool{
    PgPoolOptions::new()
        .max_connections(25)
        .acquire_timeout(Duration::from_secs(3))
        .connect(database_url)
        .await
        .expect("Failed to create Postgres Pool")
}