pub mod activity;
pub mod count;
pub mod membership;
pub mod preferences;
pub mod status;
pub mod tokens;

use crud_api::{Api, ApiInput};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Api, Serialize, Deserialize, Debug, Default, Clone)]
#[api(
  endpoint(
    route = "/users",
    multiple_results,
    query_struct = "UsersFilter",
    cli_route = "/users",
    cli_help = "Get a list of users.",
  ),
  endpoint(
    route = "/users/{id}",
    cli_route = "/users/{id}",
    cli_help = "Get a single user.",
  ),
  endpoint(
    route = "/users",
    method = "POST",
    result_ok_status = "CREATED",
    payload_struct = "UserCreatePayload",
    cli_route = "/users/create",
    cli_help = "Creates a new user.",
  ),
  endpoint(
    route = "/users/{id}",
    method = "PUT",
    payload_struct = "UserUpdatePayload",
    cli_route = "/users/{id}/edit",
    cli_help = "Modifies an existing user.",
  ),
  endpoint(
    route = "/users/{id}identities/{provider}",
    method = "DELETE",
    result_ok_status = "NO_CONTENT",
    cli_no_output,
    result_struct = "EmptyResponse",
    cli_route = "/users/{id}/identities/{provider}/delete",
    cli_help = "Deletes a user.",
  ),
  endpoint(
    route = "/users/{id}",
    method = "DELETE",
    result_ok_status = "NO_CONTENT",
    cli_no_output,
    result_struct = "EmptyResponse",
    query_struct = "UserDeletePayload",
    cli_route = "/users/{id}/delete",
    cli_help = "Deletes a user.",
  ),
  endpoint(
    route = "/users/{id}/follow",
    method = "POST",
    result_ok_status = "CREATED",
    cli_route = "/users/{id}/follow",
    cli_help = "Follow a user",
  ),
  endpoint(
    route = "/users/{id}/unfollow",
    method = "POST",
    result_ok_status = "CREATED",
    cli_route = "/users/{id}/unfollow",
    cli_help = "Unfollow a user",
  ),
  endpoint(
    route = "/users/{id}/followers",
    multiple_results,
    cli_route = "/users/{id}/followers",
    cli_help = "Get the followers of a user.",
  ),
  endpoint(
    route = "/users/{id}/following",
    multiple_results,
    cli_route = "/users/{id}/following",
    cli_help = "Get the list of users being followed.",
  ),
  endpoint(
    route = "/users/{id}/block",
    method = "POST",
    cli_route = "/users/{id}/block",
    cli_help = "Blocks the specified user.",
  ),
  endpoint(
    route = "/users/{id}/unblock",
    method = "POST",
    cli_route = "/users/{id}/unblock",
    cli_help = "Unblocks the specified user.",
  ),
  endpoint(
    route = "/users/{id}/desactivate",
    method = "POST",
    cli_route = "/users/{id}/desactivate",
    cli_help = "Desactivates the specified user.",
  ),
  endpoint(
    route = "/users/{id}/activate",
    method = "POST",
    cli_route = "/users/{id}/activate",
    cli_help = "Activates the specified user.",
  ),
  endpoint(
    route = "/users/{id}/approve",
    method = "POST",
    result_ok_status = "CREATED",
    cli_route = "/users/{id}/approve",
    cli_help = "Approves the specified user.",
  ),
  endpoint(
    route = "/users/{id}/ban",
    method = "POST",
    result_ok_status = "NO_CONTENT",
    cli_no_output,
    result_struct = "EmptyResponse",
    result_ko_status(status = "NOT_FOUND", message = "User cannot be found"),
    result_ko_status(
      status = "FORBIDDEN",
      message = "Trying to ban a user that is not active"
    ),
    cli_route = "/users/{id}/ban",
    cli_help = "Bans the specified user. Available only for administrator.",
  ),
  endpoint(
    route = "/users/{id}/unban",
    method = "POST",
    result_ok_status = "NO_CONTENT",
    cli_no_output,
    result_struct = "EmptyResponse",
    result_ko_status(status = "NOT_FOUND", message = "User cannot be found"),
    result_ko_status(
      status = "FORBIDDEN",
      message = "Trying to unban a user that is not banned."
    ),
    cli_route = "/users/{id}/unban",
    cli_help = "Unbans the specified user. Available only for administrator.",
  )
)]
#[api(
  endpoint(
    route = "/projects/{id}/issues/{iid}/participants",
    multiple_results,
    cli_route = "/projects/{id}/issues/{iid}/participants",
    cli_help = "Participants on issues",
  ),
  endpoint(
    route = "/projects/{id}/users",
    multiple_results,
    query_struct = "ProjectsUsersFilter",
    cli_route = "/projects/{id}/users",
    cli_help = "Get the users list of a project.",
  ),
  endpoint(
    route = "/projects/{id}/members",
    multiple_results,
    query_struct = "MembersFilter",
    cli_route = "/projects/{id}/members",
    cli_help = "Gets a list of project members viewable by the authenticated user.",
  ),
  endpoint(
    route = "/projects/{id}/members/{user_id}",
    cli_route = "/projects/{id}/members/{user_id}",
    cli_help = "Gets a member of a project.",
    cli_long_help = "Gets a member of a project. Returns only direct members and not inherited members through ancestor groups.",
  ),
  endpoint(
    route = "/projects/{id}/members/all",
    multiple_results,
    query_struct = "MembersFilter",
    cli_route = "/projects/{id}/members/all",
    cli_help = "Gets a list of project members viewable by the authenticated user, including inherited members and permissions through ancestor groups.",
  ),
  endpoint(
    route = "/projects/{id}/members/all/{user_id}",
    cli_route = "/projects/{id}/members/all/{user_id}",
    cli_help = "Gets a member of a project, including members inherited through ancestor groups.",
  ),
  endpoint(
    route = "/projects/{id}/members",
    method = "POST",
    result_ok_status = "CREATED",
    payload_struct = "MemberCreatePayload",
    cli_route = "/projects/{id}/members/add",
    cli_help = "Adds a member to a project.",
  ),
  endpoint(
    route = "/projects/{id}/members/{user_id}",
    method = "PUT",
    payload_struct = "MemberEditPayload",
    cli_route = "/projects/{id}/members/{user_id}/edit",
    cli_help = "Updates a member to a project.",
  ),
  endpoint(
    route = "/projects/{id}/members/{user_id}",
    method = "DELETE",
    result_ok_status = "NO_CONTENT",
    cli_no_output,
    result_struct = "EmptyResponse",
    cli_route = "/projects/{id}/members/{user_id}/delete",
    cli_help = "Removes a user from a project.",
  ),
  endpoint(
    route = "/groups/{id}/members",
    multiple_results,
    query_struct = "MembersFilter",
    cli_route = "/groups/{id}/members",
    cli_help = "Gets a list of group members viewable by the authenticated user.",
    cli_long_help = "Gets a list of group members viewable by the authenticated user. Returns only direct members and not inherited members through ancestors groups.",
  ),
  endpoint(
    route = "/groups/{id}/members/{user_id}",
    cli_route = "/groups/{id}/members/{user_id}",
    cli_help = "Gets a member of a group.",
    cli_long_help = "Gets a member of a group. Returns only direct members and not inherited members through ancestor groups.",
  ),
  endpoint(
    route = "/groups/{id}/members/all",
    multiple_results,
    query_struct = "MembersFilter",
    cli_route = "/groups/{id}/members/all",
    cli_help = "Gets a list of group or project members viewable by the authenticated user, including inherited members and permissions through ancestor groups.",
  ),
  endpoint(
    route = "/groups/{id}/pending_members",
    multiple_results,
    query_struct = "MembersPagination",
    cli_route = "/groups/{id}/pending_members",
    cli_help = "For a group and its subgroups and projects, get a list of all members in an awaiting state and those who are invited but do not have a GitLab account.",
  ),
  endpoint(
    route = "/groups/{id}/members/all/{user_id}",
    cli_route = "/groups/{id}/members/all/{user_id}",
    cli_help = "Gets a member of a group, including members inherited through ancestor groups.",
  ),
  endpoint(
    route = "/groups/{id}/members",
    method = "POST",
    result_ok_status = "CREATED",
    payload_struct = "MemberCreatePayload",
    cli_route = "/groups/{id}/members/add",
    cli_help = "Adds a member to a group.",
  ),
  endpoint(
    route = "/groups/{id}/members/{user_id}",
    method = "PUT",
    payload_struct = "MemberEditPayload",
    cli_route = "/groups/{id}/members/{user_id}/edit",
    cli_help = "Updates a member to a group.",
  ),
  endpoint(
    route = "/groups/{id}/members/{user_id}",
    method = "DELETE",
    result_ok_status = "NO_CONTENT",
    result_struct = "EmptyResponse",
    cli_route = "/groups/{id}/members/{user_id}/delete",
    cli_help = "Removes a user from a group.",
  ),
  endpoint(
    route = "/groups/{id}/provisioned_users",
    query_struct = "GroupsProvisionedUsersQuery",
    multiple_results,
    cli_route = "/groups/{id}/provisioned_users",
    cli_help = "Removes a user from a group.",
  )
)]
#[api(endpoint(
  route = "/projects/{id}/merge_requests/{iid}/participants",
  multiple_results,
  cli_route = "/projects/{id}/merge_requests/{iid}/participants",
  cli_help = "Get a list of merge request participants.",
))]
pub(crate) struct User {
  id: u32,
  name: String,
  username: String,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  email: Option<String>,
  state: String,
  #[api(table_skip)]
  web_url: String,
  #[api(table_skip)]
  avatar_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  created_at: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  expires_at: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  access_level: Option<u32>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  group_saml_identity: Option<Identity>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  bio: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  bio_html: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  location: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  public_email: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  skype: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  linkedin: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  twitter: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  website_url: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  organization: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  job_title: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  pronouns: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  bot: Option<bool>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  work_information: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  followers: Option<u32>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  following: Option<u32>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  last_sign_in_at: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  confirmed_at: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  last_activity_on: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  theme_id: Option<u32>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  color_scheme_id: Option<u32>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  projects_limit: Option<u32>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  current_sign_in_at: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  identities: Option<Vec<Identity>>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  can_create_group: Option<bool>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  can_create_project: Option<bool>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  two_factor_enabled: Option<bool>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  external: Option<bool>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  private_profile: Option<bool>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  commit_email: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  is_admin: Option<bool>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  note: Option<String>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  shared_runners_minutes_limit: Option<u32>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  extra_shared_runners_minutes_limit: Option<u32>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  is_auditor: Option<bool>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  using_license_seat: Option<bool>,
  #[api(table_skip)]
  #[serde(skip_serializing_if = "Option::is_none")]
  namespace_id: Option<u32>,
  // #[api(table_skip)]
  // #[serde(skip_serializing_if = "Option::is_none")]
  // created_by: Option<User>,
}

