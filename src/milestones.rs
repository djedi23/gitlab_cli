use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub(crate) struct Milestone {
  #[serde(skip_serializing_if = "Option::is_none")]
  project_id: Option<u32>,
  description: String,
  state: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  due_date: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  start_date: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  expired: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  iid: Option<u32>,
  created_at: String,
  title: String,
  id: u32,
  updated_at: String,
  web_url: String,
}
