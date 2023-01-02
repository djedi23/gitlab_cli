use crate::{commit::Commit, pipeline::Pipeline, user::User};
use crud_api::{Api, ApiInput};
use serde::{Deserialize, Serialize};

#[derive(Api, Serialize, Deserialize, Debug, Default)]
#[api(
  endpoint(
    route = "/projects/{id}/jobs",
    multiple_results,
    query_struct = "JobFilter",
    cli_route = "/projects/{id}/jobs",
    cli_help = "Get a list of jobs in a project.",
    cli_long_help = "Get a list of jobs in a project. Jobs are sorted in descending order of their IDs.",
  ),
  endpoint(
    route = "/projects/{id}/pipelines/{pipeline_id}/jobs",
    multiple_results,
    query_struct = "PipelineJobFilter",
    cli_route = "/projects/{id}/pipelines/{pipeline_id}/jobs",
    cli_help = "Get a list of jobs for a pipeline.",
  ),
  endpoint(
    route = "/projects/{id}/pipelines/{pipeline_id}/bridges",
    multiple_results,
    query_struct = "JobFilter",
    cli_route = "/projects/{id}/pipelines/{pipeline_id}/bridges",
    cli_help = "Get a list of bridge jobs for a pipeline.",
  ),
  endpoint(
    route = "/projects/{id}/jobs/{job_id}",
    cli_route = "/projects/{id}/jobs/{job_id}",
    cli_help = "Get a single job of a project."
  ),
  endpoint(
    route = "/projects/{id}/jobs/{job_id}/trace",
    stream,
    cli_route = "/projects/{id}/jobs/{job_id}/trace",
    cli_help = "Get a log (trace) of a specific job of a project."
  ),
  endpoint(
    route = "/projects/{id}/jobs/{job_id}/cancel",
    method = "POST",
    result_ok_status = "CREATED",
    cli_route = "/projects/{id}/jobs/{job_id}/cancel",
    cli_help = "Cancel a single job of a project"
  ),
  endpoint(
    route = "/projects/{id}/jobs/{job_id}/retry",
    method = "POST",
    result_ok_status = "CREATED",
    cli_route = "/projects/{id}/jobs/{job_id}/retry",
    cli_help = "Retry a single job of a project"
  ),
  endpoint(
    route = "/projects/{id}/jobs/{job_id}/erase",
    method = "POST",
    result_ok_status = "CREATED",
    cli_route = "/projects/{id}/jobs/{job_id}/erase",
    cli_help = "Erase a single job of a project (remove job artifacts and a job log)"
  ),
  endpoint(
    route = "/projects/{id}/jobs/{job_id}/play",
    method = "POST",
    result_ok_status = "CREATED",
    payload_struct = "PlayJobPayload",
    cli_route = "/projects/{id}/jobs/{job_id}/play",
    cli_help = "Triggers a manual action to start a job."
  )
)]
#[api(endpoint(
  route = "/job",
  query_struct = "JobTokenQuery",
  cli_route = "/job",
  cli_help = "Retrieve the job that generated a job token.",
))]
pub(crate) struct Job {
  id: u32,
  name: String,
  #[serde(rename = "ref")]
  _ref: String,
  stage: String,
  status: String,
  #[api(table_skip)]
  tag: bool,
  #[api(table_skip)]
  tag_list: Vec<String>,
  #[api(table_skip)]
  web_url: String,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  coverage: Option<String>,
  #[api(table_skip)]
  allow_failure: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  created_at: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  started_at: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  finished_at: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  duration: Option<f32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[api(table_skip)]
  artifacts_expire_at: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  queued_duration: Option<f32>,
  #[api(table_skip)]
  artifacts: Vec<Artifact>,
  user: User,
  commit: Commit,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[api(table_skip)]
  runner: Option<Runner>,
  #[api(table_skip)]
  pipeline: Pipeline,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Runner {
  id: u32,
  description: String,
  ip_address: String,
  active: bool,
  is_shared: bool,
  runner_type: String,
  name: String,
  online: bool,
  status: String,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
struct Artifact {
  file_type: String,
  size: u32,
  filename: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  file_format: Option<String>,
}

#[derive(ApiInput, Debug, Default, Serialize, Deserialize)]
#[api(no_input_file)]
pub(crate) struct JobFilter {
  #[api(
    no_short,
    heading = "Filters",
    help = "Scope of jobs to show.",
    long_help = "Scope of jobs to show. Either one of or an array of the following: `created`, `pending`, `running`, `failed`, `success`, `canceled`, `skipped`, or `manual`. All jobs are returned if scope is not provided",
    possible_values = "created,pending,running,failed,success,canceled,skipped,manual"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  scope: Option<Vec<String>>,

  #[api(
    no_short,
    heading = "Pagination",
    help = "Page number.",
    long_help = "Page number (default: 1)."
  )]
  page: Option<u32>,

  #[api(
    no_short,
    long = "per-page",
    heading = "Pagination",
    help = "Number of items to list per page.",
    long_help = "Number of items to list per page (default: 20, max: 100)."
  )]
  per_page: Option<u32>,
}

#[derive(ApiInput, Debug, Default, Serialize, Deserialize)]
#[api(no_input_file)]
pub(crate) struct PipelineJobFilter {
  #[api(
    no_short,
    heading = "Filters",
    help = "Scope of jobs to show.",
    long_help = "Scope of jobs to show. Either one of or an array of the following: `created`, `pending`, `running`, `failed`, `success`, `canceled`, `skipped`, or `manual`. All jobs are returned if scope is not provided",
    possible_values = "created,pending,running,failed,success,canceled,skipped,manual"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  scope: Option<Vec<String>>,

  #[api(
    no_short,
    long = "include-retried",
    help = "Include retried jobs in the response."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  include_retried: Option<bool>,

  #[api(
    no_short,
    heading = "Pagination",
    help = "Page number.",
    long_help = "Page number (default: 1)."
  )]
  page: Option<u32>,

  #[api(
    no_short,
    long = "per-page",
    heading = "Pagination",
    help = "Number of items to list per page.",
    long_help = "Number of items to list per page (default: 20, max: 100)."
  )]
  per_page: Option<u32>,
}

#[derive(ApiInput, Debug, Default, Serialize, Deserialize)]
#[api(no_input_file)]
pub(crate) struct JobTokenQuery {
  #[api(no_short, long = "job-token", help = "job token")]
  job_token: String,
}

#[derive(ApiInput, Debug, Default, Serialize, Deserialize)]
pub(crate) struct PlayJobPayload {
  #[api(
    no_short,
    long = "job-variables-attributes",
    help = "An array containing the custom variables available to the job."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  job_variables_attributes: Option<Vec<String>>, // FIWME: it should read KeyValue
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct KeyValue {
  key: String,
  value: String,
}
