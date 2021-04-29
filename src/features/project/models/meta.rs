use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Meta {
    pub more_info: String,
    pub message: String,
    pub status: String,
}
