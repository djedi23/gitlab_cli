use crud_api::{Api, ApiInput};
use serde::{Deserialize, Serialize};

#[derive(Api, Clone, Default, Debug, Serialize, Deserialize)]
#[api(
  endpoint(
    route = "/projects/{id}/labels",
    multiple_results,
    cli_route = "/projects/{id}/labels",
    query_struct = "LabelQuery",
    cli_help = "Get all labels for a given project.",
  ),
  endpoint(
    route = "/projects/{id}/labels/{label_id}",
    cli_route = "/projects/{id}/labels/{label_id}",
    cli_help = "Get a single label for a given project.",
  ),
  endpoint(
    route = "/projects/{id}/labels",
    method = "POST",
    result_ok_status = "CREATED",
    cli_route = "/projects/{id}/labels/create",
    payload_struct = "CreateLabelPayload",
    cli_help = "Creates a new label for the given repository with the given name and color.",
  ),
  endpoint(
    route = "/projects/{id}/labels/{label_id}",
    method = "DELETE",
    result_ok_status = "NO_CONTENT",
    result_struct = "EmptyResponse",
    cli_no_output,
    cli_route = "/projects/{id}/labels/{label_id}/delete",
    cli_help = "Deletes a label with a given name.",
  ),
  endpoint(
    route = "/projects/{id}/labels/{label_id}",
    method = "PUT",
    payload_struct = "UpdateLabelPayload",
    cli_route = "/projects/{id}/labels/{label_id}/edit",
    cli_help = "Updates an existing label with new name or new color.",
    cli_long_help = "Updates an existing label with new name or new color. At least one parameter is required, to update the label.",
  ),
  endpoint(
    route = "/projects/{id}/labels/{label_id}/promote",
    method = "PUT",
    cli_route = "/projects/{id}/labels/{label_id}/promote",
    cli_help = "Promotes a project label to a group label.",
    cli_long_help = "Promotes a project label to a group label. The label keeps its ID.",
  ),
  endpoint(
    route = "/projects/{id}/labels/{label_id}/subscribe",
    method = "POST",
    result_ok_status = "CREATED",
    cli_help = "Subscribes the authenticated user to a label to receive notifications.",
    cli_route = "/projects/{id}/labels/{label_id}/subscribe",
    cli_long_help = "Subscribes the authenticated user to a label to receive notifications. If the user is already subscribed to the label, the status code 304 is returned.",
  ),
  endpoint(
    route = "/projects/{id}/labels/{label_id}/unsubscribe",
    method = "POST",
    result_ok_status = "CREATED",
    cli_route = "/projects/{id}/labels/{label_id}/unsubscribe",
    cli_help = "Unsubscribes the authenticated user from a label to not receive notifications from it.",
    cli_long_help = "Unsubscribes the authenticated user from a label to not receive notifications from it. If the user is not subscribed to the label, the status code 304 is returned.",
  )
)]
#[api(
  endpoint(
    route = "/groups/{id}/labels",
    multiple_results,
    cli_route = "/groups/{id}/labels",
    query_struct = "GroupLabelQuery",
    cli_help = "Get all labels for a given group.",
  ),
  endpoint(
    route = "/groups/{id}/labels/{label_id}",
    cli_route = "/groups/{id}/labels/{label_id}",
    cli_help = "Get a single label for a given group.",
  ),
  endpoint(
    route = "/groups/{id}/labels",
    method = "POST",
    result_ok_status = "CREATED",
    cli_route = "/groups/{id}/labels/create",
    payload_struct = "CreateLabelPayload",
    cli_help = "Creates a new label for the given repository with the given name and color.",
  ),
  endpoint(
    route = "/groups/{id}/labels/{label_id}",
    method = "DELETE",
    result_ok_status = "NO_CONTENT",
    result_struct = "EmptyResponse",
    cli_no_output,
    cli_route = "/groups/{id}/labels/{label_id}/delete",
    cli_help = "Deletes a label with a given name.",
  ),
  endpoint(
    route = "/groups/{id}/labels/{label_id}",
    method = "PUT",
    payload_struct = "UpdateLabelPayload",
    cli_route = "/groups/{id}/labels/{label_id}/edit",
    cli_help = "Updates an existing group label.",
    cli_long_help = "Updates an existing group label. At least one parameter is required, to update the group label.",
  ),
  endpoint(
    route = "/groups/{id}/labels/{label_id}/promote",
    method = "PUT",
    cli_route = "/groups/{id}/labels/{label_id}/promote",
    cli_help = "Promotes a group label to a group label.",
    cli_long_help = "Promotes a group label to a group label. The label keeps its ID.",
  ),
  endpoint(
    route = "/groups/{id}/labels/{label_id}/subscribe",
    method = "POST",
    result_ok_status = "CREATED",
    cli_help = "Subscribes the authenticated user to a label to receive notifications.",
    cli_route = "/groups/{id}/labels/{label_id}/subscribe",
    cli_long_help = "Subscribes the authenticated user to a label to receive notifications. If the user is already subscribed to the label, the status code 304 is returned.",
  ),
  endpoint(
    route = "/groups/{id}/labels/{label_id}/unsubscribe",
    method = "POST",
    result_ok_status = "CREATED",
    cli_route = "/groups/{id}/labels/{label_id}/unsubscribe",
    cli_help = "Unsubscribes the authenticated user from a label to not receive notifications from it.",
    cli_long_help = "Unsubscribes the authenticated user from a label to not receive notifications from it. If the user is not subscribed to the label, the status code 304 is returned.",
  )
)]
pub(crate) struct Label {
  id: u32,
  name: String,
  color: String,
  text_color: String,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  description: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  description_html: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  open_issues_count: Option<u32>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  closed_issues_count: Option<u32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  open_merge_requests_count: Option<u32>,
  #[api(table_skip)]
  subscribed: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  priority: Option<u32>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  is_project_label: Option<bool>,
}

