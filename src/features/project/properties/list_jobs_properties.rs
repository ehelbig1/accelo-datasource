use super::fields::*;

pub struct ListJobsProperties<'a> {
    filters: Option<Filters<'a>>,
    fields: Option<Fields>,
}

impl<'a> ListJobsProperties<'a> {
    pub fn new() -> Self {
        Self {
            filters: None,
            fields: None,
        }
    }

    pub fn filters(mut self, filters: Filters<'a>) -> Self {
        self.filters = Some(filters);

        self
    }

    pub fn fields(mut self, fields: Fields) -> Self {
        self.fields = Some(fields);

        self
    }

    pub fn to_params(&self) -> Vec<(&str, String)> {
        let mut params = vec![];

        if let Some(filters) = &self.filters {
            params.push(filters.to_param());
        }

        if let Some(fields) = &self.fields {
            params.push(fields.to_param());
        }

        params
    }
}

pub struct Filters<'a> {
    ids: Option<Vec<usize>>,
    standings: Option<Vec<&'a str>>,
    against_ids: Option<Vec<usize>>,
    against_types: Option<Vec<&'a str>>,
    paused: Option<bool>,
    job_types: Option<Vec<usize>>,
    managers: Option<Vec<usize>>,
    modified_by: Option<Vec<usize>>,
    statuses: Option<Vec<usize>>,
    rates: Option<Vec<isize>>,
    affiliations: Option<Vec<usize>>,
    custom_ids: Option<Vec<usize>>,
    date_created_after: Option<usize>,
    date_created_before: Option<usize>,
    date_modified_after: Option<usize>,
    date_modified_before: Option<usize>,
    date_started_after: Option<usize>,
    date_started_before: Option<usize>,
    date_commenced_after: Option<usize>,
    date_commenced_before: Option<usize>,
    date_due_after: Option<usize>,
    date_due_before: Option<usize>,
    date_completed_after: Option<usize>,
    date_completed_before: Option<usize>,
    search: Option<&'a str>,
    empty: Option<Fields>,
}

impl<'a> Filters<'a> {
    pub fn new() -> Self {
        Self {
            ids: None,
            standings: None,
            against_ids: None,
            against_types: None,
            paused: None,
            job_types: None,
            managers: None,
            modified_by: None,
            statuses: None,
            rates: None,
            affiliations: None,
            custom_ids: None,
            date_created_after: None,
            date_created_before: None,
            date_modified_after: None,
            date_modified_before: None,
            date_started_after: None,
            date_started_before: None,
            date_commenced_after: None,
            date_commenced_before: None,
            date_due_after: None,
            date_due_before: None,
            date_completed_after: None,
            date_completed_before: None,
            search: None,
            empty: None,
        }
    }

    pub fn ids(mut self, ids: Vec<usize>) -> Self {
        self.ids = Some(ids);

        self
    }

    pub fn standings(mut self, standings: Vec<&'a str>) -> Self {
        self.standings = Some(standings);

        self
    }

    pub fn against_ids(mut self, against_ids: Vec<usize>) -> Self {
        self.against_ids = Some(against_ids);

        self
    }

    pub fn against_types(mut self, against_types: Vec<&'a str>) -> Self {
        self.against_types = Some(against_types);

        self
    }

    pub fn paused(mut self, paused: bool) -> Self {
        self.paused = Some(paused);

        self
    }

    pub fn job_types(mut self, job_types: Vec<usize>) -> Self {
        self.job_types = Some(job_types);

        self
    }

    pub fn managers(mut self, managers: Vec<usize>) -> Self {
        self.managers = Some(managers);

        self
    }

    pub fn modified_by(mut self, modified_by: Vec<usize>) -> Self {
        self.modified_by = Some(modified_by);

        self
    }

    pub fn statuses(mut self, statuses: Vec<usize>) -> Self {
        self.statuses = Some(statuses);

        self
    }

    pub fn rates(mut self, rates: Vec<isize>) -> Self {
        self.rates = Some(rates);

        self
    }

    pub fn affiliations(mut self, affiliations: Vec<usize>) -> Self {
        self.affiliations = Some(affiliations);

        self
    }

    pub fn custom_ids(mut self, custom_ids: Vec<usize>) -> Self {
        self.custom_ids = Some(custom_ids);

        self
    }

    pub fn date_created_after(mut self, date_created_after: usize) -> Self {
        self.date_created_after = Some(date_created_after);

        self
    }

    pub fn date_created_before(mut self, date_created_before: usize) -> Self {
        self.date_created_before = Some(date_created_before);

        self
    }

    pub fn date_modified_after(mut self, date_created_after: usize) -> Self {
        self.date_created_after = Some(date_created_after);

        self
    }

    pub fn date_modified_before(mut self, date_created_before: usize) -> Self {
        self.date_created_before = Some(date_created_before);

        self
    }

    pub fn date_started_after(mut self, date_created_after: usize) -> Self {
        self.date_created_after = Some(date_created_after);

        self
    }

    pub fn date_started_before(mut self, date_created_before: usize) -> Self {
        self.date_created_before = Some(date_created_before);

        self
    }

    pub fn date_commenced_after(mut self, date_created_after: usize) -> Self {
        self.date_created_after = Some(date_created_after);

        self
    }

    pub fn date_commenced_before(mut self, date_created_before: usize) -> Self {
        self.date_created_before = Some(date_created_before);

        self
    }

    pub fn date_due_after(mut self, date_created_after: usize) -> Self {
        self.date_created_after = Some(date_created_after);

        self
    }

    pub fn date_due_before(mut self, date_created_before: usize) -> Self {
        self.date_created_before = Some(date_created_before);

        self
    }

    pub fn date_completed_after(mut self, date_created_after: usize) -> Self {
        self.date_created_after = Some(date_created_after);

        self
    }

