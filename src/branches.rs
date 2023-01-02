use crud_api::{Api, ApiInput};
use serde::{Deserialize, Serialize};

use crate::commit::Commit;

#[derive(Api, Serialize, Deserialize, Debug, Default)]
#[api(
  endpoint(
    route = "/projects/{id}/repository/branches",
    query_struct = "BranchQuery",
    multiple_results,
    cli_route = "/projects/{id}/repository/branches",
    cli_help = "Get a list of repository branches from a project, sorted by name alphabetically.",
  ),
  endpoint(
    route = "/projects/{id}/repository/branches/{branch}",
    cli_route = "/projects/{id}/repository/branches/{branch}",
    cli_help = "Get a single project repository branch.",
  ),
  endpoint(
    route = "/projects/{id}/repository/branches",
    method = "POST",
    result_ok_status = "CREATED",
    payload_struct = "BranchCreatePayload",
    cli_route = "/projects/{id}/repository/branches/create",
    cli_help = "Create a new branch in the repository.",
  ),
  endpoint(
    route = "/projects/{id}/repository/branches/{branch}",
    method = "DELETE",
    result_ok_status = "NO_CONTENT",
    cli_no_output,
    result_struct = "EmptyResponse",
    cli_route = "/projects/{id}/repository/branches/{branch}/delete",
    cli_help = "Delete a branch from the repository",
  ),
  endpoint(
    route = "/projects/{id}/repository/merged_branches",
    method = "DELETE",
    result_ok_status = "ACCEPTED",
    cli_no_output,
    result_struct = "EmptyResponse",
    cli_route = "/projects/{id}/repository/merged_branches/delete",
    cli_help = "Deletes all branches that are merged into the project's default branch.",
  )
)]
pub struct Branch {
  name: String,
  merged: bool,
  protected: bool,
  default: bool,
  developers_can_push: bool,
  developers_can_merge: bool,
  can_push: bool,
  #[api(table_skip)]
  web_url: String,
  #[api(table_skip)]
  commit: Commit,
}

#[derive(ApiInput, Serialize, Deserialize, Debug)]
#[api(no_input_file)]
pub struct BranchQuery {
  #[api(
    no_short,
    help = "Return list of branches containing the search string.",
    long_help = "Return list of branches containing the search string. You can use `^term` and `term$` to find branches that begin and end with term respectively."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  search: Option<String>,
}

#[derive(ApiInput, Serialize, Deserialize, Debug, Default)]
pub struct BranchCreatePayload {
  #[api(no_short, help = "Name of the branch.")]
  branch: String,

  #[api(
    no_short,
    heading = "Filters",
    long = "ref",
    help = "Branch name or commit SHA to create branch from."
  )]
  #[serde(rename = "ref")]
  _ref: String,
}
