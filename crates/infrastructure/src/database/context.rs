use std::env;
use dotenvy::dotenv;
use diesel::r2d2;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};


pub fn context() -> Pool<ConnectionManager<PgConnection>> {
  dotenv().ok();
  let database = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

  let manager = ConnectionManager::<PgConnection>::new(&database);

  let pool = r2d2::Pool::builder()
    .max_size(15).build(manager).expect("Failed to create pool");

  return pool;
}