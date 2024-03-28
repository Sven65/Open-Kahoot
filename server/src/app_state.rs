use std::{env, sync::Arc};

use diesel::{r2d2::{ConnectionManager, Pool, PooledConnection}, PgConnection};
use dotenvy::dotenv;

use crate::api::files::{disk_storage::DiskStorage, file_storage_engine::FileStorageEngine};

pub type PgPooledConn = PooledConnection<ConnectionManager<PgConnection>>;
pub type PgPool = Pool<ConnectionManager<PgConnection>>;


#[derive(Clone)]
pub struct AppState {
	pub db_pool: PgPool,
	pub filestorage: Arc<dyn FileStorageEngine + Sync + Send>,
}

impl AppState {
	pub fn get_file_engine() -> Arc<dyn FileStorageEngine + Sync + Send> {
		return Arc::new(DiskStorage::new())
	}

	pub fn new() -> Self {
		dotenv().ok();
		let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

		let manager = ConnectionManager::<PgConnection>::new(database_url);
    	let pool = Pool::builder().build(manager).expect("Failed to create pool");

		Self {
			db_pool: pool,
			filestorage: AppState::get_file_engine(),
		}
	}
}



