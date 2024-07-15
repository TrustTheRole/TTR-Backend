use diesel::{deserialize::FromSqlRow, expression::AsExpression, sql_types::Text};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, AsExpression, FromSqlRow)]
#[diesel(sql_type = Text)]
pub enum Role {
    Admin,
    User,
    Guest,
}
pub fn get_uid() -> String {
    let uid = Uuid::new_v4().to_string();
    uid
}
