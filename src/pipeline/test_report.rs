use crud_api::Api;
use serde::{Deserialize, Serialize};

#[derive(Api, Serialize, Deserialize, Debug, Default)]
#[api(endpoint(
  route = "/projects/{id}/pipelines/{pipeline_id}/test_report",
  cli_route = "/projects/{id}/pipelines/{pipeline_id}/test_report",
  cli_help = "Get a pipeline's test report"
))]
pub(crate) struct TestReport {
  total_time: u32,
  total_count: u32,
  success_count: u32,
  failed_count: u32,
  skipped_count: u32,
  error_count: u32,
  #[api(table_skip)]
  test_suites: Vec<TestSuite>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct TestSuite {
  name: String,
  total_time: u32,
  total_count: u32,
  success_count: u32,
  failed_count: u32,
  skipped_count: u32,
  error_count: u32,
  #[serde(skip_serializing_if = "Option::is_none")]
  test_cases: Option<Vec<TestCase>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  build_ids: Option<Vec<u32>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  suite_error: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct TestCase {
  status: String,
  name: String,
  classname: String,
  execution_time: u32,
  #[serde(skip_serializing_if = "Option::is_none")]
  system_output: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  stack_trace: Option<String>,
}

#[derive(Api, Serialize, Deserialize, Debug, Default)]
#[api(endpoint(
  route = "/projects/{id}/pipelines/{pipeline_id}/test_report_summary",
  cli_route = "/projects/{id}/pipelines/{pipeline_id}/test_report_summary",
  cli_help = "Get a pipeline's test report summary"
))]
pub(crate) struct TestReportSummary {
  #[api(table_skip)]
  total: SummaryTotal,
  #[api(table_skip)]
  test_suites: Vec<TestSuite>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct SummaryTotal {
  time: u32,
  count: u32,
  success: u32,
  failed: u32,
  skipped: u32,
  error: u32,
  #[serde(skip_serializing_if = "Option::is_none")]
  suite_error: Option<String>,
}
