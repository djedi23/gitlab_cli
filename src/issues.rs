pub(crate) mod actions;
pub(crate) mod create;
pub(crate) mod edit;

use crate::{
  milestones::Milestone, reference::References, task::TaskCompletionStatus, time::TimeStats, User,
};
use crud_api::{Api, ApiInput};
use crud_pretty_struct::{
  formatters::{bool_check_formatter, markdown_formatter},
  PrettyPrint,
};
use miette::{IntoDiagnostic, WrapErr};
use serde::{Deserialize, Serialize};

#[derive(Api, Debug, Default, Deserialize, Serialize, Clone)] // PrettyPrint
#[api(
  endpoint(
    route = "/issues",
    multiple_results,
    cli_route = "/issues",
    query_struct = "IssueFilter",
    cli_help = "Get all issues the authenticated user has access to.",
    cli_long_help = "Get all issues the authenticated user has access to. By default it
returns only issues created by the current user. To get all issues,
use parameter `--scope=all`."
  ),
  endpoint(
    route = "/issues/{id}",
    cli_route = "/issues/{id}",
    cli_help = "Only for administrators. Get a single issue.",
    cli_long_help = "Only for administrators. Get a single issue.
The preferred way to do this is by using personal access tokens."
  )
)]
#[api(endpoint(
  route = "/groups/{id}/issues",
  multiple_results,
  cli_route = "/groups/{id}/issues",
  query_struct = "IssueFilter",
  cli_help = "Get a list of a group's issues.",
  cli_long_help = "Get a list of a group's issues.
