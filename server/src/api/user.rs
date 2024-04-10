
use std::{env, sync::Arc};

use argon2::{
    password_hash::{
        rand_core::OsRng, PasswordHasher, Salt, SaltString
    },
    Argon2, PasswordHash, PasswordVerifier
};

use axum::{extract::{Path, State}, http::StatusCode, response::{IntoResponse, Redirect, Response}, routing::{get, post, put}, Extension, Json, Router};
use diesel::{RunQueryDsl, SelectableHelper, prelude::*, QueryDsl};
use email_address::EmailAddress;
use pretty_duration::{pretty_duration, PrettyDurationOptions, PrettyDurationOutputFormat};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{api::{quiz_types::ReturnedUser, util::json_response_with_cookie}, app_state::AppState, db::{models::{EmailVerification, Files, PasswordReset, Quiz, Session, User}, schema::{password_reset, quiz, session, users}}, email::Email, middleware::CurrentSession, util::{generate_short_uuid, has_duration_passed}};

use super::util::{generic_error, generic_json_response, generic_response, json_response};

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

#[derive(Deserialize, Clone, Debug)]
struct SetAvatarIdInput {
	pub id: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
struct PasswordResetRequest {
	new_password: String,
	token: String,
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
		avatar: None,
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
	if current_session.error.is_some() { return generic_error(StatusCode::BAD_REQUEST, current_session.error.unwrap()); }
	if current_session.session.is_none() { return generic_error(StatusCode::UNAUTHORIZED, "Unauthorized."); }


	let current_session = current_session.session.unwrap();
	
	let mut conn = state.db_pool.get().expect("Failed to get DB connection from pool.");

