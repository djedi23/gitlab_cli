use crud_api::{Api, ApiInput};
use serde::{Deserialize, Serialize};

#[derive(Api, Serialize, Deserialize, Debug, Default)]
#[api(endpoint(
  route = "/markdown",
  method = "POST",
  result_ok_status = "CREATED",
  payload_struct = "MarkdownQuery",
  cli_route = "/markdown",
  cli_help = "Render an arbitrary Markdown document",
))]
pub(crate) struct Markdown {
  html: String,
}

#[derive(Debug, Default, ApiInput, Serialize, Deserialize)]
pub(crate) struct MarkdownQuery {
  #[api(no_short, help = "The Markdown text to render")]
  text: String,

  #[api(
    no_short,
    help = "Render text using GitLab Flavored Markdown.",
    long_help = "Render text using GitLab Flavored Markdown. Default is `false`"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  gfm: Option<bool>,

  #[api(
    no_short,
    help = "Use project as a context when creating references using GitLab Flavored Markdown."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  project: Option<String>,
}
