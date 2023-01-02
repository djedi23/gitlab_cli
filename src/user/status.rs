use crud_api::{Api, ApiInput};
use serde::{Deserialize, Serialize};

#[derive(Api, Serialize, Deserialize, Debug, Default, Clone)]
#[api(
  endpoint(
    route = "/user/status",
    cli_route = "/user/status",
    cli_help = "Get the status of the currently signed in user",
  ),
  endpoint(
    route = "/user/status",
    method = "PUT",
    cli_route = "/user/status/edit",
    payload_struct = "UserStatusPayload",
    cli_help = "Set the status of the current user.",
  ),
  endpoint(
    route = "/users/{id}/status",
    cli_route = "/users/{id}/status",
    cli_help = "Get the status of a user.",
  )
)]
pub(crate) struct Status {
  #[serde(skip_serializing_if = "Option::is_none")]
  emoji: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  availability: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  message: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  message_html: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  clear_status_at: Option<String>,
}

#[derive(Debug, ApiInput, Serialize, Deserialize)]
#[api(no_input_file)]
pub(crate) struct UserStatusPayload {
  #[api(
    no_short,
    help = "The name of the emoji to use as status.",
    long_help = "The name of the emoji to use as status. If omitted `speech_balloon` is used. Emoji name can be one of the specified names in the Gemojione index."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  emoji: Option<String>,

  #[api(
    no_short,
    help = "The message to set as a status.",
    long_help = "The message to set as a status. It can also contain emoji codes."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  message: Option<String>,

  #[api(
    no_short,
    long = "clear-status-after",
    help = "Automatically clean up the status after a given time interval.",
    long_help = "Automatically clean up the status after a given time interval. Allowed values: `30_minutes`, `3_hours`, `8_hours`, `1_day`, `3_days`, `7_days`, `30_days`",
    possible_values = "30_minutes,3_hours,8_hours,1_day,3_days,7_days,30_days"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  clear_status_after: Option<String>,
}
