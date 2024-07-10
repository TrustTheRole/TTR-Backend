use crate::schema::insights;
use diesel::{deserialize::Queryable, prelude::Insertable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Queryable, Deserialize, Insertable)]
#[diesel(table_name = insights)]
pub struct Insight {
    pub insight_id: String,
    pub user_id: String,
    pub insight_title: String,
    pub insight_company: String,
    pub insight_role: String,
    pub insight_tags: Vec<String>,
    pub insight_description: String,
    pub insight_picture_urls: Vec<String>,
    pub insight_focus_points: Vec<String>,
}
