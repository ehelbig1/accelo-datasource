use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct AuthResponse {
    account_id: String,
    deployment: String,
    deployment_name: String,
    expires_in: usize,
    pub access_token: String,
    token_type: String,
    deployment_uri: String,
    account_details: AccountDetails,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct AccountDetails {
    mobile: Option<String>,
}

#[cfg(test)]
mod tests {
    use serde_json;

    use super::*;

    #[test]
    fn test_deserialization() {
        let json = "{
    \"account_id\": \"test\",
    \"deployment\": \"test\",
    \"deployment_name\": \"test\",
    \"expires_in\": 3600,
    \"access_token\": \"test\",
    \"token_type\": \"test\",
    \"deployment_uri\": \"test\",
    \"account_details\": {
        \"mobile\": null,
        \"user_defined_titles\": {
            \"retainer_period\": {
                \"singular\": \"Support Contract Period\",
                \"plural\": \"Support Contract Periods\"
            },
            \"job\": {
                \"singular\": \"Project\",
                \"plural\": \"Projects\"
            },
            \"authorities\": {
                \"plural\": \"authorities\",
                \"singular\": \"authorities\"
            },
            \"sale\": {
                \"plural\": \"Sales\",
                \"singular\": \"Sale\"
            },
            \"site\": {
                \"singular\": \"Site\",
                \"plural\": \"Sites\"
            },
            \"expense\": {
                \"plural\": \"Expenses\",
                \"singular\": \"Expense\"
            },
            \"contributor\": {
                \"singular\": \"Contributor\",
                \"plural\": \"Contributors\"
            },
            \"milestone\": {
                \"singular\": \"Milestone\",
                \"plural\": \"Milestones\"
            },
            \"campaign\": {
                \"plural\": \"Campaigns\",
                \"singular\": \"Campaign\"
            },
            \"milestone_manager\": {
                \"singular\": \"BD Lead\",
                \"plural\": \"Managers\"
            },
            \"quote\": {
                \"singular\": \"Quote\",
                \"plural\": \"Quotes\"
            },
            \"contract_manager\": {
                \"singular\": \"Manager\",
                \"plural\": \"Managers\"
            },
            \"accounts\": {
                \"singular\": \"Billing\",
                \"plural\": \"Billing\"
            },
            \"purchase_order\": {
                \"plural\": \"Purchase Orders\",
                \"singular\": \"Purchase Order\"
            },
            \"contract_period\": {
                \"singular\": \"Support Contract Period\",
                \"plural\": \"Support Contract Periods\"
            },
            \"deployment\": {
                \"singular\": \"Deployment\",
                \"plural\": \"Deployments\"
            },
            \"account_purchase\": {
                \"singular\": \"Purchase\",
                \"plural\": \"Purchases\"
            },
            \"prospect_manager\": {
                \"plural\": \"Salespeople\",
                \"singular\": \"BizDev Lead\"
            },
            \"period\": {
                \"plural\": \"Periods\",
                \"singular\": \"Period\"
            },
            \"contact\": {
                \"plural\": \"Contacts\",
                \"singular\": \"Contact\"
            },
            \"issue_manager\": {
                \"singular\": \"Assignee\",
                \"plural\": \"Assignees\"
            },
            \"task\": {
                \"singular\": \"Task\",
                \"plural\": \"Tasks\"
            },
            \"job_manager\": {
                \"plural\": \"Managers\",
                \"singular\": \"BD Lead\"
            },
            \"prospect\": {
                \"plural\": \"Sales\",
                \"singular\": \"Sale\"
            },
            \"division\": {
                \"plural\": \"Divisions\",
                \"singular\": \"Division\"
            },
            \"affiliation\": {
                \"singular\": \"Contact\",
                \"plural\": \"Contacts\"
            },
            \"bookmark\": {
                \"plural\": \"Favorites\",
                \"singular\": \"Favorite\"
            },
            \"material\": {
                \"singular\": \"Material\",
                \"plural\": \"Materials\"
            },
            \"account_invoice\": {
                \"singular\": \"Invoice\",
                \"plural\": \"Invoices\"
            },
            \"purchase\": {
                \"plural\": \"Purchases\",
                \"singular\": \"Purchase\"
            },
            \"service\": {
                \"singular\": \"Service\",
                \"plural\": \"Services\"
            },
            \"company\": {
                \"singular\": \"Company\",
                \"plural\": \"Companies\"
            },
            \"contract\": {
                \"singular\": \"Support Contract\",
                \"plural\": \"Support Contracts\"
            },
            \"segmentation\": {
                \"singular\": \"Category\",
                \"plural\": \"Categories\"
            },
            \"project\": {
                \"singular\": \"Project\",
                \"plural\": \"Projects\"
            },
            \"campaign_action\": {
                \"singular\": \"Campaign Communication\",
                \"plural\": \"Campaign Communications\"
            },
            \"estimate\": {
                \"plural\": \"Estimates\",
                \"singular\": \"Estimate\"
            },
            \"request\": {
                \"singular\": \"Request\",
                \"plural\": \"Requests\"
            },
            \"asset\": {
                \"plural\": \"Assets\",
                \"singular\": \"Asset\"
            },
            \"retainer\": {
                \"singular\": \"Support Contract\",
                \"plural\": \"Support Contracts\"
            },
            \"location\": {
                \"singular\": \"Location\",
                \"plural\": \"Locations\"
            },
            \"issue\": {
                \"plural\": \"Tickets\",
                \"singular\": \"Ticket\"
            },
            \"tax\": {
                \"singular\": \"Tax\",
                \"plural\": \"Taxes\"
            }
        },
        \"position\": \"Software Engineer\",
        \"email\": \"test\",
        \"title\": null,
        \"access_level\": \"admin\",
        \"locale\": {
            \"currency\": {
                \"symbol\": \"$\"
            }
        },
        \"username\": \"test\",
        \"phone\": null,
        \"initials\": null,
        \"firstname\": \"test\",
        \"id\": \"test\",
        \"surname\": \"test\",
        \"fax\": null,
        \"user_access\": {
            \"job\": {
                \"view\": 1,
                \"manages\": 0,
                \"add\": 1,
                \"admin\": 1,
                \"dashboard\": 1
            },
            \"task\": {
                \"view\": 1,
                \"manages\": 0,
                \"add\": 1,
                \"admin\": 1,
                \"dashboard\": 1
            },
            \"contact\": {
                \"view\": 1,
                \"manages\": 0,
                \"add\": 1,
                \"dashboard\": 1,
                \"admin\": 1
            },
            \"expense\": {
                \"add\": 1,
                \"dashboard\": \"\",
                \"admin\": 1,
                \"view\": 1,
                \"manages\": 0
            },
            \"timer\": {
                \"view\": 1,
                \"manages\": 0,
                \"add\": 1,
                \"admin\": \"\",
                \"dashboard\": \"\"
            },
            \"milestone\": {
                \"view\": 1,
                \"manages\": 0,
                \"add\": 1,
                \"dashboard\": \"\",
                \"admin\": 1
            },
            \"division\": {
                \"dashboard\": \"\",
                \"admin\": \"\",
                \"add\": \"\",
                \"manages\": 0,
                \"view\": 1
            },
            \"account_invoice\": {
                \"view\": 0,
                \"manages\": 0,
                \"add\": 0,
                \"admin\": 1,
                \"dashboard\": \"\"
            },
            \"purchase\": {
                \"manages\": 0,
                \"view\": 0,
                \"admin\": 1,
                \"dashboard\": \"\",
                \"add\": 0
            },
            \"company\": {
                \"dashboard\": 1,
                \"admin\": 1,
                \"add\": 1,
                \"manages\": 0,
                \"view\": 1
            },
            \"resource\": {
                \"dashboard\": \"\",
                \"admin\": 1,
                \"add\": 1,
                \"manages\": 0,
                \"view\": 1
            },
            \"invoice\": {
                \"manages\": 0,
                \"view\": 0,
                \"admin\": 1,
                \"dashboard\": \"\",
                \"add\": 0
            },
            \"activity\": {
                \"manages\": 0,
                \"view\": 1,
                \"admin\": 1,
                \"dashboard\": 1,
                \"add\": 1
            },
            \"request\": {
                \"view\": 1,
                \"manages\": 0,
                \"add\": 1,
                \"admin\": 1,
                \"dashboard\": \"\"
            },
            \"asset\": {
                \"manages\": 0,
                \"view\": 1,
                \"admin\": 1,
                \"dashboard\": \"\",
                \"add\": 1
            },
            \"account_purchase\": {
                \"view\": 0,
                \"manages\": 0,
                \"add\": 0,
                \"dashboard\": \"\",
                \"admin\": 1
            }
        }
    }
}";

        let expect = AuthResponse {
            account_id: String::from("test"),
            deployment: String::from("test"),
            deployment_name: String::from("test"),
            expires_in: 3600,
            access_token: String::from("test"),
            token_type: String::from("test"),
            deployment_uri: String::from("test"),
            account_details: AccountDetails { mobile: None },
        };

        let got: AuthResponse = serde_json::from_str(json).expect("Failed to parse json str");

        assert_eq!(expect, got)
    }
}
