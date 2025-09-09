// @generated automatically by Diesel CLI.

diesel::table! {
    line_channels (id) {
        id -> Int4,
        #[max_length = 255]
        channel_id -> Varchar,
        #[max_length = 255]
        channel_secret -> Varchar,
        #[max_length = 255]
        channel_access_token -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    line_webhook_events (id) {
        id -> Int4,
        #[max_length = 255]
        destination -> Varchar,
        payload -> Jsonb,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    line_channels,
    line_webhook_events,
);
