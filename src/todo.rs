use crate::{issues::Issue, merge_requests::MergeRequest, projects::Project, user::User};
use crud_api::{Api, ApiInput};
use serde::{Deserialize, Serialize};

#[derive(Api, Serialize, Deserialize, Debug, Default, Clone)]
#[api(
  endpoint(
    route = "/todos",
    multiple_results,
    query_struct = "TodoFilter",
    cli_route = "/todos",
    cli_help = "Returns a list of to-do items.",
    cli_long_help = "Returns a list of to-do items. When no filter is applied, it returns all pending to-do items for the current user. Different filters allow the user to refine the request.",
  ),
  endpoint(
    route = "/todos/{id}/mark_as_done",
    method = "POST",
    result_ok_status = "CREATED",
    cli_route = "/todos/{id}/mark_as_done",
    cli_help = "Marks a single pending to-do item given by its ID for the current user as done.",
    cli_long_help = "Marks a single pending to-do item given by its ID for the current user as done. The to-do item marked as done is returned in the response.",
  ),
  endpoint(
    route = "/todos/mark_as_done",
    method = "POST",
    result_ok_status = "NO_CONTENT",
    cli_no_output,
    result_struct = "EmptyResponse",
    cli_route = "/todos/mark_all_as_done",
    cli_help = "Marks all pending to-do items for the current user as done.",
  )
)]
#[api(endpoint(
  route = "/projects/{id}/issues/{iid}/todo",
  cli_route = "/projects/{id}/issues/{iid}/todo",
  method = "POST",
  result_ok_status = "CREATED",
  result_ko_status(
    status = "NOT_MODIFIED",
    message = "There already exists a to-do item for the user on that issue."
  ),
  cli_help = "Manually creates a to-do item for the current user on an issue.",
  cli_long_help = "Manually creates a to-do item for the current user on an issue. If there already exists a to-do item for the user on that issue, status code 304 is returned.",
))]
#[api(endpoint(
  route = "/projects/{id}/merge_requests/{iid}/todo",
  cli_route = "/projects/{id}/merge_requests/{iid}/todo",
  method = "POST",
  result_ok_status = "CREATED",
  result_ko_status(
    status = "NOT_MODIFIED",
    message = "There already exists a to-do item for the user on that issue."
  ),
  cli_help = "Manually creates a to-do item for the current user on a merge request.",
  cli_long_help = "Manually creates a to-do item for the current user on a merge request. If there already exists a to-do item for the user on that merge request, status code 304 is returned.",
))]
pub(crate) struct Todo {
  id: u32,
  action_name: String,
  #[api(table_skip)]
  target_type: String,
  #[api(table_skip)]
  target_url: String,
  body: String,
  state: String,
  created_at: String,
  updated_at: String,
  #[api(table_skip)]
  target: Option<Target>,
  project: Project,
  #[api(table_skip)]
  author: User,
}

#[derive(Debug, ApiInput, Serialize, Deserialize)]
#[api(no_input_file)]
pub(crate) struct TodoFilter {
  #[api(
    no_short,
    heading = "Filters",
    help = "The action to be filtered.",
    long_help = "The action to be filtered. Can be `assigned`, `mentioned`, `build_failed`, `marked`, `approval_required`, `unmergeable`, `directly_addressed` or `merge_train_removed`.",
    possible_values = "assigned,mentioned,build_failed,marked,approval_required,unmergeable,directly_addressed,merge_train_removed"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  action: Option<String>,

  #[api(
    no_short,
    heading = "Filters",
    long = "author-id",
    help = "The ID of an author"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  author_id: Option<u32>,

  #[api(
    no_short,
    heading = "Filters",
    long = "project-id",
    help = "The ID of a project"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  project_id: Option<u32>,

  #[api(
    no_short,
    heading = "Filters",
    long = "group-id",
    help = "The ID of a group"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  group_id: Option<u32>,

  #[api(
    no_short,
    heading = "Filters",
    help = "The state of the to-do item.",
    long_help = "The state of the to-do item. Can be either `pending` or `done`",
    possible_values = "pending,done"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  state: Option<String>,

  #[api(
    no_short,
    heading = "Filters",
    long = "type",
    help = "The type of to-do item.",
    long_help = "The type of to-do item. Can be either `Issue`, `MergeRequest`, `DesignManagement::Design` or `AlertManagement::Alert`",
    possible_values = "Issue,MergeRequest,DesignManagement::Design,AlertManagement::Alert"
  )]
  #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
  _type: Option<String>,

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
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)] // Good luck serde !
enum Target {
  Issue(Issue),
  MergeRequest(MergeRequest),
}