impl Display for User {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(&self.username)
  }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Identity {
  extern_uid: String,
  provider: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  saml_provider_id: Option<u32>,
}

#[derive(Debug, ApiInput, Serialize, Deserialize)]
#[api(no_input_file)]
pub(crate) struct UsersFilter {
  #[api(no_short, heading = "Filters", help = "lookup users by username")]
  #[serde(skip_serializing_if = "Option::is_none")]
  username: Option<String>,

  #[api(no_short, heading = "Filters", help = "Search for specific users.")]
  #[serde(skip_serializing_if = "Option::is_none")]
  search: Option<String>,

  #[api(
    no_short,
    heading = "Filters",
    help = "filter users based on the state active."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  active: Option<bool>,

  #[api(
    no_short,
    heading = "Filters",
    help = "filter users based on the state blocked"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  blocked: Option<bool>,

  #[api(
    no_short,
    heading = "Filters",
    help = "search for external users only with `external=true`.",
    long_help = "search for external users only with `external=true`. It does not support `external=false`.",
    possible_values = "true"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  external: Option<bool>,

  #[api(
    no_short,
    heading = "Filters",
    long = "exclude-external",
    help = "exclude external users from the users' list"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  exclude_external: Option<bool>,

  #[api(
    no_short,
    heading = "Sorting",
    long = "order-by",
    help = "Return users ordered by `id`, `name`, `username`, `created_at`, or `updated_at` fields.",
    long_help = "Return users ordered by `id`, `name`, `username`, `created_at`, or `updated_at` fields. Default is `id`",
    possible_values = "id,name,username,created_at,updated_at"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  order_by: Option<String>,

