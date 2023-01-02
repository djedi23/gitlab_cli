use crud_api::Api;
use serde::{Deserialize, Serialize};

#[derive(Api, Serialize, Deserialize, Debug, Default, Clone)]
#[api(endpoint(
  route = "/version",
  cli_route = "/version",
  cli_help = "Retrieve version information for this GitLab instance.",
))]
pub(crate) struct Version {
  version: String,
  revision: String,
}
