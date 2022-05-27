use crate::AppState;
use actix_web::web::{Data, Json};
use actix_web::{get, Responder, Result};
use chrono::Utc;
use serde::Serialize;

#[derive(Serialize)]
struct Status {
    name: String,
    version: String,
    start_utc: String,
    uptime: i64,
}

#[get("/status")]
pub(crate) async fn get_status(data: Data<AppState>) -> Result<impl Responder> {
    let now = Utc::now();
    let uptime_in_sec = now.timestamp() - data.start_time.timestamp();

    let status = Status {
        name: data.name.clone(),
        version: data.version.clone(),
        start_utc: data.start_time.to_rfc3339(),
        uptime: uptime_in_sec,
    };
    Ok(Json(status))
}
