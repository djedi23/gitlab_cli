use crud_api::Api;
use serde::{Deserialize, Serialize};

#[derive(Api, Serialize, Deserialize, Debug, Default)]
#[api(endpoint(
  route = "/features",
  multiple_results,
  cli_route = "/features",
  cli_help = "Get a list of all persisted features, with its gate values.",
))]
pub struct Feature {
  name: String,
  state: String,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  definition: Option<Definition>,
  #[api(table_skip)]
  gates: Vec<Gate>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "key")]
pub enum Gate {
  #[serde(rename = "boolean")]
  Boolean { value: bool },
  #[serde(rename = "percentage_of_actors")]
  PercentageOfActors { value: u32 },
}

#[derive(Api, Serialize, Deserialize, Debug)]
#[api(endpoint(
  route = "/features/definitions",
  multiple_results,
  cli_route = "/features/definitions",
  cli_help = "Get a list of all feature definitions.",
))]
pub struct Definition {
  name: String,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  introduced_by_url: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  rollout_issue_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  milestone: Option<String>,
  #[serde(rename = "type")]
  _type: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  group: Option<String>,
  default_enabled: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  log_state_changes: Option<bool>,
}
