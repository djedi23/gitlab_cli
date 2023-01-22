pub mod create;
pub mod update;
use crate::projects::Project;
use crud_api::{Api, ApiInput};
use serde::{Deserialize, Serialize};

#[derive(Api, Serialize, Deserialize, Debug, Default, Clone)]
#[api(
  endpoint(
    route = "/groups",
    multiple_results,
    query_struct = "GroupsFilter",
    cli_route = "/groups",
    cli_help = "Get a list of visible groups for the authenticated user.",
    cli_long_help = "Get a list of visible groups for the authenticated user. When accessed without authentication, only public groups are returned.",
  ),
  endpoint(
    route = "/groups/{id}/subgroups",
    multiple_results,
    query_struct = "GroupsFilter",
    cli_route = "/groups/{id}/subgroups",
    cli_help = "Get a list of visible direct subgroups in this group.",
    cli_long_help = "Get a list of visible direct subgroups in this group. When accessed without authentication, only public groups are returned.",
  ),
  endpoint(
    route = "/groups/{id}/descendant_groups",
    multiple_results,
    query_struct = "GroupsFilter",
    cli_route = "/groups/{id}/descendant_groups",
    cli_help = "Get a list of visible descendant groups of this group.",
    cli_long_help = "Get a list of visible descendant groups of this group. When accessed without authentication, only public groups are returned.",
  ),
  endpoint(
    route = "/groups/{id}",
    query_struct = "GroupDetail",
    cli_route = "/groups/{id}",
    cli_help = "Get all details of a group.",
  ),
  endpoint(
    route = "/groups/{id}/avatar",
    stream,
    cli_route = "/groups/{id}/avatar",
    cli_help = "Get a group avatar.",
  ),
  endpoint(
    route = "/groups",
    method = "POST",
    payload_struct = "GroupsCreatePayload",
    cli_route = "/groups/create",
    cli_long_help = "Creates a new project group. Available only for users who can create groups.",
    cli_help = "Creates a new project group.",
  ),
  endpoint(
    route = "/groups",
    method = "PUT",
    payload_struct = "GroupsUpdatePayload",
    cli_route = "/groups/edit",
    cli_long_help = "Updates the project group. Only available to group owners and administrators.",
    cli_help = "Updates the project group.",
  ),
  endpoint(
    route = "/groups/{id}",
    method = "DELETE",
    query_struct = "GroupsDeleteQuery",
    result_ok_status = "ACCEPTED",
    cli_route = "/groups/{id}/delete",
    cli_long_help = "Removes group, and queues a background job to delete all projects in the group as well. Since GitLab 12.8, on Premium or higher tiers, marks a group for deletion. The deletion happens 7 days later by default, but this can be changed in the instance settings.",
    cli_help = "Removes group, and queues a background job to delete all projects in the group as well.",
  ),
  endpoint(
    route = "/groups/{id}/restore",
    method = "POST",
    payload_struct = "GroupsSharePayload",
    cli_route = "/groups/{id}/restore",
    cli_help = "Restores a group marked for deletion.",
  ),
  endpoint(
    route = "/groups/{id}/projects/{project_id}",
    method = "POST",
    cli_route = "/groups/{id}/projects/{project_id}/transfert",
    cli_help = "Transfer a project to the Group namespace.",
  )
)]
#[api(
  endpoint(
    route = "/groups/{id}/share",
    method = "POST",
    cli_route = "/groups/{id}/share",
    cli_help = "Share group with another group.",
  ),
  endpoint(
    route = "/groups/{id}/share/{share_id}",
    method = "DELETE",
    result_ok_status = "ACCEPTED",
    cli_route = "/groups/{id}/share/{share_id}",
    cli_help = "Unshare the group from another group.",
  )
)]
#[api(
  endpoint(
    route = "/groups/{id}/transfer",
    method = "POST",
    query_struct = "GroupTransferQuery",
    result_ok_status = "OK",
    cli_route = "/groups/{id}/transfer",
    cli_help = "Transfer a group to a new parent group or turn a subgroup to a top-level group.",
  ),
  endpoint(
    route = "/groups/{id}/transfer_locations",
    cli_route = "/groups/{id}/transfer_locations",
    query_struct = "GroupTransferLocationQuery",
    multiple_results,
    cli_help = "Retrieve a list of groups to which the user can transfer a project.",
  )
)]
pub(crate) struct Group {
  id: u32,
  name: String,
  #[api(table_skip)]
  path: String,
  description: String,
  visibility: String,
  #[api(table_skip)]
  share_with_group_lock: bool,
  #[api(table_skip)]
  require_two_factor_authentication: bool,
  #[api(table_skip)]
  two_factor_grace_period: u32,
  #[api(table_skip)]
  project_creation_level: String,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  auto_devops_enabled: Option<bool>,
  #[api(table_skip)]
  subgroup_creation_level: String,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  emails_disabled: Option<bool>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  mentions_disabled: Option<bool>,
  #[api(table_skip)]
  lfs_enabled: bool,
  #[api(table_skip)]
  default_branch_protection: u32,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  avatar_url: Option<String>,
  #[api(table_skip)]
  web_url: String,
  #[api(table_skip)]
  request_access_enabled: bool,
  #[api(table_skip)]
  full_name: String,
  full_path: String,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  file_template_project_id: Option<u32>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  parent_id: Option<u32>,
  created_at: String,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  shared_runners_minutes_limit: Option<u32>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  extra_shared_runners_minutes_limit: Option<u32>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  marked_for_deletion_on: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  shared_projects: Option<Vec<Project>>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  shared_with_groups: Option<Vec<SharedWithGroups>>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  projects: Option<Vec<Project>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct SharedWithGroups {
  group_id: u32,
  group_name: String,
  group_full_path: String,
  group_access_level: u32,
  #[serde(skip_serializing_if = "Option::is_none")]
  expires_at: Option<String>,
}

#[derive(Debug, ApiInput, Serialize, Deserialize)]
#[api(no_input_file)]
pub(crate) struct GroupsFilter {
  #[api(
    no_short,
    heading = "Filters",
    long = "skip-groups",
    help = "Skip the group IDs passed"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  skip_groups: Option<Vec<u32>>,

