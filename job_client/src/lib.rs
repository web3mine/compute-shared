use async_trait::async_trait;
use filecoin_spec::{SectorId, StorageProviderId};
use hyper::{http, StatusCode};
use job::{
    sealing::{
        C1Output, C2Output, PC1Output, PC2Output, PCOutput, SealingJob, C1, C2, PC, PC1, PC2,
    },
    JobType,
};
use mockall::automock;
use serde::{Deserialize, Serialize};

pub const ADD_JOBS_URL: &str = "/job";
pub const GET_JOBS_URL: &str = "/job/:count/:job_type";
pub const GET_ALL_JOBS_URL: &str = "/job/all/:storage_provider_id/:job_type";
pub const GET_JOB_INPUT_URI: &str = "/job/input/:storage_provider_id/:sector_id/:job_type";
pub const GET_JOB_STATE_URL: &str = "/job/state/:storage_provider_id/:sector_id/:job_type";

pub const SUBMIT_OUTPUT_URL: &str = "/job/output";
pub const GET_OUTPUT_URL: &str = "/job/output/:storage_provider_id/:sector_id/:job_type";
pub const FAIL_JOB_URL: &str = "/job/fail";

pub const GENERATE_TICKET_URL: &str = "/job/ticket/:storage_provider_id";
pub const GET_SECTOR_PATHS_URL: &str = "/sector/paths/:storage_provider_id/:sector_id";

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Json(#[from] serde_json::Error),

    #[error("{0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Job doesn't exist")]
    JobNotExist,

    #[error("Not enough jobs({0})")]
    NotEnoughJobs(usize),

    #[error("Error while fetching jobs: {0}")]
    FetchJobs(String),

    #[error("Error while submitting output: {0}")]
    SubmitOutput(String),

    #[error("Error while fetching output: {0}")]
    FetchOutput(String),

    #[error("Error while submitting job failure: {0}")]
    FailJob(String),

    #[error("Error while fetching job state: {0}")]
    GetState(String),
}

pub struct JobOutput<SealingJobT: SealingJob>(pub Result<SealingJobT::Output, String>);

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "job_type")]
pub enum JobHttp {
    PC1(PC1),
    PC2(PC2),
    C1(C1),
    C2(C2),
    PC(PC),
}

impl From<PC1> for JobHttp {
    fn from(job: PC1) -> Self {
        JobHttp::PC1(job)
    }
}

impl From<PC2> for JobHttp {
    fn from(job: PC2) -> Self {
        JobHttp::PC2(job)
    }
}

impl From<C1> for JobHttp {
    fn from(job: C1) -> Self {
        JobHttp::C1(job)
    }
}

impl From<C2> for JobHttp {
    fn from(job: C2) -> Self {
        JobHttp::C2(job)
    }
}

impl From<PC> for JobHttp {
    fn from(job: PC) -> Self {
        JobHttp::PC(job)
    }
}

impl From<JobHttp> for PC1 {
    fn from(value: JobHttp) -> Self {
        match value {
            JobHttp::PC1(job) => job,
            _ => panic!("Invalid job PC1 job {:?}", value),
        }
    }
}

impl From<JobHttp> for PC2 {
    fn from(value: JobHttp) -> Self {
        match value {
            JobHttp::PC2(job) => job,
            _ => panic!("Invalid job PC2 job {:?}", value),
        }
    }
}

impl From<JobHttp> for C1 {
    fn from(value: JobHttp) -> Self {
        match value {
            JobHttp::C1(job) => job,
            _ => panic!("Invalid job C1 job {:?}", value),
        }
    }
}

impl From<JobHttp> for C2 {
    fn from(value: JobHttp) -> Self {
        match value {
            JobHttp::C2(job) => job,
            _ => panic!("Invalid job C2 job {:?}", value),
        }
    }
}

