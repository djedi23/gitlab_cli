use crud_api::{Api, ApiInput};
use serde::{Deserialize, Serialize};

#[derive(Api, Serialize, Deserialize, Debug, Default, Clone)]
#[api(
  endpoint(
    route = "/user/preferences",
    cli_route = "/user/preferences",
    cli_help = "Get a list of currently authenticated user's preferences.",
  ),
  endpoint(
    route = "/user/preferences",
    method = "PUT",
    cli_route = "/user/preferences/edit",
    payload_struct = "UserPreferencesPayload",
    cli_help = "Update the current user's preferences.",
  )
)]
pub(crate) struct Preferences {
  id: u32,
  user_id: u32,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  view_diffs_file_by_file: Option<bool>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  show_whitespace_in_diffs: Option<bool>,
}

#[derive(Debug, ApiInput, Serialize, Deserialize)]
#[api(no_input_file)]
pub(crate) struct UserPreferencesPayload {
  #[api(
    no_short,
    long = "view-diffs-file-by-file",
    help = "Flag indicating the user sees only one file diff per page."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  view_diffs_file_by_file: Option<bool>,

  #[api(
    no_short,
    long = "show-whitespace-in-diffs",
    help = "Flag indicating the user sees whitespace changes in diffs."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  show_whitespace_in_diffs: Option<bool>,
}
