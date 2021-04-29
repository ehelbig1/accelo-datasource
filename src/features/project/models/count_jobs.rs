use serde::Deserialize;

use super::meta::*;

#[derive(Debug, Deserialize, PartialEq)]
pub struct CountJobsResponse {
    meta: Meta,
    response: Count,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Count {
    count: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json;

    #[test]
    fn test_deserialization() {
        let json = "{
            \"response\": {
                \"count\": \"1\"
            },
            \"meta\": {
                \"status\": \"ok\",
                \"message\": \"Everything executed as expected.\",
                \"more_info\": \"https://api.accelo.com/docs/#status-codes\"
            }
        }";

        let expect = CountJobsResponse {
            meta: Meta {
                more_info: String::from("https://api.accelo.com/docs/#status-codes"),
                message: String::from("Everything executed as expected."),
                status: String::from("ok"),
            },
            response: Count {
                count: String::from("1"),
            },
        };

        let got: CountJobsResponse = serde_json::from_str(json).expect("Error parsing json");

        assert_eq!(expect, got)
    }
}
