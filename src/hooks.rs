use crud_api::{Api, ApiInput};
use serde::{Deserialize, Serialize};

#[derive(Api, Default, Debug, Deserialize, Serialize)]
#[api(
  endpoint(
    route = "/projects/{id}/hooks",
    multiple_results,
    cli_route = "/projects/{id}/hooks",
    cli_help = "Get a list of project hooks.",
  ),
  endpoint(
    route = "/projects/{id}/hooks/{hook_id}",
    cli_route = "/projects/{id}/hooks/{hook_id}",
    cli_help = "Get a specific hook for a project.",
  ),
  endpoint(
    route = "/projects/{id}/hooks",
    method = "POST",
    payload_struct = "ProjectsHookPayload",
    result_ok_status = "CREATED",
    cli_route = "/projects/{id}/hooks/create",
    cli_help = "Adds a hook to a specified project.",
  ),
  endpoint(
    route = "/projects/{id}/hooks/{hook_id}",
    method = "PUT",
    payload_struct = "ProjectsHookPayload",
    cli_route = "/projects/{id}/hooks/{hook_id}/edit",
    cli_help = "Edits a hook for a specified project.",
  ),
  endpoint(
    route = "/projects/{id}/hooks/{hook_id}",
    method = "DELETE",
    result_ok_status = "NO_CONTENT",
    cli_no_output,
    result_struct = "EmptyResponse",
    cli_route = "/projects/{id}/hooks/{hook_id}/delete",
    cli_long_help = "Removes a hook from a project. This is an idempotent method and can be called multiple times. Either the hook is available or not.",
    cli_help = "Removes a hook from a project.",
  )
)]
#[api(
  endpoint(
    route = "/groups/{id}/hooks",
    multiple_results,
    cli_route = "/groups/{id}/hooks",
    cli_help = "Get a list of group hooks",
  ),
  endpoint(
    route = "/groups/{id}/hooks/{hook_id}",
    cli_route = "/groups/{id}/hooks/{hook_id}",
    cli_help = "Get a specific hook for a group.",
  ),
  endpoint(
    route = "/groups/{id}/hooks",
    method = "POST",
    payload_struct = "GroupsHookPayload",
    result_ok_status = "CREATED",
    cli_route = "/groups/{id}/hooks/create",
    cli_help = "Adds a hook to a specified group.",
  ),
  endpoint(
    route = "/groups/{id}/hooks/{hook_id}",
    method = "PUT",
    payload_struct = "GroupsHookPayload",
    cli_route = "/groups/{id}/hooks/{hook_id}/edit",
    cli_help = "Edits a hook for a specified groups.",
  ),
  endpoint(
    route = "/groups/{id}/hooks/{hook_id}",
    method = "DELETE",
    result_ok_status = "NO_CONTENT",
    cli_no_output,
    result_struct = "EmptyResponse",
    cli_route = "/groups/{id}/hooks/{hook_id}/delete",
    cli_long_help = "Removes a hook from a group. This is an idempotent method and can be called multiple times. Either the hook is available or not.",
    cli_help = "Removes a hook from a group.",
  )
)]
#[allow(dead_code, non_snake_case)]
pub(crate) struct Hook {
  id: u32,
  url: String,
  project_id: Option<u32>,
  group_id: Option<u32>,
  push_events: bool,
  push_events_branch_filter: Option<String>,
  issues_events: bool,
  confidential_issues_events: bool,
  merge_requests_events: bool,
  tag_push_events: bool,
  note_events: bool,
  confidential_note_events: Option<bool>,
  job_events: bool,
  pipeline_events: bool,
  wiki_page_events: bool,
  deployment_events: bool,
  releases_events: bool,
  #[api(table_skip)]
  subgroup_events: Option<bool>,
  #[api(table_skip)]
  enable_ssl_verification: bool,
  created_at: String,
}

#[derive(Debug, Default, ApiInput, Serialize, Deserialize)]
pub(crate) struct ProjectsHookPayload {
  #[api(
    no_short,
    long = "confidential-issues-events",
    help = "Trigger hook on confidential issues events."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  confidential_issues_events: Option<bool>,

  #[api(
    no_short,
    long = "confidential-note-events",
    help = "Trigger hook on confidential note events."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  confidential_note_events: Option<bool>,

  #[api(
    no_short,
    long = "deployment-events",
    help = "Trigger hook on deployment events."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  deployment_events: Option<bool>,

  #[api(
    no_short,
    long = "enable-ssl-verification",
    help = "Do SSL verification when triggering the hook."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  enable_ssl_verification: Option<bool>,

