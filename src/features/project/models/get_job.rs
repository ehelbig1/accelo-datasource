use serde::Deserialize;

use super::job::*;
use super::meta::*;

#[derive(Debug, Deserialize, PartialEq)]
pub struct GetJobResponse {
    meta: Meta,
    response: Job,
}
