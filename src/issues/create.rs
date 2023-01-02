use crud_api::ApiInput;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, ApiInput, Serialize, Deserialize)]
pub(crate) struct IssueCreatePayload {
  #[api(
    no_short,
    long = "assignee-id",
    help = "The ID of the user to assign the issue to.",
    long_help = "The ID of the user to assign the issue to. Only appears on GitLab Free."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  assignee_id: Option<u32>,

  #[api(
    no_short,
    long = "assignee-ids",
    help = "The IDs of the users to assign the issue to."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  assignee_ids: Option<Vec<u32>>,

  #[api(
    no_short,
    help = "Set an issue to be confidential.",
    long_help = "Set an issue to be confidential. Default is `false`."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  confidential: Option<bool>,

  #[api(
    no_short,
    long = "created-at",
    help = "When the issue was created.",
    long_help = "When the issue was created. Date time string, ISO 8601 formatted, for example `2016-03-11T03:45:40Z`. Requires administrator or project/group owner rights."
  )]
  created_at: Option<String>,

  #[api(
    no_short,
    help = "The description of an issue.",
    long_help = "The description of an issue. Limited to 1,048,576 characters."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  description: Option<String>,

  #[api(
    no_short,
    long = "discussion-to-resolve",
    help = "The ID of a discussion to resolve.",
    long_help = "The ID of a discussion to resolve. This fills out the issue with a default description and mark the discussion as resolved. Use in combination with `merge_request_to_resolve_discussions_of`."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  discussion_to_resolve: Option<String>,

  #[api(
    no_short,
    long = "due-date",
    help = "The due date.",
    long_help = "The due date. Date time string in the format `YYYY-MM-DD`, for example `2016-03-11`"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  due_date: Option<String>,

  #[api(
    no_short,
    long = "epic-id",
    help = "ID of the epic to add the issue to.",
    long_help = "ID of the epic to add the issue to. Valid values are greater than or equal to 0."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  epic_id: Option<u32>,

  #[api(
    no_short,
    help = "The internal ID of the project's issue.",
    long_help = "The internal ID of the project's issue (requires administrator or project owner rights)"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  iid: Option<String>,

  #[api(
    no_short,
    long = "issue-type",
    help = "The type of issue.",
    long_help = "The type of issue. Default is `issue`.",
    possible_values = "issue,incident,test_case"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  issue_type: Option<String>,

  #[api(no_short, help = "Comma-separated label names for an issue.")]
  #[serde(skip_serializing_if = "Option::is_none")]
  labels: Option<String>,

  #[api(
    no_short,
    long = "merge-request-to-resolve-discussions-of",
    help = "The IID of a merge request in which to resolve all issues.",
    long_help = "The IID of a merge request in which to resolve all issues. This fills out the issue with a default description and mark all discussions as resolved. When passing a description or title, these values take precedence over the default values."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  merge_request_to_resolve_discussions_of: Option<u32>,

  #[api(
    no_short,
    long = "milestone-id",
    help = "The global ID of a milestone to assign issue.",
    long_help = "The global ID of a milestone to assign issue. To find the milestone_id associated with a milestone, view an issue with the milestone assigned and use the API to retrieve the issue's details."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  milestone_id: Option<u32>,

  #[api(no_short, help = "The title of an issue")]
  title: String,

  #[api(
    no_short,
    help = "The weight of the issue.",
    long_help = "The weight of the issue. Valid values are greater than or equal to 0."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  weight: Option<u32>,
}
