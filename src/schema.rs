// @generated automatically by Diesel CLI.

diesel::table! {
    followers (user_id, follower_id) {
        #[max_length = 255]
        user_id -> Varchar,
        #[max_length = 255]
        follower_id -> Varchar,
    }
}

diesel::table! {
    insights (insight_id) {
        #[max_length = 255]
        insight_id -> Varchar,
        #[max_length = 255]
        user_id -> Nullable<Varchar>,
        #[max_length = 255]
        insight_title -> Varchar,
        #[max_length = 255]
        insight_company -> Varchar,
        #[max_length = 255]
        insight_role -> Varchar,
        insight_tags -> Nullable<Array<Nullable<Text>>>,
        insight_description -> Text,
        insight_picture_urls -> Nullable<Array<Nullable<Text>>>,
        insight_focus_points -> Nullable<Array<Nullable<Text>>>,
    }
}

diesel::table! {
    users (user_id) {
        #[max_length = 255]
        user_id -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 50]
        role -> Nullable<Varchar>,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        alternate_email -> Nullable<Varchar>,
        #[max_length = 50]
        phone -> Nullable<Varchar>,
        #[max_length = 255]
        college -> Nullable<Varchar>,
        graduation_year -> Nullable<Int4>,
        #[max_length = 255]
        linkedin -> Nullable<Varchar>,
        #[max_length = 255]
        github -> Nullable<Varchar>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    followers,
    insights,
    users,
);
