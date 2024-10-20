use crate::{
  diff::Change, milestones::Milestone, pipeline::Pipeline, reference::References,
  task::TaskCompletionStatus, time::TimeStats, user::User,
};
use crud_api::{Api, ApiInput};
use serde::{Deserialize, Serialize};

#[derive(Api, Serialize, Deserialize, Debug, Default, Clone)]
#[api(
  endpoint(
    route = "/merge_requests",
    multiple_results,
    query_struct = "MergeRequestFilter",
    cli_route = "/merge_requests",
    cli_help = "Get all merge requests the authenticated user has access to.",
    cli_long_help = "Get all merge requests the authenticated user has access to. By default it returns only merge requests created by the current user. To get all merge requests, use parameter `scope=all`."
  ),
  endpoint(
    route = "/projects/{id}/merge_requests",
    multiple_results,
    query_struct = "MergeRequestFilter",
    cli_route = "/projects/{id}/merge_requests",
    cli_help = "Get all merge requests for this project.",
  ),
  endpoint(
    route = "/projects/{id}/merge_requests",
    method = "POST",
    result_ok_status = "CREATED",
    payload_struct = "MergeRequestCreate",
    cli_route = "/projects/{id}/merge_requests/create",
    cli_help = "Creates a new merge request.",
  ),
  endpoint(
    route = "/projects/{id}/merge_requests/{iid}",
    method = "PUT",
    payload_struct = "MergeRequestUpdate",
    cli_route = "/projects/{id}/merge_requests/{iid}/edit",
    cli_help = "Updates an existing merge request.",
    cli_long_help = "Updates an existing merge request. You can change the target branch, title, or even close the MR.",
  ),
  endpoint(
    route = "/projects/{id}/merge_requests/{iid}",
    method = "DELETE",
    cli_route = "/projects/{id}/merge_requests//{iid}/delete",
    cli_help = "Deletes the merge request in question.",
    cli_long_help = "Only for administrators and project owners. Deletes the merge request in question.",
  ),
  endpoint(
    route = "/projects/{id}/merge_requests/{iid}",
    query_struct = "MergeRequestSelector",
    cli_route = "/projects/{id}/merge_requests/{iid}",
    cli_help = "Shows information about a single merge request.",
  ),
  endpoint(
    route = "/projects/{id}/merge_requests/{iid}/changes",
    query_struct = "MergeRequestChangeSelector",
    cli_route = "/projects/{id}/merge_requests/{iid}/changes",
    cli_help = "Shows information about the merge request including its files and changes.",
  ),
  endpoint(
    route = "/projects/{id}/merge_requests/{iid}/merge",
    method = "PUT",
    payload_struct = "MergeRequestMerge",
    cli_route = "/projects/{id}/merge_requests//{iid}/merge",
    cli_help = "Merge changes submitted with MR using this API.",
    cli_long_help = "Merge changes submitted with MR using this API.
If a merge request is unable to be accepted (such as Draft, Closed, Pipeline Pending Completion, or Failed while requiring Success) - you receive a 405 and the error message 'Method Not Allowed'
If it has some conflicts and can not be merged - you receive a 406 and the error message 'Branch cannot be merged'
If the sha parameter is passed and does not match the HEAD of the source - you receive a 409 and the error message 'SHA does not match HEAD of source branch'
If you don't have permissions to accept this merge request - you receive a 401",
  ),
  endpoint(
    route = "/projects/{id}/merge_requests/{iid}/cancel_merge_when_pipeline_succeeds",
    method = "POST",
    cli_route = "/projects/{id}/merge_requests//{iid}/cancel_merge_when_pipeline_succeeds",
    cli_help = "Cancel Merge When Pipeline Succeeds",
  ),
  endpoint(
    route = "/projects/{id}/merge_requests/{iid}/subscribe",
    method = "POST",
    result_ok_status = "CREATED",
    result_ko_status(status = "NOT_MODIFIED", message = "Already subscribed"),
    cli_route = "/projects/{id}/merge_requests//{iid}/subscribe",
    cli_help = "Subscribes the authenticated user to a merge request to receive notification.",
  ),
  endpoint(
    route = "/projects/{id}/merge_requests/{iid}/unsubscribe",
    method = "POST",
    result_ok_status = "CREATED",
    result_ko_status(status = "NOT_MODIFIED", message = "already unsubscribed"),
    cli_route = "/projects/{id}/merge_requests//{iid}/unsubscribe",
    cli_help = "Unsubscribes the authenticated user from a merge request to not receive notifications from that merge request.",
  ),
  endpoint(
    route = "/groups/{id}/merge_requests",
    multiple_results,
    query_struct = "MergeRequestFilter",
    cli_route = "/groups/{id}/merge_requests",
    cli_help = "Get all merge requests for this group and its subgroups.",
  )
)]
#[api(endpoint(
  route = "/groups/{id}/merge_requests",
  multiple_results,
  query_struct = "MergeRequestFilter",
  cli_route = "/groups/{id}/merge_requests",
  cli_help = "Get all merge requests for this group and its subgroups.",
))]
pub(crate) struct MergeRequest {
  id: u32,
  iid: u32,
  project_id: u32,
  #[serde(skip_serializing_if = "Option::is_none")]
  title: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  description: Option<String>,
  state: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  merged_at: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[api(table_skip)]
  closed_at: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  created_at: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  updated_at: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  target_branch: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  source_branch: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  upvotes: Option<u32>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  downvotes: Option<u32>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  source_project_id: Option<u32>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  target_project_id: Option<u32>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  draft: Option<bool>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  work_in_progress: Option<bool>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  merge_when_pipeline_succeeds: Option<bool>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  merge_status: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  sha: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  merge_commit_sha: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  squash_commit_sha: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  user_notes_count: Option<u32>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  discussion_locked: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  should_remove_source_branch: Option<bool>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  force_remove_source_branch: Option<bool>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  allow_collaboration: Option<bool>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  allow_maintainer_to_push: Option<bool>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  reference: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  has_conflicts: Option<bool>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  blocking_discussions_resolved: Option<bool>,
  #[api(table_skip)]
  web_url: String,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  squash: Option<bool>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  rebase_in_progress: Option<bool>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  diverged_commits_count: Option<u32>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  overflow: Option<bool>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  changes_count: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  latest_build_started_at: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  latest_build_finished_at: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  first_deployed_to_production_at: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  merge_error: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  first_contribution: Option<bool>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  title_html: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  description_html: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  labels: Option<Vec<String>>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  user: Option<UserCanMerge>,
  #[serde(skip_serializing_if = "Option::is_none")]
  closed_by: Option<User>,
  #[serde(skip_serializing_if = "Option::is_none")]
  merged_by: Option<User>,
  #[serde(skip_serializing_if = "Option::is_none")]
  merge_user: Option<User>,
  #[serde(skip_serializing_if = "Option::is_none")]
  author: Option<User>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  assignee: Option<User>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  milestone: Option<Milestone>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  pipeline: Option<Pipeline>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  head_pipeline: Option<Pipeline>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  references: Option<References>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  time_stats: Option<TimeStats>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  task_completion_status: Option<TaskCompletionStatus>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  diff_refs: Option<DiffRefs>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  assignees: Option<Vec<User>>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  changes: Option<Vec<Change>>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  reviewers: Vec<User>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct DiffRefs {
  base_sha: String,
  head_sha: String,
  start_sha: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct UserCanMerge {
  can_merge: bool,
}

#[derive(Debug, ApiInput, Serialize, Deserialize)]
#[api(no_input_file)]
pub(crate) struct MergeRequestFilter {
  #[api(
    no_short,
    heading = "Filters",
    help = "Return the request having the given `iid`."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  iids: Option<Vec<u32>>,

  #[api(
    no_short,
    heading = "Filters",
    help = "Return all merge requests that are `opened`, `closed`, `locked`, or `merged`.",
    long_help = "Return all merge requests or just those that are `opened`, `closed`, `locked`, or `merged`.",
    possible_values = "opened,closed,locked,merged"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  state: Option<String>,

  #[api(
    no_short,
    heading = "Filters",
    help = "Return requests sorted in `asc` or `desc` order.",
    long_help = "Return requests sorted in `asc` or `desc` order. Default is `desc`.",
    possible_values = "asc,desc"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  sort: Option<String>,

  #[api(
    no_short,
    heading = "Filters",
    help = "Return merge requests for a specific milestone.",
    long_help = "Return merge requests for a specific milestone. `None` returns merge requests with no milestone. `Any` returns merge requests that have an assigned milestone."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  milestone: Option<String>,

  #[api(
    no_short,
    heading = "Filters",
    help = "If `simple`, returns the `iid`, URL, title, description, and basic state of merge request.",
    possible_values = "simple"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  view: Option<String>,

  #[api(
    no_short,
    heading = "Filters",
    help = "Return merge requests matching a comma separated list of labels.",
    long_help = "Return merge requests matching a comma separated list of labels. `None` lists all merge requests with no labels. `Any` lists all merge requests with at least one label. "
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  labels: Option<String>,

  #[api(
    no_short,
    heading = "Filters",
    long = "with-labels-details",
    help = "If `true`, response returns more details for each label in labels field: `:name`, `:color`, `:description`, :`description_html`, `:text_color`.",
    long_help = "If `true`, response returns more details for each label in labels field: `:name`, `:color`, `:description`, :`description_html`, `:text_color`. Default is `false`."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  with_labels_details: Option<bool>,

  #[api(
    no_short,
    heading = "Filters",
    long = "with-merge-status-recheck",
    help = "If `true`, this projection requests (but does not guarantee) that the `merge_status` field be recalculated asynchronously.",
    long_help = "If `true`, this projection requests (but does not guarantee) that the `merge_status` field be recalculated asynchronously. Default is `false`."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  with_merge_status_recheck: Option<bool>,

  #[api(
    no_short,
    heading = "Filters",
    long = "created-after",
    help = "Return merge requests created on or after the given time.",
    long_help = "Return merge requests created on or after the given time. Expected in ISO 8601 format (2019-03-15T08:00:00Z)"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  created_after: Option<String>,

  #[api(
    no_short,
    heading = "Filters",
    long = "created-before",
    help = "Return merge requests created on or before the given time.",
    long_help = "Return merge requests created on or before the given time. Expected in ISO 8601 format (2019-03-15T08:00:00Z)"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  created_before: Option<String>,

  #[api(
    no_short,
    heading = "Filters",
    long = "updated-after",
    help = "Return merge requests updated on or after the given time.",
    long_help = "Return merge requests updated on or after the given time. Expected in ISO 8601 format (2019-03-15T08:00:00Z)"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  updated_after: Option<String>,

  #[api(
    no_short,
    heading = "Filters",
    long = "updated-before",
    help = "Return merge requests updated on or before the given time.",
    long_help = "Return merge requests updated on or before the given time. Expected in ISO 8601 format (2019-03-15T08:00:00Z)"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  updated_before: Option<String>,

  #[api(
    no_short,
    heading = "Filters",
    help = "Return merge requests for the given scope: `created_by_me`, `assigned_to_me` or `all`.",
    long_help = "Return merge requests for the given scope: `created_by_me`, `assigned_to_me` or `all`. Defaults to `created_by_me`",
    possible_values = "created_by_me,assigned_to_me,all"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  scope: Option<String>,

  #[api(
    no_short,
    heading = "Filters",
    long = "author-id",
    help = "Returns merge requests created by the given user `id`.",
    long_help = "Returns merge requests created by the given user `id`. Mutually exclusive with `author_username`. Combine with `scope=all` or `scope=assigned_to_me`."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  author_id: Option<u32>,

  #[api(
    no_short,
    heading = "Filters",
    long = "author-username",
    help = "Returns merge requests created by the given username.",
    long_help = "Returns merge requests created by the given username. Mutually exclusive with `author_id`."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  author_username: Option<String>,

  #[api(
    no_short,
    heading = "Filters",
    long = "assignee-id",
    help = "Returns merge requests assigned to the given user `id`.",
    long_help = "Returns merge requests assigned to the given user `id`. `None` returns unassigned merge requests. `Any` returns merge requests with an assignee."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  assignee_id: Option<String>,

  #[api(
    no_short,
    heading = "Filters",
    long = "approver-ids",
    help = "Returns merge requests which have specified all the users with the given `ids` as individual approvers.",
    long_help = "Returns merge requests which have specified all the users with the given `ids` as individual approvers. `None` returns merge requests without approvers. `Any` returns merge requests with an approver."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  approver_ids: Option<Vec<String>>,

  #[api(
    no_short,
    heading = "Filters",
    long = "approved-by-ids",
    help = "Returns merge requests which have been approved by all the users with the given `id`s (Max: 5).",
    long_help = "Returns merge requests which have been approved by all the users with the given `id`s (Max: 5). `None` returns merge requests with no approvals. `Any` returns merge requests with an approval."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  approved_by_ids: Option<Vec<String>>,

  #[api(
    no_short,
    heading = "Filters",
    long = "reviewer-id",
    help = "Returns merge requests which have the user as a reviewer with the given user `id`.",
    long_help = "Returns merge requests which have the user as a reviewer with the given user `id`. `None` returns merge requests with no reviewers. `Any` returns merge requests with any reviewer. Mutually exclusive with `reviewer_username`."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  reviewer_id: Option<String>,

  #[api(
    no_short,
    heading = "Filters",
    long = "reviewer-username",
    help = "Returns merge requests which have the user as a reviewer with the given `username`.",
    long_help = "Returns merge requests which have the user as a reviewer with the given `username`. `None` returns merge requests with no reviewers. `Any` returns merge requests with any reviewer. Mutually exclusive with `reviewer_id`."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  reviewer_username: Option<String>,

  #[api(
    no_short,
    heading = "Filters",
    long = "my-reaction-emoji",
    help = "Return merge requests reacted by the authenticated user by the given `emoji`.",
    long_help = "Return merge requests reacted by the authenticated user by the given `emoji`. `None` returns issues not given a reaction. `Any` returns issues given at least one reaction."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  my_reaction_emoji: Option<String>,

  #[api(
    no_short,
    heading = "Filters",
    long = "source-branch",
    help = "Return merge requests with the given source branch."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  source_branch: Option<String>,

  #[api(
    no_short,
    heading = "Filters",
    long = "target-branch",
    help = "Return merge requests with the given target branch."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  target_branch: Option<String>,

  #[api(
    no_short,
    heading = "Filters",
    help = "Search merge requests against their `title` and `description`."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  search: Option<String>,

  #[api(
    no_short,
    heading = "Filters",
    help = "Modify the scope of the search attribute.",
    long_help = "Modify the scope of the search attribute. `title`, `description`, or a string joining them with comma. Default is `title,description`."
  )]
  #[serde(rename = "in", skip_serializing_if = "Option::is_none")]
  _in: Option<String>,

