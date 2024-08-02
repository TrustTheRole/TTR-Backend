use diesel::{deserialize::Queryable, prelude::Insertable};
use serde::{Deserialize, Serialize};

use crate::schema::companies;

#[derive(Serialize, Deserialize, Debug, Insertable, Queryable)]
#[diesel(table_name=companies)]
pub struct Companies{
    pub id:String,
    pub company_name:String,
}