use crud_api::{Api, ApiInput};
use serde::{Deserialize, Serialize};

#[derive(Api, Serialize, Deserialize, Debug, Default)]
#[api(
  endpoint(
    route = "/projects/{id}/export",
    method = "POST",
    cli_no_output,
    result_ok_status = "ACCEPTED",
    result_struct = "EmptyResponse",
    payload_struct = "ExportPayload",
    cli_route = "/projects/{id}/export/create",
    cli_help = "Start a new export.",
  ),
  endpoint(
    route = "/projects/{id}/export",
    cli_route = "/projects/{id}/export",
    cli_help = "Get the status of export.",
  ),
  endpoint(
    route = "/projects/{id}/export/download",
    stream,
    cli_route = "/projects/{id}/export/download",
    cli_help = "Download the finished export.",
  )
)]
#[api(
  endpoint(
    route = "/groups/{id}/export",
    method = "POST",
    cli_no_output,
    result_ok_status = "ACCEPTED",
    result_struct = "EmptyResponse",
    cli_route = "/groups/{id}/export/create",
    cli_help = "Start a new export.",
  ),
  endpoint(
    route = "/groups/{id}/export/download",
    stream,
    cli_route = "/groups/{id}/export/download",
    cli_help = "Download the finished export.",
  )
)]
#[api(
  endpoint(
    route = "/groups/{id}/export",
    method = "POST",
    cli_no_output,
    result_ok_status = "ACCEPTED",
    result_struct = "EmptyResponse",
    cli_route = "/groups/{id}/export/create",
    cli_help = "Start a new export.",
  ),
  endpoint(
    route = "/groups/{id}/export/download",
    stream,
    cli_route = "/groups/{id}/export/download",
    cli_help = "Download the finished export.",
  )
)]

pub(crate) struct Export {
  id: u32,
  description: Option<String>,
  name: String,
  name_with_namespace: String,
  path: String,
  path_with_namespace: String,
  created_at: String,
  export_status: String,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  _links: Option<Links>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub(crate) struct Links {
  api_url: String,
  web_url: String,
  // TODO upload...
}

#[derive(Debug, ApiInput, Serialize, Deserialize)]
#[api(no_input_file)]
pub(crate) struct ExportPayload {
  #[api(no_short, help = "Overrides the project description")]
  #[serde(skip_serializing_if = "Option::is_none")]
  description: Option<String>,
}

#[derive(Api, Serialize, Deserialize, Debug, Default)]
#[api(
  endpoint(
    route = "/groups/{id}/export_relations",
    method = "POST",
    cli_no_output,
    result_ok_status = "ACCEPTED",
    result_struct = "EmptyResponse",
    cli_route = "/groups/{id}/export_relations/create",
    cli_help = "Start a new group relations export.",
  ),
  endpoint(
    route = "/groups/{id}/export_relations/status",
    multiple_results,
    cli_route = "/groups/{id}/export_relations/status",
    cli_help = "View the status of the relations export",
  ),
  endpoint(
    route = "/groups/{id}/export_relations/download",
    stream,
    query_struct = "ExportRelationQuery",
    cli_route = "/groups/{id}/export_relations/download",
    cli_help = "Download the finished export.",
  )
)]
pub(crate) struct ExportRelation {
  relation: String,
  status: u32,
  error: Option<String>,
  updated_at: String,
}

#[derive(Debug, ApiInput, Serialize, Deserialize)]
#[api(no_input_file)]
pub(crate) struct ExportRelationQuery {
  #[api(
    no_short,
    long = "relation",
    help = "Name of the group top-level relation to download."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  relation: Option<String>,
}
