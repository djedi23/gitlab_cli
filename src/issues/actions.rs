use crud_api::ApiInput;
use serde::{Deserialize, Serialize};

#[derive(Debug, ApiInput, Serialize, Deserialize)]
#[api(no_input_file)]
pub(crate) struct IssueReorderQuery {
  #[api(
    no_short,
    long = "move-after-id",
    help = "The ID of a project's issue that should be placed after this issue"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  move_after_id: Option<u32>,

  #[api(
    no_short,
    long = "move-before-id",
    help = "The ID of a project's issue that should be placed before this issue"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  move_before_id: Option<u32>,
}

#[derive(Debug, ApiInput, Serialize, Deserialize)]
#[api(no_input_file)]
pub(crate) struct IssueMovePayload {
  #[api(
    short = 'p',
    long = "to-project-id",
    help = "The ID of the new project"
  )]
  to_project_id: u32,
}
