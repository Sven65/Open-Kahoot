use diesel::{deserialize::FromSqlRow, prelude::*};
use serde::Serialize;

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub salt: String,
    pub password: String,
}


#[derive(Debug, Insertable)]
#[diesel(table_name = crate::db::schema::users)]
pub struct NewUser {
	pub username: String,
    pub salt: String,
    pub password: String,
}

#[derive(Debug, Serialize, Clone, Identifiable, Queryable, Selectable, Insertable, Associations)]
#[diesel(table_name = crate::db::schema::answers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Question))]
pub struct Answer {
    pub id: i32,
    pub question_id: i32,
    pub answer: String,
    pub is_correct: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Clone, Identifiable, Queryable, Selectable, Insertable, Associations)]
#[diesel(table_name = crate::db::schema::questions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Quiz))]
pub struct Question {
    pub id: i32,
    pub quiz_id: i32,
    pub question: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Clone, Identifiable, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::db::schema::quiz)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Quiz {
    pub id: i32,
    pub owner_id: i32,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
