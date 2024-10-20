use crud_api::{Api, ApiInput};
use crud_pretty_struct::PrettyPrint;
use serde::{Deserialize, Serialize};

#[derive(Api, Serialize, Deserialize, Debug, Default, Clone)] // PrettyPrint
#[api(
  endpoint(
    route = "/projects/{id}/issues/{iid}/time_estimate",
    cli_route = "/projects/{id}/issues/{iid}/time_estimate",
    method = "POST",
    query_struct = "IssueTimeEstimateQuery",
    cli_help = "Sets an estimated time of work for this issue.",
  ),
  endpoint(
    route = "/projects/{id}/issues/{iid}/add_spent_time",
    cli_route = "/projects/{id}/issues/{iid}/add_spent_time",
    method = "POST",
    result_ok_status = "CREATED",
    query_struct = "IssueAddSpentTimeQuery",
    cli_help = "Adds spent time for this issue",
  ),
  endpoint(
    route = "/projects/{id}/issues/{iid}/reset_spent_time",
    cli_route = "/projects/{id}/issues/{iid}/reset_spent_time",
    method = "POST",
    cli_help = "Resets the total spent time for this issue to 0 seconds.",
  ),
  endpoint(
    route = "/projects/{id}/issues/{iid}/time_stats",
    cli_route = "/projects/{id}/issues/{iid}/time_stats",
    method = "GET",
    cli_help = "Get time tracking stats",
  ),
  endpoint(
    route = "/projects/{id}/merge_requests/{iid}/time_stats",
    cli_route = "/projects/{id}/merge_requests/{iid}/time_stats",
    cli_help = "Get time tracking stats.",
  ),
  endpoint(
    route = "/projects/{id}/merge_requests/{iid}/time_estimate",
    method = "POST",
    query_struct = "IssueTimeEstimateQuery",
    cli_route = "/projects/{id}/merge_requests/{iid}/time_stats/set",
    cli_help = "Sets an estimated time of work for this merge request.",
  ),
  endpoint(
    route = "/projects/{id}/merge_requests/{iid}/add_spent_time",
    method = "POST",
    result_ok_status = "CREATED",
    query_struct = "IssueAddSpentTimeQuery",
    cli_route = "/projects/{id}/merge_requests/{iid}/time_stats/add",
    cli_help = "Adds spent time for this merge request.",
  ),
  endpoint(
    route = "/projects/{id}/merge_requests/{iid}/reset_time_estimate",
    method = "POST",
    cli_route = "/projects/{id}/merge_requests/{iid}/time_stats/reset_estimate_time",
    cli_help = "Resets the estimated time for this merge request to 0 seconds.",
  ),
  endpoint(
    route = "/projects/{id}/merge_requests/{iid}/reset_spent_time",
    method = "POST",
    cli_route = "/projects/{id}/merge_requests/{iid}/time_stats/reset_spent_time",
    cli_help = "Resets the total spend time for this merge request to 0 seconds.",
  )
)]
#[derive(PrettyPrint)] // skip_none
pub(crate) struct TimeStats {
  time_estimate: u32,
  total_time_spent: u32,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[pretty(skip_none)]
  human_time_estimate: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[pretty(skip_none)]
  human_total_time_spent: Option<String>,
}

#[derive(Debug, ApiInput, Serialize, Deserialize)]
#[api(no_input_file)]
pub(crate) struct IssueTimeEstimateQuery {
  #[api(no_short, help = "The duration in human format. e.g: 3h30m")]
  duration: String,
}
#[derive(Debug, ApiInput, Serialize, Deserialize)]
#[api(no_input_file)]
pub(crate) struct IssueAddSpentTimeQuery {
  #[api(no_short, help = "The duration in human format. e.g: 3h30m")]
  duration: String,
  #[api(no_short, help = "A summary of how the time was spent")]
  summary: Option<String>,
}
