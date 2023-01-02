pub(crate) mod blame;

use crud_api::{Api, ApiInput};
use serde::{Deserialize, Serialize};

#[derive(Api, Clone, Default, Debug, Serialize, Deserialize)]
#[api(endpoint(
  route = "/projects/{id}/repository/files/{file_path}",
  cli_route = "/projects/{id}/repository/files/{file_path}",
  query_struct = "FilePayload",
  cli_long_help = "Allows you to receive information about file in repository like name, size, content. File content is Base64 encoded.",
  cli_help = "Allows you to receive information about file in repository",
  cli_force_output_format	// FIXME: maybe to be removed
),
endpoint(
    route = "/projects/{id}/repository/files/{file_path}/raw",
    cli_route = "/projects/{id}/repository/files/{file_path}/raw",
    query_struct = "FilePayload",
    stream,
    cli_help = "Get raw file from repository",
    cli_force_output_format	// FIXME: maybe to be removed
),
endpoint(
    route = "/projects/{id}/repository/files/{file_path}",
    method="POST",
    result_ok_status="CREATED",
    cli_route = "/projects/{id}/repository/files/{file_path}/create",
    payload_struct = "FileCreatePayload",
    cli_help = "This allows you to create a single file.",
    cli_long_help = "This allows you to create a single file. For creating multiple files with a single request see the commits API.",
),
endpoint(
    route = "/projects/{id}/repository/files/{file_path}",
    method="PUT",
    cli_route = "/projects/{id}/repository/files/{file_path}/edit",
    payload_struct = "FileCreatePayload",
    cli_help = "This allows you to update a single file.",
    cli_long_help = "This allows you to update a single file. For updating multiple files with a single request see the commits API.",
),
endpoint(
    route = "/projects/{id}/repository/files/{file_path}",
    method="DELETE",
    result_ok_status="NO_CONTENT",
    cli_no_output,
    result_struct = "EmptyResponse",
    cli_route = "/projects/{id}/repository/files/{file_path}/delete",
    payload_struct = "FileDeletePayload",
    cli_help = "This allows you to delete a single file.",
    cli_long_help = "This allows you to delete a single file. For deleting multiple files with a single request, see the commits API.",
)
)]
pub(crate) struct File {
  #[serde(skip_serializing_if = "Option::is_none")]
  file_name: Option<String>,
  file_path: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  size: Option<u32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  encoding: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  content: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  content_sha256: Option<String>,
  #[serde(rename = "ref", skip_serializing_if = "Option::is_none")]
  _ref: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  blob_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  commit_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  last_commit_id: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  branch: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  execute_filemode: Option<bool>,
}

#[derive(Debug, ApiInput, Serialize, Deserialize)]
#[api(no_input_file)]
pub(crate) struct FilePayload {
  #[api(no_short, long = "ref", help = "The name of branch, tag or commit")]
  #[serde(rename = "ref")]
  _ref: Option<String>,
}

#[derive(Debug, ApiInput, Serialize, Deserialize)]
#[api(no_input_file)]
pub(crate) struct FileCreatePayload {
  #[api(no_short, help = "Name of the branch")]
  branch: String,

  #[api(
    no_short,
    long = "start-branch",
    help = "Name of the branch to start the new commit from"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  start_branch: Option<String>,

  #[api(
    no_short,
    help = "Change encoding to `base64`.",
    long_help = "Change encoding to `base64`. Default is `text`.",
    possible_values = "base64,text"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  encoding: Option<String>,

  #[api(
    no_short,
    long = "author-email",
    help = "Specify the commit author's email address"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  author_email: Option<String>,

  #[api(no_short, help = "Specify the commit author's name")]
  #[serde(skip_serializing_if = "Option::is_none")]
  author_name: Option<String>,

  #[api(no_short, help = "File content")]
  content: String,

  #[api(no_short, long = "commit-message", help = "Commit message")]
  commit_message: String,
}

#[derive(Debug, ApiInput, Serialize, Deserialize)]
#[api(no_input_file)]
pub(crate) struct FileDeletePayload {
  #[api(no_short, help = "Name of the branch")]
  branch: String,

  #[api(
    no_short,
    long = "start-branch",
    help = "Name of the branch to start the new commit from"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  start_branch: Option<String>,

  #[api(
    no_short,
    long = "author-email",
    help = "Specify the commit author's email address"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  author_email: Option<String>,

  #[api(no_short, help = "Specify the commit author's name")]
  #[serde(skip_serializing_if = "Option::is_none")]
  author_name: Option<String>,

  #[api(no_short, long = "commit-message", help = "Commit message")]
  commit_message: String,

  #[api(no_short, long = "last-commit-id", help = "Last known file commit ID")]
  #[serde(skip_serializing_if = "Option::is_none")]
  last_commit_id: Option<String>,
}
