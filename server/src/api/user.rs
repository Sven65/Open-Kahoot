
use argon2::{
    password_hash::{
        rand_core::OsRng, PasswordHasher, Salt, SaltString
    },
    Argon2, PasswordHash, PasswordVerifier
};

use axum::{http::StatusCode, response::Response, routing::{get, post}, Extension, Json, Router};
use diesel::{RunQueryDsl, SelectableHelper, prelude::*, QueryDsl};
use email_address::EmailAddress;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{api::{quiz_types::ReturnedUser, util::json_response_with_cookie}, db::{establish_connection, models::{Quiz, Session, User}, schema::{quiz, session, users}}, middleware::CurrentSession, util::generate_short_uuid};

use super::util::{generic_error, generic_json_response, json_response};

async fn root() -> &'static str {
	"Hello world"
}

#[derive(Deserialize)]
struct CreateUser {
    username: String,
	email: String,
	password: String,
}


#[derive(Deserialize, Clone, Debug)]
struct LoginUser {
    username: String,
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

fn validate_password (password: String, salt: String, hash: String) -> bool {
	let argon2 = Argon2::default();
	
	let salt_bytes = Salt::from_b64(&salt).expect("Invalid base64 salt");
    let mut password_hash = PasswordHash::new(&hash).expect("Invalid base64 hash");

	password_hash.salt = Some(salt_bytes);

	match argon2.verify_password(password.as_bytes(), &password_hash) {
        Ok(()) => true,  // Password is valid
        _ => false,  // Other errors (handle as invalid password for simplicity)
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

async fn login(
	Json(payload): Json<LoginUser>
) -> Response<axum::body::Body> {
	let mut conn = establish_connection();

	let user_result = users::table
		.filter(users::username.eq(payload.clone().username))
		.select((users::id, users::password, users::salt))
		.get_result::<(String, String, String)>(&mut conn);

	if user_result.is_err() {
		return generic_error(StatusCode::BAD_REQUEST, "Username or password incorrect.");
	}

	let (id, password_hash, salt) = user_result.unwrap();

	let session_id = generate_short_uuid();


	match validate_password(payload.clone().password, salt, password_hash) {
		true => {
			let _ = diesel::insert_into(session::table)
				.values(Session::new(session_id.clone(), id))
				.execute(&mut conn);

			json_response_with_cookie(StatusCode::OK, "Logged in", &format!("login_session={};Path=/", session_id))
		},
		false => generic_error(StatusCode::BAD_REQUEST, "Username or password incorrect.")
	}	
}

async fn get_me(
	Extension(current_session): Extension<CurrentSession>
) -> Response<axum::body::Body> {
	if current_session.session.is_none() { return generic_error(StatusCode::UNAUTHORIZED, "Unauthorized."); }

	let current_session = current_session.session.unwrap();
	
	let conn = &mut establish_connection();
	
	info!("session ext {:#?}", current_session);

	match users::table.find(current_session.user_id).first::<User>(conn) {
		Ok(user) => {json_response(StatusCode::OK, ReturnedUser {
			id: user.id,
			username: user.username,
		})},
		Err(_) => generic_error(StatusCode::INTERNAL_SERVER_ERROR, "Failed to login session.")
	}
}

async fn get_my_quizzes(
	Extension(current_session): Extension<CurrentSession>
) -> Response<axum::body::Body> {
	if current_session.session.is_none() { return generic_error(StatusCode::UNAUTHORIZED, "Unauthorized."); }

	let current_session = current_session.session.unwrap();
	
	let conn = &mut establish_connection();
	
	info!("session ext {:#?}", current_session);

	let quizzes = quiz::table.filter(quiz::owner_id.eq(current_session.user_id)).select(quiz::all_columns).load::<Quiz>(conn);
	
	match quizzes {
		Ok(quizzes) => json_response(StatusCode::OK, quizzes),
		Err(_) => generic_error(StatusCode::INTERNAL_SERVER_ERROR, "Failed to get quizzes.")
	}
}

pub fn user_router() -> Router {
	Router::new()
		.route("/", get(root))
		.route("/", post(create_user))
		.route("/login", post(login))
		.route("/@me", get(get_me))
		.route("/@me/quizzes", get(get_my_quizzes))
}