  #[api(
    no_short,
    heading = "Sorting",
    help = "Return users sorted in `asc` or `desc` order.",
    long_help = "Return users sorted in `asc` or `desc` order. Default is `desc`",
    possible_values = "asc,desc"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  sort: Option<String>,

  #[api(
    no_short,
    heading = "Filters",
    long = "two-factor",
    help = "Filter users by Two-factor authentication.",
    long_help = "Filter users by Two-factor authentication. Filter values are `enabled` or `disabled`. By default it returns all users",
    possible_values = "enabled,disabled"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  two_factor: Option<String>,

  #[api(
    no_short,
    heading = "Filters",
    long = "without-projects",
    help = "Filter users without projects.",
    long_help = "Filter users without projects. Default is false, which means that all users are returned, with and without projects."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  without_projects: Option<bool>,

  #[api(
    no_short,
    heading = "Filters",
    long = "admins",
    help = "Return only admin users.",
    long_help = "Return only admin users. Default is `false`"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  admins: Option<bool>,

  #[api(
    no_short,
    heading = "Filters",
    long = "saml-provider-id",
    help = "Return only users created by the specified SAML provider ID.",
    long_help = "Return only users created by the specified SAML provider ID. If not included, it returns all users."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  saml_provider_id: Option<u32>,

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

#[derive(Debug, ApiInput, Serialize, Deserialize)]
#[api(no_input_file)]
pub(crate) struct ProjectsUsersFilter {
  #[api(no_short, heading = "Filters", help = "Search for specific users.")]
  #[serde(skip_serializing_if = "Option::is_none")]
  search: Option<String>,

