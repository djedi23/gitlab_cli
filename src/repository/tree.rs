use crud_api::{Api, ApiInput};
use serde::{Deserialize, Serialize};

#[derive(Api, Default, Debug, Serialize, Deserialize)]
#[api(endpoint(
  route = "/projects/{id}/repository/tree",
  query_struct = "TreeFilter",
  multiple_results,
  cli_route = "/projects/{id}/repository/tree",
  cli_help = "Get a list of repository files and directories in a project.",
))]
pub(crate) struct Tree {
  mode: String,
  #[serde(rename = "type")]
  _type: String,
  id: String,
  #[api(table_skip)]
  name: String,
  path: String,
}

#[derive(Debug, ApiInput, Serialize, Deserialize)]
#[api(no_input_file)]
pub(crate) struct TreeFilter {
  #[api(
    no_short,
    heading = "Filters",
    help = "The path inside repository.",
    long_help = "The path inside repository. Used to get content of subdirectories."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  path: Option<String>,

  #[api(
    no_short,
    heading = "Filters",
    long = "ref",
    help = "The name of a repository branch or tag or if not given the default branch.",
    long_help = "The name of a repository branch or tag or if not given the default branch."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(rename = "ref")]
  _ref: Option<String>,

  #[api(
    no_short,
    heading = "Filters",
    help = "Boolean value used to get a recursive tree.",
    long_help = "Boolean value used to get a recursive tree. (false by default)."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  recursive: Option<bool>,

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
