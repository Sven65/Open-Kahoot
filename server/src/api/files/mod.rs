pub mod file_storage_engine;
pub mod disk_storage;
pub mod file_utils;

use std::sync::Arc;

use axum::{extract::{Multipart, Path, State}, http::{Response, StatusCode}, routing::{get, post}, Extension, Router};
use diesel::{ExpressionMethods, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::{api::{files::{disk_storage::DiskStorage, file_storage_engine::FileStorageEngine, file_utils::convert_to_webp}, util::generic_json_response}, app_state::AppState, db::{models::{FileHostProvider, Files}, schema::files}, middleware::CurrentSession, util::generate_short_uuid};

use super::util::{generic_error, json_response};

#[derive(Serialize, Deserialize, Clone, Debug)]
struct ReturnedPathId {
	pub id: String,
}

async fn root() -> &'static str {
	"Hello from files world"
}

async fn get_temp_path_id(
	Extension(current_session): Extension<CurrentSession>,
	State(state): State<Arc<AppState>>,
) -> Response<axum::body::Body> {
	if current_session.session.is_none() { return generic_error(StatusCode::UNAUTHORIZED, "Unauthorized."); }
	
	let current_session = current_session.session.unwrap();

	let mut conn = state.db_pool.get().expect("Failed to get DB connection from pool");

	let id = generate_short_uuid();

	
	let _ = diesel::insert_into(crate::api::files::files::dsl::files)
		.values(Files::new(
			id.clone(),
			current_session.user_id,
			FileHostProvider::Disk,
		))
		.execute(&mut conn);
	

	json_response(StatusCode::OK, ReturnedPathId {
		id
	})
}

async fn upload_file(
	Extension(current_session): Extension<CurrentSession>,
	Path(id): Path<String>,
	State(state): State<Arc<AppState>>,
	mut files: Multipart,
) -> Response<axum::body::Body> {
	if current_session.session.is_none() { return generic_error(StatusCode::UNAUTHORIZED, "Unauthorized."); }
	
	let current_session = current_session.session.unwrap();
	let mut conn = state.db_pool.get().expect("Failed to get DB connection from pool");

	let _ = match Files::get_by_id(id.clone(), &mut conn).await {
		Ok(file) => {
			if file.owner_id != current_session.user_id {
				return generic_error(StatusCode::UNAUTHORIZED, "You're not permitted to upload here.")
			}

			file
		},
		Err(_) => {
			return generic_error(StatusCode::NOT_FOUND, "File ID not found")
		}
	};

	while let Some(file) = files.next_field().await.unwrap() {
		let data = file.bytes().await.unwrap();

		let file_id = generate_short_uuid();

		let name = format!("{}.webp", file_id);

		let webp_image = convert_to_webp(data);

		let result = DiskStorage::new().upload_file(webp_image.as_slice(), name).await;

		if result.is_err() {
			return generic_json_response(StatusCode::INTERNAL_SERVER_ERROR, "Failed to upload file.");
		}

		let res = diesel::update(crate::api::files::files::dsl::files)
			.filter(crate::api::files::files::id.eq(id.clone()))
			.set((crate::api::files::files::file_location.eq(result.unwrap()), crate::api::files::files::has_upload.eq(true)))
			.execute(&mut conn);

		if res.is_err() {
			return generic_error(StatusCode::INTERNAL_SERVER_ERROR, &res.err().unwrap().to_string());
		}
	}

	generic_json_response(StatusCode::OK, "File uploaded.")
}

pub fn files_router(state: Arc<AppState>) -> Router {
	Router::new()
		.route("/", 
			get(root)
			.post(get_temp_path_id)
		)
		.route("/:id", post(upload_file))
		.with_state(state)
}