use serde::Deserialize;

use super::job_type::*;
use super::meta::*;

#[derive(Debug, Deserialize, PartialEq)]
pub struct ListJobTypesResponse {
    meta: Meta,
    response: Vec<JobType>,
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json;

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
                    \"id\": \"1\"
                },
                {
                    \"title\": \"test2\",
                    \"id\": \"2\"
                },
                {
                    \"id\": \"3\",
                    \"title\": \"test3\"
                }
            ]
        }";

        let expect = ListJobTypesResponse {
            meta: Meta {
                more_info: String::from("https://api.accelo.com/docs/#status-codes"),
                message: String::from("Everything executed as expected."),
                status: String::from("ok"),
            },
            response: vec![
                JobType {
                    title: String::from("test"),
                    id: String::from("1"),
                },
                JobType {
                    title: String::from("test2"),
                    id: String::from("2"),
                },
                JobType {
                    title: String::from("test3"),
                    id: String::from("3"),
                },
            ],
        };

        let got: ListJobTypesResponse = serde_json::from_str(json).expect("Error parsing json");

        assert_eq!(expect, got)
    }
}