	match users::table.find(current_session.user_id).first::<User>(&mut conn) {
		Ok(user) => {json_response(StatusCode::OK, ReturnedUser {
			id: user.id,
			username: user.username,
			avatar: user.avatar,
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

	let user: Option<User> = match users::table.filter(users::email.eq(payload.email.clone().to_lowercase())).first::<User>(&mut conn) {
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

	let _ = request.clone().insert_into(crate::db::schema::password_reset::table).execute(&mut conn);

	let valid_string = pretty_duration(&state.app_config.password_reset_valid_time.unwrap().to_std().unwrap(), Some(PrettyDurationOptions {
        output_format: Some(PrettyDurationOutputFormat::Expanded),
        singular_labels: None,
        plural_labels: None,
    }));

	let email = Email::new().unwrap().send(
		"Password reset request",
		payload.email.clone().to_lowercase().as_str(), 
		format!(r#"
		Hello,
		We received a request to reset your password. If you did not request this, you can safely ignore this email.

		To reset your password, click on the following link:

		{}/p/{}

		This link will expire in {}, so please reset your password as soon as possible.
		"#, state.app_config.frontend_url, request.reset_token, valid_string).as_str()
	).await;

	if email.is_err() {
		return generic_error(StatusCode::INTERNAL_SERVER_ERROR, "Failed to send email.")
	}

	generic_json_response(StatusCode::OK, "Request made")
}

async fn reset_password(
	State(state): State<Arc<AppState>>,
	Json(payload): Json<PasswordResetRequest>
) ->  Response<axum::body::Body> {
	let mut conn = state.db_pool.get().expect("Failed to get DB connection from pool");

	let reset_row: Option<PasswordReset> = match password_reset::table.filter(password_reset::reset_token.eq(payload.token)).first::<PasswordReset>(&mut conn) {
		Ok(row) => Some(row),
		Err(_) => None,
	};

	if reset_row.is_none() {
		return generic_error(StatusCode::NOT_FOUND, "Invalid token or email provided.");
	}

	let reset_row = reset_row.unwrap();

	let user: Option<User> = match users::table.filter(users::id.eq(reset_row.user_id)).first::<User>(&mut conn) {
		Ok(user) => Some(user),
		Err(_) => None,
	};

	if user.is_none() {
		return generic_error(StatusCode::NOT_FOUND, "Invalid token or email provided.");
	}

	let user = user.unwrap();

	if has_duration_passed(reset_row.created_at, state.app_config.password_reset_valid_time.unwrap()) {
		let _ = diesel::delete(password_reset::table)
			.filter(password_reset::id.eq(reset_row.id))
			.execute(&mut conn);

		return generic_error(StatusCode::BAD_REQUEST, "Password reset has timed out.");
	}

	let hash_tuple = hash_password(payload.new_password.as_bytes());

	if hash_tuple.is_none() {
		return generic_error(StatusCode::INTERNAL_SERVER_ERROR, "Invalid hashing data");
	}

	let (salt, password) = hash_tuple.unwrap();	

	let res = diesel::update(users::table)
		.filter(users::id.eq(user.id))
		.set((users::password.eq(password), users::salt.eq(salt)))
		.execute(&mut conn);

	if res.is_err() {
		return generic_error(StatusCode::INTERNAL_SERVER_ERROR, "Failed to save user.");
	}

	let _ = diesel::delete(password_reset::table)
		.filter(password_reset::id.eq(reset_row.id))
		.execute(&mut conn);

	generic_json_response(StatusCode::OK, "Password updated.")
}

async fn set_avatar(
	Extension(current_session): Extension<CurrentSession>,
	State(state): State<Arc<AppState>>,
	Json(payload): Json<SetAvatarIdInput>
) -> Response<axum::body::Body> {
	if current_session.error.is_some() { return generic_error(StatusCode::BAD_REQUEST, current_session.error.unwrap()); }
	if current_session.session.is_none() { return generic_error(StatusCode::UNAUTHORIZED, "Unauthorized."); }


	let current_session = current_session.session.unwrap();
	
	let mut conn = state.db_pool.get().expect("Failed to get DB connection from pool.");

	let _ = diesel::update(users::table)
		.filter(users::id.eq(current_session.user_id.clone()))
		.set(users::avatar.eq(payload.id.clone()))
		.execute(&mut conn);

	if payload.id.is_none() { return generic_json_response(StatusCode::GONE, "Avatar removed"); }

	let cloned = payload.id.unwrap().clone();

	let id = cloned.as_str();

	generic_json_response(StatusCode::OK, id)
}

async fn get_avatar(
	Path(id): Path<String>,
	State(state): State<Arc<AppState>>,
) -> Response {
	let mut conn = state.db_pool.get().expect("Failed to get DB connection from pool");

	let user: Option<User> = match users::table.filter(users::id.eq(id.clone())).first::<User>(&mut conn) {
		Ok(user) => Some(user),
		Err(_) => None,
	};

	if user.is_none() {
		return StatusCode::NOT_FOUND.into_response()
		//return generic_error(StatusCode::NOT_FOUND, "User avatar not found.");
	}

	let user = user.unwrap();

	if user.avatar.is_none() {
		// return generic_error(StatusCode::NOT_FOUND, "User avatar not found.");
		return StatusCode::NOT_FOUND.into_response()
	}

	let file = Files::get_by_id(user.avatar.unwrap(), &mut conn).await;

	if file.is_err() { return StatusCode::INTERNAL_SERVER_ERROR.into_response(); }

	let file = file.unwrap();

	let file_url = state.filestorage.serve_file(file.file_location.unwrap()).await;

	if file_url.is_err() { return StatusCode::INTERNAL_SERVER_ERROR.into_response(); }
	
	let redir_url = format!("/api/{}", file_url.unwrap().as_str());

	Redirect::to(redir_url.as_str()).into_response()

	//generic_json_response(StatusCode::OK, file_url.unwrap().as_str())
}

pub fn user_router(state: Arc<AppState>) -> Router {
	Router::new()
		.route("/", get(root))
		.route("/", post(create_user))
		.route("/login", post(login))
		.route("/@me", get(get_me))
		.route("/@me/quizzes", get(get_my_quizzes))
		.route("/@me/avatar", put(set_avatar))
		.route("/avatar/:id", get(get_avatar))
		.route("/password/reset", post(reset_password))
		.route("/password/reset/request", post(request_password_reset))
		.with_state(state)
}