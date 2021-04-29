use async_trait::async_trait;
use reqwest;

use super::models::AuthResponse;

use crate::core::error::AcceloError;

#[async_trait]
pub trait Datasource {
    async fn authenticate(
        &self,
        grant_type: &str,
        scope: &str,
        expires_in: &str,
    ) -> Result<AuthResponse, AcceloError>;
}

struct AuthDatasource {
    base_url: String,
    client_id: String,
    client_secret: String,
}

#[async_trait]
impl Datasource for AuthDatasource {
    async fn authenticate(
        &self,
        grant_type: &str,
        scope: &str,
        expires_in: &str,
    ) -> Result<AuthResponse, AcceloError> {
        let url = format!(
            "{}/oauth2/v0/token?grant_type={}&scope={}&expires_in={}",
            self.base_url, grant_type, scope, expires_in
        );
        let client = reqwest::Client::new();
        let res = client
            .post(&url)
            .basic_auth(&self.client_id, Some(&self.client_secret))
            .send()
            .await
            .map_err(|_| AcceloError::HttpError)?;

        match res.status() {
            reqwest::StatusCode::OK => {
                let data: AuthResponse = res.json().await.map_err(|_| AcceloError::ParseError)?;

                return Ok(data);
            }
            _ => return Err(AcceloError::RequestError),
        }
    }
}
