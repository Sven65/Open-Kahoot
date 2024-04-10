pub mod file_storage_engine;
pub mod disk_storage;
pub mod file_utils;
pub mod s3_storage;

use std::{env, sync::Arc};

use axum::{extract::{Multipart, Path, State}, http::{Response, StatusCode}, routing::{get, post}, Extension, Json, Router};
use diesel::{ExpressionMethods, RunQueryDsl};
use serde::{Deserialize, Serialize};
use tokio::{fs::File, io::AsyncReadExt};

use crate::{api::{files::file_utils::convert_to_webp, util::generic_json_response}, app_state::AppState, db::{models::{FileHostProvider, Files}, schema::files}, middleware::CurrentSession, util::generate_short_uuid};

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
			match env::var("FILE_STORAGE_ENGINE").expect("File storage engine must be set.").to_lowercase().as_str() {
				"s3" => FileHostProvider::S3,
				"disk" => FileHostProvider::Disk,
				_ => panic!("File storage engine must be set in env"),
			}
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

		let webp_image = convert_to_webp(data.to_vec());

		let result = state.filestorage.upload_file(webp_image.as_slice(), name).await;

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

async fn serve_file(
	Extension(current_session): Extension<CurrentSession>,
	Path(id): Path<String>,
	State(state): State<Arc<AppState>>,
) -> Response<axum::body::Body> {
	if current_session.session.is_none() { return generic_error(StatusCode::UNAUTHORIZED, "Unauthorized."); }
	
	let current_session = current_session.session.unwrap();
	let mut conn = state.db_pool.get().expect("Failed to get DB connection from pool");

	let file = Files::get_by_id(id, &mut conn).await;

	if file.is_err() { return generic_error(StatusCode::INTERNAL_SERVER_ERROR, "Failed to get file from DB."); }

	let file = file.unwrap();

	if file.owner_id != current_session.user_id { return generic_error(StatusCode::UNAUTHORIZED, "Unauthorized."); }

	let file_url = state.filestorage.serve_file(file.file_location.unwrap()).await;

	if file_url.is_err() { return generic_error(StatusCode::INTERNAL_SERVER_ERROR, "Failed to get file from storage."); }

	generic_json_response(StatusCode::OK, file_url.unwrap().as_str())
}

async fn file_handler(
	Path(file_name): Path<String>,
	State(state): State<Arc<AppState>>,
) -> Response<axum::body::Body> {

	let path = format!("./{}/{}", state.filestorage.get_file_path(), file_name);
	
	let file = File::open(path).await;

	if file.is_err() {
		return generic_error(StatusCode::INTERNAL_SERVER_ERROR, "Failed to open file");
	}

	let mut file = file.unwrap();

    let mut contents = vec![];
    let res = file.read_to_end(&mut contents).await;

	if res.is_err()  {
		return generic_error(StatusCode::INTERNAL_SERVER_ERROR, "Failed to read file");
	}

	Response::builder()
		.status(StatusCode::OK)
		.header("Content-Type", "image/webp")
		.body(axum::body::Body::from(
			contents
		))
		.expect("Failed to build response")
}

pub fn files_router(state: Arc<AppState>) -> Router {
	Router::new()
		.route("/", 
			get(root)
			.post(get_temp_path_id)
		)
		.route("/:id", post(upload_file).get(serve_file))
		.route("/f/:file_name", get(file_handler))
		.with_state(state)
}