use crud_api::Api;
use serde::{Deserialize, Serialize};

#[derive(Api, Serialize, Deserialize, Debug, Default)]
#[api(endpoint(
  route = "/projects/{id}/pipelines/{pipeline_id}/variables",
  multiple_results,
  cli_route = "/projects/{id}/pipelines/{pipeline_id}/variables",
  cli_help = "Get variables of a pipeline."
))]
pub(crate) struct Variables {
  key: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  variable_type: Option<String>,
  value: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  protected: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  masked: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  raw: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  environment_scope: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  description: Option<String>,
}
