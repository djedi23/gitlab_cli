mod badges;
mod branches;
mod commit;
mod diff;
mod emails;
mod export;
mod groups;
mod hooks;
mod issues;
mod jobs;
mod keys;
mod labels;
mod lint;
mod markdown;
mod merge_requests;
mod milestones;
mod pipeline;
mod projects;
mod reference;
mod repository;
mod star;
mod tag;
mod task;
mod time;
mod todo;
mod user;
mod version;

use crate::{
  badges::{Badge, BadgeCreatePayload, BadgeQuery},
  branches::{Branch, BranchCreatePayload, BranchQuery},
  commit::{Commit, MergeBaseQuery},
  diff::DiffVersion,
  emails::{Email, EmailCreatePayload, EmailCreateWithValidationPayload},
  export::{Export, ExportPayload, ExportRelation, ExportRelationQuery},
  groups::{
    create::GroupsCreatePayload, update::GroupsUpdatePayload, Group, GroupDetail,
    GroupTransferLocationQuery, GroupTransferQuery, GroupsDeleteQuery, GroupsFilter,
    GroupsSharePayload,
  },
  hooks::{GroupsHookPayload, Hook, ProjectsHookPayload},
  issues::{
    actions::{IssueMovePayload, IssueReorderQuery},
    create::IssueCreatePayload,
    edit::IssueUpdatePayload,
    Issue, IssueFilter,
  },
  jobs::{Job, JobFilter, JobTokenQuery, PipelineJobFilter, PlayJobPayload},
  keys::{GpGKey, GpgKeyCreatePayload, SSHKey, SSHKeyCreatePayload},
  labels::{CreateLabelPayload, GroupLabelQuery, Label, LabelQuery, UpdateLabelPayload},
  lint::{Lint, LintCurrentProjectPayload, LintPayload, LintProjectPayload},
  markdown::{Markdown, MarkdownQuery},
  merge_requests::{
    CommidId, MergeRequest, MergeRequestChangeSelector, MergeRequestCreate, MergeRequestFilter,
    MergeRequestMerge, MergeRequestSelector, MergeRequestUpdate,
  },
  pipeline::{Pipeline, PipelineCreate, PipelineFilter, TestReport, TestReportSummary, Variables},
  projects::{
    actions::{
      ProjectSharePayload, ProjectsForkQuery, ProjectsForksQuery, ProjectsSnapshot, TransferQuery,
    },
    create::ProjectsCreatePayload,
    Project, ProjectDetailFilter, ProjectsFilter, UsersProjectsFilter,
  },
  repository::{
    blob::Blob,
    changelog::{Changelog, ChangelogPayload},
    compare::{Compare, CompareQuery},
    contributor::{Contributor, ContributorSort},
    file::{blame::Blame, File, FileCreatePayload, FileDeletePayload, FilePayload},
    tree::{Tree, TreeFilter},
  },
  star::Starrers,
  tag::{CreateTagPayload, Tag, TagQuery},
  time::{IssueAddSpentTimeQuery, IssueTimeEstimateQuery, TimeStats},
  todo::{Todo, TodoFilter},
  user::{
    activity::{Activity, ActivityPagination},
    count::UserCount,
    membership::{MembersState, MembersStateQuery, Membership, MembershipPagination},
    preferences::{Preferences, UserPreferencesPayload},
    status::{Status, UserStatusPayload},
    tokens::{ImpersonationToken, TokenCreatePayload, TokenFilter},
    CurrentUserSelector, GroupsProvisionedUsersQuery, MemberCreatePayload, MemberEditPayload,
    MembersFilter, MembersPagination, ProjectsUsersFilter, User, UserCreatePayload,
    UserDeletePayload, UserUpdatePayload, UsersFilter,
  },
  version::Version,
};
use crud_api::{Api, ApiInput, ApiRun, EmptyResponse, Query};
use crud_auth::CrudAuth;
use crud_auth_bearer::Auth;
use miette::{IntoDiagnostic, Result, WrapErr};

#[derive(ApiRun)]
#[api(infos(
  base_url = "https://gitlab.com/api/v4",
  name = "gitlab-cli",
  author = "Valvassori Moïse <moise.valvassori@gmail.com>",
  about = "Command line interface to Gitlab's API",
  qualifier = "org",
  organisation = "djedi",
  env_prefix = "GITLAB"
))]
struct Gitlab {}

#[tokio::main]
async fn main() -> Result<()> {
  Gitlab::run().await
}
