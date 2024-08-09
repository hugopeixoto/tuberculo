// @generated automatically by Diesel CLI.

diesel::table! {
    queue (id) {
        id -> Integer,
        job -> Text,
        args -> Text,
        created_at -> Timestamp,
        locked_at -> Nullable<Timestamp>,
    }
}