  #[api(
    no_short,
    heading = "Filters",
    help = "Filter merge requests against their wip status.",
    long_help = "Filter merge requests against their wip status. `yes` to return only draft merge requests, `no` to return non-draft merge requests."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  wip: Option<String>,

  #[api(
    no_short,
    heading = "Filters",
    long = "non-archived",
    help = "Return merge requests from non archived projects only.",
    long_help = "Return merge requests from non archived projects only. Default is true."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  non_archived: Option<bool>,

  #[api(
    no_short,
    heading = "Filters",
    help = "Return merge requests that do not match the parameters supplied.",
    long_help = "Return merge requests that do not match the parameters supplied. Accepts: `labels`, `milestone`, `author_id`, `author_username`, `assignee_id`, `assignee_username`, `reviewer_id`, `reviewer_username`, `my_reaction_emoji`.",
    possible_values = "labels,milestone,author_id,author_username,assignee_id,assignee_username,reviewer_id,reviewer_username,my_reaction_emoji"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  not: Option<String>,

  #[api(
    no_short,
    heading = "Filters",
    help = "Returns merge requests deployed to the given environment.",
    long_help = "Returns merge requests deployed to the given environment. Expected in ISO 8601 format (2019-03-15T08:00:00Z)"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  environment: Option<String>,

