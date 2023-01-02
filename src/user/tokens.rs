use crud_api::{Api, ApiInput};
use serde::{Deserialize, Serialize};

#[derive(Api, Serialize, Deserialize, Debug, Default, Clone)]
#[api(
  endpoint(
    route = "/users/{id}/impersonation_tokens",
    multiple_results,
    query_struct = "TokenFilter",
    cli_route = "/users/{id}/impersonation_tokens",
    cli_help = "It retrieves every impersonation token of the user.",
  ),
  endpoint(
    route = "/users/{id}/impersonation_tokens/{tid}",
    cli_route = "/users/{id}/impersonation_tokens/{tid}",
    cli_help = "It retrieves every impersonation token of the user.",
  ),
  endpoint(
    route = "/users/{id}/impersonation_tokens/{tid}",
    method = "DELETE",
    result_ok_status = "NO_CONTENT",
    cli_no_output,
    result_struct = "EmptyResponse",
    cli_route = "/users/{id}/impersonation_tokens/{tid}/revoke",
    cli_help = "It revokes an impersonation token.",
  ),
  endpoint(
    route = "/users/{id}/impersonation_tokens",
    method = "POST",
    payload_struct = "TokenCreatePayload",
    result_ok_status = "CREATED",
    cli_route = "/users/{id}/impersonation_tokens/create",
    cli_help = "It creates a new impersonation token.",
    cli_long_help = "It creates a new impersonation token. Only administrators can do this. You are only able to create impersonation tokens to impersonate the user and perform both API calls and Git reads and writes. The user can't see these tokens in their profile settings page."
  )
)]
#[api(
  endpoint(
    route = "/users/{id}/personal_access_tokens",
    method = "POST",
    payload_struct = "TokenCreatePayload",
    result_ok_status = "CREATED",
    cli_route = "/users/{id}/personal_access_tokens/create",
    cli_help = "Use this API to create a new personal access token.",
    cli_long_help = "Use this API to create a new personal access token. Token values are returned once so, make sure you save it as you can't access it again."
  ),
  endpoint(
    route = "/personal_access_tokens",
    multiple_results,
    cli_route = "/personal_access_tokens",
    cli_help = "Get a list of personal access tokens.",
  ),
  endpoint(
    route = "/personal_access_tokens/{id}",
    cli_route = "/personal_access_tokens/{id}",
    cli_help = "Get a single personal access token by its ID.",
  ),
  endpoint(
    route = "/personal_access_tokens/{id}",
    method = "DELETE",
    result_ok_status = "NO_CONTENT",
    cli_no_output,
    result_struct = "EmptyResponse",
    cli_route = "/personal_access_tokens/{id}/revoke",
    cli_help = "Revoke a personal access token by either:",
  )
)]
pub(crate) struct ImpersonationToken {
  id: u32,
  active: bool,
  user_id: u32,
  scopes: Vec<String>,
  revoked: bool,
  name: String,
  created_at: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  last_used_at: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  impersonation: Option<bool>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  expires_at: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  token: Option<String>,
}

#[derive(Debug, ApiInput, Serialize, Deserialize)]
#[api(no_input_file)]
pub(crate) struct TokenFilter {
  #[api(
    no_short,
    heading = "Pagination",
    help = "Page number.",
    long_help = "Page number (default: 1)."
  )]
  page: Option<u32>,

  #[api(
    no_short,
    long = "per-page",
    heading = "Pagination",
    help = "Number of items to list per page.",
    long_help = "Number of items to list per page (default: 20, max: 100)."
  )]
  per_page: Option<u32>,
}

#[derive(Debug, Default, ApiInput, Serialize, Deserialize)]
pub(crate) struct TokenCreatePayload {
  #[api(no_short, help = "The name of the impersonation token")]
  name: String,

  #[api(
    no_short,
    long = "expires-at",
    help = "The expiration date of the impersonation token in ISO format (YYYY-MM-DD)"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  expires_at: Option<String>,

  #[api(
    no_short,
    help = "The array of scopes of the impersonation token (api, read_user)"
  )]
  scopes: Vec<String>,
}
