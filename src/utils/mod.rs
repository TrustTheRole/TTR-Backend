use diesel::{deserialize::FromSqlRow, expression::AsExpression, sql_types::Text};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, AsExpression, FromSqlRow)]
#[diesel(sql_type = Text)]
pub enum Role {
    SuperAdmin,
    Professional,
    Student,
}

pub fn get_role(role: &str) -> Role {
    match role {
        "super_admin" => Role::SuperAdmin,
        "professional" => Role::Professional,
        "student" => Role::Student,
        _ => Role::Student,
    }
}

pub fn get_role_str(role: &Role) -> &str {
    match role {
        Role::SuperAdmin => "super_admin",
        Role::Professional => "professional",
        Role::Student => "student",
    }
}

pub fn get_uid() -> String {
    Uuid::new_v4().to_string()
}
