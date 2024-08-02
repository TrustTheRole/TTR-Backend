use chrono::NaiveDateTime;
use crate::schema::tags;
use diesel::{deserialize::Queryable, prelude::Insertable};
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Queryable, Deserialize, Insertable)]
#[diesel(table_name = tags)]
pub struct Tag{
    pub name: String,
    pub created_at: NaiveDateTime,
}