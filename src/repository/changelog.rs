use crud_api::{Api, ApiInput};
use serde::{Deserialize, Serialize};

#[derive(Api, Clone, Default, Debug, Serialize, Deserialize)]
#[api(
  endpoint(
    route = "/projects/{id}/repository/changelog",
    method = "POST",
    result_ok_status = "OK",
    cli_no_output,
    cli_route = "/projects/{id}/repository/changelog/generate",
    query_struct = "ChangelogPayload",
    cli_help = "Generate changelog data based on commits in a repository.",
  ),
  endpoint(
    route = "/projects/{id}/repository/changelog",
    result_ok_status = "OK",
    cli_no_output,
    cli_route = "/projects/{id}/repository/changelog",
    query_struct = "ChangelogPayload",
    cli_help = "Generate changelog data based on commits in a repository, without committing them to a changelog file.",
  )
)]
pub(crate) struct Changelog {}

#[derive(Debug, ApiInput, Serialize, Deserialize)]
#[api(no_input_file)]
pub(crate) struct ChangelogPayload {
  #[api(
    no_short,
    help = "The version to generate the changelog for.",
    long_help = "The version to generate the changelog for. The format must follow semantic versioning."
  )]
  version: String,

  #[api(
    no_short,
    help = "The start of the range of commits (as a SHA) to use for generating the changelog.",
    long_help = "The start of the range of commits (as a SHA) to use for generating the changelog. This commit itself isn't included in the list."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  from: Option<String>,

  #[api(
    no_short,
    help = "The end of the range of commits (as a SHA) to use for the changelog.",
    long_help = "The end of the range of commits (as a SHA) to use for the changelog. This commit is included in the list. Defaults to the branch specified in the `branch` attribute."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  to: Option<String>,

  #[api(
    no_short,
    help = "The date and time of the release.",
    long_help = "The date and time of the release. defaults to the current time."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  date: Option<String>,

  #[api(
    no_short,
    help = "The branch to commit the changelog changes to.",
    long_help = "The branch to commit the changelog changes to. defaults to the project's default branch."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  branch: Option<String>,

  #[api(
    no_short,
    help = "The Git trailer to use for including commits.",
    long_help = "The Git trailer to use for including commits. defaults to `Changelog`."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  trailer: Option<String>,

  #[api(
    no_short,
    help = "The file to commit the changes to.",
    long_help = "The file to commit the changes to. defaults to `CHANGELOG.md`."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  file: Option<String>,

  #[api(
    no_short,
    help = "The commit message to produce when committing the changes.",
    long_help = "The commit message to produce when committing the changes. defaults to `Add changelog for version X` where X is the value of the `version` argument."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  message: Option<String>,
}
