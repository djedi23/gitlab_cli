use crate::commit::Commit;
use crud_api::Api;
use serde::{Deserialize, Serialize};

#[derive(Api, Clone, Default, Debug, Serialize, Deserialize)]
#[api(endpoint(
  route = "/projects/{id}/repository/files/{file_path}/blame",
  multiple_results,
  cli_route = "/projects/{id}/repository/files/{file_path}/blame",
  query_struct = "FilePayload",
  cli_long_help = "Allows you to receive blame information. Each blame range contains lines and corresponding commit information.",
  cli_help = "Allows you to receive blame information.",
  cli_force_output_format	// FIXME: maybe to be removed
))]
pub(crate) struct Blame {
  commit: Commit,
  lines: Vec<String>,
}

// #[derive(Debug, ApiInput, Serialize, Deserialize)]
// #[api(no_input_file)]
// pub(crate) struct FilePayload {
//   #[api(no_short, long = "ref", help = "The name of branch, tag or commit")]
//   #[serde(rename = "ref")]
//   _ref: String,
// }
