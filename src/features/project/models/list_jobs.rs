use serde::Deserialize;

use super::job::*;
use super::meta::*;

#[derive(Debug, Deserialize, PartialEq)]
pub struct ListJobsResponse {
    meta: Meta,
    response: Vec<Job>,
}

#[cfg(test)]
mod tests {
    use serde_json;

    use super::*;

    #[test]
    fn test_deserialization() {
        let json = "{
    \"meta\": {
        \"more_info\": \"https://api.accelo.com/docs/#status-codes\",
        \"message\": \"Everything executed as expected.\",
        \"status\": \"ok\"
    },
    \"response\": [
        {
            \"title\": \"test\",
            \"id\": \"1\",
            \"standing\": \"test\",
            \"against_id\": \"1\",
            \"against_type\": \"1\",
            \"paused\": \"1\",
            \"job_type\": \"1\",
            \"manager\": \"1\",
            \"modified_by\": \"1\",
            \"status\": \"1\",
            \"rate\": \"1\",
            \"affiliation\": \"1\",
            \"custom_id\": \"1\",
            \"against\": \"1\",
            \"staff_bookmarked\": \"1\",
            \"date_created\": \"1\",
            \"date_modified\": \"1\",
            \"date_commenced\": \"1\",
            \"date_started\": \"1\",
            \"date_due\": \"1\",
            \"date_completed\": \"1\",
            \"date_last_interacted\": \"1\",
            \"rate_charged\": \"1\",
            \"company\": \"1\",
            \"job_object_budget\": \"1\",
            \"job_contract\": \"1\"
        },
        {
            \"title\": \"test2\",
            \"id\": \"2\",
            \"standing\": \"test\",
            \"against_id\": \"1\",
            \"affiliation\": \"1\",
            \"custom_id\": \"1\"
        },
        {
            \"id\": \"3\",
            \"title\": \"test3\",
            \"standing\": \"test\",
            \"against_id\": \"1\"
        }
    ]
}";

        let expect = ListJobsResponse {
            meta: Meta {
                more_info: String::from("https://api.accelo.com/docs/#status-codes"),
                message: String::from("Everything executed as expected."),
                status: String::from("ok"),
            },
            response: vec![
                Job {
                    title: String::from("test"),
                    id: String::from("1"),
                    standing: Some(String::from("test")),
                    against_id: Some(String::from("1")),
                    against_type: Some(String::from("1")),
                    paused: Some(String::from("1")),
                    job_type: Some(String::from("1")),
                    manager: Some(String::from("1")),
                    modified_by: Some(String::from("1")),
                    status: Some(String::from("1")),
                    rate: Some(String::from("1")),
                    affiliation: Some(String::from("1")),
                    custom_id: Some(String::from("1")),
                    against: Some(String::from("1")),
                    staff_bookmarked: Some(String::from("1")),
                    date_created: Some(String::from("1")),
                    date_modified: Some(String::from("1")),
                    date_commenced: Some(String::from("1")),
                    date_started: Some(String::from("1")),
                    date_due: Some(String::from("1")),
                    date_completed: Some(String::from("1")),
                    date_last_interacted: Some(String::from("1")),
                    rate_charged: Some(String::from("1")),
                    company: Some(String::from("1")),
                    job_object_budget: Some(String::from("1")),
                    job_contract: Some(String::from("1")),
                },
                Job {
                    title: String::from("test2"),
                    id: String::from("2"),
                    standing: Some(String::from("test")),
                    against_id: Some(String::from("1")),
                    against_type: None,
                    paused: None,
                    job_type: None,
                    manager: None,
                    modified_by: None,
                    status: None,
                    rate: None,
                    affiliation: Some(String::from("1")),
                    custom_id: Some(String::from("1")),
                    against: None,
                    staff_bookmarked: None,
                    date_created: None,
                    date_modified: None,
                    date_commenced: None,
                    date_started: None,
                    date_due: None,
                    date_completed: None,
                    date_last_interacted: None,
                    rate_charged: None,
                    company: None,
                    job_object_budget: None,
                    job_contract: None,
                },
                Job {
                    title: String::from("test3"),
                    id: String::from("3"),
                    standing: Some(String::from("test")),
                    against_id: Some(String::from("1")),
                    against_type: None,
                    paused: None,
                    job_type: None,
                    manager: None,
                    modified_by: None,
                    status: None,
                    rate: None,
                    affiliation: None,
                    custom_id: None,
                    against: None,
                    staff_bookmarked: None,
                    date_created: None,
                    date_modified: None,
                    date_commenced: None,
                    date_started: None,
                    date_due: None,
                    date_completed: None,
                    date_last_interacted: None,
                    rate_charged: None,
                    company: None,
                    job_object_budget: None,
                    job_contract: None,
                },
            ],
        };

        let got: ListJobsResponse = serde_json::from_str(json).expect("Error parsing json");

        assert_eq!(expect, got)
    }
}
