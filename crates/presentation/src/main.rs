mod startup;
mod controllers;

use crate::startup::build_rocket;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let rocket = build_rocket();
    rocket.launch().await?;
    Ok(())
}
