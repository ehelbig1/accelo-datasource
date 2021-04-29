#[derive(Debug, PartialEq)]
pub struct Fields(pub Vec<Field>);

impl Fields {
    pub fn new(fields: Vec<Field>) -> Self {
        Self(fields)
    }

    pub fn to_param(&self) -> (&str, String) {
        let mut param_values = String::new();

        for field in &self.0 {
            param_values.push_str(&field.as_str());
            param_values.push_str(",");
        }

        println!("field params: {}", param_values);

        ("_fields", param_values.trim_end_matches(",").to_string())
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Field {
    Standing,
    AgainstId,
    AgainstType,
    Paused,
    JobType,
    Manager,
    ModifiedBy,
    Status,
    Rate,
    Affiliation,
    CustomId,
    Against,
    StaffBookmarked,
    DateCreated,
    DateModified,
    DateCommenced,
    DateStarted,
    DateDue,
    DateCompleted,
    DateLastInteracted,
    RateCharged,
    Company,
    JobObjectBudget,
    JobContract,
}

impl Field {
    pub fn as_str(&self) -> &str {
        match self {
            Field::Standing => "standing",
            Field::AgainstId => "against_id",
            Field::AgainstType => "against_type",
            Field::Paused => "paused",
            Field::JobType => "job_type",
            Field::Manager => "manager",
            Field::ModifiedBy => "modified_by",
            Field::Status => "status",
            Field::Rate => "rate",
            Field::Affiliation => "affiliation",
            Field::CustomId => "custom_id",
            Field::Against => "against",
            Field::StaffBookmarked => "staff_bookmarked",
            Field::DateCreated => "date_created",
            Field::DateModified => "date_modified",
            Field::DateCommenced => "date_commenced",
            Field::DateStarted => "date_started",
            Field::DateDue => "date_due",
            Field::DateCompleted => "date_completed",
            Field::DateLastInteracted => "date_last_interacted",
            Field::RateCharged => "rate_charged",
            Field::Company => "company",
            Field::JobObjectBudget => "job_object_budget",
            Field::JobContract => "job_contract",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_against_id() {
        let fields = Fields::new(vec![Field::AgainstId]);

        let expect = ("_fields", String::from("against_id"));
        let got = fields.to_param();

        assert_eq!(expect, got)
    }

    #[test]
    fn test_field_against_type_and_custom_id() {
        let fields = Fields::new(vec![Field::AgainstType, Field::CustomId]);

        let expect = ("_fields", String::from("against_type,custom_id"));
        let got = fields.to_param();

        assert_eq!(expect, got)
    }
}
