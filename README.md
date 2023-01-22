# Gitlab CLI

`gitlab-cli` is a command line wrapper for _gitlab_'s API.

## Quick start

This application have an integrated help:
```shell
gitlab-cli help
```

This application have shell completions. 
You can install the completion for `bash` with this snippet:
```shell
mkdir -p $HOME/.local/share/bash-completion/completions
gitlab-cli completion bash > $HOME/.local/share/bash-completion/completions/gitlab-cli
source $HOME/.local/share/bash-completion/completions/gitlab-cli
```

You can access to your private resources with a _[personal access token](https://docs.gitlab.com/ee/user/profile/personal_access_tokens.html#create-a-personal-access-token)_.
```shell
gitlab-cli -t PAT_TOKEN ...
```

## Short Examples:

List the projects related to gitlab.
``` shell
gitlab-cli projects --search gitlab
```
``` shell
id       path_with_namespace                            description                                        last_activity_at         
[ ... ]

```

List the issues of a projects (gitlal/gitlab):
``` shell
gitlab-cli projects 1025 issues
```
``` shell
id        state  project_id iid    title                                              updated_at          created_at          labels                                             
[ ... ]
```


Create an export of the 2 last projects updated you are member of.
```shell
gitlab-cli projects --membership true --order-by updated_at -f json --per-page 2 | jq -r '.[].id | @sh' | xargs -I{} gitlab-cli projects {} export create
```
Then download the previoudly created exports. _You may reach the limit of gitlab..._
```shell
gitlab-cli projects --membership true --order-by updated_at -f json --per-page 2 | jq -r '.[].id | @sh' | xargs -I{} gitlab-cli projects {} export download -o {}.tar
```

## Installation

### From source

Fisrt, install [rust dev tools](https://www.rust-lang.org/tools/install) like `rustup`.

Clone the repository
```shell
git clone https://github.com/djedi23/gitlab_cli
```

Install the application
```shell
cargo install --path .
```

Install the completions. For Bash:
``` shell
gitlab-cli completion bash > ~/.local/share/bash-completion/completions/gitlab-cli
```

## Configuration
`gitlab-cli` can configured, by order of precedence:
- by arguments in the command line
- by environment variables
- by a config file
- by default values

### CLI

These arguments are available before the first command:
```
    -b, --base-url <base_url>        Override the base url
    -t, --auth-token <auth_token>    Authorization token
```

### Environment
Two environment variables can be set:
``` shell
GITLAB_BASE_URL=https://gitlab.example.com/api/v4
GITLAB_AUTH_TOKEN=""
```
### Configuration file
You can edit the file `$HOME/.config/gitlab-cli/settings.toml`.
The format is:
```
base_url = "https://www.gitlab.com/api/v4"
auth_token = ""
```

## Features

- output formats: human, json, yaml, toml
- payloads can be created by the CLI or read from a file or stdin.

## Implemented Resources
### Project resources
|       | Available endpoints                                                                                                                                                                                   |
|-------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| ☐     | `/projects/:id/access_requests` (also available for groups)                                                                                                                                           |
| ☐     | `/projects/:id/access_tokens` (also available for groups)                                                                                                                                             |
| ☐     | `/projects/:id/cluster_agents`                                                                                                                                                                        |
| ☐     | `/projects/:id/issues/.../award_emoji`, `/projects/:id/merge_requests/.../award_emoji`, `/projects/:id/snippets/.../award_emoji`                                                                      |
| **☑** | `/projects/:id/repository/branches/`, `/projects/:id/repository/merged_branches`                                                                                                                      |
| ☐     | `/projects/:id/repository/commits`, `/projects/:id/statuses`                                                                                                                                          |
| ☐     | `/projects/:id/registry/repositories`                                                                                                                                                                 |
| ☐     | `/projects/:id/custom_attributes` (also available for groups and users)                                                                                                                               |
| ☐     | `/projects/:id/packages/composer` (also available for groups)                                                                                                                                         |
| ☐     | `/projects/:id/packages/conan` (also available standalone)                                                                                                                                            |
| ☐     | `/projects/:id/debian_distributions` (also available for groups)                                                                                                                                      |
| ☐     | `/projects/:id/packages/debian` (also available for groups)                                                                                                                                           |
| ☐     | `/projects/:id/dependencies`                                                                                                                                                                          |
| ☐     | `/projects/:id/deploy_keys` (also available standalone)                                                                                                                                               |
| ☐     | `/projects/:id/deploy_tokens` (also available for groups and standalone)                                                                                                                              |
| ☐     | `/projects/:id/deployments`                                                                                                                                                                           |
| ☐     | `/projects/:id/issues/.../discussions`, `/projects/:id/snippets/.../discussions`, `/projects/:id/merge_requests/.../discussions`, `/projects/:id/commits/.../discussions` (also available for groups) |
| ☐     | `/projects/:id/environments`                                                                                                                                                                          |
| ☐     | `/projects/:id/error_tracking/settings`                                                                                                                                                               |
| ☐     | `/projects/:id/events` (also available for users and standalone)                                                                                                                                      |
| ☐     | `/projects/:id/feature_flags_user_lists`                                                                                                                                                              |
| ☐     | `/projects/:id/feature_flags`                                                                                                                                                                         |
| ☐     | `/projects/:id/freeze_periods`                                                                                                                                                                        |
| ☐     | `/projects/:id/packages/go`                                                                                                                                                                           |
| ☐     | `/projects/:id/packages/helm_repository`                                                                                                                                                              |
| ☐     | `/projects/:id/integrations`                                                                                                                                                                          |
| ☐     | `/projects/:id/invitations` (also available for groups)                                                                                                                                               |
| ☐     | `/projects/:id/boards`                                                                                                                                                                                |
| ☐     | `/projects/:id/issues/.../links`                                                                                                                                                                      |
| ☐     | `/projects/:id/issues_statistics` (also available for groups and standalone)                                                                                                                          |
| **☑** | `/projects/:id/issues` (also available for groups and standalone)                                                                                                                                     |
| ☐     | `/projects/:id/iterations` (also available for groups)                                                                                                                                                |
| **☑** | `/projects/:id/jobs`, `/projects/:id/pipelines/.../jobs`                                                                                                                                              |
| ☐     | `/projects/:id/jobs/:job_id/artifacts`                                                                                                                                                                |
| **☑** | `/projects/:id/labels`                                                                                                                                                                                |
| ☐     | `/projects/:id/managed_licenses`                                                                                                                                                                      |
| ☐     | `/projects/:id/packages/maven` (also available for groups and standalone)                                                                                                                             |
| **☑** | `/projects/:id/members` (also available for groups)                                                                                                                                                   |
| ☐     | `/projects/:id/approvals`, `/projects/:id/merge_requests/.../approvals`                                                                                                                               |
| **☑** | `/projects/:id/merge_requests` (also available for groups and standalone)                                                                                                                             |
| ☐     | `/projects/:id/merge_trains`                                                                                                                                                                          |
| ☐     | `/metadata`                                                                                                                                                                                           |
| ☐     | `/projects/:id/issues/.../notes`, `/projects/:id/snippets/.../notes`, `/projects/:id/merge_requests/.../notes` (also available for groups)                                                            |
| ☐     | `/projects/:id/notification_settings` (also available for groups and standalone)                                                                                                                      |
| ☐     | `/projects/:id/packages/npm`                                                                                                                                                                          |
| ☐     | `/projects/:id/packages/nuget` (also available for groups)                                                                                                                                            |
| ☐     | `/projects/:id/packages`                                                                                                                                                                              |
| ☐     | `/projects/:id/pages` (also available standalone)                                                                                                                                                     |
| ☐     | `/projects/:id/pipeline_schedules`                                                                                                                                                                    |
| ☐     | `/projects/:id/triggers`                                                                                                                                                                              |
| **☑** | `/projects/:id/pipelines`                                                                                                                                                                             |
| **☑** | `/projects/:id/badges`                                                                                                                                                                                |
| ☐     | `/projects/:id/clusters`                                                                                                                                                                              |
| **☑** | `/projects/:id/export`                                                                                                                                                                                |
| ☐     | `/projects/:id/milestones`                                                                                                                                                                            |
| ☐     | `/projects/:id/snippets`                                                                                                                                                                              |
| ☐     | `/projects/:id/templates`                                                                                                                                                                             |
| ☐     | `/projects/:id/vulnerabilities`                                                                                                                                                                       |
| ☐     | `/projects/:id/wikis`                                                                                                                                                                                 |
| ☐     | `/projects/:id/variables`                                                                                                                                                                             |
| **☑** | `/projects`, `/projects/:id/hooks` (also available for users)                                                                                                                                         |
| ☐     | `/projects/:id/protected_branches`                                                                                                                                                                    |
| ☐     | `/projects/:id/protected_environments`                                                                                                                                                                |
| ☐     | `/projects/:id/protected_tags`                                                                                                                                                                        |
| ☐     | `/projects/:id/packages/pypi` (also available for groups)                                                                                                                                             |
| ☐     | `/projects/:id/releases/.../assets/links`                                                                                                                                                             |
| ☐     | `/projects/:id/releases`                                                                                                                                                                              |
| ☐     | `/projects/:id/remote_mirrors`                                                                                                                                                                        |
| **☑** | `/projects/:id/repository`                                                                                                                                                                            |
| **☑** | `/projects/:id/repository/files`                                                                                                                                                                      |
| ☐     | `/projects/:id/repository/submodules`                                                                                                                                                                 |
| ☐     | `/projects/:id/issues/.../resource_label_events`, `/projects/:id/merge_requests/.../resource_label_events` (also available for groups)                                                                |
| ☐     | `/projects/:id/packages/rubygems`                                                                                                                                                                     |
| **☑** | `/projects/:id/runners` (also available standalone)                                                                                                                                                   |
| ☐     | `/projects/:id/search` (also available for groups and standalone)                                                                                                                                     |
| **☑** | `/projects/:id/repository/tags`                                                                                                                                                                       |
| ☐     | `/projects/:id/packages/terraform/mdoules` (also available standalone)                                                                                                                                |
| ☐     | `/projects/:id/metrics/user_starred_dashboards`                                                                                                                                                       |
| ☐     | `/projects/:id/merge_requests/:merge_request_id/visual_review_discussions`                                                                                                                            |
| ☐     | `/vulnerabilities/:id`                                                                                                                                                                                |
| ☐     | `/projects/:id/vulnerability_exports`                                                                                                                                                                 |
| ☐     | `/projects/:id/vulnerability_findings`                                                                                                                                                                |

### Group resources

|       | Available endpoints                                                              |
|-------|----------------------------------------------------------------------------------|
| ☐     | `/groups/:id/access_requests/` (also available for projects)                     |
| ☐     | `/groups/:id/access_tokens` (also available for projects)                        |
| ☐     | `/groups/:id/custom_attributes` (also available for projects and users)          |
| ☐     | `/groups/:id/-/packages/debian` (also available for projects)                    |
| ☐     | `/groups/:id/deploy_tokens` (also available for projects and standalone)         |
| ☐     | `/groups/:id/epics/.../discussions` (also available for projects)                |
| ☐     | `/groups/:id/epics/.../issues`                                                   |
| ☐     | `/groups/:id/epics/.../epics`                                                    |
| ☐     | `/groups/:id/epics`                                                              |
| **☑** | `/groups`, `/groups/.../subgroups`                                               |
| **☑** | `/groups/:id/badges`                                                             |
| ☐     | `/groups/:id/boards`                                                             |
| ☐     | `/groups/:id/iterations` (also available for projects)                           |
| **☑** | `/groups/:id/labels`                                                             |
| ☐     | `/groups/:id/variables`                                                          |
| ☐     | `/groups/:id/milestones`                                                         |
| ☐     | `/groups/:id/releases`                                                           |
| ☐     | `/groups/:id/wikis`                                                              |
| ☐     | `/groups/:id/invitations` (also available for projects)                          |
| **☑** | `/groups/:id/issues` (also available for projects and standalone)                |
| ☐     | `/groups/:id/issues_statistics` (also available for projects and standalone)     |
| ☐     | `/groups/:id/epics/.../related_epics`                                            |
| **☑** | `/groups/:id/members` (also available for projects)                              |
| **☑** | `/groups/:id/merge_requests` (also available for projects and standalone)        |
| ☐     | `/groups/:id/epics/.../notes` (also available for projects)                      |
| ☐     | `/groups/:id/notification_settings` (also available for projects and standalone) |
| ☐     | `/groups/:id/epics/.../resource_label_events` (also available for projects)      |
| ☐     | `/groups/:id/search` (also available for projects and standalone)                |
| **☑** | `groups/:id/hooks`                                                               |

### Standalone resources

|       | Available endpoints                                                                              |
|-------|--------------------------------------------------------------------------------------------------|
| ☐     | `/application/appearance`                                                                        |
| ☐     | `/applications`                                                                                  |
| ☐     | `/audit_events`                                                                                  |
| ☐     | `/avatar`                                                                                        |
| ☐     | `/broadcast_messages`                                                                            |
| ☐     | `/snippets`                                                                                      |
| ☐     | `/users/:id/custom_attributes` (also available for groups and projects)                          |
| ☐     | `/deploy_keys` (also available for projects)                                                     |
| ☐     | `/deploy_tokens` (also available for projects and groups)                                        |
| ☐     | `/events`, `/users/:id/events` (also available for projects)                                     |
| ☐     | `/features`                                                                                      |
| ☐     | `/geo_nodes`                                                                                     |
| ☐     | `/analytics/group_activity/{issues_count}`                                                       |
| ☐     | `/group_repository_storage_moves`                                                                |
| ☐     | `/import/github`                                                                                 |
| ☐     | `/admin/clusters`                                                                                |
| ☐     | `/admin/ci/variables`                                                                            |
| ☐     | `/issues_statistics` (also available for groups and projects)                                    |
| **☑** | `/issues` (also available for groups and projects)                                               |
| **☑** | `/job`                                                                                           |
| ☐     | `/keys`                                                                                          |
| ☐     | `/license`                                                                                       |
| **☑** | `/markdown`                                                                                      |
| **☑** | `/merge_requests` (also available for groups and projects)                                       |
| ☐     | `/environments/:id/metrics_dashboard/annotations`, `/clusters/:id/metrics_dashboard/annotations` |
| ☐     | `/namespaces`                                                                                    |
| ☐     | `/notification_settings` (also available for groups and projects)                                |
| ☐     | `/pages/domains` (also available for projects)                                                   |
| **☑** | `/personal_access_tokens`                                                                        |
| ☐     | `/application/plan_limits`                                                                       |
| ☐     | `/project_repository_storage_moves`                                                              |
| ☐     | `/users/:id/projects` (also available for projects)                                              |
| **☑** | `/runners` (also available for projects)                                                         |
| ☐     | `/search` (also available for groups and projects)                                               |
| ☐     | `/usage_data` (For GitLab instance [Administrator](../user/permissions.md) users only)           |
| ☐     | `/application/settings`                                                                          |
| ☐     | `/sidekiq`                                                                                       |
| ☐     | `/admin/sidekiq/queues/:queue_name`                                                              |
| ☐     | `/snippet_repository_storage_moves`                                                              |
| ☐     | `/application/statistics`                                                                        |
| ☐     | `/suggestions`                                                                                   |
| ☐     | `/hooks`                                                                                         |
| **☑** | `/todos`                                                                                         |
| ☐     | `/topics`                                                                                        |
| **☑** | `/users`                                                                                         |
| **☑** | `/lint`                                                                                          |
| **☑** | `/version`                                                                                       |
