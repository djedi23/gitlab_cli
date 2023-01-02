use crud_api::Api;
use serde::{Deserialize, Serialize};

#[derive(Api, Serialize, Deserialize, Debug, Default, Clone)]
#[api(endpoint(
  route = "/user_counts",
  cli_route = "/user/counts",
  cli_help = "Get the counts (same as in top right menu) of the currently signed in user.",
))]
pub(crate) struct UserCount {
  merge_requests: u32,
  assigned_issues: u32,
  assigned_merge_requests: u32,
  review_requested_merge_requests: u32,
  todos: u32,
}