  #[api(
    no_short,
    heading = "Filters",
    long = "all-available",
    help = "Show all the groups you have access to.",
    long_help = "Show all the groups you have access to. (defaults to `false` for authenticated users, `true` for administrators); Attributes `owned` and `min_access_level` have precedence"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  all_available: Option<bool>,

  #[api(
    no_short,
    heading = "Filters",
    help = "Return the list of authorized groups matching the search criteria"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  search: Option<String>,

  #[api(
    no_short,
    heading = "Sorting",
    long = "order-by",
    help = "Order groups by `name`, `path`, `id`, or `similarity`.",
    long_help = "Order groups by `name`, `path`, `id`, or `similarity`. Default is name",
    possible_values = "name,path,id,similarity"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  order_by: Option<String>,

  #[api(
    no_short,
    heading = "Sorting",
    help = "Order groups in asc or desc order.",
    long_help = "Order groups in asc or desc order. Default is asc",
    possible_values = "asc,desc"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  sort: Option<String>,

  #[api(
    no_short,
    heading = "Filters",
    help = "Include group statistics",
    long_help = "Include group statistics (administrators only)"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  statistics: Option<bool>,

  #[api(
    no_short,
    heading = "Filters",
    long = "with-custom-attributes",
    help = "Include custom attributes in response.",
    long_help = "Include custom attributes in response. (administrators only)",
    possible_values = ""
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  with_custom_attributes: Option<bool>,

  #[api(
    no_short,
    heading = "Filters",
    help = "Limit to groups explicitly owned by the current user"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  owned: Option<bool>,

  #[api(
    no_short,
    heading = "Filters",
    long = "min-access-level",
    help = "Limit to groups where current user has at least this access level"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  min_access_level: Option<u32>,

  #[api(
    no_short,
    heading = "Filters",
    long = "top-level-only",
    help = "Limit to top level groups, excluding all subgroups"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  top_level_only: Option<bool>,

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

#[derive(Debug, ApiInput, Serialize, Deserialize)]
#[api(no_input_file)]
pub(crate) struct GroupDetail {
  #[api(
    no_short,
    heading = "Selectors",
    long = "with-custom-attributes-detail",
    help = "Include custom attributes in response.",
    long_help = "Include custom attributes in response. (administrators only)."
  )]
  #[serde(
    skip_serializing_if = "Option::is_none",
    rename = "with_custom_attributes"
  )]
  with_custom_attributes_detail: Option<bool>,

  #[api(
    no_short,
    heading = "Selectors",
    long = "with-projects",
    help = "Include details from projects that belong to the specified group.",
    long_help = "Include details from projects that belong to the specified group. (defaults to true). (Deprecated, scheduled for removal in API v5.)"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  with_projects: Option<bool>,
}

#[derive(Debug, Default, ApiInput, Serialize, Deserialize)]
pub(crate) struct GroupsSharePayload {
  #[api(
    no_short,
    long = "group-id",
    help = "The ID of the group to share with"
  )]
  group_id: u32,

  #[api(
    no_short,
    long = "group-access",
    help = "The access level to grant the group",
    long_help = "The access level to grant the group",
    possible_values = ""
  )]
  group_access: u32,

  #[api(
    no_short,
    long = "expires-at",
    help = "Share expiration date in ISO 8601 format: 2016-09-26"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  expires_at: Option<String>,
}

#[derive(Debug, Default, ApiInput, Serialize, Deserialize)]
pub(crate) struct GroupTransferQuery {
  #[api(
    no_short,
    long = "group-id",
    help = "ID of the new parent group.",
    long_help = "ID of the new parent group. When not specified, the group to transfer is instead turned into a top-level group."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  group_id: Option<u32>,
}

#[derive(Debug, Default, ApiInput, Serialize, Deserialize)]
pub(crate) struct GroupTransferLocationQuery {
  #[api(no_short, help = "The group names to search for.")]
  #[serde(skip_serializing_if = "Option::is_none")]
  search: Option<String>,
}

#[derive(Debug, Default, ApiInput, Serialize, Deserialize)]
pub(crate) struct GroupsDeleteQuery {
  #[api(
    no_short,
    long = "permanently-remove",
    help = "Immediately deletes a subgroup if it is marked for deletion."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  permanently_remove: Option<bool>,

  #[api(
    no_short,
    long = "full-path",
    help = "Full path of subgroup to use with permanently_remove."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  full_path: Option<String>,
}
