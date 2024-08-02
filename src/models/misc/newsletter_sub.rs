use chrono::NaiveDateTime;
use diesel::prelude::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

use crate::schema::newsletter_sub;

#[derive(Debug, Serialize, Queryable, Deserialize, Insertable)]
#[diesel(table_name = newsletter_sub)]
pub struct Newsletter {
    pub email: String,
    pub created_at: NaiveDateTime,
}