impl From<JobHttp> for PC {
    fn from(value: JobHttp) -> Self {
        match value {
            JobHttp::PC(job) => job,
            _ => panic!("Invalid job PC job {:?}", value),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "job_type", content = "output")]
pub enum JobOutputHttp {
    PC1(PC1Output),
    PC2(PC2Output),
    C1(C1Output),
    C2(C2Output),
    PC(PCOutput),
}

impl From<PC1Output> for JobOutputHttp {
    fn from(output: PC1Output) -> Self {
        JobOutputHttp::PC1(output)
    }
}

impl From<PC2Output> for JobOutputHttp {
    fn from(output: PC2Output) -> Self {
        JobOutputHttp::PC2(output)
    }
}

impl From<C1Output> for JobOutputHttp {
    fn from(output: C1Output) -> Self {
        JobOutputHttp::C1(output)
    }
}

impl From<C2Output> for JobOutputHttp {
    fn from(output: C2Output) -> Self {
        JobOutputHttp::C2(output)
    }
}

impl From<PCOutput> for JobOutputHttp {
    fn from(output: PCOutput) -> Self {
        JobOutputHttp::PC(output)
    }
}

impl From<JobOutputHttp> for PC1Output {
    fn from(value: JobOutputHttp) -> Self {
        match value {
            JobOutputHttp::PC1(output) => output,
            _ => panic!("Invalid PC1 output {:?}", value),
        }
    }
}

impl From<JobOutputHttp> for PC2Output {
    fn from(value: JobOutputHttp) -> Self {
        match value {
            JobOutputHttp::PC2(output) => output,
            _ => panic!("Invalid PC2 output {:?}", value),
        }
    }
}

impl From<JobOutputHttp> for C1Output {
    fn from(value: JobOutputHttp) -> Self {
        match value {
            JobOutputHttp::C1(output) => output,
            _ => panic!("Invalid C1 output {:?}", value),
        }
    }
}

impl From<JobOutputHttp> for C2Output {
    fn from(value: JobOutputHttp) -> Self {
        match value {
            JobOutputHttp::C2(output) => output,
            _ => panic!("Invalid C2 output {:?}", value),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct GetSealingJobsResponse {
    pub jobs: Vec<JobHttp>,
}

#[derive(Serialize, Debug)]
pub struct SubmitSealingJobOutput {
    pub storage_provider_id: StorageProviderId,
    pub sector_id: SectorId,

    #[serde(flatten)]
    pub job: JobOutputHttp,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FailJob {
    pub storage_provider_id: StorageProviderId,
    pub sector_id: SectorId,
    pub job_type: JobType,
    pub error: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum JobState {
    Pending,
    Done,
    Failed,
}

#[automock]
#[async_trait]
pub trait SealingJobManagerClient: Clone + Send + Sync {
    async fn add_job<SealingJobT: SealingJob + Into<JobHttp> + 'static>(
        &self,
        job: SealingJobT,
    ) -> Result<(), Error>;

    async fn request_jobs<SealingJobT: SealingJob + From<JobHttp> + 'static>(
        &self,
        count: usize,
    ) -> Result<Vec<SealingJobT>, Error>;

    async fn submit_job_output<SealingJobT: SealingJob + 'static>(
        &self,
        storage_provider_id: StorageProviderId,
        sector_id: SectorId,
        output: SealingJobT::Output,
    ) -> Result<(), Error>
    where
        JobOutputHttp: From<SealingJobT::Output>;

    async fn get_job_output<SealingJobT: SealingJob + 'static>(
        &self,
        storage_provider_id: StorageProviderId,
        sector_id: SectorId,
    ) -> Result<Option<JobOutput<SealingJobT>>, Error>
    where
        SealingJobT::Output: From<JobOutputHttp>;

    async fn get_job_input<SealingJobT: SealingJob + From<JobHttp> + 'static>(
        &self,
        storage_provider_id: StorageProviderId,
        sector_id: SectorId,
    ) -> Result<Option<SealingJobT>, Error>
    where
        SealingJobT::Output: From<JobOutputHttp>;

    async fn fail_job<SealingJobT: SealingJob + 'static>(
        &self,
        storage_provider_id: StorageProviderId,
        sector_id: SectorId,
        error: &str,
    ) -> Result<(), Error>;

    async fn get_job_state<SealingJobT: SealingJob + 'static>(
        &self,
        storage_provider_id: StorageProviderId,
        sector_id: SectorId,
    ) -> Result<Option<JobState>, Error>;
}

impl Clone for MockSealingJobManagerClient {
    fn clone(&self) -> Self {
        todo!()
    }
}

#[derive(Clone)]
pub struct SealingJobManagerHttpClient {
    http_client: reqwest::Client,
    add_jobs_uri: String,
    request_jobs_uri: String,
    submit_output_uri: String,
    get_job_output_uri: String,
    get_job_input_uri: String,
    fail_job_uri: String,
    get_job_state_uri: String,
}

