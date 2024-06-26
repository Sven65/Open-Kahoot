// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "answer_color"))]
    pub struct AnswerColor;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "filehostprovider"))]
    pub struct Filehostprovider;
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
    email_verification (id) {
        id -> Varchar,
        user_id -> Varchar,
        verification_token -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Filehostprovider;

    files (id) {
        id -> Varchar,
        owner_id -> Varchar,
        question_id -> Nullable<Varchar>,
        file_location -> Nullable<Varchar>,
        host -> Filehostprovider,
        has_upload -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    password_reset (id) {
        id -> Varchar,
        user_id -> Varchar,
        reset_token -> Varchar,
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
        verified_email -> Nullable<Bool>,
        avatar -> Nullable<Varchar>,
    }
}

diesel::joinable!(answers -> questions (question_id));
diesel::joinable!(email_verification -> users (user_id));
diesel::joinable!(files -> questions (question_id));
diesel::joinable!(files -> users (owner_id));
diesel::joinable!(password_reset -> users (user_id));
diesel::joinable!(questions -> quiz (quiz_id));
diesel::joinable!(quiz -> users (owner_id));
diesel::joinable!(session -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    answers,
    email_verification,
    files,
    password_reset,
    questions,
    quiz,
    session,
    users,
);
