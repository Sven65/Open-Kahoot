// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "answer_color"))]
    pub struct AnswerColor;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::AnswerColor;

    answers (id) {
        id -> Int4,
        question_id -> Int4,
        answer -> Varchar,
        is_correct -> Bool,
        answer_color -> AnswerColor,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    questions (id) {
        id -> Int4,
        quiz_id -> Int4,
        question -> Varchar,
        question_rank -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    quiz (id) {
        id -> Int4,
        owner_id -> Int4,
        name -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        salt -> Varchar,
        password -> Varchar,
    }
}

diesel::joinable!(answers -> questions (question_id));
diesel::joinable!(questions -> quiz (quiz_id));
diesel::joinable!(quiz -> users (owner_id));

diesel::allow_tables_to_appear_in_same_query!(
    answers,
    questions,
    quiz,
    users,
);
