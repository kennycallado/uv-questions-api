// @generated automatically by Diesel CLI.

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

diesel::joinable!(form_questions -> forms (form_id));
diesel::joinable!(form_questions -> questions (question_id));

diesel::allow_tables_to_appear_in_same_query!(
    form_questions,
    forms,
    questions,
    usuarios,
);
