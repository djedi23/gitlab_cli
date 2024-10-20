use crud_pretty_struct::{formatters::bool_check_formatter, PrettyPrint};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone, PrettyPrint)] // skip_none formatter bool
pub(crate) struct Milestone {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[pretty(skip_none)]
  project_id: Option<u32>,
  description: String,
  state: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[pretty(skip_none)]
  due_date: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[pretty(skip_none)]
  start_date: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[pretty(skip_none, formatter=bool_check_formatter)]
  expired: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[pretty(skip_none)]
  iid: Option<u32>,
  created_at: String,
  title: String,
  id: u32,
  updated_at: String,
  web_url: String,
}
