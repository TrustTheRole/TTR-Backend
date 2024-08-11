use crate::schema::users;
use diesel::AsChangeset;
use diesel::{deserialize::Queryable, prelude::Insertable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Queryable, Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct User {
    pub user_id: String,
    pub name: String,
    pub role: Option<String>,
    pub email: String,
    pub alternate_email: Option<String>,
    pub phone: Option<String>,
    pub college: Option<String>,
    pub graduation_year: Option<i32>,
    pub linkedin: Option<String>,
    pub github: Option<String>,
    pub gender: String,
}



#[derive(AsChangeset)]
#[diesel(table_name = users)]
pub struct UpdateUser<'a> {
    pub name: Option<&'a str>,
    pub role: Option<&'a str>,
    pub alternate_email: Option<&'a str>,
    pub phone: Option<&'a str>,
    pub college: Option<&'a str>,
    pub graduation_year: Option<i32>,
    pub linkedin: Option<&'a str>,
    pub github: Option<&'a str>,
    pub gender: Option<&'a str>,
}
