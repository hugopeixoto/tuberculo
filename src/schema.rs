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

diesel::table! {
    videos (id) {
        id -> Text,
        title -> Text,
        description -> Nullable<Text>,
        duration -> Integer,
        aspect_ratio -> Float,
        fulltitle -> Nullable<Text>,
        categories -> Nullable<Text>,
        full_metadata -> Text,
        fetched_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    queue,
    videos,
);
