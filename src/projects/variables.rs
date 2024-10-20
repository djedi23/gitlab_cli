use crud_api::Api;
use crud_api::ApiInput;
use crud_pretty_struct::PrettyPrint;
use serde::{Deserialize, Serialize};

#[derive(Api, Serialize, Deserialize, Debug, Default)]
#[api(
  endpoint(
    route = "/projects/{id}/variables",
    multiple_results,
    cli_route = "/projects/{id}/variables",
    cli_help = "Get list of a project's variables."
  ),
  endpoint(
    route = "/projects/{id}/variables/{var_key}",
    cli_route = "/projects/{id}/variables/{var_key}",
    cli_long_help = "Get the details of a single variable. If there are multiple variables with the same key, use filter to select the correct environment_scope.",
    cli_help = "Get the details of a single variable."
  ),
  endpoint(
    route = "/projects/{id}/variables",
    method = "POST",
    payload_struct = "ProjectsVariableCreatePayload",
    result_ok_status = "CREATED",
    cli_route = "/projects/{id}/variables/create",
    cli_long_help = "Create a new variable. If a variable with the same key already exists, the new variable
must have a different environment_scope. Otherwise, GitLab returns a message similar to:
VARIABLE_NAME has already been taken.",
    cli_help = "Create a new variable."
  ),
  endpoint(
    route = "/projects/{id}/variables/{var_key}",
    method = "PUT",
    payload_struct = "ProjectsVariableUpdatePayload",
    cli_route = "/projects/{id}/variables/{var_key}/update",
    cli_long_help = "Update a project's variable. If there are multiple variables with the same key,
use filter to select the correct environment_scope.",
    cli_help = "Update a project's variable."
  ),
  endpoint(
    route = "/projects/{id}/variables/{var_key}",
    method = "DELETE",
    payload_struct = "ProjectsVariableDeletePayload",
    result_ok_status = "NO_CONTENT",
    result_struct = "EmptyResponse",
    cli_no_output,
    cli_route = "/projects/{id}/variables/{var_key}/delete",
    cli_long_help = "Delete a project's variable. If there are multiple variables with the same key,
use filter to select the correct environment_scope.",
    cli_help = "Delete a project's variable."
  )
)]
#[derive(PrettyPrint)] // skip_none  formatter bool
pub(crate) struct ProjectVariables {
  key: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[api(table_skip)]
  variable_type: Option<String>,
  value: String,
  protected: bool,
  masked: bool,
  #[api(table_skip)]
  raw: bool,
  environment_scope: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[api(table_skip)]
  description: Option<String>,
}

#[derive(Debug, ApiInput, Serialize, Deserialize)]
#[api(no_input_file)]
pub(crate) struct ProjectsVariableCreatePayload {
  #[api(
    no_short,
    help = "The key of a variable.",
    long_help = "The key of a variable; must have no more than 255 characters; only A-Z, a-z, 0-9, and _ are allowed"
  )]
  key: String,

  #[api(no_short, help = "The value of a variable")]
  value: String,

  #[api(
    no_short,
    help = "The description of the variable.",
    long_help = "The description of the variable. Default: null."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  description: Option<String>,

  #[api(
    no_short,
    long = "environment-scope",
    help = "The environment_scope of the variable.",
    long_help = "The environment_scope of the variable. Default: *"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  environment_scope: Option<String>,

  #[api(
    no_short,
    help = "Whether the variable is masked.",
    long_help = "Whether the variable is masked. Default: false"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  masked: Option<bool>,

  #[api(
    no_short,
    help = "Whether the variable is protected.",
    long_help = "Whether the variable is protected. Default: false"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  protected: Option<bool>,

  #[api(
    no_short,
    help = "Whether the variable is treated as a raw string.",
    long_help = "Whether the variable is treated as a raw string. Default: false. When true, variables in the value are not expanded."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  raw: Option<bool>,

  #[api(
    no_short,
    long = "variable-type",
    help = "The type of a variable.",
    long_help = "The type of a variable. Available types are: env_var (default) and file",
    possible_values = ""
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  variable_type: Option<String>,
}

#[derive(Debug, ApiInput, Serialize, Deserialize)]
#[api(no_input_file)]
pub(crate) struct ProjectsVariableUpdatePayload {
  #[api(no_short, help = "The value of a variable")]
  value: String,

  #[api(
    no_short,
    help = "The description of the variable.",
    long_help = "The description of the variable. Default: null."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  description: Option<String>,

  #[api(
    no_short,
    long = "environment-scope",
    help = "The environment_scope of the variable.",
    long_help = "The environment_scope of the variable. Default: *"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  environment_scope: Option<String>,
  #[api(no_short, help = "Available filters: [environment_scope].")]
  #[serde(skip_serializing_if = "Option::is_none")]
  filter: Option<String>,

  #[api(
    no_short,
    help = "Whether the variable is masked.",
    long_help = "Whether the variable is masked. Default: false"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  masked: Option<bool>,

  #[api(
    no_short,
    help = "Whether the variable is protected.",
    long_help = "Whether the variable is protected. Default: false"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  protected: Option<bool>,

  #[api(
    no_short,
    help = "Whether the variable is treated as a raw string.",
    long_help = "Whether the variable is treated as a raw string. Default: false. When true, variables in the value are not expanded."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  raw: Option<bool>,

  #[api(
    no_short,
    long = "variable-type",
    help = "The type of a variable.",
    long_help = "The type of a variable. Available types are: env_var (default) and file",
    possible_values = ""
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  variable_type: Option<String>,
}

#[derive(Debug, ApiInput, Serialize, Deserialize)]
#[api(no_input_file)]
pub(crate) struct ProjectsVariableDeletePayload {
  #[api(no_short, help = "Available filters: [environment_scope].")]
  #[serde(skip_serializing_if = "Option::is_none")]
  filter: Option<String>,
}
