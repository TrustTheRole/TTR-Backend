use crate::schema::insights;
use diesel::AsChangeset;
use diesel::{deserialize::Queryable, prelude::Insertable};
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Queryable, Deserialize, Insertable)]
#[diesel(table_name = insights)]
pub struct Insight {
    pub insight_id: String,
    pub user_id: String,
    pub user_name:String,
    pub insight_title: String,
    pub insight_company: String,
    pub insight_role: String,
    pub insight_tags: Vec<String>,
    pub insight_description: String,
    pub insight_picture_urls: Vec<String>,
    pub insight_focus_points: Vec<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct InsightResponse {
    pub insight_id: String,
    pub insight_title: String,
    pub insight_company: String,
    pub insight_role: String,
    pub insight_tags: Vec<String>,
    pub insight_description: String,
    pub insight_picture_urls: Vec<String>,
    pub insight_focus_points: Vec<String>,
    pub created_at: NaiveDateTime,
}

#[derive(AsChangeset)]
#[diesel(table_name = insights)]
pub struct UpdateInsight<'a> {
    pub insight_title: Option<&'a str>,
    pub insight_role: Option<&'a str>,
    pub insight_tags: Option<Vec<String>>,
    pub insight_description: Option<&'a str>,
    pub insight_focus_points: Option<Vec<String>>,
}
