use serde::Serialize;

#[derive(Serialize)]
pub struct Product {
  pub name: String,
}