impl SealingJobManagerHttpClient {
    pub fn new(uri: String) -> Self {
        Self {
            http_client: reqwest::Client::new(),
            add_jobs_uri: uri.clone() + ADD_JOBS_URL,
            request_jobs_uri: uri.clone() + GET_JOBS_URL,
            submit_output_uri: uri.clone() + SUBMIT_OUTPUT_URL,
            get_job_output_uri: uri.clone() + GET_OUTPUT_URL,
            get_job_input_uri: uri.clone() + GET_JOB_INPUT_URI,
            fail_job_uri: uri.clone() + FAIL_JOB_URL,
            get_job_state_uri: uri + GET_JOB_STATE_URL,
        }
    }
}

#[async_trait]
impl SealingJobManagerClient for SealingJobManagerHttpClient {
    async fn add_job<SealingJobT: SealingJob + Into<JobHttp> + 'static>(
        &self,
        job: SealingJobT,
    ) -> Result<(), Error> {
        let job: JobHttp = job.into();
        let response = reqwest::Client::new()
            .post(&self.add_jobs_uri)
            .body(serde_json::to_string(&job)?)
            .header(http::header::CONTENT_TYPE, "application/json")
            .send()
            .await?;

        if response.status() != StatusCode::OK {
            let resp = response.text().await?;
            tracing::error!("Failed to submit jobs: {}", resp,);

            return Err(Error::FetchJobs(resp));
        } else {
            tracing::info!("Succesfully submited jobs");
        }

        Ok(())
    }

    async fn request_jobs<SealingJobT: SealingJob + From<JobHttp> + 'static>(
        &self,
        count: usize,
    ) -> Result<Vec<SealingJobT>, Error> {
        let job_type = SealingJobT::job_type().to_string();
        let uri = self
            .request_jobs_uri
            .replace(":count", count.to_string().as_str());
        let uri = uri.replace(":job_type", job_type.as_str());

        tracing::debug!(
            "Requesting {} {} jobs from Job Management Server",
            count,
            job_type
        );
        let response = self
            .http_client
            .get(uri)
            .header(http::header::CONTENT_TYPE, "application/json")
            .send()
            .await?;

        if response.status() == StatusCode::NO_CONTENT {
            tracing::error!("{} jobs not available", count);
            return Err(Error::NotEnoughJobs(count));
        }

        if response.status() != StatusCode::OK {
            let resp = response.text().await?;
            tracing::error!("Error while fetching jobs: {}", &resp);
            return Err(Error::FetchJobs(resp));
        }

        let response: GetSealingJobsResponse = response.json().await?;
        tracing::trace!("request_jobs response {:?}", response);

        let jobs: Vec<SealingJobT> = response.jobs.into_iter().map(|job| job.into()).collect();
        Ok(jobs)
    }

    async fn get_job_input<SealingJobT: SealingJob + From<JobHttp> + 'static>(
        &self,
        storage_provider_id: StorageProviderId,
        sector_id: SectorId,
    ) -> Result<Option<SealingJobT>, Error>
    where
        SealingJobT::Output: From<JobOutputHttp>,
    {
        let job_type = SealingJobT::job_type().to_string();
        let uri = self
            .get_job_input_uri
            .replace(":storage_provider_id", &storage_provider_id.0.to_string())
            .replace(":sector_id", &sector_id.0.to_string())
            .replace(":job_type", job_type.as_str());

        tracing::debug!(
            "Requesting {} input for storage_provider_id: {}, sector_id: {}",
            job_type,
            storage_provider_id.0,
            sector_id.0
        );
        let response = self
            .http_client
            .get(uri)
            .header(http::header::CONTENT_TYPE, "application/json")
            .send()
            .await?;

        if response.status() == StatusCode::NO_CONTENT {
            return Ok(None);
        }

        if response.status() != StatusCode::OK {
            let resp = response.text().await?;
            tracing::error!("Error while fetching input: {}", &resp);
            return Err(Error::FetchOutput(resp));
        }

        let input: JobHttp = response.json().await?;
        tracing::trace!("job_input response {:?}", input);

        let input: SealingJobT = input.into();
        Ok(Some(input))
    }

    async fn submit_job_output<SealingJobT: SealingJob + 'static>(
        &self,
        storage_provider_id: StorageProviderId,
        sector_id: SectorId,
        output: SealingJobT::Output,
    ) -> Result<(), Error>
    where
        JobOutputHttp: From<SealingJobT::Output>,
    {
        let request = SubmitSealingJobOutput {
            storage_provider_id,
            sector_id,
            job: output.into(),
        };
        let response = reqwest::Client::new()
            .post(&self.submit_output_uri)
            .body(serde_json::to_string(&request)?)
            .header(http::header::CONTENT_TYPE, "application/json")
            .send()
            .await?;

        if response.status() != StatusCode::OK {
            let resp = response.text().await?;
            tracing::error!(
                "Failed to submit results for storage_provider_id: {}, sector_id: {}, response: {}",
                storage_provider_id.0,
                sector_id.0,
                resp,
            );

            return Err(Error::FetchJobs(resp));
        } else {
            tracing::info!(
                "Succesfully submited result for storage_provider_id: {}, sector_id: {}",
                storage_provider_id.0,
                sector_id.0
            );
        }

        Ok(())
    }

    async fn get_job_output<SealingJobT: SealingJob + 'static>(
        &self,
        storage_provider_id: StorageProviderId,
        sector_id: SectorId,
    ) -> Result<Option<JobOutput<SealingJobT>>, Error>
    where
        SealingJobT::Output: From<JobOutputHttp>,
    {
        let job_type = SealingJobT::job_type().to_string();
        let uri = self
            .get_job_output_uri
            .replace(":storage_provider_id", &storage_provider_id.0.to_string())
            .replace(":sector_id", &sector_id.0.to_string())
            .replace(":job_type", job_type.as_str());

        tracing::debug!(
            "Requesting {} output for storage_provider_id: {}, sector_id: {}",
            job_type,
            storage_provider_id.0,
            sector_id.0
        );
        let response = self
            .http_client
            .get(uri)
            .header(http::header::CONTENT_TYPE, "application/json")
            .send()
            .await?;

        if response.status() == StatusCode::NO_CONTENT {
            return Ok(None);
        }

        if response.status() == StatusCode::FAILED_DEPENDENCY {
            #[derive(Deserialize)]
            struct ErrorResp {
                pub err: String,
            }
            let resp = response.json::<ErrorResp>().await?;

            return Ok(Some(JobOutput(Err(resp.err))));
        }

        if response.status() != StatusCode::OK {
            let resp = response.text().await?;
            tracing::error!("Error while fetching output: {}", &resp);
            return Err(Error::FetchOutput(resp));
        }

        let output: JobOutputHttp = response.json().await?;
        tracing::trace!("request_jobs response {:?}", output);

        let output: SealingJobT::Output = output.into();
        Ok(Some(JobOutput(Ok(output))))
    }

    async fn fail_job<SealingJobT: SealingJob + 'static>(
        &self,
        storage_provider_id: StorageProviderId,
        sector_id: SectorId,
        error: &str,
    ) -> Result<(), Error> {
        let request = FailJob {
            storage_provider_id,
            sector_id,
            job_type: SealingJobT::job_type(),
            error: error.to_string(),
        };
        let response = reqwest::Client::new()
            .post(&self.fail_job_uri)
            .body(serde_json::to_string(&request)?)
            .header(http::header::CONTENT_TYPE, "application/json")
            .send()
            .await?;

        if response.status() != StatusCode::OK {
            let resp = response.text().await?;
            tracing::error!(
                "Failed to submit job failure for storage_provider_id: {}, sector_id: {}, response: {}",
                storage_provider_id.0,
                sector_id.0,
                resp,
            );

            return Err(Error::FetchJobs(resp));
        }

        tracing::info!(
            "Succesfully submited job failure for storage_provider_id: {}, sector_id: {}",
            storage_provider_id.0,
            sector_id.0
        );

        Ok(())
    }

    async fn get_job_state<SealingJobT: SealingJob + 'static>(
        &self,
        storage_provider_id: StorageProviderId,
        sector_id: SectorId,
    ) -> Result<Option<JobState>, Error> {
        let job_type = SealingJobT::job_type().to_string();
        let uri = self
            .get_job_state_uri
            .replace(":storage_provider_id", &storage_provider_id.0.to_string())
            .replace(":sector_id", &sector_id.0.to_string())
            .replace(":job_type", job_type.as_str());

        tracing::debug!(
            "Requesting {} state for storage_provider_id: {}, sector_id: {}",
            job_type,
            storage_provider_id.0,
            sector_id.0
        );
        let response = self
            .http_client
            .get(uri)
            .header(http::header::CONTENT_TYPE, "application/json")
            .send()
            .await?;

        if response.status() == StatusCode::NO_CONTENT {
            return Ok(None);
        }

        if response.status() != StatusCode::OK {
            let resp = response.text().await?;
            tracing::error!("Error while fetching state: {}", &resp);
            return Err(Error::GetState(resp));
        }

        #[derive(Deserialize, Debug)]
        struct Response {
            pub state: JobState,
        }

        let response: Response = response.json().await?;
        tracing::trace!("get_state response {:?}", response);

        Ok(Some(response.state))
    }
}