  #[api(
    no_short,
    heading = "Filters",
    long = "deployed-before",
    help = "Return merge requests deployed before the given date/time.",
    long_help = "Return merge requests deployed before the given date/time. Expected in ISO 8601 format (2019-03-15T08:00:00Z)"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  deployed_before: Option<String>,

  #[api(
    no_short,
    heading = "Filters",
    long = "deployed-after",
    help = "Return merge requests deployed after the given date/time.",
    long_help = "Return merge requests deployed after the given date/time. Expected in ISO 8601 format (2019-03-15T08:00:00Z)"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  deployed_after: Option<String>,

  #[api(
    no_short,
    heading = "Sorting",
    long = "order-by",
    help = "Return requests ordered by created_at, title, or updated_at fields.",
    possible_values = "created_at,title,updated_at"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  order_by: Option<String>,

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
pub(crate) struct MergeRequestSelector {
  #[api(
    no_short,
    heading = "Selectors",
    long = "render-html",
    help = "If true response includes rendered HTML for title and description."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  render_html: Option<bool>,

  #[api(
    no_short,
    heading = "Selectors",
    long = "include-diverged-commits-count",
    help = "If true response includes the commits behind the target branch."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  include_diverged_commits_count: Option<bool>,

  #[api(
    no_short,
    heading = "Selectors",
    long = "include-rebase-in-progress",
    help = "If true response includes whether a rebase operation is in progress."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  include_rebase_in_progress: Option<bool>,
}

#[derive(Debug, ApiInput, Serialize, Deserialize)]
#[api(no_input_file)]
pub(crate) struct MergeRequestChangeSelector {
  #[api(
    no_short,
    heading = "Selectors",
    long = "access-raw-diffs",
    help = "Retrieve change diffs via Gitaly."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  access_raw_diffs: Option<bool>,
}

#[derive(Debug, Default, ApiInput, Serialize, Deserialize)]
pub(crate) struct MergeRequestCreate {
  #[api(no_short, long = "source-branch", help = "The source branch.")]
  source_branch: String,

