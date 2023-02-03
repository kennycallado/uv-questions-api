// @generated automatically by Diesel CLI.

diesel::table! {
    questions (id) {
        id -> Int4,
        q_type -> Varchar,
        question -> Varchar,
        answer -> Nullable<Varchar>,
    }
}
