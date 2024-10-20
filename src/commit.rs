use crud_api::{Api, ApiInput};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Api, Clone, Default, Debug, Serialize, Deserialize)]
#[api(endpoint(
  route = "/projects/{id}/merge_requests/{iid}/commits",
  multiple_results,
  cli_route = "/projects/{id}/merge_requests/{iid}/commits",
  cli_help = "Get a list of merge request commits.",
))]
#[api(endpoint(
  route = "/projects/{id}/repository/merge_base",
  cli_route = "/projects/{id}/repository/merge_base",
  query_struct = "MergeBaseQuery",
  cli_help = "Get the common ancestor for 2 or more refs (commit SHAs, branch names or tags).",
))]
pub(crate) struct Commit {
  #[api(table_skip)]
  id: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  short_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  created_at: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  parent_ids: Option<Vec<String>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  title: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  message: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  author_name: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  author_email: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  authored_date: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  committer_name: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  committer_email: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  committed_date: Option<String>,
  //  TODO    trailers: {},
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  web_url: Option<String>,
}

impl Display for Commit {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.short_id.as_ref().unwrap_or(&self.id))
  }
}

#[derive(Debug, ApiInput, Serialize, Deserialize)]
#[api(no_input_file)]
pub(crate) struct MergeBaseQuery {
  #[api(
    no_short,
    help = "The refs to find the common ancestor of, multiple refs can be passed"
  )]
  refs: Vec<String>,
}
