use pattrick_clap::Scope;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
#[serde(remote = "Scope")]
pub enum ScopeDef {
    #[serde(rename = "app_token")]
    FullAccess,
    #[serde(rename = "vso.advsec")]
    Advsec,
    #[serde(rename = "vso.advsec_write")]
    AdvsecWrite,
    #[serde(rename = "vso.advsec_manage")]
    AdvsecManage,
    #[serde(rename = "vso.agentpools")]
    AgentPools,
    #[serde(rename = "vso.agentpools_manage")]
    AgentPoolsManage,
    #[serde(rename = "vso.analytics")]
    Analytics,
    #[serde(rename = "vso.auditlog")]
    AuditLog,
    #[serde(rename = "vso.auditstreams_manage")]
    AuditStreamsManage,
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
    #[serde(rename = "vso.code_full")]
    CodeFull,
    #[serde(rename = "vso.code_status")]
    CodeStatus,
    #[serde(rename = "vso.connectedserver")]
    ConnectedServer,
    #[serde(rename = "vso.dashboards")]
    Dashboards,
    #[serde(rename = "vso.dashboards_manage")]
    DashboardsManage,
    #[serde(rename = "vso.drop_write")]
    DropWrite,
    #[serde(rename = "vso.entitlements")]
    Entitlements,
    #[serde(rename = "vso.memberentitlementmanagement")]
    MemberEntitlementManagement,
    #[serde(rename = "vso.memberentitlementmanagement_write")]
    MemberEntitlementManagementWrite,
    #[serde(rename = "vso.environment_manage")]
    EnvironmentManage,
    #[serde(rename = "vso.extension")]
    Extension,
    #[serde(rename = "vso.extension_manage")]
    ExtensionManage,
    #[serde(rename = "vso.extension.data")]
    ExtensionData,
    #[serde(rename = "vso.extension.data_write")]
    ExtensionDataWrite,
    #[serde(rename = "vso.gallery")]
    Gallery,
    #[serde(rename = "vso.gallery_acquire")]
    GalleryAcquire,
    #[serde(rename = "vso.gallery_publish")]
    GalleryPublish,
    #[serde(rename = "vso.gallery_manage")]
    GalleryManage,
    #[serde(rename = "vso.githubconnections")]
    GitHubConnections,
    #[serde(rename = "vso.githubconnections_manage")]
    GitHubConnectionsManage,
    #[serde(rename = "vso.governance")]
    Governance,
    #[serde(rename = "vso.graph")]
    Graph,
    #[serde(rename = "vso.graph_manage")]
    GraphManage,
    #[serde(rename = "vso.hooks")]
    Hooks,
    #[serde(rename = "vso.hooks_write")]
    HooksWrite,
    #[serde(rename = "vso.hooks_interact")]
    HooksInteract,
    #[serde(rename = "vso.identity")]
    Identity,
    #[serde(rename = "vso.identity_manage")]
    IdentityManage,
    #[serde(rename = "vso.machinegroup_manage")]
    MachineGroupManage,
    #[serde(rename = "vso.notification")]
    Notification,
    #[serde(rename = "vso.notification_write")]
    NotificationWrite,
    #[serde(rename = "vso.notification_manage")]
    NotificationManage,
    #[serde(rename = "vso.notification_diagnostics")]
    NotificationDiagnostics,
    #[default]
    #[serde(rename = "vso.packaging")]
    Packaging,
    #[serde(rename = "vso.packaging_manage")]
    PackagingManage,
    #[serde(rename = "vso.packaging_write")]
    PackagingWrite,
    #[serde(rename = "vso.pipelineresources_use")]
    PipelineResourcesUse,
    #[serde(rename = "vso.pipelineresources_manage")]
    PipelineResourcesManage,
    #[serde(rename = "vso.profile")]
    Profile,
    #[serde(rename = "vso.project")]
    Project,
    #[serde(rename = "vso.project_write")]
    ProjectWrite,
    #[serde(rename = "vso.project_manage")]
    ProjectManage,
    #[serde(rename = "vso.release")]
    Release,
    #[serde(rename = "vso.release_manage")]
    ReleaseManage,
    #[serde(rename = "vso.release_execute")]
    ReleaseExecute,
    #[serde(rename = "vso.securefiles_read")]
    SecureFilesRead,
    #[serde(rename = "vso.securefiles_write")]
    SecureFilesWrite,
    #[serde(rename = "vso.securefiles_manage")]
    SecureFilesManage,
    #[serde(rename = "vso.security")]
    Security,
    #[serde(rename = "vso.security_manage")]
    SecurityManage,
    #[serde(rename = "vso.serviceendpoint")]
    ServiceEndpoint,
    #[serde(rename = "vso.serviceendpoint_query")]
    ServiceEndpointQuery,
    #[serde(rename = "vso.serviceendpoint_manage")]
    ServiceEndpointManage,
    #[serde(rename = "vso.settings")]
    Settings,
    #[serde(rename = "vso.settings_write")]
    SettingsWrite,
    #[serde(rename = "vso.symbols")]
    Symbols,
    #[serde(rename = "vso.symbols_write")]
    SymbolsWrite,
    #[serde(rename = "vso.symbols_manage")]
    SymbolsManage,
    #[serde(rename = "vso.taskgroups_read")]
    TaskGroupsRead,
    #[serde(rename = "vso.taskgroups_write")]
    TaskGroupsWrite,
    #[serde(rename = "vso.taskgroups_manage")]
    TaskGroupsManage,
    #[serde(rename = "vso.test")]
    Test,
    #[serde(rename = "vso.test_write")]
    TestWrite,
    #[serde(rename = "vso.threads_full")]
    ThreadsFull,
    #[serde(rename = "vso.tokens")]
    Tokens,
    #[serde(rename = "vso.tokens_administration")]
    TokensAdministration,
    #[serde(rename = "vso.variablegroups_read")]
    VariableGroupsRead,
    #[serde(rename = "vso.variablegroups_write")]
    VariableGroupsWrite,
    #[serde(rename = "vso.variablegroups_manage")]
    VariableGroupsManage,
    #[serde(rename = "vso.wiki")]
    Wiki,
    #[serde(rename = "vso.wiki_write")]
    WikiWrite,
    #[serde(rename = "vso.work")]
    Work,
    #[serde(rename = "vso.work_write")]
    WorkWrite,
    #[serde(rename = "vso.work_full")]
    WorkFull,
}

#[derive(serde::Serialize)]
pub struct ScopeWrapper<'a>(#[serde(with = "ScopeDef")] pub &'a Scope);