If the group is private, credentials need to be provided for authorization.
The preferred way to do this, is by using personal access tokens."
))]
#[api(
  endpoint(
    route = "/projects/{id}/issues",
    multiple_results,
    cli_route = "/projects/{id}/issues",
    query_struct = "IssueFilter",
    cli_help = "Get a list of a project's issues.",
    cli_long_help = "Get a list of a project's issues.
If the project is private, you need to provide credentials to authorize.
The preferred way to do this, is by using personal access tokens."
  ),
  endpoint(
    route = "/projects/{id}/issues/{iid}",
    cli_route = "/projects/{id}/issues/{iid}",
    cli_help = "Get a single project issue.",
    cli_long_help = "Get a single project issue.
If the project is private or the issue is confidential, you need to provide credentials to authorize.
The preferred way to do this, is by using personal access tokens."
  ),
  endpoint(
    route = "/projects/{id}/issues",
    cli_route = "/projects/{id}/issues/create",
    method = "POST",
    result_ok_status = "CREATED",
    payload_struct = "IssueCreatePayload",
    cli_help = "Creates a new project issue."
  ),
  endpoint(
    route = "/projects/{id}/issues/{iid}",
    cli_route = "/projects/{id}/issues/{iid}/edit",
    method = "PUT",
    payload_struct = "IssueUpdatePayload",
    cli_help = "Updates an existing project issue.",
    cli_long_help = "Updates an existing project issue. This call is also used to mark an issue as
closed.",
  ),
  endpoint(
    route = "/projects/{id}/issues/{iid}",
    cli_route = "/projects/{id}/issues/{iid}/delete",
    method = "DELETE",
    result_ok_status = "NO_CONTENT",
    cli_help = "Deletes an issue.",
    cli_long_help = "Deletes an issue. Only for administrators and project owners.",
  ),
  endpoint(
    route = "/projects/{id}/issues/{iid}/reorder",
    cli_route = "/projects/{id}/issues/{iid}/reorder",
    method = "PUT",
    query_struct = "IssueReorderQuery",
    cli_help = "Reorders an issue.",
    cli_long_help = "Reorders an issue, you can see the results when sorting issues manually",
  ),
  endpoint(
    route = "/projects/{id}/issues/{iid}/move",
    cli_route = "/projects/{id}/issues/{iid}/move",
    method = "POST",
    payload_struct = "IssueMovePayload",
    result_ok_status = "CREATED",
    cli_help = "Moves an issue to a different project.",
    cli_long_help = "Moves an issue to a different project. If the target project is the source project or the user has insufficient permissions, an error message with status code 400 is returned.
If a given label or milestone with the same name also exists in the target project, it's then assigned to the issue being moved.",
  ),
  endpoint(
    route = "/projects/{id}/issues/{iid}/subscribe",
    cli_route = "/projects/{id}/issues/{iid}/subscribe",
    method = "POST",
    result_ok_status = "CREATED",
    result_ko_status(
      status = "NOT_MODIFIED",
      message = "The user is already subscribed to the issue."
    ),
    cli_help = "Subscribes the authenticated user to an issue to receive notifications.",
    cli_long_help = "Subscribes the authenticated user to an issue to receive notifications.
If the user is already subscribed to the issue, the status code 304 is returned.",
  ),
  endpoint(
    route = "/projects/{id}/issues/{iid}/unsubscribe",
    cli_route = "/projects/{id}/issues/{iid}/unsubscribe",
    method = "POST",
    result_ok_status = "CREATED",
    result_ko_status(
      status = "NOT_MODIFIED",
      message = "The user is not subscribed to the issue."
    ),
    cli_help = "Unsubscribes the authenticated user from the issue to not receive notifications from it.",
    cli_long_help = "Unsubscribes the authenticated user from the issue to not receive notifications from it. If the user is not subscribed to the issue, the status code 304 is returned.",
  )
)]
#[api(endpoint(
  route = "/projects/{id}/merge_requests/{iid}/closes_issues",
  multiple_results,
  cli_route = "/projects/{id}/merge_requests/{iid}/closes_issues",
  cli_help = "Get all the issues that would be closed by merging the provided merge request.",
))]
#[derive(PrettyPrint)] // skip_none formatter bool
#[allow(dead_code, non_snake_case)]
pub(crate) struct Issue {
  id: u32,
  state: String,
  project_id: u32,
  iid: u32,
  title: String,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[pretty(skip_none, formatter=markdown_formatter)]
  description: Option<String>,
  #[api(table_format(date(format = "%Y-%m-%d %H:%M:%S")))]
  updated_at: String,
  #[api(table_skip)]
  web_url: String,
  #[api(table_format(date(format = "%Y-%m-%d %H:%M:%S")))]
  created_at: String,
  #[serde(rename = "type")]
  #[api(table_skip)]
  type_: String,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[pretty(skip_none)]
  closed_at: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[pretty(skip_none)]
  moved_to_id: Option<u32>,
  #[api(table_skip)]
  upvotes: u32,
  #[api(table_skip)]
  downvotes: u32,
  #[api(table_skip)]
  merge_requests_count: u32,
  #[api(table_skip)]
  user_notes_count: u32,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[pretty(skip_none)]
  due_date: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[pretty(skip_none, formatter=bool_check_formatter)]
  has_tasks: Option<bool>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[pretty(skip_none)]
  task_status: Option<String>,
  #[api(table_skip)]
  #[pretty(formatter=bool_check_formatter)]
  confidential: bool,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[pretty(skip_none, formatter=bool_check_formatter)]
  discussion_locked: Option<bool>,
  #[api(table_skip)]
  issue_type: String,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[pretty(skip_none)]
  weight: Option<u32>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[pretty(skip_none)]
  health_status: Option<String>,
  labels: Vec<String>,
  #[api(table_skip)]
  #[pretty(is_pretty)]
  assignees: Vec<User>,
  #[api(table_skip)]
  #[pretty(is_pretty)]
  author: User,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[pretty(is_pretty, skip_none)]
  milestone: Option<Milestone>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[pretty(is_pretty, skip_none)]
  assignee: Option<User>, // WARNING: The assignee column is deprecated. We now show it as a single-sized array assignees to conform to the GitLab EE API.
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[pretty(is_pretty, skip_none)]
  closed_by: Option<User>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[pretty(is_pretty, skip_none)]
  references: Option<References>,
  #[api(table_skip)]
  #[pretty(is_pretty)]
  time_stats: TimeStats,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[pretty(is_pretty, skip_none)]
  _links: Option<Links>,
  #[api(table_skip)]
  #[pretty(is_pretty)]
  task_completion_status: TaskCompletionStatus,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  #[pretty(is_pretty, skip_none)]
  epic: Option<Epic>,
}

