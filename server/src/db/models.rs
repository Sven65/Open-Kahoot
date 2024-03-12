use std::io::Write;

use diesel::{deserialize::{self, FromSql, FromSqlRow}, expression::AsExpression, pg::{Pg, PgValue}, prelude::*, serialize::{self, IsNull, Output, ToSql}, sql_types::SqlType};
use serde::Serialize;

use super::schema::sql_types::AnswerColor;


#[derive(Debug, SqlType, PartialEq, FromSqlRow, AsExpression, Eq, Serialize, Clone)]
#[diesel(sql_type = AnswerColor)]
pub enum RealAnswerColor {
    Red,
    Yellow,
    Blue,
    Green,
}


impl ToSql<AnswerColor, Pg> for RealAnswerColor {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            RealAnswerColor::Blue => out.write_all(b"Blue")?,
            RealAnswerColor::Green => out.write_all(b"Green")?,
            RealAnswerColor::Red => out.write_all(b"Red")?,
            RealAnswerColor::Yellow => out.write_all(b"Yellow")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<AnswerColor, Pg> for RealAnswerColor {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"Blue" => Ok(RealAnswerColor::Blue),
            b"Green" => Ok(RealAnswerColor::Green),
            b"Red" => Ok(RealAnswerColor::Red),
            b"Yellow" => Ok(RealAnswerColor::Yellow),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}


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
    pub answer_color: RealAnswerColor,
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
    pub question_rank: i32,
    pub max_time: f32,
    pub max_points: f32,
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
