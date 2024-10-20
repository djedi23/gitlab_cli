use crud_api::ApiInput;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, ApiInput, Serialize, Deserialize)]
pub(crate) struct GroupsUpdatePayload {
  #[api(no_short, help = "The name of the group.")]
  #[serde(skip_serializing_if = "Option::is_none")]
  name: Option<String>,

  #[api(no_short, help = "The path of the group.")]
  #[serde(skip_serializing_if = "Option::is_none")]
  path: Option<String>,

  #[api(no_short, help = "The description of the group.")]
  #[serde(skip_serializing_if = "Option::is_none")]
  description: Option<String>,

  #[api(
    no_short,
    long = "membership-lock",
    help = "Prevent adding new members to project membership within this group."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  membership_lock: Option<bool>,

  #[api(
    no_short,
    long = "visibility",
    help = "The visibility level of the group.",
    long_help = "The visibility level of the group. Can be `private`, `internal`, or `public`.",
    possible_values = "private,internal,public"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  visibility: Option<String>,

  #[api(
    no_short,
    long = "share-with-group-lock",
    help = "Prevent sharing a project with another group within this group."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  share_with_group_lock: Option<bool>,

  #[api(
    no_short,
    long = "require-two-factor-authentication",
    help = "Require all users in this group to setup Two-factor authentication."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  require_two_factor_authentication: Option<bool>,

  #[api(
    no_short,
    long = "two-factor-grace-period",
    help = "Time before Two-factor authentication is enforced (in hours)."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  two_factor_grace_period: Option<u32>,

  #[api(
    no_short,
    long = "project-creation-level",
    help = "Determine if developers can create projects in the group.",
    long_help = "Determine if developers can create projects in the group. Can be `noone` (No one), `maintainer` (Maintainers), or `developer` (Developers + Maintainers).",
    possible_values = "noone,maintainer,developer"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  project_creation_level: Option<String>,

  #[api(
    no_short,
    long = "auto-devops-enabled",
    help = "Default to Auto DevOps pipeline for all projects within this group."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  auto_devops_enabled: Option<bool>,

  #[api(
    no_short,
    long = "subgroup-creation-level",
    help = "Allowed to create subgroups.",
    long_help = "Allowed to create subgroups. Can be `owner` (Owners), or `maintainer` (Maintainers).",
    possible_values = "owner,maintainer"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  subgroup_creation_level: Option<String>,

  #[api(
    no_short,
    long = "emails-disabled",
    help = "Disable email notifications"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  emails_disabled: Option<bool>,

  // #[api(no_short, help = "Image file for avatar of the group.")]
  // #[serde(skip_serializing_if = "Option::is_none")]
  // avatar: Option<String>,
  #[api(
    no_short,
    long = "mentions-disabled",
    help = "Disable the capability of a group from getting mentioned"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  mentions_disabled: Option<bool>,

  #[api(
    no_short,
    long = "lfs-enabled",
    help = "Enable/disable Large File Storage (LFS) for the projects in this group."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  lfs_enabled: Option<bool>,

  #[api(
    no_short,
    long = "request-access-enabled",
    help = "Allow users to request member access."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  request_access_enabled: Option<bool>,

  // #[api(
  //   no_short,
  //   long = "parent-id",
  //   help = "The parent group ID for creating nested group."
  // )]
  // #[serde(skip_serializing_if = "Option::is_none")]
  // parent_id: Option<u32>,
  #[api(
    no_short,
    long = "default-branch-protection",
    help = "See Options for default_branch_protection."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  default_branch_protection: Option<u32>,

  #[api(
    no_short,
    long = "file-template-project-id",
    help = "The ID of a project to load custom file templates from."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  file_template_project_id: Option<u32>,

  #[api(
    no_short,
    long = "shared-runners-minutes-limit",
    help = "Pipeline minutes quota for this group (included in plan).",
    long_help = "Pipeline minutes quota for this group (included in plan). Can be nil (default; inherit system default), 0 (unlimited) or > 0"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  shared_runners_minutes_limit: Option<u32>,

  #[api(
    no_short,
    long = "extra-shared-runners-minutes-limit",
    help = "Extra pipeline minutes quota for this group (purchased in addition to the minutes included in the plan)."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  extra_shared_runners_minutes_limit: Option<u32>,

  #[api(
    no_short,
    long = "prevent-forking-outside-group",
    help = "When enabled, users can not fork projects from this group to external namespaces"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  prevent_forking_outside_group: Option<bool>,

  #[api(
    no_short,
    long = "shared-runners-setting",
    help = "Enable or disable shared runners for a group's subgroups and projects.",
    possible_values = "enabled,disabled_with_override,disabled_and_unoverridable"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  shared_runners_setting: Option<String>,

  #[api(
    no_short,
    long = "prevent-sharing-groups-outside-hierarchy",
    help = "See Prevent group sharing outside the group hierarchy.",
    long_help = "See Prevent group sharing outside the group hierarchy. This attribute is only available on top-level groups."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  prevent_sharing_groups_outside_hierarchy: Option<bool>,

  #[api(
    no_short,
    long = "unique-project-download-limit",
    help = "Maximum number of unique projects a user can download in the specified time period before they are banned.",
    long_help = "Maximum number of unique projects a user can download in the specified time period before they are banned. Available only on top-level groups. Default: 0, Maximum: 10,000."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  unique_project_download_limit: Option<u32>,

  #[api(
    no_short,
    long = "unique-project-download-limit-interval-in-seconds",
    help = "Time period during which a user can download a maximum amount of projects before they are banned.",
    long_help = "Time period during which a user can download a maximum amount of projects before they are banned. Available only on top-level groups. Default: 0, Maximum: 864,000 seconds (10 days)."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  unique_project_download_limit_interval_in_seconds: Option<u32>,

  #[api(
    no_short,
    long = "unique-project-download-limit-allowlist",
    help = "List of usernames excluded from the unique project download limit.",
    long_help = "List of usernames excluded from the unique project download limit. Available only on top-level groups. Default: [], Maximum: 100 usernames."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  unique_project_download_limit_allowlist: Option<Vec<String>>,

  #[api(
    no_short,
    long = "auto-ban-user-on-excessive-projects-download",
    help = "When enabled, users are automatically banned from the group when they download more than the maximum number of unique projects specified by unique_project_download_limit and unique_project_download_limit_interval_in_seconds."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  auto_ban_user_on_excessive_projects_download: Option<bool>,

  #[api(
    no_short,
    long = "ip-restriction-ranges",
    help = "Comma-separated list of IP addresses or subnet masks to restrict group access."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  ip_restriction_ranges: Option<String>,
}
