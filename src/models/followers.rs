use diesel::{deserialize::Queryable, prelude::Insertable};
use serde::{Deserialize, Serialize};

use crate::schema::followers;

#[derive(Serialize, Deserialize, Debug, Insertable, Queryable)]
#[diesel(table_name=followers)]
pub struct Followers {
    pub user_id: String,
    pub follower_id: String,
}
