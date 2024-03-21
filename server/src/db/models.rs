use std::io::Write;

use chrono::Local;
use diesel::{deserialize::{self, FromSql, FromSqlRow}, expression::AsExpression, pg::{Pg, PgValue}, prelude::*, serialize::{self, IsNull, Output, ToSql}, sql_types::SqlType};
use serde::{Deserialize, Serialize};

use crate::{api::quiz_types::{ReturnedAnswer, ReturnedQuestion, ReturnedQuiz}};

use super::schema::sql_types::AnswerColor;


#[derive(Debug, Deserialize, SqlType, PartialEq, FromSqlRow, AsExpression, Eq, Serialize, Clone, Hash)]
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


#[derive(Debug, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::db::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub salt: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Identifiable, Queryable, Selectable, Insertable, Associations, AsChangeset)]
#[diesel(table_name = crate::db::schema::answers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Question))]
pub struct Answer {
    pub id: String,
    pub question_id: String,
    pub answer: String,
    pub is_correct: bool,
    pub answer_color: RealAnswerColor,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl From<ReturnedAnswer> for Answer {
    fn from(value: ReturnedAnswer) -> Self {
        Self {
            id: value.id.unwrap(),
            question_id: value.question_id.unwrap(),
            answer: value.answer,
            is_correct: value.is_correct,
            answer_color: value.answer_color,
            created_at: value.created_at.unwrap_or(Local::now().naive_local()),
            updated_at: value.updated_at.unwrap_or(Local::now().naive_local()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Identifiable, Queryable, Selectable, Insertable, Associations, AsChangeset)]
#[diesel(table_name = crate::db::schema::questions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Quiz))]
pub struct Question {
    pub id: String,
    pub quiz_id: String,
    pub question: String,
    pub question_rank: i32,
    pub max_time: f32,
    pub max_points: f32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl From<ReturnedQuestion> for Question {
    fn from(value: ReturnedQuestion) -> Self {
        Self {
            id: value.id.unwrap(),
            max_points: value.max_points,
            max_time: value.max_time,
            question: value.question,
            question_rank: value.question_rank,
            quiz_id: value.quiz_id,
            created_at: value.created_at.unwrap_or(Local::now().naive_local()),
            updated_at: value.updated_at.unwrap_or(Local::now().naive_local())
        }
    }
}

#[derive(Debug, Serialize, Clone, Identifiable, Queryable, Selectable, Insertable, AsChangeset)]
#[diesel(table_name = crate::db::schema::quiz)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Quiz {
    pub id: String,
    pub owner_id: String,
    pub name: String,
    pub public: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl From<ReturnedQuiz> for Quiz {
    fn from(value: ReturnedQuiz) -> Self {
        Self {
            id: value.id.unwrap(),
            owner_id: value.owner.id,
            name: value.name,
            public: value.public,
            created_at: value.created_at.unwrap_or(Local::now().naive_local()),
            updated_at: value.updated_at.unwrap_or(Local::now().naive_local()),
        }
    }
}
