use diesel::{deserialize::Queryable, prelude::Insertable};
use serde::{Deserialize, Serialize};

use crate::schema::colleges;

#[derive(Serialize, Deserialize, Debug, Insertable, Queryable)]
#[diesel(table_name=colleges)]
pub struct College{
    pub id:String,
    pub college_name:String,
    pub college_location:String,
    pub college_state:String,
}