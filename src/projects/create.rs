use crud_api::ApiInput;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, ApiInput, Serialize, Deserialize)]
pub(crate) struct ProjectsCreatePayload {
  #[api(
    no_short,
    help = "The name of the new project.",
    long_help = "The name of the new project. Equals path if not provided"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  name: Option<String>,

  #[api(
    no_short,
    help = "Repository name for new project.",
    long_help = "Repository name for new project. Generated based on name if not provided (generated as lowercase with dashes)."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  path: Option<String>,

  #[api(
    no_short,
    long = "allow-merge-on-skipped-pipeline",
    help = "Set whether or not merge requests can be merged with skipped jobs."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  allow_merge_on_skipped_pipeline: Option<bool>,

  #[api(
    no_short,
    long = "analytics-access-level",
    help = "One of disabled, private or enabled",
    possible_values = "disabled,private,enabled"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  analytics_access_level: Option<String>,

  #[api(
    no_short,
    long = "approvals-before-merge",
    help = "How many approvers should approve merge requests by default."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  approvals_before_merge: Option<u32>,

  #[api(
    no_short,
    long = "auto-cancel-pending-pipelines",
    help = "Auto-cancel pending pipelines.",
    long_help = "Auto-cancel pending pipelines. This isn't a boolean, but enabled/disabled.",
    possible_values = "enabled,disabled"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  auto_cancel_pending_pipelines: Option<String>,

  #[api(
    no_short,
    long = "auto-devops-deploy-strategy",
    help = "Auto Deploy strategy.",
    possible_values = "continuous,manual,timed_incremental"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  auto_devops_deploy_strategy: Option<String>,

  #[api(
    no_short,
    long = "auto-devops-enabled",
    help = "Enable Auto DevOps for this project."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  auto_devops_enabled: Option<bool>,

  #[api(
    no_short,
    long = "autoclose-referenced-issues",
    help = "Set whether auto-closing referenced issues on default branch."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  autoclose_referenced_issues: Option<bool>,
  // #[api(
  //     no_short,
  //     help = "Image file for avatar of the project.",
  // )]
  //   avatar: Option<String>,
  #[api(
    no_short,
    long = "build-coverage-regex",
    help = "Test coverage parsing."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  build_coverage_regex: Option<String>,

  #[api(
    no_short,
    long = "build-git-strategy",
    help = "The Git strategy.",
    long_help = "The Git strategy. Defaults to `fetch`."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  build_git_strategy: Option<String>,

  #[api(
    no_short,
    long = "build-timeout",
    help = "The maximum amount of time, in seconds, that a job can run."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  build_timeout: Option<u32>,

  #[api(
    no_short,
    long = "builds-access-level",
    help = "One of disabled, private, or enabled.",
    possible_values = "disabled,private,enabled"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  builds_access_level: Option<String>,

  #[api(
    no_short,
    long = "ci-config-path",
    help = "The path to CI configuration file."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  ci_config_path: Option<String>,

  // #[api(
  //   no_short,
  //   long = "container-expiration-policy-attributes",
  //   help = "Update the image cleanup policy for this project.",
  //   long_help = "Update the image cleanup policy for this project. Accepts: cadence (string), keep_n (integer), older_than (string), name_regex (string), name_regex_delete (string), name_regex_keep (string), enabled (boolean). Valid values for cadence are: 1d (every day), 7d (every week), 14d (every two weeks), 1month (every month), or 3month (every quarter)."
  // )]
  // #[serde(skip_serializing_if = "Option::is_none")]
  // container_expiration_policy_attributes: Option<String>,
  #[api(
    no_short,
    long = "container-registry-access-level",
    help = "Set visibility of container registry, for this project.",
    long_help = "Set visibility of container registry, for this project, to one of disabled, private or enabled.",
    possible_values = "disabled,private,enabled"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  container_registry_access_level: Option<String>,

  #[api(
    no_short,
    long = "default-branch",
    help = "The default branch name.",
    long_help = "The default branch name. Requires `initialize_with_readme` to be `true`."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  default_branch: Option<String>,

