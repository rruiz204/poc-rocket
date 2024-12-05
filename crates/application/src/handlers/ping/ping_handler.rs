use super::PingResponse;

pub fn ping_handler() -> PingResponse {
  PingResponse { ping: String::from("pong") }
}