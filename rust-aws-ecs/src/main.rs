use actix_web::web::{Data, scope};
use actix_web::{App, HttpServer};
use chrono::{DateTime, Utc};

mod service;

use crate::service::status::get_status;

struct AppState {
    name: String,
    version: String,
    start_time: DateTime<Utc>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let app_state = Data::new(AppState {
        name: "AWS ECS test".to_string(),
        version: "1.0".to_string(),
        start_time: Utc::now(),
    });

    HttpServer::new(move || App::new().app_data(app_state.clone()).service(scope("/api/v1").service(get_status)))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
