use crate::Commit;
use crud_api::Api;
use serde::{Deserialize, Serialize};

#[derive(Api, Serialize, Deserialize, Debug, Default, Clone)]
#[api(
  endpoint(
    route = "/projects/{id}/merge_requests/{iid}/versions",
    multiple_results,
    cli_route = "/projects/{id}/merge_requests/{iid}/versions",
    cli_help = "Get a list of merge request diff versions.",
  ),
  endpoint(
    route = "/projects/{id}/merge_requests/{iid}/versions/{verion_id}",
    cli_route = "/projects/{id}/merge_requests/{iid}/versions/{verion_id}",
    cli_help = "Get a single merge request diff version.",
  )
)]
pub(crate) struct DiffVersion {
  id: u32,
  head_commit_sha: String,
  base_commit_sha: String,
  start_commit_sha: String,
  created_at: String,
  merge_request_id: u32,
  state: String,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  real_size: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  commits: Option<Vec<Commit>>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  diffs: Option<Vec<Change>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Change {
  old_path: String,
  new_path: String,
  a_mode: String,
  b_mode: String,
  new_file: bool,
  renamed_file: bool,
  deleted_file: bool,
  diff: String,
}
