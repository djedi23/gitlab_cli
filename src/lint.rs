use crud_api::{Api, ApiInput};
use serde::{Deserialize, Serialize};

#[derive(Api, Serialize, Deserialize, Debug, Default, Clone)]
#[api(
  endpoint(
    route = "/ci/lint",
    method = "POST",
    payload_struct = "LintPayload",
    cli_route = "/ci/lint",
    cli_help = "Checks if CI/CD YAML configuration is valid.",
    cli_long_help = "Checks if CI/CD YAML configuration is valid. This endpoint validates basic CI/CD configuration syntax.",
  ),
  endpoint(
    route = "/projects/{id}/ci/lint",
    method = "POST",
    payload_struct = "LintProjectPayload",
    cli_route = "/projects/{id}/ci/lint",
    cli_help = "Checks if CI/CD YAML configuration is valid.",
    cli_long_help = "Checks if CI/CD YAML configuration is valid. This endpoint has namespace specific context.",
  ),
  endpoint(
    route = "/projects/{id}/ci/lint",
    payload_struct = "LintCurrentProjectPayload",
    cli_route = "/projects/{id}/ci/lint_project",
    cli_help = "Checks if a project's latest (HEAD of the project's default branch) .gitlab-ci.yml configuration is valid.",
    cli_long_help = "Checks if a project's latest (HEAD of the project's default branch) .gitlab-ci.yml configuration is valid. This endpoint uses all namespace specific data available, including variables, local includes, and so on.",
  )
)]
pub(crate) struct Lint {
  #[serde(skip_serializing_if = "Option::is_none")]
  status: Option<String>,
  valid: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  errors: Option<Vec<String>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  warning: Option<Vec<String>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  merged_yaml: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  includes: Option<Vec<String>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  jobs: Option<Vec<String>>,
}

#[derive(Debug, Default, ApiInput, Serialize, Deserialize)]
pub(crate) struct LintPayload {
  #[api(no_short, help = "The CI/CD configuration content.")]
  content: String,

  #[api(
    no_short,
    long = "include-merged-yaml",
    help = "If the expanded CI/CD configuration should be included in the response."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  include_merged_yaml: Option<bool>,

  #[api(
    no_short,
    long = "include-jobs",
    help = "If the list of jobs should be included in the response.",
    long_help = "If the list of jobs should be included in the response. This is false by default."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  include_jobs: Option<bool>,
}

#[derive(Debug, Default, ApiInput, Serialize, Deserialize)]
pub(crate) struct LintProjectPayload {
  #[api(no_short, help = "The CI/CD configuration content.")]
  content: String,

  #[api(
    no_short,
    long = "dry-run",
    help = "Run pipeline creation simulation, or only do static check.",
    long_help = "Run pipeline creation simulation, or only do static check. This is false by default."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  dry_run: Option<bool>,

  #[api(
    no_short,
    long = "include-jobs",
    help = "If the list of jobs should be included in the response.",
    long_help = "If the list of jobs should be included in the response. This is false by default."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  include_jobs: Option<bool>,

  #[api(
    no_short,
    long = "ref",
    help = "When `dry_run` is `true`, sets the branch or tag to use.",
    long_help = "When `dry_run` is `true`, sets the branch or tag to use. Defaults to the project's default branch when not set.",
    possible_values = ""
  )]
  #[serde(skip_serializing_if = "Option::is_none", rename = "ref")]
  _ref: Option<String>,
}

#[derive(Debug, Default, ApiInput, Serialize, Deserialize)]
pub(crate) struct LintCurrentProjectPayload {
  #[api(
    no_short,
    long = "dry-run",
    help = "Run pipeline creation simulation, or only do static check.",
    long_help = "Run pipeline creation simulation, or only do static check. This is false by default."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  dry_run: Option<bool>,

  #[api(
    no_short,
    long = "include-jobs",
    help = "If the list of jobs should be included in the response.",
    long_help = "If the list of jobs should be included in the response. This is false by default."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  include_jobs: Option<bool>,

  #[api(
    no_short,
    long = "ref",
    help = "When `dry_run` is `true`, sets the branch or tag to use.",
    long_help = "When `dry_run` is `true`, sets the branch or tag to use. Defaults to the project's default branch when not set.",
    possible_values = ""
  )]
  #[serde(skip_serializing_if = "Option::is_none", rename = "ref")]
  _ref: Option<String>,
}