  #[api(
    no_short,
    long = "skip-users",
    heading = "Filters",
    help = "Filter out users with the specified IDs."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  skip_users: Option<Vec<u32>>,

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

#[derive(Debug, ApiInput, Serialize, Deserialize)]
#[api(no_input_file)]
pub(crate) struct MembersFilter {
  #[api(
    no_short,
    heading = "Filters",
    help = "A query string to search for members"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  query: Option<String>,

  #[api(
    no_short,
    long = "user-ids",
    heading = "Filters",
    help = "Filter the results on the given user IDs"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  user_ids: Option<Vec<u32>>,

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

#[derive(Debug, ApiInput, Serialize, Deserialize)]
#[api(no_input_file)]
pub(crate) struct MembersPagination {
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
pub(crate) struct MemberCreatePayload {
  #[api(
    no_short,
    long = "user-id",
    help = "The user ID of the new member or multiple IDs separated by commas"
  )]
  user_id: String,

  #[api(no_short, long = "access-level", help = "A valid access level")]
  access_level: u32,

  #[api(
    no_short,
    long = "expires-at",
    help = "A date string in the format `YEAR-MONTH-DAY`"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  expires_at: Option<String>,

  #[api(
    no_short,
    long = "invite-source",
    help = "The source of the invitation that starts the member creation process."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  invite_source: Option<String>,

