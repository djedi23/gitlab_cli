use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub(crate) struct TaskCompletionStatus {
  count: u32,
  completed_count: u32,
}