  #[api(no_short, help = "Short project description.")]
  #[serde(skip_serializing_if = "Option::is_none")]
  description: Option<String>,

  #[api(
    no_short,
    long = "emails-disabled",
    help = "Disable email notifications."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  emails_disabled: Option<bool>,

  #[api(
    no_short,
    long = "external-authorization-classification-label",
    help = "The classification label for the project."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  external_authorization_classification_label: Option<String>,

  #[api(
    no_short,
    long = "forking-access-level",
    help = "One of `disabled`, `private`, or `enabled`.",
    possible_values = "disabled,private,enabled"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  forking_access_level: Option<String>,

  #[api(
    no_short,
    long = "group-with-project-templates-id",
    help = "For group-level custom templates, specifies ID of group from which all the custom project templates are sourced.",
    long_help = "For group-level custom templates, specifies ID of group from which all the custom project templates are sourced. Leave empty for instance-level templates. Requires `use_custom_template` to be true."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  group_with_project_templates_id: Option<u32>,

  #[api(no_short, long = "import-url", help = "URL to import repository from.")]
  #[serde(skip_serializing_if = "Option::is_none")]
  import_url: Option<String>,

  #[api(no_short, long = "initialize-with-readme", help = "false by default.")]
  #[serde(skip_serializing_if = "Option::is_none")]
  initialize_with_readme: Option<bool>,

  #[api(
    no_short,
    long = "issues-access-level",
    help = "One of `disabled`, `private`, or `enabled`.",
    possible_values = "disabled,private,enabled"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  issues_access_level: Option<String>,

  #[api(no_short, long = "lfs-enabled", help = "Enable LFS.")]
  #[serde(skip_serializing_if = "Option::is_none")]
  lfs_enabled: Option<bool>,

  #[api(
    no_short,
    long = "merge-commit-template",
    help = "Template used to create merge commit message in merge requests."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  merge_commit_template: Option<String>,

  #[api(no_short, long = "merge-method", help = "Set the merge method used.")]
  #[serde(skip_serializing_if = "Option::is_none")]
  merge_method: Option<String>,

  #[api(
    no_short,
    long = "merge-requests-access-level",
    help = "One of `disabled`, `private`, or `enabled`.",
    possible_values = "disabled,private,enabled"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  merge_requests_access_level: Option<String>,

  #[api(
    no_short,
    long = "mirror-trigger-builds",
    help = "Pull mirroring triggers builds."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  mirror_trigger_builds: Option<bool>,

  #[api(no_short, help = "Enables pull mirroring in a project.")]
  #[serde(skip_serializing_if = "Option::is_none")]
  mirror: Option<bool>,

  #[api(
    no_short,
    long = "namespace-id",
    help = "Namespace for the new project.",
    long_help = "Namespace for the new project. defaults to the current user's namespace."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  namespace_id: Option<u32>,

  #[api(
    no_short,
    long = "operations-access-level",
    help = "One of `disabled`, `private`, or `enabled`.",
    long_help = "One of `disabled`, `private`, or `enabled`.",
    possible_values = "disabled,private,enabled"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  operations_access_level: Option<String>,

  #[api(
    no_short,
    long = "only-allow-merge-if-all-discussions-are-resolved",
    help = "Set whether merge requests can only be merged when all the discussions are resolved."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  only_allow_merge_if_all_discussions_are_resolved: Option<bool>,

  #[api(
    no_short,
    long = "only-allow-merge-if-pipeline-succeeds",
    help = "Set whether merge requests can only be merged with successful pipelines.",
    long_help = "Set whether merge requests can only be merged with successful pipelines. This setting is named Pipelines must succeed in the project settings."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  only_allow_merge_if_pipeline_succeeds: Option<bool>,

  #[api(
    no_short,
    long = "packages-enabled",
    help = "Enable or disable packages repository feature."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  packages_enabled: Option<bool>,

  #[api(
    no_short,
    long = "pages-access-level",
    help = "One of `disabled`, `private`, `enabled`, or `public`.",
    long_help = "One of `disabled`, `private`, `enabled`, or `public`.",
    possible_values = "disabled,private,enabled,public"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  pages_access_level: Option<String>,

