// @generated automatically by Diesel CLI.

diesel::table! {
    users (user_id) {
        #[max_length = 255]
        user_id -> Varchar,
        #[max_length = 255]
        first_name -> Varchar,
        #[max_length = 255]
        last_name -> Varchar,
        #[max_length = 50]
        role -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        alternate_email -> Nullable<Varchar>,
        #[max_length = 50]
        phone -> Varchar,
        #[max_length = 255]
        college -> Varchar,
        graduation_year -> Int4,
        #[max_length = 255]
        linkedin -> Nullable<Varchar>,
        #[max_length = 255]
        github -> Nullable<Varchar>,
    }
}
