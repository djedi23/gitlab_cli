use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub(crate) struct References {
  short: String,
  relative: String,
  full: String,
}