  #[api(no_short, long = "target-branch", help = "The target branch.")]
  target_branch: String,

  #[api(no_short, help = "Title of MR.")]
  title: String,

  #[api(no_short, long = "assignee-id", help = "Assignee user ID.")]
  #[serde(skip_serializing_if = "Option::is_none")]
  assignee_id: Option<u32>,

  #[api(
    no_short,
    long = "assignee-ids",
    help = "The ID of the user(s) to assign the MR to.",
    long_help = "The ID of the user(s) to assign the MR to. Set to 0 or provide an empty value to unassign all assignees."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  assignee_ids: Option<Vec<u32>>,

  #[api(
    no_short,
    long = "reviewer-ids",
    help = "The ID of the user(s) added as a reviewer to the MR.",
    long_help = "The ID of the user(s) added as a reviewer to the MR. If set to 0 or left empty, no reviewers are added."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  reviewer_ids: Option<Vec<u32>>,

  #[api(
    no_short,
    help = "Description of MR.",
    long_help = "Description of MR. Limited to 1,048,576 characters."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  description: Option<String>,

  #[api(
    no_short,
    long = "target-project-id",
    help = "The target project (numeric ID)."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  target_project_id: Option<u32>,

  #[api(no_short, help = "Labels for MR as a comma-separated list.")]
  #[serde(skip_serializing_if = "Option::is_none")]
  labels: Option<String>,

