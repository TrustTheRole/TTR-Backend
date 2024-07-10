use crate::schema::accounts;
use diesel::{deserialize::Queryable, prelude::Insertable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Queryable, Deserialize, Insertable)]
#[table_name = "accounts"]
pub struct Account {
    pub user_id: String,
    pub account_number: String,
    pub ifsc: String,
    pub bank_name: String,
    pub upi_id: String,
}
