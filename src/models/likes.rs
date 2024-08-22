use crate::schema::likes;
use diesel::{deserialize::Queryable, prelude::Insertable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Queryable, Deserialize, Insertable)]
#[diesel(table_name = likes)]
pub struct Likes {
    pub insight_id: String,
    pub like_count: i32,
    pub view_count: i32,
}