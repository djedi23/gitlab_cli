mod test_report;
mod variables;

pub(crate) use self::{
  test_report::{TestReport, TestReportSummary},
  variables::Variables,
};
use crate::user::User;
use crud_api::{Api, ApiInput};
use serde::{Deserialize, Serialize};

#[derive(Api, Serialize, Deserialize, Debug, Default, Clone)]
#[api(
  endpoint(
    route = "/projects/{id}/pipelines",
    multiple_results,
    query_struct = "PipelineFilter",
    cli_route = "/projects/{id}/pipelines",
    cli_help = "Get a list of pipelines in a project."
  ),
  endpoint(
    route = "/projects/{id}/pipeline",
    method = "POST",
    result_ok_status = "CREATED",
    payload_struct = "PipelineCreate",
    cli_route = "/projects/{id}/pipelines/create",
    cli_help = "Create a new pipeline"
  ),
  endpoint(
    route = "/projects/{id}/pipelines/{pipeline_id}/retry",
    method = "POST",
    result_ok_status = "CREATED",
    cli_route = "/projects/{id}/pipelines/{pipeline_id}/retry",
    cli_help = "Retry jobs in a pipeline"
  ),
  endpoint(
    route = "/projects/{id}/pipelines/{pipeline_id}/cancel",
    method = "POST",
    cli_route = "/projects/{id}/pipelines/{pipeline_id}/cancel",
    cli_help = "Cancel a pipeline's jobs"
  ),
  endpoint(
    route = "/projects/{id}/pipelines/{pipeline_id}",
    cli_route = "/projects/{id}/pipelines/{pipeline_id}",
    cli_help = "Get a single pipeline."
  ),
  endpoint(
    route = "/projects/{id}/pipelines/{pipeline_id}",
    method = "DELETE",
    result_ok_status = "NO_CONTENT",
    cli_no_output,
    result_struct = "EmptyResponse",
    cli_route = "/projects/{id}/pipelines/{pipeline_id}/delete",
    cli_help = "Delete a pipeline."
  ),
  endpoint(
    route = "/projects/{id}/merge_requests/{iid}/pipelines",
    multiple_results,
    cli_route = "/projects/{id}/merge_requests/{iid}/pipelines",
    cli_help = "Get a list of merge request pipelines",
  ),
  endpoint(
    route = "/projects/{id}/merge_requests/{iid}/pipelines",
    method = "POST",
    result_ok_status = "CREATED",
    cli_route = "/projects/{id}/merge_requests/{iid}/pipelines/create",
    cli_help = "Create a new pipeline for a merge request.",
    cli_long_help = "Create a new pipeline for a merge request. A pipeline created via this endpoint doesn't run a regular branch/tag pipeline. It requires .gitlab-ci.yml to be configured with only: [merge_requests] to create jobs."
  )
)]
pub(crate) struct Pipeline {
  id: u32,
  project_id: u32,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  iid: Option<u32>,
  #[api(table_skip)]
  sha: String,
  #[serde(rename = "ref")]
  _ref: String,
  status: String,
  created_at: String,
  #[api(table_skip)]
  updated_at: String,
  #[api(table_skip)]
  web_url: String,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  before_sha: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  tag: Option<bool>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  yaml_errors: Option<bool>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  started_at: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  finished_at: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  committed_at: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  duration: Option<u32>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  queued_duration: Option<u32>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  coverage: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  user: Option<User>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  detailed_status: Option<PipelineStatus>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
struct PipelineStatus {
  #[serde(skip_serializing_if = "Option::is_none")]
  icon: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  text: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  label: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  group: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  tooltip: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  has_details: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  details_path: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  illustration: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  favicon: Option<String>,
}

#[derive(ApiInput, Debug, Default, Serialize, Deserialize)]
#[api(no_input_file)]
pub(crate) struct PipelineFilter {
  #[api(
    no_short,
    heading = "Filters",
    help = "The scope of pipelines.",
    long_help = "The scope of pipelines, one of: `running`, `pending`, `finished`, `branches`, `tags`",
    possible_values = "running,pending,finished,branches,tags"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  scope: Option<String>,

  #[api(
    no_short,
    heading = "Filters",
    help = "The status of pipelines",
    long_help = "The status of pipelines",
    possible_values = "created,waiting_for_resource,preparing,pending,running,success,failed,canceled,skipped,manual,scheduled"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  status: Option<String>,

  #[api(
    no_short,
    heading = "Filters",
    help = "how the pipeline was triggered",
    possible_values = "push,web,trigger,schedule,api,external,pipeline,chat,webide,merge_request_event,external_pull_request_event,parent_pipeline,ondemand_dast_scan,ondemand_dast_validation"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  source: Option<String>,

  #[api(
    no_short,
    heading = "Filters",
    long = "ref",
    help = "The ref of pipelines"
  )]
  #[serde(rename = "ref", skip_serializing_if = "Option::is_none")]
  _ref: Option<String>,

  #[api(no_short, heading = "Filters", help = "The SHA of pipelines")]
  #[serde(skip_serializing_if = "Option::is_none")]
  sha: Option<String>,

  #[api(
    no_short,
    heading = "Filters",
    long = "yaml-errors",
    help = "Returns pipelines with invalid configurations"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  yaml_errors: Option<bool>,

  #[api(
    no_short,
    heading = "Filters",
    help = "The username of the user who triggered pipelines"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  username: Option<String>,

  #[api(
    no_short,
    heading = "Filters",
    long = "updated-after",
    help = "Return pipelines updated after the specified date.",
    long_help = "Return pipelines updated after the specified date. Expected in ISO 8601 format (2019-03-15T08:00:00Z)."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  updated_after: Option<String>,

  #[api(
    no_short,
    heading = "Filters",
    long = "updated-before",
    help = "Return pipelines updated before the specified date.",
    long_help = "Return pipelines updated before the specified date. Expected in ISO 8601 format (2019-03-15T08:00:00Z)."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  updated_before: Option<String>,

  #[api(
    no_short,
    heading = "Sorting",
    long = "order-by",
    help = "Order pipelines by `id`, `status`, `ref`, `updated_at` or `user_id`.",
    long_help = "Order pipelines by `id`, `status`, `ref`, `updated_at` or `user_id`. (default: `id`)",
    possible_values = "id,status,ref,updated_at,user_id"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  order_by: Option<String>,

  #[api(
    no_short,
    heading = "Sorting",
    long = "sort",
    help = "Sort pipelines in `asc` or `desc` order.",
    long_help = "Sort pipelines in `asc` or `desc` order. (default: `desc`)",
    possible_values = "asc,desc"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  sort: Option<String>,

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
pub(crate) struct PipelineCreate {
  #[api(no_short, long = "ref", help = "Reference to commit")]
  #[serde(rename = "ref")]
  _ref: String,

  #[api(
    no_short,
    help = "An array containing the variables available in the pipeline.",
    long_help = "An array containing the variables available in the pipeline, matching the structure `[{ 'key': 'UPLOAD_TO_S3', 'variable_type': 'file', 'value': 'true' }]`"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  variables: Option<Vec<String>>,
}
