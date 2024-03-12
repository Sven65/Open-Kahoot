// @generated automatically by Diesel CLI.

diesel::table! {
    answers (id) {
        id -> Int4,
        question_id -> Int4,
        answer -> Varchar,
        is_correct -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    questions (id) {
        id -> Int4,
        quiz_id -> Int4,
        question -> Varchar,
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
