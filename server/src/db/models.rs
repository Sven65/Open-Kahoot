use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub salt: String,
    pub password: String,
}


#[derive(Insertable)]
#[diesel(table_name = crate::db::schema::users)]
pub struct NewUser {
	pub username: String,
    pub salt: String,
    pub password: String,
}