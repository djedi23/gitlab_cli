use crate::{commit::Commit, diff::Change};
use crud_api::{Api, ApiInput};
use serde::{Deserialize, Serialize};

#[derive(Api, Default, Debug, Serialize, Deserialize, Clone)]
#[api(endpoint(
  route = "/projects/{id}/repository/compare",
  query_struct = "CompareQuery",
  cli_route = "/projects/{id}/repository/compare",
  cli_help = "This endpoint can be accessed without authentication if the repository is
publicly accessible.",
))]
pub(crate) struct Compare {
  commit: Commit,
  #[api(table_skip)]
  commits: Vec<Commit>,
  #[api(table_skip)]
  diffs: Vec<Change>,
}

#[derive(Debug, ApiInput, Serialize, Deserialize)]
#[api(no_input_file)]
pub(crate) struct CompareQuery {
  #[api(no_short, help = "The commit SHA or branch name.")]
  from: String,

  #[api(no_short, help = "The commit SHA or branch name.")]
  to: String,

  #[api(no_short, long = "from-project-id", help = "The ID to compare from")]
  #[serde(skip_serializing_if = "Option::is_none")]
  from_project_id: Option<u32>,

  #[api(
    no_short,
    help = "Comparison method, `true` for direct comparison between `from` and `to`.",
    long_help = "Comparison method, `true` for direct comparison between `from` and `to`. (`from`..`to`), `false` to compare using merge base (`from`...`to`)'. Default is `false`."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  straight: Option<bool>,
}
