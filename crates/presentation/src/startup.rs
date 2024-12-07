use rocket::{routes, Rocket};
use crate::{controllers::ping_controller::index, states::database::DatabaseState};

pub fn build_rocket() -> Rocket<rocket::Build> {
  rocket::build()
    .manage(DatabaseState::new())
    .mount("/api/ping", routes![index])
}