  #[api(
    no_short,
    long = "milestone-id",
    help = "The global ID of a milestone."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  milestone_id: Option<u32>,

  #[api(
    no_short,
    long = "remove-source-branch",
    help = "Flag indicating if a merge request should remove the source branch when merging."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  remove_source_branch: Option<bool>,

  #[api(
    no_short,
    long = "allow-collaboration",
    help = "Allow commits from members who can merge to the target branch."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  allow_collaboration: Option<bool>,

  #[api(no_short, help = "Squash commits into a single commit when merging.")]
  #[serde(skip_serializing_if = "Option::is_none")]
  squash: Option<bool>,
}

#[derive(Debug, Default, ApiInput, Serialize, Deserialize)]
pub(crate) struct MergeRequestUpdate {
  #[api(no_short, long = "target-branch", help = "The target branch.")]
  #[serde(skip_serializing_if = "Option::is_none")]
  target_branch: Option<String>,

  #[api(no_short, help = "Title of MR.")]
  #[serde(skip_serializing_if = "Option::is_none")]
  title: Option<String>,

  #[api(
    no_short,
    long = "assignee-id",
    help = "The ID of the user to assign the merge request to.",
    long_help = "The ID of the user to assign the merge request to. Set to 0 or provide an empty value to unassign all assignees."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  assignee_id: Option<u32>,

