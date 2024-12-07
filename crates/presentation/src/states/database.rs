use diesel::{r2d2::{ConnectionManager, Pool}, PgConnection};
use infrastructure::database::context::context;

pub struct DatabaseState {
  context: Pool<ConnectionManager<PgConnection>>
}

impl DatabaseState {
  pub fn new() -> Self {
    DatabaseState { context: context() }
  }
}