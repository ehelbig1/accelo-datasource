use std::vec;

use async_trait::async_trait;
use reqwest;

use super::models::{
    count_jobs::CountJobsResponse, get_job::GetJobResponse, list_job_types::ListJobTypesResponse,
    list_jobs::ListJobsResponse,
};
use super::properties::{get_job_properties::*, list_jobs_properties::*};

use crate::core::error::AcceloError;

#[async_trait]
pub trait Datasource<'a> {
    async fn list_jobs(
        &self,
        properties: ListJobsProperties<'a>,
    ) -> Result<ListJobsResponse, AcceloError>;

    async fn get_job(&self, properties: GetJobProperties) -> Result<GetJobResponse, AcceloError>;
    async fn count_jobs(&self) -> Result<CountJobsResponse, AcceloError>;
    async fn list_job_types(&self) -> Result<ListJobTypesResponse, AcceloError>;
}

pub struct ProjectDatasource<'a> {
    base_url: &'a str,
    access_token: &'a str,
}

#[async_trait]
impl<'a> Datasource<'a> for ProjectDatasource<'a> {
    async fn list_jobs(
        &self,
        properties: ListJobsProperties<'a>,
    ) -> Result<ListJobsResponse, AcceloError> {
        let url = format!("{}/api/v0/jobs", &self.base_url);
        let params = properties.to_params();

        let res = reqwest::Client::new()
            .get(&url)
            .query(&params)
            .bearer_auth(&self.access_token)
            .send()
            .await
            .map_err(|_| AcceloError::HttpError)?;

        match res.status() {
            reqwest::StatusCode::OK => {
                let data: ListJobsResponse =
                    res.json().await.map_err(|_| AcceloError::ParseError)?;

                return Ok(data);
            }
            reqwest::StatusCode::UNAUTHORIZED => return Err(AcceloError::UnauthorizedError),
            _ => return Err(AcceloError::RequestError),
        }
    }

    async fn get_job(&self, properties: GetJobProperties) -> Result<GetJobResponse, AcceloError> {
        let url = format!("{}/api/v0/jobs/{}", self.base_url, properties.id());
        let params = properties.to_params();

        let res = reqwest::Client::new()
            .get(&url)
            .query(&params)
            .bearer_auth(&self.access_token)
            .send()
            .await
            .map_err(|_| AcceloError::HttpError)?;

        match res.status() {
            reqwest::StatusCode::OK => {
                let data: GetJobResponse = res.json().await.map_err(|_| AcceloError::ParseError)?;

                return Ok(data);
            }
            reqwest::StatusCode::UNAUTHORIZED => return Err(AcceloError::UnauthorizedError),
            _ => return Err(AcceloError::RequestError),
        }
    }

    async fn count_jobs(&self) -> Result<CountJobsResponse, AcceloError> {
        let url = format!("{}/api/v0/jobs/count", self.base_url);

        let res = reqwest::Client::new()
            .get(&url)
            .bearer_auth(&self.access_token)
            .send()
            .await
            .map_err(|_| AcceloError::HttpError)?;

        match res.status() {
            reqwest::StatusCode::OK => {
                let data: CountJobsResponse =
                    res.json().await.map_err(|_| AcceloError::ParseError)?;

                return Ok(data);
            }
            reqwest::StatusCode::UNAUTHORIZED => return Err(AcceloError::UnauthorizedError),
            _ => return Err(AcceloError::RequestError),
        }
    }

    async fn list_job_types(&self) -> Result<ListJobTypesResponse, AcceloError> {
        let url = format!("{}/api/v0/jobs/types", self.base_url);

        let res = reqwest::Client::new()
            .get(&url)
            .bearer_auth(&self.access_token)
            .send()
            .await
            .map_err(|_| AcceloError::HttpError)?;

        match res.status() {
            reqwest::StatusCode::OK => {
                let data: ListJobTypesResponse =
                    res.json().await.map_err(|_| AcceloError::ParseError)?;

                return Ok(data);
            }
            reqwest::StatusCode::UNAUTHORIZED => return Err(AcceloError::UnauthorizedError),
            _ => return Err(AcceloError::RequestError),
        }
    }
}
