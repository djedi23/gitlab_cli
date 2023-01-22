use crate::User;
use crud_api::{Api, ApiInput};
use serde::{Deserialize, Serialize};

#[derive(Api, Serialize, Deserialize, Debug, Default)]
#[api(
  endpoint(
    route = "/events",
    query_struct = "EventFilter",
    multiple_results,
    cli_route = "/events",
    cli_help = "Get a list of events for the authenticated user.",
    cli_long_help = "Get a list of events for the authenticated user. Scope read_user or api is required. Events associated with epics are not available using the API.",
  ),
  endpoint(
    route = "/users/{id}/events",
    query_struct = "EventFilter",
    multiple_results,
    cli_route = "/users/{id}/events",
    cli_help = "Get the contribution events for the specified user,",
    cli_long_help = "Get the contribution events for the specified user, sorted from newest to oldest. Scope read_user or api is required. Events associated with epics are not available using API.",
  ),
  endpoint(
    route = "/projects/{id}/events",
    query_struct = "EventFilter",
    multiple_results,
    cli_route = "/projects/{id}/events",
    cli_help = "Get a list of visible events for a particular project.",
  )
)]
pub struct Event {
  id: u32,
  #[api(table_skip)]
  title: Option<String>,
  project_id: u32,
  action_name: String,
  #[api(table_skip)]
  target_id: Option<u32>,
  #[api(table_skip)]
  target_iid: Option<u32>,
  #[api(table_skip)]
  target_type: Option<String>,
  #[api(table_skip)]
  author_id: u32,
  target_title: Option<String>,
  created_at: String,
  author_username: String,
  #[api(table_skip)]
  author: User,

  #[api(table_skip)]
  #[serde(flatten)]
  payload: Option<EventPayload>, //  #[serde(skip_serializing_if = "Option::is_none")]
                                 //  wiki_page: Option<WikiPage>,
}

#[derive(Debug, Deserialize, Serialize)]
//#[allow(dead_code)]
enum EventPayload {
  #[serde(rename = "wiki_page")]
  WikiData(WikiPage),
  #[serde(rename = "push_data")]
  PushData(PushData),
  #[serde(rename = "note")]
  Note(Note),
}

#[derive(Debug, Serialize, Deserialize)]
struct WikiPage {
  format: String,
  slug: String,
  title: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PushData {
  commit_count: u32,
  action: String,
  ref_type: String,
  commit_from: String,
  commit_to: String,
  #[serde(rename = "ref")]
  ref_: String,
  commit_title: String,
  ref_count: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Note {
  id: u32,
  #[serde(rename = "type")]
  type_: Option<String>,
  body: String,
  attachment: Option<String>,
  author: User,
  created_at: String,
  updated_at: String,
  system: bool,
  noteable_id: u32,
  noteable_type: String,
  resolvable: bool,
  confidential: bool,
  internal: bool,
  noteable_iid: u32,
  //  commands_changes:{}
}

#[derive(Debug, ApiInput, Serialize, Deserialize)]
#[api(no_input_file)]
pub(crate) struct EventFilter {
  #[api(
    no_short,
    help = "Include only events of a particular action type",
    possible_values = "approved,closed,commented,created,destroyed,expired,joined,left,merged,pushed,reopened,updated"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  action: Option<String>,

  #[api(
    no_short,
    long = "target-type",
    help = "Include only events of a particular target type",
    possible_values = "issue,milestone,merge_request,note,project,snippet,user"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  target_type: Option<String>,

  #[api(
    no_short,
    help = "Include only events created before a particular date."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  before: Option<String>,

  #[api(
    no_short,
    help = "Include only events created after a particular date."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  after: Option<String>,

  #[api(no_short, help = "Include all events across a user's projects.")]
  #[serde(skip_serializing_if = "Option::is_none")]
  scope: Option<String>,

  #[api(
    no_short,
    help = "Sort events in asc or desc order by created_at.",
    long_help = "Sort events in asc or desc order by created_at. Default is desc.",
    possible_values = "asc,desc"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  sort: Option<String>,

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
