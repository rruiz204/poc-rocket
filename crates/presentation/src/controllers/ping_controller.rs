use application::handlers::ping::{PingResponse, ping_handler};
use rocket::{get, serde::json::Json};

#[get("/")]
pub fn index() -> Json<PingResponse> {
  let response = ping_handler();
  Json(response)
}