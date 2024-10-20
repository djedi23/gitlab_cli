use crud_pretty_struct::PrettyPrint;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone, PrettyPrint)]
pub(crate) struct TaskCompletionStatus {
  count: u32,
  completed_count: u32,
}