  #[api(
    no_short,
    long = "areas-of-focus",
    help = "Areas the inviter wants the member to focus upon."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  areas_of_focus: Option<String>,
}

#[derive(Debug, Default, ApiInput, Serialize, Deserialize)]
pub(crate) struct MemberEditPayload {
  #[api(
    no_short,
    long = "expires-at",
    help = "A date string in the format `YEAR-MONTH-DAY`"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  expires_at: Option<String>,

  #[api(no_short, long = "access-level", help = "A valid access level")]
  #[serde(skip_serializing_if = "Option::is_none")]
  access_level: Option<u32>,

  #[api(
    no_short,
    long = "areas-of-focus",
    help = "Areas the inviter wants the member to focus upon."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  areas_of_focus: Option<String>,
}

#[derive(Debug, Default, ApiInput, Serialize, Deserialize)]
pub(crate) struct UserCreatePayload {
  #[api(no_short, help = "User is admin")]
  #[serde(skip_serializing_if = "Option::is_none")]
  admin: Option<bool>,

  #[api(no_short, help = "User is an auditor.")]
  #[serde(skip_serializing_if = "Option::is_none")]
  auditor: Option<bool>,

  #[api(no_short, help = "Image file for user's avatar")]
  #[serde(skip_serializing_if = "Option::is_none")]
  avatar: Option<String>,

  #[api(no_short, help = "User's biography")]
  #[serde(skip_serializing_if = "Option::is_none")]
  bio: Option<String>,

  #[api(no_short, long = "can-create-group", help = "User can create groups")]
  #[serde(skip_serializing_if = "Option::is_none")]
  can_create_group: Option<bool>,

  #[api(
    no_short,
    long = "color-scheme-id",
    help = "User's color scheme for the file viewer"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  color_scheme_id: Option<String>,

  #[api(no_short, help = "Email")]
  email: String,

  #[api(no_short, long = "extern-uid", help = "External UID")]
  #[serde(skip_serializing_if = "Option::is_none")]
  extern_uid: Option<String>,

  #[api(no_short, help = "Flags the user as external")]
  #[serde(skip_serializing_if = "Option::is_none")]
  external: Option<bool>,

  #[api(
    no_short,
    long = "extra-shared-runners-minutes-limit",
    help = "Extra pipeline minutes quota for this user"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  extra_shared_runners_minutes_limit: Option<String>,

  #[api(
    no_short,
    long = "force-random-password",
    help = "Set user password to a random value"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  force_random_password: Option<bool>,

  #[api(
    no_short,
    long = "group-id-for-saml",
    help = "ID of group where SAML has been configured"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  group_id_for_saml: Option<String>,

  #[api(no_short, help = "LinkedIn")]
  #[serde(skip_serializing_if = "Option::is_none")]
  linkedin: Option<String>,

  #[api(no_short, help = "User's location")]
  #[serde(skip_serializing_if = "Option::is_none")]
  location: Option<String>,

  #[api(no_short, help = "Name")]
  name: String,

  #[api(no_short, help = "Admin notes for this user")]
  #[serde(skip_serializing_if = "Option::is_none")]
  note: Option<String>,

  #[api(no_short, help = "Organization name")]
  #[serde(skip_serializing_if = "Option::is_none")]
  organization: Option<String>,

  #[api(no_short, help = "Password")]
  #[serde(skip_serializing_if = "Option::is_none")]
  password: Option<String>,

  #[api(no_short, long = "private-profile", help = "User's profile is private")]
  #[serde(skip_serializing_if = "Option::is_none")]
  private_profile: Option<bool>,

  #[api(
    no_short,
    long = "projects-limit",
    help = "Number of projects user can create"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  projects_limit: Option<u32>,

  #[api(no_short, help = "External provider name")]
  #[serde(skip_serializing_if = "Option::is_none")]
  provider: Option<String>,

