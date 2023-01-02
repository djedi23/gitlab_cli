use crud_api::{Api, ApiInput};
use serde::{Deserialize, Serialize};

#[derive(Api, Serialize, Deserialize, Debug, Default, Clone)]
#[api(endpoint(
  route = "/user/activities",
  multiple_results,
  query_struct = "ActivityPagination",
  cli_route = "/user/activities",
  cli_help = "Get the last activity date for all users, sorted from oldest to newest.",
))]
pub(crate) struct Activity {
  username: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  last_activity_on: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  last_activity_at: Option<String>,
}

#[derive(Debug, ApiInput, Serialize, Deserialize)]
#[api(no_input_file)]
pub(crate) struct ActivityPagination {
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