  #[api(
    no_short,
    long = "requirements-access-level",
    help = "One of `disabled`, `private`, `enabled`, or `public`.",
    possible_values = "disabled,private,enabled,public"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  requirements_access_level: Option<String>,

  #[api(
    no_short,
    long = "printing-merge-request-link-enabled",
    help = "Show link to create/view merge request when pushing from the command line."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  printing_merge_request_link_enabled: Option<bool>,

  #[api(
    no_short,
    long = "public-builds",
    help = "If `true`, jobs can be viewed by non-project members."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  public_builds: Option<bool>,

  #[api(
    no_short,
    long = "remove-source-branch-after-merge",
    help = "Enable `Delete source branch` option by default for all new merge requests."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  remove_source_branch_after_merge: Option<bool>,

  #[api(
    no_short,
    long = "repository-access-level",
    help = "One of `disabled`, `private`, or `enabled`.",
    possible_values = "disabled,private,enabled"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  repository_access_level: Option<String>,

  #[api(
    no_short,
    long = "repository-storage",
    help = "Which storage shard the repository is on.",
    long_help = "Which storage shard the repository is on. (admins only)"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  repository_storage: Option<String>,

  #[api(
    no_short,
    long = "request-access-enabled",
    help = "Allow users to request member access."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  request_access_enabled: Option<bool>,

  #[api(
    no_short,
    long = "resolve-outdated-diff-discussions",
    help = "Automatically resolve merge request diffs discussions on lines changed with a push."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  resolve_outdated_diff_discussions: Option<bool>,

  #[api(
    no_short,
    long = "shared-runners-enabled",
    help = "Enable shared runners for this project."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  shared_runners_enabled: Option<bool>,

  #[api(
    no_short,
    long = "show-default-award-emojis",
    help = "Show default award emojis."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  show_default_award_emojis: Option<bool>,

  #[api(
    no_short,
    long = "snippets-access-level",
    help = "One of `disabled`, `private`, or `enabled`."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  snippets_access_level: Option<String>,

  #[api(
    no_short,
    long = "squash-commit-template",
    help = "Template used to create squash commit message in merge requests."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  squash_commit_template: Option<String>,

  #[api(
    no_short,
    long = "squash-option",
    help = "One of `never`, `always`, `default_on`, or `default_off`.",
    possible_values = "never,always,default_on,default_off"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  squash_option: Option<String>,

  #[api(
    no_short,
    long = "template-name",
    help = "When used without `use_custom_template`, name of a built-in project template.",
    long_help = "When used without `use_custom_template`, name of a built-in project template. When used with `use_custom_template`, name of a custom project template."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  template_name: Option<String>,

  #[api(
    no_short,
    long = "template-project-id",
    help = "When used with `use_custom_template`, project ID of a custom project template.",
    long_help = "When used with `use_custom_template`, project ID of a custom project template. This is preferable to using `template_name` since `template_name` may be ambiguous."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  template_project_id: Option<u32>,

  #[api(
    no_short,
    long = "topics",
    help = "The list of topics for a project; put array of topics, that should be finally assigned to a project.",
    long_help = "The list of topics for a project; put array of topics, that should be finally assigned to a project."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  topics: Option<Vec<String>>,

  #[api(
    no_short,
    long = "use-custom-template",
    help = "Use either custom instance or group (with `group_with_project_templates_id`) project template."
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  use_custom_template: Option<bool>,

  #[api(
    no_short,
    long = "visibility",
    help = "Values for the project visibility level.",
    long_help = "Values for the project visibility level are:
- private: project access must be granted explicitly for each user.
- internal: the project can be cloned by any signed-in user except external users.
- public: the project can be accessed without any authentication.",
    possible_values = "private,internal,public"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  visibility: Option<String>,

  #[api(
    no_short,
    long = "wiki-access-level",
    help = "One of `disabled`, `private`, or `enabled`.",
    possible_values = "disabled,private,enabled"
  )]
  #[serde(skip_serializing_if = "Option::is_none")]
  wiki_access_level: Option<String>,
}
