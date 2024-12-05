use rocket::{routes, Rocket};
use crate::controllers::ping_controller::index;

pub fn build_rocket() -> Rocket<rocket::Build> {
  rocket::build()
    .mount("/api/ping", routes![index])
}