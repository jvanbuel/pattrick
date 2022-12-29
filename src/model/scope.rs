use pattrick_clap::Scope;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
#[serde(remote = "Scope")]
pub enum ScopeDef {
    #[serde(rename = "app_token")]
    FullAccess,
    #[serde(rename = "vso.agentpools")]
    AgentPools,
    #[serde(rename = "vso.agentpools_manage")]
    AgentPoolsManage,
    #[serde(rename = "vso.build")]
    Build,
    #[serde(rename = "vso.build_execute")]
    BuildExecute,
    #[serde(rename = "vso.code")]
    Code,
    #[serde(rename = "vso.code_write")]
    CodeWrite,
    #[serde(rename = "vso.code_manage")]
    CodeManage,
    #[serde(rename = "vso.dashboards")]
    Dashboards,
    #[serde(rename = "vso.dashboards_manage")]
    DashboardsManage,
    #[serde(rename = "vso.extension")]
    Extension,
    #[serde(rename = "vso.extension_manage")]
    ExtensionManage,
    #[serde(rename = "vso.governance")]
    Governance,
    #[serde(rename = "vso.graph")]
    Graph,
    #[serde(rename = "vso.graph_manage")]
    GraphManage,
    #[serde(rename = "vso.notification")]
    Notification,
    #[serde(rename = "vso.notification_diagnostics")]
    NotificationDiagnostics,
    #[default]
    #[serde(rename = "vso.packaging")]
    Packaging,
    #[serde(rename = "vso.packaging_manage")]
    PackagingManage,
    #[serde(rename = "vso.packaging_write")]
    PackagingWrite,
    #[serde(rename = "vso.profile")]
    Profile,
    #[serde(rename = "vso.project")]
    Project,
    #[serde(rename = "vso.project_manage")]
    ProjectManage,
    #[serde(rename = "vso.release")]
    Release,
    #[serde(rename = "vso.release_execute")]
    ReleaseExecute,
    #[serde(rename = "vso.security")]
    Security,
    #[serde(rename = "vso.security_manage")]
    SecurityManage,
    #[serde(rename = "vso.test")]
    Test,
    #[serde(rename = "vso.test_write")]
    TestWrite,
    #[serde(rename = "vso.work")]
    Work,
    #[serde(rename = "vso.work_write")]
    WorkWrite,
    #[serde(rename = "vso.wiki")]
    Wiki,
    #[serde(rename = "vso.wiki_write")]
    WikiWrite,
}
