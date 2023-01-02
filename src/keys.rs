use crud_api::{Api, ApiInput};
use serde::{Deserialize, Serialize};

#[derive(Api, Serialize, Deserialize, Debug, Default)]
#[api(
  endpoint(
    route = "/user/keys",
    multiple_results,
    cli_route = "/user/keys",
    cli_help = "Get a list of currently authenticated user's SSH keys.",
  ),
  endpoint(
    route = "/user/keys",
    method = "POST",
    payload_struct = "SSHKeyCreatePayload",
    result_ok_status = "CREATED",
    cli_route = "/user/keys/create",
    cli_help = "Creates a new key owned by the currently authenticated user.",
  ),
  endpoint(
    route = "/users/{id}/keys",
    method = "POST",
    payload_struct = "SSHKeyCreatePayload",
    result_ok_status = "CREATED",
    cli_no_output,
    result_struct = "EmptyResponse",
    cli_route = "/users/{id}/keys/create",
    cli_long_help = "Create new key owned by specified user. Available only for admin.",
    cli_help = "Create new key owned by specified user.",
  ),
  endpoint(
    route = "/user/keys/{id}",
    method = "DELETE",
    result_ok_status = "NO_CONTENT",
    cli_route = "/user/keys/{id}/delete",
    cli_help = "Deletes key owned by currently authenticated user.",
  ),
  endpoint(
    route = "/users/{id}/keys/{kid}",
    method = "DELETE",
    result_ok_status = "NO_CONTENT",
    cli_no_output,
    result_struct = "EmptyResponse",
    cli_route = "/users/{id}/keys/{kid}/delete",
    cli_long_help = "Deletes key owned by a specified user. Available only for admin.",
    cli_help = "Deletes key owned by a specified user.",
  ),
  endpoint(
    route = "/users/{id}/keys",
    multiple_results,
    cli_route = "/users/{id}/keys",
    cli_help = "Get a list of a specified user's SSH keys.",
  ),
  endpoint(
    route = "/users/{id}/keys/{kid}",
    cli_route = "/users/{id}/keys/{kid}",
    cli_help = "Get a single key for a given user.",
  ),
  endpoint(
    route = "/user/keys/{id}",
    cli_route = "/user/keys/{id}",
    cli_help = "Get a single key.",
  )
)]
pub(crate) struct SSHKey {
  id: u32,
  #[serde(skip_serializing_if = "Option::is_none")]
  title: Option<String>,
  #[api(table_skip)]
  key: String,
  created_at: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  expires_at: Option<String>,
}

#[derive(ApiInput, Serialize, Deserialize, Debug, Default)]
pub(crate) struct SSHKeyCreatePayload {
  #[api(no_short, help = "new SSH key's title")]
  title: String,

  #[api(no_short, help = "new SSH key")]
  key: String,

  #[api(
    no_short,
    long = "expires-at",
    help = "The expiration date of the SSH key.",
    long_help = "The expiration date of the SSH key in ISO 8601 format (`YYYY-MM-DDTHH:MM:SSZ`)"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  expires_at: Option<String>,
}

#[derive(Api, Serialize, Deserialize, Debug, Default)]
#[api(
  endpoint(
    route = "/user/gpg_keys",
    multiple_results,
    cli_route = "/user/gpg_keys",
    cli_help = "Get a list of currently authenticated user's GPG keys.",
  ),
  endpoint(
    route = "/user/gpg_keys/{id}",
    cli_route = "/user/gpg_keys/{id}",
    cli_help = "Get a specific GPG key of currently authenticated user.",
  ),
  endpoint(
    route = "/user/gpg_keys",
    method = "POST",
    payload_struct = "GpgKeyCreatePayload",
    result_ok_status = "CREATED",
    cli_route = "/user/gpg_keys/create",
    cli_help = "Creates a new GPG key owned by the currently authenticated user.",
  ),
  endpoint(
    route = "/user/gpg_keys/{id}",
    method = "DELETE",
    result_ok_status = "NO_CONTENT",
    cli_no_output,
    result_struct = "EmptyResponse",
    cli_route = "/user/gpg_keys/{id}/delete",
    cli_help = "Delete a GPG key owned by currently authenticated user.",
  )
)]
#[api(
  endpoint(
    route = "/users/{id}/gpg_keys",
    multiple_results,
    cli_route = "/users/{id}/gpg_keys",
    cli_long_help = "Get a list of a specified user's GPG keys. This endpoint can be accessed without authentication.",
    cli_help = "Get a list of a specified user's GPG keys.",
  ),
  endpoint(
    route = "/users/{id}/gpg_keys/{key_id}",
    cli_route = "/users/{id}/gpg_keys/{key_id}",
    cli_help = "Get a specific GPG key for a given user.",
  ),
  endpoint(
    route = "/users/{id}/gpg_keys",
    method = "POST",
    payload_struct = "GpgKeyCreatePayload",
    result_ok_status = "CREATED",
    cli_route = "/users/{id}/gpg_keys/create",
    cli_help = "Create new GPG key owned by the specified user.",
  ),
  endpoint(
    route = "/users/{id}/gpg_keys/{key_id}",
    method = "DELETE",
    result_ok_status = "NO_CONTENT",
    cli_no_output,
    result_struct = "EmptyResponse",
    cli_route = "/users/{id}/gpg_keys/{key_id}/delete",
    cli_help = "Delete a GPG key owned by a specified user.",
  )
)]
pub(crate) struct GpGKey {
  id: u32,
  #[api(table_skip)]
  key: String,
  created_at: String,
}

#[derive(ApiInput, Serialize, Deserialize, Debug, Default)]
pub(crate) struct GpgKeyCreatePayload {
  #[api(no_short, help = "The new GPG key")]
  key: String,
}
