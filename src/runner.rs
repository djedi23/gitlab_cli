use crate::{groups::Group, projects::Project};
use crud_api::{Api, ApiInput};
use serde::{Deserialize, Serialize};

#[derive(Api, Clone, Debug, Serialize, Deserialize, Default)]
#[api(
  endpoint(
    route = "/runners",
    query_struct = "RunnerFilter",
    multiple_results,
    cli_route = "/runners",
    cli_help = "Get a list of specific runners available to the user."
  ),
  endpoint(
    route = "/runners/{id}",
    cli_route = "/runners/{id}",
    cli_help = "Get details of a runner.",
    cli_long_help = "Get details of a runner. At least the Maintainer role is required to get runner details at the project and group level. Instance-level runner details via this endpoint are available to all signed in users."
  ),
  endpoint(
    route = "/runners/{id}",
    method = "PUT",
    payload_struct = "RunnerUpdate",
    cli_route = "/runners/{id}/update",
    cli_help = "Update details of a runner.",
  ),
  endpoint(
    route = "/runners/all",
    query_struct = "RunnerFilter",
    multiple_results,
    cli_route = "/runners/all",
    cli_help = "Get a list of all runners in the GitLab instance (specific and shared).",
    cli_long_help = "Get a list of all runners in the GitLab instance (specific and shared). Access is restricted to users with administrator access."
  ),
  endpoint(
    route = "/runners",
    payload_struct = "RunnerCreatePayload",
    method = "POST",
    cli_route = "/runners/",
    cli_help = "Register a new runner for the instance."
  ),
  endpoint(
    route = "/runners/{id}",
    method = "DELETE",
    result_ok_status = "NO_CONTENT",
    result_struct = "EmptyResponse",
    cli_route = "/runners/{id}/delete",
    cli_help = "Delete the runner by ID"
  ),
  endpoint(
    route = "/runners",
    payload_struct = "RunnerTokenPayload",
    result_ok_status = "NO_CONTENT",
    result_struct = "EmptyResponse",
    method = "DELETE",
    cli_route = "/runners/delete_by_token",
    cli_help = "Delete a runner by authentication token."
  ),
  endpoint(
    route = "/runners/verify",
    method = "POST",
    payload_struct = "RunnerTokenPayload",
    result_ok_status = "OK",
    result_struct = "EmptyResponse",
    cli_route = "/runners/verify",
    cli_help = "Validates authentication credentials for a registered runner."
  ),
  endpoint(
    route = "/runners/reset_registration_token",
    method = "POST",
    result_ok_status = "OK",
    result_struct = "EmptyResponse",
    cli_route = "/runners/reset_registration_token",
    cli_help = "Resets the runner registration token for the GitLab instance."
  ),
  endpoint(
    route = "/runners/reset_authentication_token",
    method = "POST",
    payload_struct = "RunnerTokenPayload",
    result_ok_status = "OK",
    result_struct = "EmptyResponse",
    cli_route = "/runners/reset_authentication_token_by_token",
    cli_help = "Resets the runner's authentication token by using the current token's value as an input."
  ),
  endpoint(
    route = "/runners/{id}/reset_authentication_token",
    method = "POST",
    result_ok_status = "OK",
    result_struct = "EmptyResponse",
    cli_route = "/runners/{id}/reset_authentication_token",
    cli_help = "Resets the runner's authentication token by using its runner ID."
  )
)]
#[api(
  endpoint(
    route = "/projects/{id}/runners",
    query_struct = "RunnerFilter",
    multiple_results,
    cli_route = "/projects/{id}//runners",
    cli_help = "List all runners available in the project.",
    cli_long_help = "List all runners available in the project, including from ancestor groups and any allowed shared runners."
  ),
  endpoint(
    route = "/projects/{id}/runners",
    payload_struct = "RunnerId",
    method = "POST",
    cli_route = "/projects/{id}/runners/enable",
    cli_help = "Enable an available specific runner in the project."
  ),
  endpoint(
    route = "/projects/{id}/runners/{runner_id}",
    method = "DELETE",
    cli_route = "/projects/{id}/runners/{runner_id}/disable",
    cli_help = "Disable a specific runner from the project.",
    cli_long_help = "Disable a specific runner from the project. It works only if the project isn't
the only project associated with the specified runner."
  ),
  endpoint(
    route = "/projects/{id}/runners/reset_registration_token",
    method = "POST",
    result_ok_status = "OK",
    result_struct = "EmptyResponse",
    cli_route = "/projects/{id}/runners/reset_registration_token",
    cli_help = "Resets the runner registration token for a project."
  )
)]
#[api(
  endpoint(
    route = "/groups/{id}/runners",
    query_struct = "RunnerFilter",
    multiple_results,
    cli_route = "/groups/{id}/runners",
    cli_help = "List all runners available in the project.",
    cli_long_help = "List all runners available in the group as well as its ancestor groups, including any allowed shared runners."
  ),
  endpoint(
    route = "/groups/{id}/runners/reset_registration_token",
    method = "POST",
    result_ok_status = "OK",
    result_struct = "EmptyResponse",
    cli_route = "/groups/{id}/runners/reset_registration_token",
    cli_help = "Resets the runner registration token for a group."
  )
)]
pub struct Runner {
  id: u32,
  description: String,
  ip_address: Option<String>,
  active: bool,
  paused: bool,
  is_shared: bool,
  runner_type: String,
  name: String,
  online: bool,
  status: String,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  tag_list: Option<Vec<String>>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  run_untagged: Option<bool>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  locked: Option<bool>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  maximum_timeout: Option<u64>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  access_level: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  version: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  revision: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  platform: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  architecture: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  contacted_at: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  projects: Option<Vec<Project>>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  groups: Option<Vec<Group>>,
}

