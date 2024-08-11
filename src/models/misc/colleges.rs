use diesel::{deserialize::Queryable, prelude::Insertable};
use serde::{Deserialize, Serialize};

use crate::schema::colleges;

#[derive(Debug, Serialize, Queryable, Deserialize, Insertable)]
#[diesel(table_name=colleges)]
pub struct College{
    pub id:String,
    pub college_name:String,
    pub college_location:String,
    pub students_registered:i32,
}