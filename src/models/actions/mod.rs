

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct EmailAction {
    pub user_name: String,
    pub user_email: String,
    pub message: String,
    pub subject: String,
    pub html_content: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ActionType {
    IncrementLikes,
    DecrementLikes,
    IncrementViews,
}