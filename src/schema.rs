// @generated automatically by Diesel CLI.

diesel::table! {
    colleges (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        college_name -> Varchar,
        #[max_length = 255]
        college_location -> Varchar,
        students_registered -> Int4,
    }
}

diesel::table! {
    companies (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        company_name -> Varchar,
    }
}

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
        user_id -> Varchar,
        #[max_length = 255]
        user_name -> Varchar,
        #[max_length = 255]
        insight_title -> Varchar,
        #[max_length = 255]
        insight_company -> Varchar,
        #[max_length = 255]
        insight_role -> Varchar,
        insight_tags -> Array<Text>,
        insight_description -> Text,
        insight_picture_urls -> Array<Text>,
        insight_focus_points -> Array<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    likes (insight_id) {
        #[max_length = 255]
        insight_id -> Varchar,
        like_count -> Int4,
        view_count -> Int4,
    }
}

diesel::table! {
    newsletter_sub (email) {
        #[max_length = 255]
        email -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    subscription (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        user_id -> Varchar,
    }
}

diesel::table! {
    tags (name) {
        #[max_length = 255]
        name -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (user_id) {
        #[max_length = 255]
        user_id -> Varchar,
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
        #[max_length = 50]
        gender -> Varchar,
    }
}

diesel::joinable!(insights -> users (user_id));
diesel::joinable!(likes -> insights (insight_id));
diesel::joinable!(subscription -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    colleges,
    companies,
    followers,
    insights,
    likes,
    newsletter_sub,
    subscription,
    tags,
    users,
);
