use crate::schema::users;
use diesel::{deserialize::Queryable, prelude::Insertable, query_builder::AsChangeset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Queryable, Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = users)]
pub struct User {
    pub user_id: String,
    pub password: String,
    pub name: String,
    pub role: Option<String>,
    pub email: String,
    pub alternate_email: Option<String>,
    pub phone: Option<String>,
    pub college: Option<String>,
    pub graduation_year: Option<i32>,
    pub linkedin: Option<String>,
    pub github: Option<String>,
}
