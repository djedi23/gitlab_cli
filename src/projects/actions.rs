use crud_api::ApiInput;
use serde::{Deserialize, Serialize};

#[derive(Debug, ApiInput, Serialize, Deserialize)]
#[api(no_input_file)]
pub(crate) struct ProjectsForkQuery {
  #[api(
    no_short,
    help = "The name assigned to the resultant project after forking."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  name: Option<String>,

  #[api(
    no_short,
    long = "namespace-id",
    help = "The ID of the namespace that the project is forked to."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  namespace_id: Option<u32>,

  #[api(
    no_short,
    long = "namespace-path",
    help = "The path of the namespace that the project is forked to."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  namespace_path: Option<String>,

  #[api(
    no_short,
    help = "The path assigned to the resultant project after forking."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  path: Option<String>,

  #[api(
    no_short,
    help = "The description assigned to the resultant project after forking."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  description: Option<String>,

  #[api(
    no_short,
    help = "The visibility level assigned to the resultant project after forking.",
    possible_values = "private,internal,public"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  visibility: Option<String>,
}

#[derive(Debug, ApiInput, Serialize, Deserialize)]
#[api(no_input_file)]
pub(crate) struct ProjectsForksQuery {
  #[api(no_short, heading = "Filters", help = "Limit by archived status.")]
  #[serde(skip_serializing_if = "Option::is_none")]
  archived: Option<String>,

  #[api(
    no_short,
    heading = "Filters",
    help = "Limit by projects that the current user is a member of."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  membership: Option<bool>,

  #[api(
    no_short,
    heading = "Filters",
    long = "min-access-level",
    help = "Limit by current user minimal access level.",
    possible_values = "0,5,10,20,30,40,50"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  min_access_level: Option<u32>,

  #[api(
    no_short,
    heading = "Sorting",
    long = "order-by",
    help = "Return projects ordered by ...",
    long_help = "Return projects ordered by `id`, `name`, `path`, `created_at`, `updated_at`, or `last_activity_at` fields. Default is `created_at`.",
    possible_values = "id,name,path,created_at,updated_at,last_activity_at"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  order_by: Option<String>,

  #[api(
    no_short,
    heading = "Filters",
    help = "Limit by projects explicitly owned by the current user."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  owned: Option<bool>,

  #[api(
    no_short,
    heading = "Filters",
    help = "Return list of projects matching the search criteria."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  search: Option<String>,

  #[api(
    no_short,
    heading = "Filters",
    help = "Return only limited fields for each project.",
    long_help = "Return only limited fields for each project. This is a no-op without authentication as then only simple fields are returned."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  simple: Option<bool>,

  #[api(
    no_short,
    heading = "Sorting",
    help = "Return projects sorted in `asc` or `desc` order.",
    long_help = "Return projects sorted in `asc` or `desc` order. Default is `desc`.",
    possible_values = "asc,desc"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  sort: Option<String>,

  #[api(
    no_short,
    heading = "Filters",
    help = "Limit by projects starred by the current user."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  starred: Option<bool>,

  #[api(
    no_short,
    heading = "Filters",
    help = "Include project statistics.",
    long_help = "Include project statistics. Only available to Reporter or higher level role members."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  statistics: Option<bool>,

  #[api(
    no_short,
    heading = "Filters",
    long = "visibility",
    help = "Limit by visibility `public`, `internal`, or `private`.",
    long_help = "Limit by visibility `public`, `internal`, or `private`.",
    possible_values = "public,internal,private"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  visibility: Option<String>,

  #[api(
    no_short,
    heading = "Filters",
    long = "with-custom-attributes",
    help = "Include custom attributes in response.",
    long_help = "Include custom attributes in response. (admins only)"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  with_custom_attributes: Option<bool>,

  #[api(
    no_short,
    heading = "Filters",
    long = "with-issues-enabled",
    help = "Limit by enabled issues feature."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  with_issues_enabled: Option<bool>,

  #[api(
    no_short,
    heading = "Filters",
    long = "with-merge-requests-enabled",
    help = "Limit by enabled merge requests feature."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  with_merge_requests_enabled: Option<bool>,
}

#[derive(Debug, ApiInput, Serialize, Deserialize)]
#[api(no_input_file)]
pub(crate) struct ProjectSharePayload {
  #[api(
    no_short,
    long = "group-access",
    help = "The access level to grant the group.",
//    possible_values = "0,5,10,20,30,40,50"
  )]
  //    #[serde(skip_serializing_if = "Option::is_none")]
  group_access: Option<u32>,

  #[api(
    no_short,
    long = "group-id",
    help = "The ID of the group to share with."
  )]
  //    #[serde(skip_serializing_if = "Option::is_none")]
  group_id: Option<u32>,

  #[api(
    no_short,
    long = "expires-at",
    help = "Share expiration date in ISO 8601 format: 2016-09-26"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  expires_at: Option<String>,
}

#[derive(Debug, ApiInput, Serialize, Deserialize)]
#[api(no_input_file)]
pub(crate) struct TransferQuery {
  #[api(
    no_short,
    help = "The ID or path of the namespace to transfer to project to."
  )]
  namespace: String,
}

#[derive(Debug, ApiInput, Serialize, Deserialize)]
#[api(no_input_file)]
pub(crate) struct ProjectsSnapshot {
  #[api(
    no_short,
    help = "Whether to download the wiki, rather than project, repository."
  )]
  wiki: Option<bool>,
}
