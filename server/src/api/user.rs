
use std::{env, sync::Arc};

use argon2::{
    password_hash::{
        rand_core::OsRng, PasswordHasher, Salt, SaltString
    },
    Argon2, PasswordHash, PasswordVerifier
};

use axum::{extract::State, http::StatusCode, response::Response, routing::{get, post}, Extension, Json, Router};
use diesel::{RunQueryDsl, SelectableHelper, prelude::*, QueryDsl};
use email_address::EmailAddress;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{api::{quiz_types::ReturnedUser, util::json_response_with_cookie}, app_state::AppState, db::{models::{EmailVerification, PasswordReset, Quiz, Session, User}, schema::{password_reset, quiz, session, users}}, email::Email, middleware::CurrentSession, util::{generate_short_uuid, has_duration_passed}};

use super::util::{generic_error, generic_response, json_response};

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

#[derive(Deserialize, Clone, Debug)]
struct SingleEmail {
	email: String,
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

fn get_default_verification_status() -> bool {
	let res = env::var("ENABLE_EMAIL_VERIFICATION").expect("Expected ENABLE_EMAIL_VERIFICATION to be set.");

	res != "true"
}

async fn create_user(
	State(state): State<Arc<AppState>>,
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

	let mut conn = state.db_pool.get().expect("Failed to get DB connection from pool");

	let new_user = User {
		id: user_id.clone(),
		salt,
		email: payload.email.clone(),
		password,
		username: payload.username.clone(),
		verified_email: Some(get_default_verification_status()),
	};

	let result = diesel::insert_into(users::table)
		.values(&new_user)
		.returning(User::as_returning())
		.get_result(&mut conn);

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

	if env::var("ENABLE_EMAIL_VERIFICATION").unwrap() == "true" {
		let verification = EmailVerification::new(user_id);

		let res = verification.clone().insert_into(crate::db::schema::email_verification::table).execute(&mut conn);

		if res.is_err() {
			info!("res is {:#?}", res.err());
			return generic_error(StatusCode::INTERNAL_SERVER_ERROR, "Failed to create verification token");
		}

		let email = Email::new().unwrap();

		let _ = email.send(
			"Please verify your email.",
			payload.email.clone().as_str(),
			format!(
				"Plase use the following link to verify ypur email: {}/v/{}",
				env::var("FRONTEND_URL").expect("Expected FRONTEND_URL to be set."),
				verification.verification_token,
			).as_str()
		).await;
	}

	json_response(StatusCode::CREATED, user)

}

async fn login(
	State(state): State<Arc<AppState>>,
	Json(payload): Json<LoginUser>
) -> Response<axum::body::Body> {
	let mut conn = state.db_pool.get().expect("Failed to get DB connection from pool");

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
	Extension(current_session): Extension<CurrentSession>,
	State(state): State<Arc<AppState>>,
) -> Response<axum::body::Body> {
	if current_session.session.is_none() { return generic_error(StatusCode::UNAUTHORIZED, "Unauthorized."); }

	let current_session = current_session.session.unwrap();
	
	let mut conn = state.db_pool.get().expect("Failed to get DB connection from pool.");

	match users::table.find(current_session.user_id).first::<User>(&mut conn) {
		Ok(user) => {json_response(StatusCode::OK, ReturnedUser {
			id: user.id,
			username: user.username,
		})},
		Err(_) => generic_error(StatusCode::INTERNAL_SERVER_ERROR, "Failed to login session.")
	}
}

async fn get_my_quizzes(
	Extension(current_session): Extension<CurrentSession>,
	State(state): State<Arc<AppState>>,
) -> Response<axum::body::Body> {
	if current_session.session.is_none() { return generic_error(StatusCode::UNAUTHORIZED, "Unauthorized."); }

	let current_session = current_session.session.unwrap();
	
	let mut conn = state.db_pool.get().expect("Failed to get DB connection from pool");
	
	info!("session ext {:#?}", current_session);

	let quizzes = quiz::table.filter(quiz::owner_id.eq(current_session.user_id)).select(quiz::all_columns).load::<Quiz>(&mut conn);
	
	match quizzes {
		Ok(quizzes) => json_response(StatusCode::OK, quizzes),
		Err(_) => generic_error(StatusCode::INTERNAL_SERVER_ERROR, "Failed to get quizzes.")
	}
}

async fn request_password_reset(	
	State(state): State<Arc<AppState>>,
	Json(payload): Json<SingleEmail>
) -> Response<axum::body::Body> {
	let mut conn = state.db_pool.get().expect("Failed to get DB connection from pool");

	let user: Option<User> = match users::table.filter(users::email.eq(payload.email.to_lowercase())).first::<User>(&mut conn) {
		Ok(user) => Some(user),
		Err(_) => None,
	};

	if user.is_none() {
		return generic_error(StatusCode::INTERNAL_SERVER_ERROR, "Password reset request failed");
	}

	let user = user.unwrap();

	let reset_row: Option<PasswordReset> = match password_reset::table.filter(password_reset::user_id.eq(user.id.clone())).first::<PasswordReset>(&mut conn) {
		Ok(row) => Some(row),
		Err(_) => None,
	};

	
	if reset_row.is_some() {
		let reset_row = reset_row.unwrap();

		if !has_duration_passed(reset_row.created_at, state.app_config.password_reset_request_time.unwrap()) {
			return generic_error(StatusCode::BAD_REQUEST, "Please try again later.")
		} else {
			let _ = diesel::delete(password_reset::table).filter(password_reset::id.eq(reset_row.id)).execute(&mut conn);
		}
	}

	let request = PasswordReset::new(user.id);

	let _ = request.insert_into(crate::db::schema::password_reset::table).execute(&mut conn);

	generic_response(StatusCode::OK, "Request made")
}

pub fn user_router(state: Arc<AppState>) -> Router {
	Router::new()
		.route("/", get(root))
		.route("/", post(create_user))
		.route("/login", post(login))
		.route("/@me", get(get_me))
		.route("/@me/quizzes", get(get_my_quizzes))
		.route("/password/reset", post(request_password_reset))
		.with_state(state)
}