  #[api(no_short, long = "reset-password", help = "Send user password reset")]
  #[serde(skip_serializing_if = "Option::is_none")]
  reset_password: Option<String>,

  #[api(
    no_short,
    long = "shared-runners-minutes-limit",
    help = "Pipeline minutes quota for this user.",
    long_help = "Pipeline minutes quota for this user (included in plan). Can be `nil` (default; inherit system default), 0 (unlimited) or > 0"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  shared_runners_minutes_limit: Option<String>,

  #[api(no_short, long = "skip-confirmation", help = "Skip confirmation")]
  #[serde(skip_serializing_if = "Option::is_none")]
  skip_confirmation: Option<bool>,

  #[api(no_short, help = "Skype ID")]
  #[serde(skip_serializing_if = "Option::is_none")]
  skype: Option<String>,

  #[api(no_short, long = "theme-id", help = "The GitLab theme for the user")]
  #[serde(skip_serializing_if = "Option::is_none")]
  theme_id: Option<String>,

  #[api(no_short, help = "Twitter account")]
  #[serde(skip_serializing_if = "Option::is_none")]
  twitter: Option<String>,

  #[api(no_short, help = "Username")]
  username: String,

  #[api(
    no_short,
    long = "view-diffs-file-by-file",
    help = "Flag indicating the user sees only one file diff per page"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  view_diffs_file_by_file: Option<bool>,

  #[api(no_short, long = "website-url", help = "Website URL")]
  #[serde(skip_serializing_if = "Option::is_none")]
  website_url: Option<String>,
}

#[derive(Debug, Default, ApiInput, Serialize, Deserialize)]
pub(crate) struct UserUpdatePayload {
  #[api(no_short, help = "User is admin")]
  #[serde(skip_serializing_if = "Option::is_none")]
  admin: Option<bool>,

  #[api(no_short, help = "User is an auditor.")]
  #[serde(skip_serializing_if = "Option::is_none")]
  auditor: Option<bool>,

  #[api(no_short, help = "Image file for user's avatar")]
  #[serde(skip_serializing_if = "Option::is_none")]
  avatar: Option<String>,

  #[api(no_short, help = "User's biography")]
  #[serde(skip_serializing_if = "Option::is_none")]
  bio: Option<String>,

  #[api(no_short, long = "can-create-group", help = "User can create groups")]
  #[serde(skip_serializing_if = "Option::is_none")]
  can_create_group: Option<bool>,

  #[api(
    no_short,
    long = "color-scheme-id",
    help = "User's color scheme for the file viewer"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  color_scheme_id: Option<String>,

  #[api(no_short, help = "Email")]
  email: String,

  #[api(no_short, long = "extern-uid", help = "External UID")]
  #[serde(skip_serializing_if = "Option::is_none")]
  extern_uid: Option<String>,

  #[api(no_short, help = "Flags the user as external")]
  #[serde(skip_serializing_if = "Option::is_none")]
  external: Option<bool>,

  #[api(
    no_short,
    long = "extra-shared-runners-minutes-limit",
    help = "Extra pipeline minutes quota for this user"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  extra_shared_runners_minutes_limit: Option<String>,

  #[api(
    no_short,
    long = "group-id-for-saml",
    help = "ID of group where SAML has been configured"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  group_id_for_saml: Option<String>,

  #[api(no_short, help = "LinkedIn")]
  #[serde(skip_serializing_if = "Option::is_none")]
  linkedin: Option<String>,

  #[api(no_short, help = "User's location")]
  #[serde(skip_serializing_if = "Option::is_none")]
  location: Option<String>,

  #[api(no_short, help = "Name")]
  name: String,

  #[api(no_short, help = "Admin notes for this user")]
  #[serde(skip_serializing_if = "Option::is_none")]
  note: Option<String>,

  #[api(no_short, help = "Organization name")]
  #[serde(skip_serializing_if = "Option::is_none")]
  organization: Option<String>,

