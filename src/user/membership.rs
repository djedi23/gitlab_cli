use crud_api::{Api, ApiInput};
use serde::{Deserialize, Serialize};

#[derive(Api, Serialize, Deserialize, Debug, Default, Clone)]
#[api(endpoint(
  route = "/users/{id}/memberships",
  multiple_results,
  query_struct = "MembershipPagination",
  cli_route = "/users/{id}/memberships",
  cli_help = "Lists all projects and groups a user is a member of.",
))]
pub(crate) struct Membership {
  source_id: u32,
  source_name: String,
  source_type: String,
  access_level: u32,
}

#[derive(Debug, ApiInput, Serialize, Deserialize)]
#[api(no_input_file)]
pub(crate) struct MembershipPagination {
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

#[derive(Api, Serialize, Deserialize, Debug, Default, Clone)]
#[api(endpoint(
  route = "/groups/{id}/members/{user_id}/state",
  query_struct = "MembersStateQuery",
  cli_route = "/groups/{id}/members/{user_id}/state",
  cli_help = "Changes the membership state of a user in a group.",
  cli_long_help = "Changes the membership state of a user in a group. The state is applied to all subgroups and projects.",
))]
pub(crate) struct MembersState {
  success: bool,
}

#[derive(Debug, ApiInput, Serialize, Deserialize)]
#[api(no_input_file)]
pub(crate) struct MembersStateQuery {
  #[api(
    no_short,
    help = "The new state for the user.",
    long_help = "The new state for the user. State is either `awaiting` or `active`.",
    possible_values = "awaiting,active"
  )]
  state: String,
}
