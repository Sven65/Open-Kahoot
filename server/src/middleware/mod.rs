use std::sync::Arc;

use axum::{
   extract::State, http::{Request, StatusCode}, middleware::Next, response::Response
};

use axum_extra::extract::CookieJar;
use diesel::{prelude::*, QueryDsl};
use crate::{app_state::AppState, db::{models::Session, schema::session}};


#[derive(Clone, Debug)]
pub struct SessionInternal {
	pub user_id: String,
	pub session_id: String,
}

#[derive(Clone, Debug)]
pub struct CurrentSession {
	pub session: Option<SessionInternal>
}

impl CurrentSession {
	pub fn match_user_id (&self, user_id: String) -> bool {
		match &self.session {
			Some(session) => session.user_id == user_id,
			None => false,
		}
	}
}

pub async fn auth_session<B>(
	state: State<Arc<AppState>>,
	mut req: Request<B>,
	next: Next<B>
) -> Result<Response, StatusCode> {	
	let headers = req.headers();

	let cookies = CookieJar::from_headers(headers);

	match cookies.get("login_session") {
		Some(session_id) => {
			let mut conn = state.db_pool.get().expect("Failed to get database pool.");
			match session::table.find(session_id.clone().value()).first::<Session>(&mut conn) {
				Ok(session) => {
					req.extensions_mut().insert(CurrentSession {
						session: Some(SessionInternal {
							session_id: session_id.value().to_string(),
							user_id: session.user_id,
						})
					});
				},
				Err(_) => {
					req.extensions_mut().insert(CurrentSession { session: None });
				},
			}
		},
		None => { req.extensions_mut().insert(CurrentSession { session: None }); }
	}


	Ok(next.run(req).await)
}
