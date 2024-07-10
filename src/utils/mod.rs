use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Role {
    Admin,
    User,
    Guest,
}
