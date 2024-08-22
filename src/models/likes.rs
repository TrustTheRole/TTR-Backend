use crate::schema::likes;
use diesel::AsChangeset;
use diesel::{deserialize::Queryable, prelude::Insertable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Queryable, Deserialize, Insertable)]
#[diesel(table_name = likes)]
pub struct Likes {
    pub insight_id: String,
    pub like_count: i32,
    pub view_count: i32,
}

#[derive(AsChangeset)]
#[diesel(table_name=likes)]
pub struct UpdateLikes<'a>{
    pub like_count: Option<&'a i32>,
    pub view_count: Option<&'a i32>
}