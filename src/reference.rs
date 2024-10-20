use crud_pretty_struct::PrettyPrint;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone, PrettyPrint)]
pub(crate) struct References {
  short: String,
  relative: String,
  full: String,
}
