pub mod actions;
pub mod create;

use crate::{groups::SharedWithGroups, user::User};
use crud_api::{Api, ApiInput};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Api, Serialize, Deserialize, Debug, Default, Clone)]
#[api(
  endpoint(
    route = "/projects",
    multiple_results,
    query_struct = "ProjectsFilter",
    cli_route = "/projects",
    cli_help = "Get a list of all visible projects across GitLab.",
  ),
  endpoint(
    route = "/projects/{id}",
    query_struct = "ProjectDetailFilter",
    cli_route = "/projects/{id}",
    cli_help = "Get a specific project.",
    cli_long_help = "Get a specific project. This endpoint can be accessed without authentication if the project is publicly accessible.",
  ),
  endpoint(
    route = "/projects",
    method = "POST",
    payload_struct = "ProjectsCreatePayload",
    result_ok_status = "CREATED",
    cli_route = "/projects/create",
    cli_help = "Creates a new project owned by the authenticated user.",
  ),
  endpoint(
    route = "/projects/{id}",
    method = "DELETE",
    result_ok_status = "ACCEPTED",
    result_struct = "EmptyResponse",
    cli_route = "/projects/{id}/delete",
    cli_help = "Deletes a project including all associated resources (including issues and
merge requests).",
  ),
  endpoint(
    route = "/projects/{id}/restore",
    method = "POST",
//    result_ok_status = "CREATED",
    cli_route = "/projects/{id}/restore",
    cli_help = "Restores project marked for deletion.",
  ),
)]
#[api(
  endpoint(
    route = "/projects/{id}/fork",
    method = "POST",
    query_struct = "ProjectsForkQuery",
    result_ok_status = "CREATED",
    cli_route = "/projects/{id}/fork",
    cli_help = "Forks a project into the user namespace of the authenticated user or the one provided.",
    cli_long_help = "Forks a project into the user namespace of the authenticated user or the one provided.
The forking operation for a project is asynchronous and is completed in a background job. The request returns immediately. To determine whether the fork of the project has completed, query the import_status for the new project."
  ),
  endpoint(
    route = "/projects/{id}/forks",
    multiple_results,
    query_struct = "ProjectsForksQuery",
    cli_route = "/projects/{id}/forks",
    cli_help = "List the projects accessible to the calling user that have an established, forked relationship with the specified project",
  ),
  endpoint(
    route = "/projects/{id}/fork",
    method = "DELETE",
    result_ok_status = "NO_CONTENT",
    cli_no_output,
    result_struct = "EmptyResponse",
    cli_route = "/projects/{id}/fork/delete",
    cli_help = "Delete an existing forked from relationship",
  ),
  endpoint(
    route = "/projects/{id}/housekeeping",
    method = "POST",
    result_ok_status = "CREATED",
    cli_no_output,
    result_struct = "EmptyResponse",
    cli_route = "/projects/{id}/housekeeping",
    cli_help = "Start the Housekeeping task for a project",
  )
)]
#[api(
  endpoint(
    route = "/projects/{id}/star",
    method = "POST",
    result_ok_status = "CREATED",
    result_ko_status(status = "NOT_MODIFIED", message = "The project is already starred."),
    cli_route = "/projects/{id}/star",
    cli_help = "Stars a given project.",
  ),
  endpoint(
    route = "/projects/{id}/unstar",
    method = "POST",
    result_ok_status = "CREATED",
    result_ko_status(status = "NOT_MODIFIED", message = "The project is not starred."),
    cli_route = "/projects/{id}/unstar",
    cli_help = "Unstars a given project.",
  )
)]
#[api(
  endpoint(
    route = "/projects/{id}/archive",
    method = "POST",
    result_ok_status = "CREATED",
    cli_route = "/projects/{id}/archive",
    cli_help = "Archives the project if the user is either an administrator or the owner of this project.",
    cli_long_help = "Archives the project if the user is either an administrator or the owner of this project. This action is idempotent, thus archiving an already archived project does not change the project."
  ),
  endpoint(
    route = "/projects/{id}/unarchive",
    method = "POST",
    result_ok_status = "CREATED",
    cli_route = "/projects/{id}/unarchive",
    cli_help = "Unarchives the project if the user is either an administrator or the owner of this project.",
    cli_long_help = "Unarchives the project if the user is either an administrator or the owner of this project. This action is idempotent, thus unarchiving a non-archived project doesn't change the project.",
  )
)]
#[api(
  endpoint(
    route = "/projects/{id}/share",
    method = "POST",
    result_ok_status = "CREATED",
    cli_no_output,
    result_struct = "EmptyResponse",
    query_struct = "ProjectSharePayload",
    cli_route = "/projects/{id}/share",
    cli_help = "Allow to share project with group.",
  ),
  endpoint(
    route = "/projects/{id}/share/{gid}",
    method = "DELETE",
    result_ok_status = "NO_CONTENT",
    cli_no_output,
    result_struct = "EmptyResponse",
    cli_route = "/projects/{id}/share/{gid}/delete",
    cli_help = "Unshare the project from the group.",
  )
)]
#[api(endpoint(
  route = "/projects/{id}/transfer",
  method = "PUT",
  query_struct = "TransferQuery",
  cli_route = "/projects/{id}/transfert",
  cli_help = "Transfer a project to a new namespace.",
))]
#[api(endpoint(
  route = "/projects/{id}/import_project_members/{project_id}",
  method = "POST",
  result_ok_status = "CREATED",
  cli_no_output,
  result_struct = "EmptyResponse",
  cli_route = "/projects/{id}/import_project_members/{project_id}",
  cli_help = "Import members from another project.",
))]
#[api(endpoint(
  route = "/projects/{id}/snapshot",
  stream,
  query_struct = "ProjectsSnapshot",
  cli_route = "/projects/{id}/snapshot",
  cli_help = "",
))]
#[api(
  endpoint(
    route = "/users/{id}/projects",
    multiple_results,
    query_struct = "UsersProjectsFilter",
    cli_route = "/users/{id}/projects",
    cli_help = "Get a list of visible projects owned by the given user.",
    cli_long_help = "Get a list of visible projects owned by the given user. When accessed without authentication, only public projects are returned."
  ),
  endpoint(
    route = "/users/{id}/starred_projects",
    multiple_results,
    query_struct = "ProjectsUsersFilter",
    cli_route = "/users/{id}/starred_projects",
    cli_help = "Get a list of visible projects owned by the given user.",
    cli_long_help = "Get a list of visible projects owned by the given user. When accessed without
authentication, only public projects are returned."
  )
)]
#[api(
  endpoint(
    route = "/groups/{id}/projects",
    multiple_results,
    query_struct = "ProjectsFilter",
    cli_route = "/groups/{id}/projects",
    cli_help = "Get a list of projects in this group.",
  ),
  endpoint(
    route = "/groups/{id}/projects/shared",
    multiple_results,
    query_struct = "ProjectsFilter",
    cli_route = "/groups/{id}/projects/shared",
    cli_help = "Get a list of projects shared to this group.",
  )
)]
pub(crate) struct Project {
  id: u32,
  path_with_namespace: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  description: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  default_branch: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[api(table_skip)]
  visibility: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  ssh_url_to_repo: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  http_url_to_repo: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  web_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[api(table_skip)]
  readme_url: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  topics: Option<Vec<String>>,
  #[api(table_skip)]
  name: String,
  #[api(table_skip)]
  name_with_namespace: String,
  #[api(table_skip)]
  path: String,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  issues_enabled: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[api(table_skip)]
  open_issues_count: Option<u32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[api(table_skip)]
  merge_requests_enabled: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[api(table_skip)]
  jobs_enabled: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[api(table_skip)]
  wiki_enabled: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[api(table_skip)]
  snippets_enabled: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[api(table_skip)]
  can_create_merge_request_in: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[api(table_skip)]
  resolve_outdated_diff_discussions: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[api(table_skip)]
  container_registry_access_level: Option<String>,
  #[api(table_skip)]
  created_at: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  last_activity_at: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  creator_id: Option<u32>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  import_status: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  archived: Option<bool>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  forks_count: Option<u32>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  star_count: Option<usize>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  avatar_url: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  shared_runners_enabled: Option<bool>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  runners_token: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  ci_default_git_depth: Option<u32>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  ci_forward_deployment_enabled: Option<bool>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  public_jobs: Option<bool>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  shared_with_groups: Option<Vec<SharedWithGroups>>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  only_allow_merge_if_pipeline_succeeds: Option<bool>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  allow_merge_on_skipped_pipeline: Option<bool>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  restrict_user_defined_variables: Option<bool>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  only_allow_merge_if_all_discussions_are_resolved: Option<bool>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  remove_source_branch_after_merge: Option<bool>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  request_access_enabled: Option<bool>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  merge_method: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  squash_option: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  autoclose_referenced_issues: Option<bool>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  suggestion_commit_message: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  marked_for_deletion_on: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  container_registry_image_prefix: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  issues_template: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  merge_requests_template: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[api(table_skip)]
  owner: Option<User>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  namespace: Option<Namespace>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  statistics: Option<Stattistics>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  _links: Option<Links>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Namespace {
  #[serde(skip_serializing_if = "Option::is_none")]
  id: Option<u32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  path: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  kind: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  full_path: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Stattistics {
  #[serde(skip_serializing_if = "Option::is_none")]
  commit_count: Option<u32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  storage_size: Option<u32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  repository_size: Option<u32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  wiki_size: Option<u32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  lfs_objects_size: Option<u32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  job_artifacts_size: Option<u32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  packages_size: Option<u32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  snippets_size: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub(crate) struct Links {
  #[serde(rename = "self")]
  self_: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  issues: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  merge_requests: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  repo_branches: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  labels: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  events: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  members: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  cluster_agents: Option<String>,
}

impl Display for Project {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(&self.path_with_namespace)
  }
}

#[derive(Debug, ApiInput, Serialize, Deserialize)]
#[api(no_input_file)]
pub(crate) struct ProjectsFilter {
  #[api(no_short, heading = "Filters", help = "Limit by archived status.")]
  archived: Option<bool>,

  #[api(
    no_short,
    long = "id-after",
    heading = "Filters",
    help = "Limit results to projects with IDs greater than the specified ID."
  )]
  id_after: Option<u32>,
  #[api(
    no_short,
    long = "id-before",
    heading = "Filters",
    help = "Limit results to projects with IDs less than the specified ID."
  )]
  id_before: Option<u32>,

  #[api(
    no_short,
    long = "last-activity-after",
    heading = "Filters",
    help = "Limit results to projects with last_activity after specified time.",
    long_help = "Limit results to projects with last_activity after specified time. Format: ISO 8601 (YYYY-MM-DDTHH:MM:SSZ)"
  )]
  last_activity_after: Option<String>,

  #[api(
    no_short,
    long = "last-activity-before",
    heading = "Filters",
    help = "Limit results to projects with last_activity before specified time.",
    long_help = "Limit results to projects with last_activity before specified time. Format: ISO 8601 (YYYY-MM-DDTHH:MM:SSZ)"
  )]
  last_activity_before: Option<String>,

  #[api(
    no_short,
    heading = "Filters",
    help = "Limit by projects that the current user is a member of."
  )]
  membership: Option<bool>,

  #[api(
    no_short,
    long = "min-access-level",
    heading = "Filters",
    help = "Limit by current user minimal access level."
  )]
  min_access_level: Option<u32>,

  #[api(
    no_short,
    long = "order-by",
    heading = "Sorting",
    help = "Return projects ordered by",
    long_help = "Return projects ordered by `id`, `name`, `path`, `created_at`, `updated_at`, `last_activity_at`, or `similarity` fields. `repository_size`, `storage_size`, `packages_size` or `wiki_size` fields are only allowed for admins. `similarity` is only available when searching and is limited to projects that the current user is a member of. Default is `created_at`.",
    possible_values = "id,name,path,created_at,updated_at,last_activity_at,similarity,repository_size,storage_size,packages_size,wiki_size"
  )]
  order_by: Option<String>,

  #[api(
    no_short,
    heading = "Filters",
    help = "Limit by projects explicitly owned by the current user."
  )]
  owned: Option<bool>,

  #[api(
    no_short,
    long = "repository-checksum-failed",
    heading = "Filters",
    help = "Limit projects where the repository checksum calculation has failed."
  )]
  repository_checksum_failed: Option<bool>,

  #[api(
    no_short,
    long = "repository-storage",
    heading = "Filters",
    help = "Limit results to projects stored on `repository_storage`."
  )]
  repository_storage: Option<String>,

  #[api(
    no_short,
    long = "search-namespaces",
    heading = "Filters",
    help = "Include ancestor namespaces when matching search criteria.",
    long_help = "Include ancestor namespaces when matching search criteria. Default is false."
  )]
  search_namespaces: Option<bool>,

  #[api(
    no_short,
    heading = "Filters",
    help = "Return list of projects matching the search criteria."
  )]
  search: Option<String>,

  #[api(
    no_short,
    heading = "Filters",
    help = "Return only limited fields for each project.",
    long_help = "Return only limited fields for each project. This is a no-op without authentication as then only simple fields are returned."
  )]
  simple: Option<bool>,

  #[api(
    no_short,
    heading = "Sorting",
    long_help = "Return projects sorted in `asc` or `desc` order. Default is `desc`.",
    help = "Return projects sorted in `asc` or `desc` order.",
    possible_values = "asc,desc"
  )]
  sort: Option<String>,

  #[api(
    no_short,
    heading = "Filters",
    help = "Limit by projects starred by the current user."
  )]
  starred: Option<bool>,

  #[api(
    no_short,
    heading = "Filters",
    help = "Include project statistics.",
    long_help = "Include project statistics. Only available to Reporter or higher level role members."
  )]
  statistics: Option<bool>,

  #[api(
    no_short,
    heading = "Filters",
    long_help = "Comma-separated topic names. Limit results to projects that match all of given topics. See `topics` attribute.",
    help = "Limit results to projects that match all of given topics."
  )]
  topic: Option<String>,

  #[api(
    no_short,
    heading = "Filters",
    long_help = "Limit by visibility `public`, `internal`, or `private`.",
    help = "Limit by visibility."
  )]
  visibility: Option<String>,

  #[api(
    no_short,
    long = "wiki-checksum-failed",
    heading = "Filters",
    help = "Limit projects where the wiki checksum calculation has failed."
  )]
  wiki_checksum_failed: Option<bool>,

  #[api(
    no_short,
    long = "with-custom-attributes",
    heading = "Filters",
    help = "Include custom attributes in response."
  )]
  with_custom_attributes: Option<bool>,

  #[api(
    no_short,
    long = "with-issues-enabled",
    heading = "Filters",
    help = "Limit by enabled issues feature."
  )]
  with_issues_enabled: Option<bool>,

  #[api(
    no_short,
    long = "with-merge-requests-enabled",
    heading = "Filters",
    help = "Limit by enabled merge requests feature."
  )]
  with_merge_requests_enabled: Option<bool>,

  #[api(
    no_short,
    long = "with-programming-language",
    heading = "Filters",
    help = "Limit by projects which use the given programming language."
  )]
  with_programming_language: Option<String>,

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
pub(crate) struct ProjectDetailFilter {
  #[api(
    no_short,
    long = "show-license",
    heading = "Selectors",
    help = "Include project license data."
  )]
  license: Option<bool>,
  #[api(
    no_short,
    long = "show-statistics",
    heading = "Selectors",
    help = "Include project statistics.",
    long_help = "Include project statistics. Only available to Reporter or higher level role members."
  )]
  #[serde(rename = "statistics")]
  statistics_detail: Option<bool>,

  #[api(
    no_short,
    long = "show-custom-attributes",
    heading = "Selectors",
    help = "Include custom attributes in response.",
    long_help = "Include custom attributes in response. (admins only)"
  )]
  #[serde(rename = "with-custom-attributes")]
  with_custom_attributes_detail: Option<bool>,
}

