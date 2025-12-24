use std::sync::Arc;

use actix_web::{App, HttpServer, web};

use crate::{adaptors::{InMemoryRepo, PostgresUrlRepository, RedisRepository, create, resolve}, app_state::AppState, application::{UrlCache, UrlService}, infrastructure::create_pool};

mod app_state;
mod application;
mod domain;
mod adaptors;
mod infrastructure;

#[actix_web::main]
async fn main()->std::io::Result<()>{

    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").unwrap_or("abcd".to_string());
    let redis_url = std::env::var("REDIS_URL").expect("REDIS URL must be set");

    let pool = create_pool(&database_url).await;

    // creating shared application state
    let repo = Arc::new(PostgresUrlRepository::new(pool));
    let cache = Arc::new(RedisRepository::new(&redis_url));
    let service = Arc::new(UrlService::new(repo,cache));

    let state = web::Data::new(AppState{
        url_service:service
    });

    HttpServer::new(move || {
            App::new()
            // Injecting shared state into the app
            .app_data(state.clone())
            .route("/shorten", web::post().to(create))
            .route("/{code}", web::get().to(resolve))  
    })
    .bind("localhost:3000")?
    .run()
    .await
}