  #[api(no_short, help = "Password")]
  #[serde(skip_serializing_if = "Option::is_none")]
  password: Option<String>,

  #[api(no_short, long = "private-profile", help = "User's profile is private")]
  #[serde(skip_serializing_if = "Option::is_none")]
  private_profile: Option<bool>,

  #[api(
    no_short,
    long = "projects-limit",
    help = "Number of projects user can create"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  projects_limit: Option<u32>,

  #[api(no_short, help = "External provider name")]
  #[serde(skip_serializing_if = "Option::is_none")]
  provider: Option<String>,

  #[api(
    no_short,
    long = "public-email",
    help = "The public email of the user.",
    long_help = "The public email of the user. (must be already verified)"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  public_email: Option<String>,

  #[api(
    no_short,
    long = "shared-runners-minutes-limit",
    help = "Pipeline minutes quota for this user.",
    long_help = "Pipeline minutes quota for this user (included in plan). Can be `nil` (default; inherit system default), 0 (unlimited) or > 0"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  shared_runners_minutes_limit: Option<String>,

  #[api(no_short, long = "skip-confirmation", help = "Skip confirmation")]
  #[serde(skip_serializing_if = "Option::is_none")]
  skip_confirmation: Option<bool>,

  #[api(no_short, help = "Skype ID")]
  #[serde(skip_serializing_if = "Option::is_none")]
  skype: Option<String>,

  #[api(no_short, long = "theme-id", help = "The GitLab theme for the user")]
  #[serde(skip_serializing_if = "Option::is_none")]
  theme_id: Option<String>,

  #[api(no_short, help = "Twitter account")]
  #[serde(skip_serializing_if = "Option::is_none")]
  twitter: Option<String>,

  #[api(no_short, help = "Username")]
  username: String,

  #[api(
    no_short,
    long = "view-diffs-file-by-file",
    help = "Flag indicating the user sees only one file diff per page"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  view_diffs_file_by_file: Option<bool>,

  #[api(no_short, long = "website-url", help = "Website URL")]
  #[serde(skip_serializing_if = "Option::is_none")]
  website_url: Option<String>,
}

#[derive(Debug, ApiInput, Serialize, Deserialize)]
#[api(no_input_file)]
pub(crate) struct UserDeletePayload {
  #[api(
    no_short,
    long = "hard-delete",
    help = "f true, contributions that would usually be moved to the `ghost user` are deleted instead, as well as groups owned solely by this user."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  hard_delete: Option<bool>,
}

#[derive(Debug, ApiInput, Serialize, Deserialize)]
#[api(no_input_file)]
pub(crate) struct CurrentUserSelector {
  #[api(no_short, help = "the ID of a user to make the call in their place")]
  #[serde(skip_serializing_if = "Option::is_none")]
  sudo: Option<String>,
}

#[derive(Debug, ApiInput, Serialize, Deserialize)]
#[api(no_input_file)]
pub(crate) struct GroupsProvisionedUsersQuery {
  #[api(
    no_short,
    heading = "Filters",
    help = "Return single user with a specific username"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  username: Option<String>,

  #[api(
    no_short,
    heading = "Filters",
    help = "Search users by name, email, username"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  search: Option<String>,

  #[api(no_short, heading = "Selectors", help = "Return only active users")]
  #[serde(skip_serializing_if = "Option::is_none")]
  active: Option<bool>,

  #[api(no_short, heading = "Selectors", help = "Return only blocked users")]
  #[serde(skip_serializing_if = "Option::is_none")]
  blocked: Option<bool>,

  #[api(
    no_short,
    heading = "Filters",
    long = "created-after",
    help = "Return users created after the specified time"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  created_after: Option<String>,

  #[api(
    no_short,
    heading = "Filters",
    long = "created-before",
    help = "Return users created before the specified time"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  created_before: Option<String>,
}