#[derive(ApiInput, Debug, Serialize, Deserialize)]
#[api(no_input_file)]
pub(crate) struct RunnerFilter {
  #[api(
    no_short,
    long = "type",
    help = "The type of runners to show.",
    long_help = "The type of runners to show, one of: `instance_type`, `group_type`, `project_type`",
    possible_values = "instance_type,group_type,project_type"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  type_: Option<String>,

  #[api(
    no_short,
    help = "The status of runners to show, one of: `online`, `offline`, `stale`, and `never_contacted`.",
    long_help = "The status of runners to show, one of: `online`, `offline`, `stale`, and `never_contacted`. `active` and `paused` are also possible values which were deprecated in GitLab 14.8 and will be removed in GitLab 16.0",
    possible_values = "online,offline,stale,never_contacted"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  status: Option<String>,

  #[api(
    no_short,
    help = "Whether to include only runners that are accepting or ignoring new jobs",
    long_help = "Whether to include only runners that are accepting or ignoring new jobs"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  paused: Option<bool>,

  #[api(no_short, long = "tag-list", help = "List of the runner's tags")]
  #[serde(skip_serializing_if = "Option::is_none")]
  tag_list: Option<Vec<String>>,

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

#[derive(ApiInput, Debug, Default, Serialize, Deserialize)]
pub(crate) struct RunnerUpdate {
  #[api(no_short, help = "The description of a runner")]
  #[serde(skip_serializing_if = "Option::is_none")]
  description: Option<String>,

  #[api(
    no_short,
    help = "Flag indicating whether the runner should ignore new jobs"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  paused: Option<String>,

  #[api(
    no_short,
    long = "tag-list",
    help = "The list of tags for a runner; put array of tags, that should be finally assigned to a runner"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  tag_list: Option<Vec<String>>,

  #[api(
    no_short,
    long = "run-untagged",
    help = "Flag indicating the runner can execute untagged jobs"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  run_untagged: Option<bool>,

  #[api(no_short, help = "Flag indicating the runner is locked")]
  #[serde(skip_serializing_if = "Option::is_none")]
  locked: Option<bool>,

  #[api(
    no_short,
    long = "access-level",
    help = "The access_level of the runner; `not_protected` or `ref_protected`",
    long_help = "The access_level of the runner; `not_protected` or `ref_protected`",
    possible_values = "not_protected,ref_protected"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  access_level: Option<String>,

  #[api(
    no_short,
    long = "maximum-timeout",
    help = "Maximum timeout set when this runner handles the job"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  maximum_timeout: Option<u64>,
}

#[derive(ApiInput, Debug, Serialize, Deserialize)]
#[api(no_input_file)]
pub(crate) struct RunnerId {
  #[api(no_short, long = "runner-id", help = "The ID of a runner")]
  runner_id: u64,
}

#[derive(ApiInput, Debug, Clone, Default, Serialize, Deserialize)]
pub(crate) struct RunnerCreateInfo {
  #[api(no_short)]
  #[serde(skip_serializing_if = "Option::is_none")]
  name: Option<String>,

  #[api(no_short)]
  #[serde(skip_serializing_if = "Option::is_none")]
  version: Option<String>,

  #[api(no_short)]
  #[serde(skip_serializing_if = "Option::is_none")]
  revision: Option<String>,
}

#[derive(ApiInput, Debug, Default, Serialize, Deserialize)]
pub(crate) struct RunnerCreatePayload {
  #[api(no_short, help = "Registration token")]
  token: String,

  #[api(no_short, help = "Runner's description")]
  #[serde(skip_serializing_if = "Option::is_none")]
  description: Option<String>,

  #[api(no_short, long = "infos", help = "Runner's metadata.")]
  #[serde(skip_serializing_if = "Option::is_none")]
  infos: Option<RunnerCreateInfo>,

  #[api(no_short, help = "Whether the runner should ignore new jobs")]
  #[serde(skip_serializing_if = "Option::is_none")]
  paused: Option<bool>,

  #[api(
    no_short,
    help = "Whether the runner should be locked for current project"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  locked: Option<bool>,

  #[api(
    no_short,
    long = "run-untagged",
    help = "Whether the runner should handle untagged jobs"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  run_untagged: Option<bool>,

  #[api(no_short, long = "tag-list", help = "List of runner's tags")]
  #[serde(skip_serializing_if = "Option::is_none")]
  tag_list: Option<Vec<String>>,

  #[api(
    no_short,
    long = "access-level",
    help = "The access_level of the runner",
    long_help = "The access_level of the runner",
    possible_values = "not_protected,ref_protected"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  access_level: Option<String>,

  #[api(
    no_short,
    long = "maximum-timeout",
    help = "Maximum timeout set when this runner handles the job"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  maximum_timeout: Option<u32>,

  #[api(
    no_short,
    long = "maintenance-note",
    help = "Free-form maintenance notes for the runner (1024 characters)"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  maintenance_note: Option<String>,
}

#[derive(ApiInput, Debug, Default, Serialize, Deserialize)]
pub(crate) struct RunnerTokenPayload {
  #[api(no_short, help = "The runner's authentication token.")]
  token: String,
}