#[derive(Debug, ApiInput, Serialize, Deserialize)]
#[api(no_input_file)]
pub(crate) struct IssueFilter {
  #[api(
    no_short,
    long = "assignee-id",
    heading = "Filters",
    help = "Return issues assigned to the given user id.",
    long_help = "Return issues assigned to the given user id. Mutually exclusive with `assignee_username`. `None` returns unassigned issues. `Any` returns issues with an assignee."
  )]
  assignee_id: Option<u32>,

  #[api(
    heading = "Filters",
    long = "assignee-username",
    help = "Return issues assigned to the given `username`.",
    long_help = "Return issues assigned to the given `username`. Similar to `assignee_id` and mutually exclusive with `assignee_id`. In GitLab CE, the `assignee_username` array should only contain a single value. Otherwise, an invalid parameter error is returned."
  )]
  assignee_username: Option<String>,

  #[api(
    no_short,
    long = "author-id",
    heading = "Filters",
    help = "Return issues created by the given user `id`.",
    long_help = "Return issues created by the given user `id`. Mutually exclusive with `author_username`. Combine with `scope=all` or `scope=assigned_to_me`."
  )]
  author_id: Option<u32>,

  #[api(
    no_short,
    long = "author-username",
    heading = "Filters",
    help = "Return issues created by the given `username`.",
    long_help = "Return issues created by the given `username`. Similar to `author_id` and mutually exclusive with `author_id`."
  )]
  author_username: Option<String>,

  #[api(
    no_short,
    heading = "Filters",
    help = "Filter confidential or public issues."
  )]
  confidential: Option<bool>,

  #[api(
    no_short,
    long = "created-after",
    heading = "Filters",
    help = "Return issues created on or after the given time.",
    long_help = "Return issues created on or after the given time. Expected in ISO 8601 format (2019-03-15T08:00:00Z)"
  )]
  created_after: Option<String>,

  #[api(
    no_short,
    long = "created-before",
    heading = "Filters",
    help = "Return issues created on or before the given time.",
    long_help = "Return issues created on or before the given time. Expected in ISO 8601 format (2019-03-15T08:00:00Z)"
  )]
  created_before: Option<String>,

  #[api(
    no_short,
    long = "due-date",
    heading = "Filters",
    help = "Return issues that have no due date, are overdue, ...",
    long_help = "Return issues that have no due date, are overdue, or whose due date is this week, this month, or between two weeks ago and next month",
    possible_values = "0,any,today,tomorrow,overdue,week,month,next_month_and_previous_two_weeks"
  )]
  due_date: Option<String>,

  #[api(
    no_short,
    heading = "Filters",
    help = "Return only the issues having the given `iid`"
  )]
  iids: Option<Vec<u32>>,

  #[api(
    no_short,
    long = "in",
    heading = "Filters",
    help = "Modify the scope of the `search` attribute.",
    long_help = "Modify the scope of the `search` attribute. `title`, `description`, or a string joining them with comma. Default is `title,description`"
  )]
  #[serde(rename = "in")]
  in_scope: Option<String>,

  #[api(
    no_short,
    long = "issue-type",
    heading = "Filters",
    help = "Filter to a given type of issue.",
    possible_values = "issue,incident,test_case"
  )]
  issue_type: Option<String>,

  #[api(
    no_short,
    long = "iteration-id",
    heading = "Filters",
    help = "Return issues assigned to the given iteration ID.",
    long_help = "Return issues assigned to the given iteration ID. `None` returns issues that do not belong to an iteration. `Any` returns issues that belong to an iteration. Mutually exclusive with `iteration_title`."
  )]
  iteration_id: Option<String>,

  #[api(
    no_short,
    long = "iteration-title",
    heading = "Filters",
    help = "Return issues assigned to the iteration with the given title.",
    long_help = "Return issues assigned to the iteration with the given title. Similar to `iteration_id` and mutually exclusive with `iteration_id`."
  )]
  iteration_title: Option<String>,

  #[api(
    no_short,
    heading = "Filters",
    help = "Comma-separated list of label names, issues must have all labels to be returned.",
    long_help = "Comma-separated list of label names, issues must have all labels to be returned. `None` lists all issues with no labels. `Any` lists all issues with at least one label. `No+Label` (Deprecated) lists all issues with no labels. Predefined names are case-insensitive."
  )]
  labels: Option<String>,

  #[api(
    no_short,
    heading = "Filters",
    help = "The milestone title.",
    long_help = "The milestone title. `None` lists all issues with no milestone. `Any` lists all issues that have an assigned milestone."
  )]
  milestone: Option<String>,

  #[api(
    no_short,
    long = "milestone-id",
    help = "Returns issues assigned to milestones with a given timebox value (`None`, `Any`, `Upcoming`, and `Started`).",
    long_help = "Returns issues assigned to milestones with a given timebox value (`None`, `Any`, `Upcoming`, and `Started`). `None` lists all issues with no milestone. `Any` lists all issues that have an assigned milestone. `Upcoming` lists all issues assigned to milestones due in the future. `Started` lists all issues assigned to open, started milestones. `milestone` and `milestone_id` are mutually exclusive.",
    possible_values = "None,Any,Upcoming,Started"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  milestone_id: Option<String>,

  #[api(
    no_short,
    long = "my-reaction-emoji",
    heading = "Filters",
    help = "Return issues reacted by the authenticated user by the given emoji.",
    long_help = "Return issues reacted by the authenticated user by the given emoji. `None` returns issues not given a reaction. `Any` returns issues given at least one reaction."
  )]
  my_reaction_emoji: Option<String>,

  #[api(
    no_short,
    long = "non-archived",
    heading = "Filters",
    help = "Return issues only from non-archived projects.",
    long_help = "Return issues only from non-archived projects. If `false`, the response returns issues from both archived and non-archived projects. Default is `true`."
  )]
  non_archived: Option<bool>,

  #[api(
    no_short,
    heading = "Filters",
    help = "Return issues that do not match the parameters supplied.",
    long_help = "Return issues that do not match the parameters supplied. Accepts: `assignee_id`, `assignee_username`, `author_id`, `author_username`, `iids`, `iteration_id`, `iteration_title`, `labels`, `milestone`, and `weight`."
  )]
  not: Option<String>,

  #[api(
    long = "order-by",
    heading = "Sorting",
    help = "Return issues ordered by field.",
    long_help = "Return issues ordered by `created_at`, `updated_at`, `priority`, `due_date`, `relative_position`, `label_priority`, `milestone_due`, `popularity`, `weight` fields. Default is `created_at`",
    possible_values = "created_at,updated_at,priority,due_date,relative_position,label_priority,milestone_due,popularity,weight"
  )]
  order_by: Option<String>,

  #[api(
    no_short,
    heading = "Filters",
    help = "Return issues for the given scope.",
    long_help = "Return issues for the given scope: `created_by_me`, `assigned_to_me` or `all`. Defaults to `created_by_me`",
    possible_values = "created_by_me,assigned_to_me,all"
  )]
  scope: Option<String>,

  #[api(
    heading = "Filters",
    help = "Search issues.",
    long_help = "Search issues against their `title` and `description`."
  )]
  search: Option<String>,

  #[api(
    no_short,
    heading = "Sorting",
    help = "Return issues sorted in `asc` or `desc` order.",
    long_help = "Return issues sorted in `asc` or `desc` order. Default is `desc`",
    possible_values = "asc,desc"
  )]
  sort: Option<String>,

  #[api(
    no_short,
    heading = "Filters",
    help = "Return `all` issues or just those that are `opened` or `closed`",
    possible_values = "all,opened,closed"
  )]
  state: Option<String>,

  #[api(
    no_short,
    long = "updated-after",
    heading = "Filters",
    help = "Return issues updated on or after the given time.",
    long_help = "Return issues updated on or after the given time. Expected in ISO 8601 format (2019-03-15T08:00:00Z)"
  )]
  updated_after: Option<String>,

  #[api(
    no_short,
    long = "updated-before",
    heading = "Filters",
    help = "Return issues updated on or before the given time.",
    long_help = "Return issues updated on or before the given time. Expected in ISO 8601 format (2019-03-15T08:00:00Z)"
  )]
  updated_before: Option<String>,

  #[api(
    no_short,
    heading = "Filters",
    help = "Return issues with the specified `weight`.",
    long_help = "Return issues with the specified `weight`. `None` returns issues with no weight assigned. `Any` returns issues with a weight assigned."
  )]
  weight: Option<String>,

  #[api(
    no_short,
    long = "with-labels-details",
    heading = "Filters",
    help = "If true, the response returns more details for each label in labels field",
    long_help = "If true, the response returns more details for each label in labels field: `:name`, `:color`, `:description`, `:description_html`, `:text_color`. Default is `false`."
  )]
  with_labels_details: Option<bool>,

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

#[derive(Serialize, Deserialize, Debug, Default, Clone, PrettyPrint)] // skip_none
pub(crate) struct Links {
  #[serde(rename = "self")]
  self_: String,
  notes: String,
  award_emoji: String,
  project: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PrettyPrint)]
pub(crate) struct Epic {
  id: u32,
  iid: u32,
  title: String,
  url: String,
  group_id: u32,
}
