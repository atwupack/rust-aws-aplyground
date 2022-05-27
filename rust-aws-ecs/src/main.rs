use actix_web::web::{scope, Data};
use actix_web::{App, HttpServer};
use chrono::{DateTime, Utc};
use config::{Config, Environment, File};
use std::error::Error;

mod service;

use crate::service::status::get_status;

struct AppState {
    name: String,
    version: String,
    start_time: DateTime<Utc>,
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::builder()
        .add_source(File::with_name("application"))
        .add_source(Environment::with_prefix("APP"))
        .build()?;

    let app_state = Data::new(AppState {
        name: "AWS ECS test".to_string(),
        version: "1.0".to_string(),
        start_time: Utc::now(),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(scope("/api/v1").service(get_status))
    })
    .bind((
        config.get_string("server.address").unwrap_or("127.0.0.1".to_string()),
        config.get::<u16>("server.port")?
    ))?
    .run()
    .await?;

    Ok(())
}
