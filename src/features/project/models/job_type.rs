use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct JobType {
    pub title: String,
    pub id: String,
}
