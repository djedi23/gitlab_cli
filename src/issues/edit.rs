use crud_api::ApiInput;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, ApiInput, Serialize, Deserialize)]
pub(crate) struct IssueUpdatePayload {
  #[api(
    no_short,
    long = "add-labels",
    help = "Comma-separated label names to add to an issue."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  add_labels: Option<String>,

  #[api(
    no_short,
    long = "assignee-ids",
    help = "The ID of the user(s) to assign the issue to.",
    long_help = "The ID of the user(s) to assign the issue to. Set to 0 or provide an empty value to unassign all assignees."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  assignee_ids: Option<Vec<u32>>,

  #[api(no_short, help = "Updates an issue to be confidential")]
  #[serde(skip_serializing_if = "Option::is_none")]
  confidential: Option<bool>,

  // #[api(
  //   no_short,
  //   long = "created-at",
  //   help = "When the issue was created.",
  //   long_help = "When the issue was created. Date time string, ISO 8601 formatted, for example `2016-03-11T03:45:40Z`. Requires administrator or project/group owner rights."
  // )]
  // created_at: Option<String>,
  #[api(
    no_short,
    help = "The description of an issue.",
    long_help = "The description of an issue. Limited to 1,048,576 characters."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  description: Option<String>,

  #[api(
    no_short,
    long = "discussion-locked",
    help = "Flag indicating if the issue's discussion is locked.",
    long_help = "Flag indicating if the issue's discussion is locked. If the discussion is locked only project members can add or edit comments."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  discussion_locked: Option<bool>,

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
    long = "issue-type",
    help = "Updates the type of issue.",
    possible_values = "issue,incident,test_case"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  issue_type: Option<String>,

  #[api(
    no_short,
    help = "Comma-separated label names for an issue.",
    long_help = "Comma-separated label names for an issue. Set to an empty string to unassign all labels."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  labels: Option<String>,

  #[api(
    no_short,
    long = "milestone-id",
    help = "The global ID of a milestone to assign issue.",
    long_help = "The global ID of a milestone to assign issue. Set to an empty string to unassign all labels."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  milestone_id: Option<u32>,

  #[api(
    no_short,
    long = "remove-labels",
    help = "Comma-separated label names to remove from an issue."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  remove_labels: Option<String>,

  #[api(
    no_short,
    long = "state-event",
    help = "The state event of an issue.",
    long_help = "The state event of an issue. Set `close` to close the issue and `reopen` to reopen it",
    possible_values = "close,reopen"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  state_event: Option<String>,

  #[api(no_short, help = "The title of an issue")]
  #[serde(skip_serializing_if = "Option::is_none")]
  title: Option<String>,

  #[api(
    no_short,
    long = "updated-at",
    help = "When the issue was updated.",
    long_help = "When the issue was updated. Date time string, ISO 8601 formatted, for example 2016-03-11T03:45:40Z (requires administrator or project owner rights). Empty string or null values are not accepted."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  updated_at: Option<String>,

  #[api(
    no_short,
    help = "The weight of the issue.",
    long_help = "The weight of the issue. Valid values are greater than or equal to 0."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  weight: Option<u32>,
}
