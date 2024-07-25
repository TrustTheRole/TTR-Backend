use diesel::{deserialize::Queryable, prelude::Insertable};
use serde::{Deserialize, Serialize};

use crate::schema::subscription;

#[derive(Serialize, Deserialize, Debug, Insertable, Queryable)]
#[diesel(table_name=subscription)]
pub struct Subsciption{
    pub id:String,
    pub user_id:String,
}