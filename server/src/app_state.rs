use std::env;

use diesel::{r2d2::{ConnectionManager, Pool, PooledConnection}, PgConnection};
use dotenvy::dotenv;

pub type PgPooledConn = PooledConnection<ConnectionManager<PgConnection>>;
pub type PgPool = Pool<ConnectionManager<PgConnection>>;


#[derive(Clone, Debug)]
pub struct AppState {
    // pub db_pool: Pool<ConnectionManager<PgConnection>>,
	pub db_pool: PgPool,
}

impl AppState {
	pub fn new() -> Self {
		dotenv().ok();
		let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

		let manager = ConnectionManager::<PgConnection>::new(database_url);
    	let pool = Pool::builder().build(manager).expect("Failed to create pool");

		Self {
			db_pool: pool
		}
	}
}