#[derive(Debug, ApiInput, Serialize, Deserialize)]
#[api(no_input_file)]
pub(crate) struct UsersProjectsFilter {
  #[api(no_short, heading = "Filters", help = "Limit by archived status.")]
  archived: Option<u32>,

  #[api(
    no_short,
    long = "id-after",
    heading = "Filters",
    help = "Limit results to projects with IDs greater than the specified ID."
  )]
  id_after: Option<u32>,
  #[api(
    no_short,
    long = "id-before",
    heading = "Filters",
    help = "Limit results to projects with IDs less than the specified ID."
  )]
  id_before: Option<u32>,

  #[api(
    no_short,
    heading = "Filters",
    help = "Limit by projects that the current user is a member of."
  )]
  membership: Option<bool>,

  #[api(
    no_short,
    long = "min-access-level",
    heading = "Filters",
    help = "Limit by current user minimal access level."
  )]
  min_access_level: Option<u32>,

  #[api(
    no_short,
    long = "order-by",
    heading = "Sorting",
    help = "Return projects ordered by",
    long_help = "Return projects ordered by `id`, `name`, `path`, `created_at`, `updated_at`, or `last_activity_at` fields. Default is `created_at`.",
    possible_values = "id,name,path,created_at,updated_at,last_activity_at"
  )]
  order_by: Option<String>,

  #[api(
    no_short,
    heading = "Filters",
    help = "Limit by projects explicitly owned by the current user."
  )]
  owned: Option<bool>,

  #[api(
    no_short,
    heading = "Filters",
    help = "Return list of projects matching the search criteria."
  )]
  search: Option<String>,

  #[api(
    no_short,
    heading = "Filters",
    help = "Return only limited fields for each project.",
    long_help = "Return only limited fields for each project. This is a no-op without authentication as then only simple fields are returned."
  )]
  simple: Option<bool>,

  #[api(
    no_short,
    heading = "Sorting",
    long_help = "Return projects sorted in `asc` or `desc` order. Default is `desc`.",
    help = "Return projects sorted in `asc` or `desc` order."
  )]
  sort: Option<String>,

  #[api(
    no_short,
    heading = "Filters",
    help = "Limit by projects starred by the current user."
  )]
  starred: Option<bool>,

  #[api(
    no_short,
    heading = "Filters",
    help = "Include project statistics.",
    long_help = "Include project statistics. Only available to Reporter or higher level role members."
  )]
  statistics: Option<bool>,

  #[api(
    no_short,
    heading = "Filters",
    long_help = "Limit by visibility `public`, `internal`, or `private`.",
    help = "Limit by visibility."
  )]
  visibility: Option<String>,

  #[api(
    no_short,
    long = "with-custom-attributes",
    heading = "Filters",
    help = "Include custom attributes in response."
  )]
  with_custom_attributes: Option<bool>,

  #[api(
    no_short,
    long = "with-issues-enabled",
    heading = "Filters",
    help = "Limit by enabled issues feature."
  )]
  with_issues_enabled: Option<bool>,

  #[api(
    no_short,
    long = "with-merge-requests-enabled",
    heading = "Filters",
    help = "Limit by enabled merge requests feature."
  )]
  with_merge_requests_enabled: Option<bool>,

  #[api(
    no_short,
    long = "with-programming-language",
    heading = "Filters",
    help = "Limit by projects which use the given programming language."
  )]
  with_programming_language: Option<String>,

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