  #[api(
    no_short,
    long = "assignee-ids",
    help = "The ID of the user(s) to assign the MR to.",
    long_help = "The ID of the user(s) to assign the MR to. Set to 0 or provide an empty value to unassign all assignees."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  assignee_ids: Option<Vec<u32>>,

  #[api(
    no_short,
    long = "reviewer-ids",
    help = "The ID of the user(s) set as a reviewer to the MR.",
    long_help = "The ID of the user(s) set as a reviewer to the MR. Set the value to 0 or provide an empty value to unset all reviewers."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  reviewer_ids: Option<Vec<u32>>,

  #[api(
    no_short,
    long = "milestone-id",
    help = "The global ID of a milestone to assign the merge request to.",
    long_help = "The global ID of a milestone to assign the merge request to. Set to 0 or provide an empty value to unassign a milestone."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  milestone_id: Option<u32>,

  #[api(
    no_short,
    help = "Comma-separated label names for a merge request.",
    long_help = "Comma-separated label names for a merge request. Set to an empty string to unassign all labels."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  labels: Option<String>,

  #[api(
    no_short,
    long = "add-labels",
    help = "Comma-separated label names to add to a merge request."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  add_labels: Option<String>,

  #[api(
    no_short,
    long = "remove-labels",
    help = "Comma-separated label names to remove from a merge request."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  remove_labels: Option<String>,

  #[api(
    no_short,
    help = "Description of MR.",
    long_help = "Description of MR. Limited to 1,048,576 characters."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  description: Option<String>,

  #[api(no_short, long = "state-event", help = "New state (close/reopen).")]
  #[serde(skip_serializing_if = "Option::is_none")]
  state_event: Option<String>,

  #[api(
    no_short,
    long = "remove-source-branch",
    help = "Flag indicating if a merge request should remove the source branch when merging."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  remove_source_branch: Option<bool>,

  #[api(no_short, help = "Squash commits into a single commit when merging.")]
  #[serde(skip_serializing_if = "Option::is_none")]
  squash: Option<bool>,

  #[api(
    no_short,
    long = "discussion-locked",
    help = "Flag indicating if the merge request's discussion is locked.",
    long_help = "Flag indicating if the merge request's discussion is locked. If the discussion is locked only project members can add, edit or resolve comments."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  discussion_locked: Option<bool>,

  #[api(
    no_short,
    long = "allow-collaboration",
    help = "Allow commits from members who can merge to the target branch."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  allow_collaboration: Option<bool>,
}

#[derive(Debug, Default, ApiInput, Serialize, Deserialize)]
pub(crate) struct MergeRequestMerge {
  #[api(
    no_short,
    long = "merge-commit-message",
    help = "Custom merge commit message."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  merge_commit_message: Option<String>,

  #[api(
    no_short,
    long = "squash-commit-message",
    help = "Custom squash commit message."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  squash_commit_message: Option<String>,

  #[api(
    no_short,
    help = "If `true` the commits are squashed into a single commit on merge."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  squash: Option<bool>,

  #[api(
    no_short,
    long = "should-remove-source-branch",
    help = "If `true` removes the source branch."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  should_remove_source_branch: Option<bool>,

  #[api(
    no_short,
    long = "merge-when-pipeline-succeeds",
    help = "If `true` the MR is merged when the pipeline succeeds."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  merge_when_pipeline_succeeds: Option<bool>,

  #[api(
    no_short,
    long = "sha",
    help = "If present, then this SHA must match the HEAD of the source branch, otherwise the merge fails."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  sha: Option<String>,
}

#[derive(Api, Serialize, Deserialize, Debug, Default)]
#[api(endpoint(
  route = "/projects/{id}/merge_requests/{iid}/merge_ref",
  cli_route = "/projects/{id}/merge_requests/{iid}//merge_ref",
  cli_help = "Merge the changes between the merge request source and target branches into refs/merge-requests/:iid/merge ref, of the target project repository, if possible.",
))]
pub(crate) struct CommidId {
  commit_id: String,
}