    pub fn date_completed_before(mut self, date_created_before: usize) -> Self {
        self.date_created_before = Some(date_created_before);

        self
    }

    pub fn search(mut self, criteria: &'a str) -> Self {
        self.search = Some(criteria);

        self
    }

    pub fn empty(mut self, fields: Fields) -> Self {
        self.empty = Some(fields);

        self
    }

    pub fn to_param(&self) -> (&str, String) {
        let mut param_values = String::new();

        if let Some(ids) = &self.ids {
            for id in ids {
                param_values.push_str(&format!("id({})", id));
                param_values.push(',');
            }
        }

        if let Some(standings) = &self.standings {
            for standing in standings {
                param_values.push_str(&format!("standing({})", standing));
                param_values.push(',');
            }
        }

        if let Some(against_ids) = &self.against_ids {
            for against_id in against_ids {
                param_values.push_str(&format!("against_id({})", against_id));
                param_values.push(',');
            }
        }

        if let Some(against_types) = &self.against_types {
            for against_type in against_types {
                param_values.push_str(&format!("against_type({})", against_type));
                param_values.push(',');
            }
        }

        if let Some(paused) = &self.paused {
            param_values.push_str(&format!("paused({})", paused));
            param_values.push(',');
        }

        if let Some(job_types) = &self.job_types {
            for job_type in job_types {
                param_values.push_str(&format!("job_type({})", job_type));
                param_values.push(',');
            }
        }

        if let Some(managers) = &self.managers {
            for manager in managers {
                param_values.push_str(&format!("manager({})", manager));
                param_values.push(',');
            }
        }

        if let Some(modified_by) = &self.modified_by {
            for user in modified_by {
                param_values.push_str(&format!("modified_by({})", user));
                param_values.push(',');
            }
        }

        if let Some(statuses) = &self.statuses {
            for status in statuses {
                param_values.push_str(&format!("status({})", status));
                param_values.push(',');
            }
        }

        if let Some(rates) = &self.rates {
            for rate in rates {
                param_values.push_str(&format!("rate({})", rate));
                param_values.push(',');
            }
        }

        if let Some(affiliations) = &self.affiliations {
            for affiliation in affiliations {
                param_values.push_str(&format!("affiliation({})", affiliation));
                param_values.push(',');
            }
        }

        if let Some(custom_ids) = &self.custom_ids {
            for custom_id in custom_ids {
                param_values.push_str(&format!("custom_id({})", custom_id));
                param_values.push(',');
            }
        }

        if let Some(fields) = &self.empty {
            for field in &fields.0 {
                param_values.push_str(&format!("empty({})", field.as_str()));
                param_values.push(',');
            }
        }

        if let Some(criteria) = self.search {
            param_values.push_str(&format!("search({})", criteria));
            param_values.push(',');
        }

        ("_filters", param_values.trim_end_matches(",").to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_by_id() {
        let properties = ListJobsProperties::new().filters(Filters::new().ids(vec![1]));

        let expect = ("_filters", String::from("id(1)"));
        if let Some(filters) = &properties.filters {
            let got = filters.to_param();

            assert_eq!(expect, got)
        }
    }

    #[test]
    fn test_filter_by_ids() {
        let properties = ListJobsProperties::new().filters(Filters::new().ids(vec![1, 2]));

        let expect = ("_filters", String::from("id(1),id(2)"));
        if let Some(filters) = &properties.filters {
            let got = filters.to_param();

            assert_eq!(expect, got)
        }
    }

    #[test]
    fn test_filter_by_standing_and_status() {
        let properties = ListJobsProperties::new()
            .filters(Filters::new().standings(vec!["active"]).statuses(vec![1]));

        let expect = ("_filters", String::from("standing(active),status(1)"));
        if let Some(filters) = &properties.filters {
            let got = filters.to_param();

            assert_eq!(expect, got)
        }
    }

    #[test]
    fn test_field_affiliation_and_filter_by_job_type() {
        let properties = ListJobsProperties::new()
            .fields(Fields::new(vec![Field::Affiliation]))
            .filters(Filters::new().job_types(vec![1]));

        let expect = vec![
            ("_fields", String::from("affiliation")),
            ("_filters", String::from("job_type(1)")),
        ];

        let mut got: Vec<(&str, String)> = vec![];

        if let Some(fields) = &properties.fields {
            let param = fields.to_param();

            got.push(param);
        }

        if let Some(filters) = &properties.filters {
            let param = filters.to_param();

            got.push(param);
        }

        assert_eq!(expect, got)
    }

    #[test]
    fn test_filter_by_standing_job_type_and_empty_manager() {
        let properties = ListJobsProperties::new().filters(
            Filters::new()
                .job_types(vec![1])
                .standings(vec!["active"])
                .empty(Fields::new(vec![Field::Manager])),
        );

        let expect = vec![(
            "_filters",
            String::from("standing(active),job_type(1),empty(manager)"),
        )];

        let mut got: Vec<(&str, String)> = vec![];

        if let Some(fields) = &properties.fields {
            let param = fields.to_param();

            got.push(param);
        }

        if let Some(filters) = &properties.filters {
            let param = filters.to_param();

            got.push(param);
        }

        assert_eq!(expect, got)
    }

    #[test]
    fn test_search() {
        let properties = ListJobsProperties::new().filters(Filters::new().search("test"));

        let expect = vec![("_filters", String::from("search(test)"))];

        let mut got: Vec<(&str, String)> = vec![];

        if let Some(fields) = &properties.fields {
            let param = fields.to_param();

            got.push(param);
        }

        if let Some(filters) = &properties.filters {
            let param = filters.to_param();

            got.push(param);
        }

        assert_eq!(expect, got)
    }
}
