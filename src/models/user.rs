use crate::schema::users;
use diesel::{deserialize::Queryable, prelude::Insertable};
use serde::{Deserialize, Serialize};

use crate::utils::Role;

#[derive(Debug, Serialize, Queryable, Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct User {
    pub user_id: String,
    pub first_name: String,
    pub last_name: String,
    pub role: Role,
    pub email: String,
    pub alternate_email: Option<String>,
    pub phone: String,
    pub college: String,
    pub graduation_year: i32,
    pub linkedin: Option<String>,
    pub github: Option<String>,
}
