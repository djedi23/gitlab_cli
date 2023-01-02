use crud_api::Api;
use serde::{Deserialize, Serialize};

#[derive(Api, Default, Debug, Serialize, Deserialize)]
#[api(
  endpoint(
    route = "/projects/{id}/repository/blobs/{sha}",
    cli_route = "/projects/{id}/repository/blobs/{sha}",
    cli_help = "Allows you to receive information about blob in repository like size and content.",
    cli_long_help = "Allows you to receive information about blob in repository like size and content. Blob content is Base64 encoded.",
  ),
  endpoint(
    route = "/projects/{id}/repository/blobs/{sha}/raw",
    cli_route = "/projects/{id}/repository/blobs/{sha}/raw",
    stream,
    cli_help = "Get the raw file contents for a blob by blob SHA.",
  )
)]
pub(crate) struct Blob {
  size: u32,
  encoding: String,
  #[api(table_skip)]
  content: String,
  sha: String,
}
