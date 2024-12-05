use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PingResponse {
  pub ping: String,
}