  #[api(
    no_short,
    long = "issues-events",
    help = "Trigger hook on issues events."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  issues_events: Option<bool>,

  #[api(no_short, long = "job-events", help = "Trigger hook on job events.")]
  #[serde(skip_serializing_if = "Option::is_none")]
  job_events: Option<bool>,

  #[api(
    no_short,
    long = "merge-requests-events",
    help = "Trigger hook on merge requests events."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  merge_requests_events: Option<bool>,

  #[api(no_short, long = "note-events", help = "Trigger hook on note events.")]
  #[serde(skip_serializing_if = "Option::is_none")]
  note_events: Option<bool>,

  #[api(
    no_short,
    long = "pipeline-events",
    help = "Trigger hook on pipeline events."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  pipeline_events: Option<bool>,

  #[api(
    no_short,
    long = "push-events-branch-filter",
    help = "Trigger hook on push events for matching branches only."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  push_events_branch_filter: Option<String>,

  #[api(no_short, long = "push-events", help = "Trigger hook on push events.")]
  #[serde(skip_serializing_if = "Option::is_none")]
  push_events: Option<bool>,

  #[api(
    no_short,
    long = "tag-push-events",
    help = "Trigger hook on tag push events."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  tag_push_events: Option<bool>,

  #[api(
    no_short,
    help = "Secret token to validate received payloads.",
    long_help = "Secret token to validate received payloads. This isn't returned in the response."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  token: Option<String>,

  #[api(no_short, help = "The hook URL.")]
  url: String,

  #[api(
    no_short,
    long = "wiki-page-events",
    help = "Trigger hook on wiki events."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  wiki_page_events: Option<bool>,

  #[api(
    no_short,
    long = "releases-events",
    help = "Trigger hook on release events."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  releases_events: Option<bool>,
}

#[derive(Debug, Default, ApiInput, Serialize, Deserialize)]
pub(crate) struct GroupsHookPayload {
  #[api(
    no_short,
    long = "confidential-issues-events",
    help = "Trigger hook on confidential issues events."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  confidential_issues_events: Option<bool>,

  #[api(
    no_short,
    long = "confidential-note-events",
    help = "Trigger hook on confidential note events."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  confidential_note_events: Option<bool>,

  #[api(
    no_short,
    long = "deployment-events",
    help = "Trigger hook on deployment events."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  deployment_events: Option<bool>,

  #[api(
    no_short,
    long = "enable-ssl-verification",
    help = "Do SSL verification when triggering the hook."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  enable_ssl_verification: Option<bool>,

  #[api(
    no_short,
    long = "issues-events",
    help = "Trigger hook on issues events."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  issues_events: Option<bool>,

  #[api(no_short, long = "job-events", help = "Trigger hook on job events.")]
  #[serde(skip_serializing_if = "Option::is_none")]
  job_events: Option<bool>,

  #[api(
    no_short,
    long = "merge-requests-events",
    help = "Trigger hook on merge requests events."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  merge_requests_events: Option<bool>,

  #[api(no_short, long = "note-events", help = "Trigger hook on note events.")]
  #[serde(skip_serializing_if = "Option::is_none")]
  note_events: Option<bool>,

  #[api(
    no_short,
    long = "pipeline-events",
    help = "Trigger hook on pipeline events."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  pipeline_events: Option<bool>,

  #[api(
    no_short,
    long = "push-events-branch-filter",
    help = "Trigger hook on push events for matching branches only."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  push_events_branch_filter: Option<String>,

  #[api(no_short, long = "push-events", help = "Trigger hook on push events.")]
  #[serde(skip_serializing_if = "Option::is_none")]
  push_events: Option<bool>,

  #[api(
    no_short,
    long = "tag-push-events",
    help = "Trigger hook on tag push events."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  tag_push_events: Option<bool>,

  #[api(
    no_short,
    help = "Secret token to validate received payloads.",
    long_help = "Secret token to validate received payloads. This isn't returned in the response."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  token: Option<String>,

  #[api(no_short, help = "The hook URL.")]
  url: String,

  #[api(
    no_short,
    long = "wiki-page-events",
    help = "Trigger hook on wiki events."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  wiki_page_events: Option<bool>,

  #[api(
    no_short,
    long = "releases-events",
    help = "Trigger hook on release events."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  releases_events: Option<bool>,

  #[api(
    no_short,
    long = "subgroup-events",
    help = "Trigger hook on subgroup events"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  subgroup_events: Option<bool>,
}
