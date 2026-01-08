use serde::{Deserialize, Serialize};
use tower::common::random_id;

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub struct User {
    pub user_id: i32,
    pub nickname: String,
    pub avatar: u16,
}
impl User {
    pub fn avatar(&self) -> String {
        format!("/public/avatar/{}.webp", self.avatar)
    }
    pub fn to_avatar(avatar: u16) -> String {
        format!("/public/avatar/{}.webp", avatar)
    }
}
impl Default for User {
    fn default() -> Self {
        Self {
            user_id: 0,
            nickname: random_id(12),
            avatar: 0,
        }
    }
}
