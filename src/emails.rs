use crud_api::{Api, ApiInput};
use serde::{Deserialize, Serialize};

#[derive(Api, Serialize, Deserialize, Debug, Default)]
#[api(
  endpoint(
    route = "/user/emails",
    multiple_results,
    cli_route = "/user/emails",
    cli_help = "Get a list of currently authenticated user's emails.",
  ),
  endpoint(
    route = "/user/emails/{id}",
    cli_route = "/user/emails/{id}",
    cli_help = "Get a single email.",
  ),
  endpoint(
    route = "/user/emails",
    method = "POST",
    result_ok_status = "CREATED",
    payload_struct = "EmailCreatePayload",
    cli_route = "/user/emails/create",
    cli_help = "Creates a new email owned by the currently authenticated user.",
  ),
  endpoint(
    route = "/user/emails/{id}",
    method = "DELETE",
    result_ok_status = "NO_CONTENT",
    cli_no_output,
    result_struct = "EmptyResponse",
    cli_route = "/user/emails/{id}/delete",
    cli_help = "Deletes email owned by currently authenticated user.",
  )
)]
#[api(
  endpoint(
    route = "/users/{id}/emails",
    multiple_results,
    cli_route = "/users/{id}/emails",
    cli_help = "Get a single email.",
  ),
  endpoint(
    route = "/users/{id}/emails",
    method = "POST",
    result_ok_status = "CREATED",
    payload_struct = "EmailCreateWithValidationPayload",
    cli_route = "/users/{id}/emails/create",
    cli_help = "Create new email owned by specified user.",
  ),
  endpoint(
    route = "/users/{id}/emails/{eid}",
    method = "DELETE",
    result_ok_status = "NO_CONTENT",
    cli_no_output,
    result_struct = "EmptyResponse",
    cli_route = "/users/{id}/emails/{eid}/delete",
    cli_help = "Deletes email owned by a specified user.",
  )
)]
pub(crate) struct Email {
  id: u32,
  email: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  confirmed_at: Option<String>,
}

#[derive(ApiInput, Serialize, Deserialize, Debug, Default)]
pub(crate) struct EmailCreatePayload {
  #[api(no_short, help = "Email address")]
  email: String,
}

#[derive(ApiInput, Serialize, Deserialize, Debug, Default)]
pub(crate) struct EmailCreateWithValidationPayload {
  #[api(no_short, help = "Email address")]
  email: String,

  #[api(
    no_short,
    long = "skip-confirmation",
    help = "Skip confirmation and assume email is verified"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  skip_confirmation: Option<bool>,
}
