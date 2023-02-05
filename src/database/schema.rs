// @generated automatically by Diesel CLI.

diesel::table! {
    answers (id) {
        id -> Int4,
        question_id -> Int4,
        answer -> Varchar,
    }
}

diesel::table! {
    form_questions (id) {
        id -> Int4,
        form_id -> Int4,
        question_id -> Int4,
    }
}

diesel::table! {
    forms (id) {
        id -> Int4,
        title -> Varchar,
        description -> Varchar,
    }
}

diesel::table! {
    papers (id) {
        id -> Int4,
        user_id -> Int4,
        form_id -> Int4,
    }
}

diesel::table! {
    questions (id) {
        id -> Int4,
        q_type -> Varchar,
        question -> Varchar,
    }
}

diesel::table! {
    usuarios (id) {
        id -> Int4,
        email -> Varchar,
        token -> Varchar,
    }
}

diesel::joinable!(answers -> questions (question_id));
diesel::joinable!(form_questions -> forms (form_id));
diesel::joinable!(form_questions -> questions (question_id));
diesel::joinable!(papers -> forms (form_id));
diesel::joinable!(papers -> usuarios (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    answers,
    form_questions,
    forms,
    papers,
    questions,
    usuarios,
);
