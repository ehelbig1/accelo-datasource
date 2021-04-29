use super::fields::*;

#[derive(Debug, PartialEq)]
pub struct GetJobProperties {
    fields: Option<Fields>,
    id: usize,
}

impl GetJobProperties {
    pub fn new(id: usize) -> Self {
        Self { fields: None, id }
    }

    pub fn fields(mut self, fields: Vec<Field>) -> Self {
        self.fields = Some(Fields::new(fields));

        self
    }

    pub fn id(&self) -> &usize {
        &self.id
    }

    pub fn to_params(&self) -> Vec<(&str, String)> {
        let mut params = vec![];

        if let Some(fields) = &self.fields {
            params.push(fields.to_param());
        }

        params
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let expect = GetJobProperties {
            fields: None,
            id: 1,
        };

        let got = GetJobProperties::new(1);

        assert_eq!(expect, got)
    }

    #[test]
    fn test_fields() {
        let expect = GetJobProperties {
            fields: Some(Fields::new(vec![Field::Status])),
            id: 1,
        };

        let got = GetJobProperties::new(1).fields(vec![Field::Status]);

        assert_eq!(expect, got)
    }
}
