use crud_api::{Api, ApiInput};
use serde::{Deserialize, Serialize};

#[derive(Api, Serialize, Deserialize, Debug, Default)]
#[api(endpoint(
  route = "/projects/{id}/repository/contributors",
  multiple_results,
  query_struct = "ContributorSort",
  cli_route = "/projects/{id}/repository/contributors",
  cli_help = "Get repository contributors list.",
))]
pub(crate) struct Contributor {
  name: String,
  email: String,
  commits: u32,
}

#[derive(Debug, ApiInput, Serialize, Deserialize)]
#[api(no_input_file)]
pub(crate) struct ContributorSort {
  #[api(
    no_short,
    heading = "Sorting",
    long = "order-by",
    help = "Return contributors ordered by `name`, `email`, or `commits` (orders by commit date) fields.",
    long_help = "Return contributors ordered by `name`, `email`, or `commits` (orders by commit date) fields. Default is `commits`.",
    possible_values = "name,email,commits"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  order_by: Option<String>,

  #[api(
    no_short,
    heading = "Sorting",
    help = "Return contributors sorted in `asc` or `desc` order.",
    long_help = "Return contributors sorted in `asc` or `desc` order. Default is `asc`.",
    possible_values = "asc,desc"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  sort: Option<String>,
}
