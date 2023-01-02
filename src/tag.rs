use crate::commit::Commit;
use crud_api::{Api, ApiInput};
use serde::{Deserialize, Serialize};

#[derive(Api, Clone, Default, Debug, Serialize, Deserialize)]
#[api(
  endpoint(
    route = "/projects/{id}/repository/tags",
    multiple_results,
    query_struct = "TagQuery",
    cli_route = "/projects/{id}/repository/tags",
    cli_help = "Get a list of repository tags from a project, sorted by update date and time in descending order.",
  ),
  endpoint(
    route = "/projects/{id}/repository/tags/{tag_name}",
    cli_route = "/projects/{id}/repository/tags/{tag_name}",
    cli_help = "Get a specific repository tag determined by its name.",
  ),
  endpoint(
    route = "/projects/{id}/repository/tags",
    method = "POST",
    payload_struct = "CreateTagPayload",
    result_ok_status = "CREATED",
    cli_route = "/projects/{id}/repository/tags/create",
    cli_help = "Creates a new tag in the repository that points to the supplied ref.",
  ),
  endpoint(
    route = "/projects/{id}/repository/tags/{tag_name}",
    method = "DELETE",
    result_ok_status = "NO_CONTENT",
    result_struct = "EmptyResponse",
    cli_no_output,
    cli_route = "/projects/{id}/repository/tags/{tag_name}/delete",
    cli_help = "Deletes a tag of a repository with given name.",
  )
)]
pub(crate) struct Tag {
  name: String,
  message: String,
  #[api(table_skip)]
  target: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[api(table_skip)]
  release: Option<Release>,
  #[serde(skip_serializing_if = "Option::is_none")]
  projected: Option<bool>,
  commit: Commit,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub(crate) struct Release {
  tag_name: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  description: Option<String>,
}

#[derive(Debug, ApiInput, Serialize, Deserialize)]
#[api(no_input_file)]
pub(crate) struct TagQuery {
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

#[derive(Debug, Default, ApiInput, Serialize, Deserialize)]
pub(crate) struct CreateTagPayload {
  #[api(no_short, long = "tag-name", help = "The name of a tag")]
  tag_name: String,

  #[api(
    no_short,
    long = "ref",
    help = "Create tag using commit SHA, another tag name, or branch name"
  )]
  #[serde(rename = "ref")]
  _ref: String,

  #[api(no_short, help = "Creates annotated tag")]
  #[serde(skip_serializing_if = "Option::is_none")]
  message: Option<String>,
}
