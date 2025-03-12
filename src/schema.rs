// @generated automatically by Diesel CLI.

diesel::table! {
    todos (id) {
        id -> Integer,
        title -> Text,
        description -> Text,
        importance -> Text,
        completed -> Bool,
    }
}
