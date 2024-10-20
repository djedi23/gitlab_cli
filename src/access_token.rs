use crate::user::AccessLevel;
use crud_api::{Api, ApiInput};
use miette::{IntoDiagnostic, WrapErr};
use serde::{Deserialize, Serialize};

#[derive(Api, Serialize, Deserialize, Debug, Default)]
#[api(
  endpoint(
    route = "/projects/{id}/access_tokens",
    multiple_results,
    cli_route = "/projects/{id}/access_tokens",
    cli_help = "Get a list of project access tokens.",
  ),
  endpoint(
    route = "/projects/{id}/access_tokens/{token_id}",
    cli_route = "/projects/{id}/access_tokens/{token_id}",
    cli_help = "Get a project access token by ID.",
  ),
  endpoint(
    route = "/projects/{id}/access_tokens",
    method = "POST",
    result_ok_status = "CREATED",
    payload_struct = "AccessTokenCreatePayload",
    cli_route = "/projects/{id}/access_tokens/create",
    cli_help = "Create a project access token.",
  ),
  endpoint(
    route = "/projects/{id}/access_tokens/{token_id}",
    method = "DELETE",
    result_ok_status = "NO_CONTENT",
    cli_no_output,
    result_struct = "EmptyResponse",
    cli_route = "/projects/{id}/access_tokens/{token_id}/revoke",
    cli_help = "Revoke a project access token.",
  )
)]
#[api(
  endpoint(
    route = "/groups/{id}/access_tokens",
    multiple_results,
    cli_route = "/groups/{id}/access_tokens",
    cli_help = "Get a list of group access tokens.",
  ),
  endpoint(
    route = "/groups/{id}/access_tokens/{token_id}",
    cli_route = "/groups/{id}/access_tokens/{token_id}",
    cli_help = "Get a group access token by ID.",
  ),
  endpoint(
    route = "/groups/{id}/access_tokens",
    method = "POST",
    result_ok_status = "CREATED",
    payload_struct = "AccessTokenCreatePayload",
    cli_route = "/groups/{id}/access_tokens/create",
    cli_help = "Create a group access token.",
  ),
  endpoint(
    route = "/groups/{id}/access_tokens/{token_id}",
    method = "DELETE",
    result_ok_status = "NO_CONTENT",
    cli_no_output,
    result_struct = "EmptyResponse",
    cli_route = "/groups/{id}/access_tokens/{token_id}/revoke",
    cli_help = "Revoke a group access token.",
  )
)]
pub struct AccessToken {
  id: u32,
  user_id: u32,
  name: String,
  scopes: Vec<String>,
  #[api(table_format(date(format = "%Y-%m-%d")))]
  created_at: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  expires_at: Option<String>,
  #[api(table_skip, table_format(date(format = "%Y-%m-%d")))]
  #[serde(skip_serializing_if = "Option::is_none")]
  last_used_at: Option<String>,
  access_level: AccessLevel,
  active: bool,
  revoked: bool,
}

#[derive(Debug, Default, ApiInput, Serialize, Deserialize)]
pub(crate) struct AccessTokenCreatePayload {
  #[api(help = "Name of the access token")]
  name: String,
  #[api(
    help = "List of scopes",
    possible_values = "api,read_api,read_registry,write_registry,read_repository,write_repository"
  )]
  scopes: Vec<String>,
  #[api(long = "access-level", help = "Access level.")]
  #[serde(skip_serializing_if = "Option::is_none")]
  access_level: Option<AccessLevel>,
  #[api(
    long = "expires-at",
    help = "Token expires at midnight UTC on that date"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  expires_at: Option<String>,
}