#[derive(Debug, ApiInput, Serialize, Deserialize)]
#[api(no_input_file)]
pub(crate) struct LabelQuery {
  #[api(
    no_short,
    heading = "Filter",
    long = "with-counts",
    help = "Whether or not to include issue and merge request counts.",
    long_help = "Whether or not to include issue and merge request counts. Defaults to `false`."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  with_counts: Option<bool>,

  #[api(
    no_short,
    heading = "Filter",
    long = "include-ancestor-groups",
    help = "Include ancestor groups.",
    long_help = "Include ancestor groups. Defaults to `true`."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  include_ancestor_groups: Option<bool>,

  #[api(
    no_short,
    heading = "Filter",
    long = "search",
    help = "Keyword to filter labels by."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  search: Option<String>,

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
pub(crate) struct CreateLabelPayload {
  #[api(no_short, help = "The name of the label")]
  name: String,

  #[api(
    no_short,
    help = "The color of the label given in 6-digit hex notation with leading '#' sign (for example, #FFAABB) or one of the CSS color names."
  )]
  color: String,

  #[api(no_short, help = "The description of the label")]
  #[serde(skip_serializing_if = "Option::is_none")]
  description: Option<String>,

  #[api(
    no_short,
    help = "The priority of the label.",
    long_help = "The priority of the label. Must be greater or equal than zero or `null` to remove the priority."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  priority: Option<u32>,
}

#[derive(Debug, Default, ApiInput, Serialize, Deserialize)]
pub(crate) struct UpdateLabelPayload {
  #[api(no_short, long = "new-name", help = "The new name of the label")]
  #[serde(skip_serializing_if = "Option::is_none")]
  new_name: Option<String>,
  #[api(
    no_short,
    help = "The color of the label given in 6-digit hex notation with leading '#' sign (for example, #FFAABB) or one of the CSS color names."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  color: Option<String>,

  #[api(no_short, help = "The description of the label")]
  #[serde(skip_serializing_if = "Option::is_none")]
  description: Option<String>,

  #[api(
    no_short,
    help = "The priority of the label.",
    long_help = "The priority of the label. Must be greater or equal than zero or `null` to remove the priority."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  priority: Option<u32>,
}

#[derive(Debug, ApiInput, Serialize, Deserialize)]
#[api(no_input_file)]
pub(crate) struct GroupLabelQuery {
  #[api(
    no_short,
    heading = "Filter",
    long = "with-counts",
    help = "Whether or not to include issue and merge request counts.",
    long_help = "Whether or not to include issue and merge request counts. Defaults to `false`."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  with_counts: Option<bool>,

  #[api(
    no_short,
    heading = "Filter",
    long = "include-ancestor-groups",
    help = "Include ancestor groups.",
    long_help = "Include ancestor groups. Defaults to `true`."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  include_ancestor_groups: Option<bool>,

  #[api(
    no_short,
    long = "include-descendant-groups",
    help = "Include descendant groups.",
    long_help = "Include descendant groups. Defaults to `false`."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  include_descendant_groups: Option<bool>,

  #[api(
    no_short,
    long = "only-group-labels",
    help = "Toggle to include only group labels or also project labels.",
    long_help = "Toggle to include only group labels or also project labels. Defaults to `true`."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  only_group_labels: Option<String>,

  #[api(
    no_short,
    heading = "Filter",
    long = "search",
    help = "Keyword to filter labels by."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  search: Option<String>,

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
