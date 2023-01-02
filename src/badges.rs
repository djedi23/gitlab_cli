use crud_api::{Api, ApiInput};
use serde::{Deserialize, Serialize};

#[derive(Api, Serialize, Deserialize, Debug, Default)]
#[api(
  endpoint(
    route = "/projects/{id}/badges",
    query_struct = "BadgeQuery",
    multiple_results,
    cli_route = "/projects/{id}/badges",
    cli_help = "Gets a list of a project's badges and its group badges.",
  ),
  endpoint(
    route = "/projects/{id}/badges/{badge_id}",
    cli_route = "/projects/{id}/badges/{badge_id}",
    cli_help = "Gets a badge of a project.",
  ),
  endpoint(
    route = "/projects/{id}/badges",
    method = "POST",
    payload_struct = "BadgeCreatePayload",
    result_ok_status = "CREATED",
    cli_route = "/projects/{id}/badges/create",
    cli_help = "Adds a badge to a project.",
  ),
  endpoint(
    route = "/projects/{id}/badges/{badge_id}",
    method = "PUT",
    payload_struct = "BadgeCreatePayload",
    cli_route = "/projects/{id}/badges/{badge_id}/edit",
    cli_help = "Updates a badge of a project.",
  ),
  endpoint(
    route = "/projects/{id}/badges/{badge_id}",
    method = "DELETE",
    result_ok_status = "NO_CONTENT",
    cli_no_output,
    result_struct = "EmptyResponse",
    cli_route = "/projects/{id}/badges/{badge_id}/delete",
    cli_help = "Removes a badge from a project.",
    cli_long_help = "Removes a badge from a project. Only project badges are removed by using this endpoint.",
  )
)]
#[api(
  endpoint(
    route = "/groups/{id}/badges",
    query_struct = "BadgeQuery",
    multiple_results,
    cli_route = "/groups/{id}/badges",
    cli_help = "Gets a list of a group's badges.",
  ),
  endpoint(
    route = "/groups/{id}/badges/{badge_id}",
    cli_route = "/groups/{id}/badges/{badge_id}",
    cli_help = "Gets a badge of a group.",
  ),
  endpoint(
    route = "/groups/{id}/badges",
    method = "POST",
    payload_struct = "BadgeCreatePayload",
    result_ok_status = "CREATED",
    cli_route = "/groups/{id}/badges/create",
    cli_help = "Adds a badge to a group.",
  ),
  endpoint(
    route = "/groups/{id}/badges/{badge_id}",
    method = "PUT",
    payload_struct = "BadgeCreatePayload",
    cli_route = "/groups/{id}/badges/{badge_id}/edit",
    cli_help = "Updates a badge of a group.",
  ),
  endpoint(
    route = "/groups/{id}/badges/{badge_id}",
    method = "DELETE",
    result_ok_status = "NO_CONTENT",
    cli_no_output,
    result_struct = "EmptyResponse",
    cli_route = "/groups/{id}/badges/{badge_id}/delete",
    cli_help = "Removes a badge from a group.",
    cli_long_help = "Removes a badge from a group. Only group badges are removed by using this endpoint.",
  )
)]
// TODO Preview a badge from project https://gitlab.com/help/api/group_badges.md#preview-a-badge-from-a-group
pub(crate) struct Badge {
  id: u32,
  kind: String,
  name: Option<String>,
  link_url: String,
  image_url: String,
  #[api(table_skip)]
  rendered_link_url: String,
  #[api(table_skip)]
  rendered_image_url: String,
}

#[derive(Debug, ApiInput, Serialize, Deserialize)]
#[api(no_input_file)]
pub(crate) struct BadgeQuery {
  #[api(no_short, help = "Name of the badges to return (case-sensitive).")]
  #[serde(skip_serializing_if = "Option::is_none")]
  name: Option<String>,
}

#[derive(Debug, Default, ApiInput, Serialize, Deserialize)]
pub(crate) struct BadgeCreatePayload {
  #[api(no_short, long = "link-url", help = "URL of the badge link")]
  link_url: String,

  #[api(no_short, long = "image-url", help = "URL of the badge image")]
  image_url: String,

  #[api(no_short, long = "name", help = "Name of the badge")]
  #[serde(skip_serializing_if = "Option::is_none")]
  name: Option<String>,
}
