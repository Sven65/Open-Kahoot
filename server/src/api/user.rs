
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHasher, SaltString
    },
    Argon2
};

use axum::{http::StatusCode, response::Response, routing::{get, post}, Json, Router};
use diesel::{RunQueryDsl, SelectableHelper};
use email_address::EmailAddress;
use serde::{Deserialize, Serialize};

use crate::{db::{establish_connection, models::User, schema::users}, util::generate_short_uuid};

use super::util::{generic_error, json_response};

async fn root() -> &'static str {
	"Hello world"
}

#[derive(Deserialize)]
struct CreateUser {
    username: String,
	email: String,
	password: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct CreatedUser {
    id: String,
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
	if !EmailAddress::is_valid(&payload.email.clone()) {
		return generic_error(StatusCode::BAD_REQUEST, "Email is invalid.")
	}

	let hash_tuple = hash_password(payload.password.as_bytes());

	if hash_tuple.is_none() {
		return generic_error(StatusCode::INTERNAL_SERVER_ERROR, "Invalid hashing data");
	}

	let (salt, password) = hash_tuple.unwrap();	
	let user_id = generate_short_uuid();

	let conn = &mut establish_connection();

	let new_user = User {
		id: user_id,
		salt,
		email: payload.email.clone(),
		password,
		username: payload.username.clone(),
	};

	let result = diesel::insert_into(users::table)
		.values(&new_user)
		.returning(User::as_returning())
		.get_result(conn);

	if result.is_err() {
		let error = result.err().unwrap();

		if error.to_string().contains("users_username_key") {
			return generic_error(StatusCode::CONFLICT, "Username already exists.");
		}

		if error.to_string().contains("users_email_key") {
			return generic_error(StatusCode::CONFLICT, "Email already exists.");
		}

		return generic_error(StatusCode::INTERNAL_SERVER_ERROR, error.to_string().as_str());
	}

	let result = result.unwrap();

	let user = CreatedUser {
		id: result.id,
		username: result.username
	};

	json_response(StatusCode::CREATED, user)

}

pub fn user_router() -> Router {
	Router::new()
		.route("/", get(root))
		.route("/", post(create_user))
}