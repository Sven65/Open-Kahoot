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
        id -> Varchar,
        question_id -> Varchar,
        answer -> Varchar,
        is_correct -> Bool,
        answer_color -> AnswerColor,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    questions (id) {
        id -> Varchar,
        quiz_id -> Varchar,
        question -> Varchar,
        question_rank -> Int4,
        max_time -> Float4,
        max_points -> Float4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    quiz (id) {
        id -> Varchar,
        owner_id -> Varchar,
        name -> Varchar,
        public -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    session (id) {
        id -> Varchar,
        user_id -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    users (id) {
        id -> Varchar,
        username -> Varchar,
        email -> Varchar,
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
    session,
    users,
);
