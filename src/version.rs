use crud_api::Api;
use crud_pretty_struct::PrettyPrint;
use serde::{Deserialize, Serialize};

#[derive(Api, Serialize, Deserialize, Debug, Default, Clone)] // PrettyPrint
#[api(endpoint(
  route = "/version",
  cli_route = "/version",
  cli_help = "Retrieve version information for this GitLab instance.",
))]
#[derive(PrettyPrint)]
pub(crate) struct Version {
  version: String,
  revision: String,
}
