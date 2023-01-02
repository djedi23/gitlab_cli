use crate::user::User;
use crud_api::Api;
use serde::{Deserialize, Serialize};

#[derive(Api, Serialize, Deserialize, Debug, Default)]
#[api(endpoint(
  route = "/projects/{id}/starrers",
  multiple_results,
  query_struct = "ProjectsUsersFilter",
  cli_route = "/projects/{id}/starrers",
  cli_help = "List the users who starred the specified project.",
))]
pub(crate) struct Starrers {
  starred_since: String,
  user: User,
}
