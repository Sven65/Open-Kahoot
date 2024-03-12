
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHasher, SaltString
    },
    Argon2
};

use axum::{http::StatusCode, response::Response, routing::{get, post}, Json, Router};
use diesel::{RunQueryDsl, SelectableHelper};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::db::{establish_connection, models::NewUser, models::User, schema::users};

async fn root() -> &'static str {
	"Hello world"
}

#[derive(Deserialize)]
struct CreateUser {
    username: String,
	password: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct CreatedUser {
    id: i32,
    username: String,
}

fn hash_password(password: &[u8]) -> Option<(String, String)> {
	let salt = SaltString::generate(&mut OsRng);

	let argon2 = Argon2::default();

	let password_hash = argon2.hash_password(password, &salt);

	match password_hash {
		Ok(hash) => {
			Some((salt.to_string(), hash.to_string()))
		},
		Err(_) => None,
	}
}

async fn create_user(
	Json(payload): Json<CreateUser>,
) -> Response<axum::body::Body> {	
	let hash_tuple = hash_password(payload.password.as_bytes());

	if hash_tuple.is_none() {
		return Response::builder()
			.status(StatusCode::INTERNAL_SERVER_ERROR)
			.body(axum::body::Body::from(
                "Invalid data".to_string(),
            ))
			.unwrap();
	}

	let (salt, password) = hash_tuple.unwrap();	

	let conn = &mut establish_connection();

	let new_user = NewUser {
		salt,
		password,
		username: payload.username.clone(),
	};

	let result = diesel::insert_into(users::table)
		.values(&new_user)
		.returning(User::as_returning())
		.get_result(conn);

	if result.is_err() {
		let error = result.err().unwrap();

		return Response::builder()
			.status(StatusCode::INTERNAL_SERVER_ERROR)
			.body(axum::body::Body::from(
				error.to_string()
			))
			.unwrap();
	}

	let result = result.unwrap();

	let user = CreatedUser {
		id: result.id,
		username: result.username
	};

	return Response::builder()
		.status(StatusCode::CREATED)
		.body(axum::body::Body::from(
			serde_json::to_string(&user).expect("Failed to serialize json")
		))
		.unwrap();

}

pub fn user_router() -> Router {
	Router::new()
		.route("/", get(root))
		.route("/", post(create_user))
}