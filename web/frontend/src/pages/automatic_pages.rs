use crate::components::*;
use web_common::database::*;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct CountryPageProp {
    pub id: i32,
}

impl From<&CountryPageProp> for PrimaryKey {
    fn from(prop: &CountryPageProp) -> Self {
        prop.id.into()
    }
}

impl CountryPageProp {
    fn filter_organizations_by_country_id(&self) -> OrganizationFilter {
        let mut filter = OrganizationFilter::default();
        filter.country_id = Some(self.id);
        filter
    }
}

#[function_component(CountryPage)]
pub fn country_page(props: &CountryPageProp) -> Html {
    let can_update = use_state(|| false);
    let can_admin = use_state(|| false);

    let can_update_callback = {
        let can_update = can_update.clone();
        Callback::from(move |value: bool| can_update.set(value))
    };
    let can_admin_callback = {
        let can_admin = can_admin.clone();
        Callback::from(move |value: bool| can_admin.set(value))
    };
    html! {
        <BasicPage<Country> id={PrimaryKey::from(props)} can_update={can_update_callback} can_admin={can_admin_callback}>
            // Linked with foreign key organizations.country_id
            <BasicList<NestedOrganization> column_name={"country_id"} filters={props.filter_organizations_by_country_id()} can_create={*can_update} can_truncate={*can_admin}/>
        </BasicPage<Country>>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct DerivedSamplePageProp {
    pub parent_sample_id: uuid::Uuid,
    pub child_sample_id: uuid::Uuid,
}

impl From<&DerivedSamplePageProp> for PrimaryKey {
    fn from(prop: &DerivedSamplePageProp) -> Self {
        (prop.parent_sample_id, prop.child_sample_id).into()
    }
}

#[function_component(DerivedSamplePage)]
pub fn derived_sample_page(props: &DerivedSamplePageProp) -> Html {
    let can_update = use_state(|| false);
    let can_admin = use_state(|| false);

    let can_update_callback = {
        let can_update = can_update.clone();
        Callback::from(move |value: bool| can_update.set(value))
    };
    let can_admin_callback = {
        let can_admin = can_admin.clone();
        Callback::from(move |value: bool| can_admin.set(value))
    };
    html! {
        <BasicPage<NestedDerivedSample> id={PrimaryKey::from(props)} can_update={can_update_callback} can_admin={can_admin_callback}>
            <span>{"No content available yet."}</span>
        </BasicPage<NestedDerivedSample>>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct NameplatePageProp {
    pub id: i32,
}

impl From<&NameplatePageProp> for PrimaryKey {
    fn from(prop: &NameplatePageProp) -> Self {
        prop.id.into()
    }
}

impl NameplatePageProp {
    fn filter_organisms_by_nameplate_id(&self) -> OrganismFilter {
        let mut filter = OrganismFilter::default();
        filter.nameplate_id = Some(self.id);
        filter
    }
}

#[function_component(NameplatePage)]
pub fn nameplate_page(props: &NameplatePageProp) -> Html {
    let can_update = use_state(|| false);
    let can_admin = use_state(|| false);

    let can_update_callback = {
        let can_update = can_update.clone();
        Callback::from(move |value: bool| can_update.set(value))
    };
    let can_admin_callback = {
        let can_admin = can_admin.clone();
        Callback::from(move |value: bool| can_admin.set(value))
    };
    html! {
        <BasicPage<NestedNameplate> id={PrimaryKey::from(props)} can_update={can_update_callback} can_admin={can_admin_callback}>
            // Linked with foreign key organisms.nameplate_id
            <BasicList<NestedOrganism> column_name={"nameplate_id"} filters={props.filter_organisms_by_nameplate_id()} can_create={*can_update} can_truncate={*can_admin}/>
        </BasicPage<NestedNameplate>>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct ObservationSubjectPageProp {
    pub id: i32,
}

impl From<&ObservationSubjectPageProp> for PrimaryKey {
    fn from(prop: &ObservationSubjectPageProp) -> Self {
        prop.id.into()
    }
}

impl ObservationSubjectPageProp {
    fn filter_observations_by_subject_id(&self) -> ObservationFilter {
        let mut filter = ObservationFilter::default();
        filter.subject_id = Some(self.id);
        filter
    }
}

#[function_component(ObservationSubjectPage)]
pub fn observation_subject_page(props: &ObservationSubjectPageProp) -> Html {
    let can_update = use_state(|| false);
    let can_admin = use_state(|| false);

    let can_update_callback = {
        let can_update = can_update.clone();
        Callback::from(move |value: bool| can_update.set(value))
    };
    let can_admin_callback = {
        let can_admin = can_admin.clone();
        Callback::from(move |value: bool| can_admin.set(value))
    };
    html! {
        <BasicPage<NestedObservationSubject> id={PrimaryKey::from(props)} can_update={can_update_callback} can_admin={can_admin_callback}>
            // Linked with foreign key observations.subject_id
            <BasicList<NestedObservation> column_name={"subject_id"} filters={props.filter_observations_by_subject_id()} can_create={*can_update} can_truncate={*can_admin}/>
        </BasicPage<NestedObservationSubject>>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct ObservationPageProp {
    pub id: uuid::Uuid,
}

impl From<&ObservationPageProp> for PrimaryKey {
    fn from(prop: &ObservationPageProp) -> Self {
        prop.id.into()
    }
}

impl ObservationPageProp {
    fn filter_observations_by_parent_observation_id(&self) -> ObservationFilter {
        let mut filter = ObservationFilter::default();
        filter.parent_observation_id = Some(self.id);
        filter
    }
}

#[function_component(ObservationPage)]
pub fn observation_page(props: &ObservationPageProp) -> Html {
    let can_update = use_state(|| false);
    let can_admin = use_state(|| false);

    let can_update_callback = {
        let can_update = can_update.clone();
        Callback::from(move |value: bool| can_update.set(value))
    };
    let can_admin_callback = {
        let can_admin = can_admin.clone();
        Callback::from(move |value: bool| can_admin.set(value))
    };
    html! {
        <BasicPage<NestedObservation> id={PrimaryKey::from(props)} can_update={can_update_callback} can_admin={can_admin_callback}>
            // Linked with foreign key observations.parent_observation_id
            <BasicList<NestedObservation> column_name={"parent_observation_id"} filters={props.filter_observations_by_parent_observation_id()} can_create={*can_update} can_truncate={*can_admin}/>
        </BasicPage<NestedObservation>>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct OrganismTaxonPageProp {
    pub organism_id: uuid::Uuid,
    pub taxon_id: i32,
}

impl From<&OrganismTaxonPageProp> for PrimaryKey {
    fn from(prop: &OrganismTaxonPageProp) -> Self {
        (prop.organism_id, prop.taxon_id).into()
    }
}

#[function_component(OrganismTaxonPage)]
pub fn organism_taxon_page(props: &OrganismTaxonPageProp) -> Html {
    let can_update = use_state(|| false);
    let can_admin = use_state(|| false);

    let can_update_callback = {
        let can_update = can_update.clone();
        Callback::from(move |value: bool| can_update.set(value))
    };
    let can_admin_callback = {
        let can_admin = can_admin.clone();
        Callback::from(move |value: bool| can_admin.set(value))
    };
    html! {
        <BasicPage<NestedOrganismTaxon> id={PrimaryKey::from(props)} can_update={can_update_callback} can_admin={can_admin_callback}>
            <span>{"No content available yet."}</span>
        </BasicPage<NestedOrganismTaxon>>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct OrganismPageProp {
    pub id: uuid::Uuid,
}

impl From<&OrganismPageProp> for PrimaryKey {
    fn from(prop: &OrganismPageProp) -> Self {
        prop.id.into()
    }
}

impl OrganismPageProp {
    fn filter_organisms_by_host_organism_id(&self) -> OrganismFilter {
        let mut filter = OrganismFilter::default();
        filter.host_organism_id = Some(self.id);
        filter
    }
    fn filter_observations_by_organism_id(&self) -> ObservationFilter {
        let mut filter = ObservationFilter::default();
        filter.organism_id = Some(self.id);
        filter
    }
    fn filter_organism_taxa_by_organism_id(&self) -> OrganismTaxonFilter {
        let mut filter = OrganismTaxonFilter::default();
        filter.organism_id = Some(self.id);
        filter
    }
}

#[function_component(OrganismPage)]
pub fn organism_page(props: &OrganismPageProp) -> Html {
    let can_update = use_state(|| false);
    let can_admin = use_state(|| false);

    let can_update_callback = {
        let can_update = can_update.clone();
        Callback::from(move |value: bool| can_update.set(value))
    };
    let can_admin_callback = {
        let can_admin = can_admin.clone();
        Callback::from(move |value: bool| can_admin.set(value))
    };
    html! {
        <BasicPage<NestedOrganism> id={PrimaryKey::from(props)} can_update={can_update_callback} can_admin={can_admin_callback}>
            // Linked with foreign key organisms.host_organism_id
            <BasicList<NestedOrganism> column_name={"host_organism_id"} filters={props.filter_organisms_by_host_organism_id()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key observations.organism_id
            <BasicList<NestedObservation> column_name={"organism_id"} filters={props.filter_observations_by_organism_id()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key organism_taxa.organism_id
            <BasicList<NestedOrganismTaxon> column_name={"organism_id"} filters={props.filter_organism_taxa_by_organism_id()} can_create={*can_update} can_truncate={*can_admin}/>
        </BasicPage<NestedOrganism>>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct OrganizationPageProp {
    pub id: i32,
}

impl From<&OrganizationPageProp> for PrimaryKey {
    fn from(prop: &OrganizationPageProp) -> Self {
        prop.id.into()
    }
}

impl OrganizationPageProp {
    fn filter_users_by_organization_id(&self) -> UserFilter {
        let mut filter = UserFilter::default();
        filter.organization_id = Some(self.id);
        filter
    }
}

#[function_component(OrganizationPage)]
pub fn organization_page(props: &OrganizationPageProp) -> Html {
    let can_update = use_state(|| false);
    let can_admin = use_state(|| false);

    let can_update_callback = {
        let can_update = can_update.clone();
        Callback::from(move |value: bool| can_update.set(value))
    };
    let can_admin_callback = {
        let can_admin = can_admin.clone();
        Callback::from(move |value: bool| can_admin.set(value))
    };
    html! {
        <BasicPage<NestedOrganization> id={PrimaryKey::from(props)} can_update={can_update_callback} can_admin={can_admin_callback}>
            // Linked with foreign key users.organization_id
            <BasicList<NestedUser> column_name={"organization_id"} filters={props.filter_users_by_organization_id()} can_create={*can_update} can_truncate={*can_admin}/>
        </BasicPage<NestedOrganization>>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct ProjectPageProp {
    pub id: i32,
}

impl From<&ProjectPageProp> for PrimaryKey {
    fn from(prop: &ProjectPageProp) -> Self {
        prop.id.into()
    }
}

impl ProjectPageProp {
    fn filter_projects_by_parent_project_id(&self) -> ProjectFilter {
        let mut filter = ProjectFilter::default();
        filter.parent_project_id = Some(self.id);
        filter
    }
    fn filter_projects_users_role_invitations_by_table_id(
        &self,
    ) -> ProjectsUsersRoleInvitationFilter {
        let mut filter = ProjectsUsersRoleInvitationFilter::default();
        filter.table_id = Some(self.id);
        filter
    }
    fn filter_projects_users_role_requests_by_table_id(&self) -> ProjectsUsersRoleRequestFilter {
        let mut filter = ProjectsUsersRoleRequestFilter::default();
        filter.table_id = Some(self.id);
        filter
    }
    fn filter_projects_users_roles_by_table_id(&self) -> ProjectsUsersRoleFilter {
        let mut filter = ProjectsUsersRoleFilter::default();
        filter.table_id = Some(self.id);
        filter
    }
    fn filter_sample_containers_by_project_id(&self) -> SampleContainerFilter {
        let mut filter = SampleContainerFilter::default();
        filter.project_id = Some(self.id);
        filter
    }
    fn filter_samples_by_project_id(&self) -> SampleFilter {
        let mut filter = SampleFilter::default();
        filter.project_id = Some(self.id);
        filter
    }
    fn filter_nameplates_by_project_id(&self) -> NameplateFilter {
        let mut filter = NameplateFilter::default();
        filter.project_id = Some(self.id);
        filter
    }
    fn filter_organisms_by_project_id(&self) -> OrganismFilter {
        let mut filter = OrganismFilter::default();
        filter.project_id = Some(self.id);
        filter
    }
    fn filter_projects_teams_role_invitations_by_table_id(
        &self,
    ) -> ProjectsTeamsRoleInvitationFilter {
        let mut filter = ProjectsTeamsRoleInvitationFilter::default();
        filter.table_id = Some(self.id);
        filter
    }
    fn filter_projects_teams_role_requests_by_table_id(&self) -> ProjectsTeamsRoleRequestFilter {
        let mut filter = ProjectsTeamsRoleRequestFilter::default();
        filter.table_id = Some(self.id);
        filter
    }
    fn filter_projects_teams_roles_by_table_id(&self) -> ProjectsTeamsRoleFilter {
        let mut filter = ProjectsTeamsRoleFilter::default();
        filter.table_id = Some(self.id);
        filter
    }
    fn filter_observations_by_project_id(&self) -> ObservationFilter {
        let mut filter = ObservationFilter::default();
        filter.project_id = Some(self.id);
        filter
    }
}

#[function_component(ProjectPage)]
pub fn project_page(props: &ProjectPageProp) -> Html {
    let can_update = use_state(|| false);
    let can_admin = use_state(|| false);

    let can_update_callback = {
        let can_update = can_update.clone();
        Callback::from(move |value: bool| can_update.set(value))
    };
    let can_admin_callback = {
        let can_admin = can_admin.clone();
        Callback::from(move |value: bool| can_admin.set(value))
    };
    html! {
        <BasicPage<NestedProject> id={PrimaryKey::from(props)} can_update={can_update_callback} can_admin={can_admin_callback}>
            // Linked with foreign key projects.parent_project_id
            <BasicList<NestedProject> column_name={"parent_project_id"} filters={props.filter_projects_by_parent_project_id()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key projects_users_role_invitations.table_id
            <BasicList<NestedProjectsUsersRoleInvitation> column_name={"table_id"} filters={props.filter_projects_users_role_invitations_by_table_id()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key projects_users_role_requests.table_id
            <BasicList<NestedProjectsUsersRoleRequest> column_name={"table_id"} filters={props.filter_projects_users_role_requests_by_table_id()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key projects_users_roles.table_id
            <BasicList<NestedProjectsUsersRole> column_name={"table_id"} filters={props.filter_projects_users_roles_by_table_id()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key sample_containers.project_id
            <BasicList<NestedSampleContainer> column_name={"project_id"} filters={props.filter_sample_containers_by_project_id()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key samples.project_id
            <BasicList<NestedSample> column_name={"project_id"} filters={props.filter_samples_by_project_id()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key nameplates.project_id
            <BasicList<NestedNameplate> column_name={"project_id"} filters={props.filter_nameplates_by_project_id()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key organisms.project_id
            <BasicList<NestedOrganism> column_name={"project_id"} filters={props.filter_organisms_by_project_id()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key projects_teams_role_invitations.table_id
            <BasicList<NestedProjectsTeamsRoleInvitation> column_name={"table_id"} filters={props.filter_projects_teams_role_invitations_by_table_id()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key projects_teams_role_requests.table_id
            <BasicList<NestedProjectsTeamsRoleRequest> column_name={"table_id"} filters={props.filter_projects_teams_role_requests_by_table_id()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key projects_teams_roles.table_id
            <BasicList<NestedProjectsTeamsRole> column_name={"table_id"} filters={props.filter_projects_teams_roles_by_table_id()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key observations.project_id
            <BasicList<NestedObservation> column_name={"project_id"} filters={props.filter_observations_by_project_id()} can_create={*can_update} can_truncate={*can_admin}/>
        </BasicPage<NestedProject>>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct ProjectsTeamsRoleInvitationPageProp {
    pub table_id: i32,
    pub team_id: i32,
}

impl From<&ProjectsTeamsRoleInvitationPageProp> for PrimaryKey {
    fn from(prop: &ProjectsTeamsRoleInvitationPageProp) -> Self {
        (prop.table_id, prop.team_id).into()
    }
}

#[function_component(ProjectsTeamsRoleInvitationPage)]
pub fn projects_teams_role_invitation_page(props: &ProjectsTeamsRoleInvitationPageProp) -> Html {
    let can_update = use_state(|| false);
    let can_admin = use_state(|| false);

    let can_update_callback = {
        let can_update = can_update.clone();
        Callback::from(move |value: bool| can_update.set(value))
    };
    let can_admin_callback = {
        let can_admin = can_admin.clone();
        Callback::from(move |value: bool| can_admin.set(value))
    };
    html! {
        <BasicPage<NestedProjectsTeamsRoleInvitation> id={PrimaryKey::from(props)} can_update={can_update_callback} can_admin={can_admin_callback}>
            <span>{"No content available yet."}</span>
        </BasicPage<NestedProjectsTeamsRoleInvitation>>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct ProjectsTeamsRoleRequestPageProp {
    pub table_id: i32,
    pub team_id: i32,
}

impl From<&ProjectsTeamsRoleRequestPageProp> for PrimaryKey {
    fn from(prop: &ProjectsTeamsRoleRequestPageProp) -> Self {
        (prop.table_id, prop.team_id).into()
    }
}

#[function_component(ProjectsTeamsRoleRequestPage)]
pub fn projects_teams_role_request_page(props: &ProjectsTeamsRoleRequestPageProp) -> Html {
    let can_update = use_state(|| false);
    let can_admin = use_state(|| false);

    let can_update_callback = {
        let can_update = can_update.clone();
        Callback::from(move |value: bool| can_update.set(value))
    };
    let can_admin_callback = {
        let can_admin = can_admin.clone();
        Callback::from(move |value: bool| can_admin.set(value))
    };
    html! {
        <BasicPage<NestedProjectsTeamsRoleRequest> id={PrimaryKey::from(props)} can_update={can_update_callback} can_admin={can_admin_callback}>
            <span>{"No content available yet."}</span>
        </BasicPage<NestedProjectsTeamsRoleRequest>>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct ProjectsTeamsRolePageProp {
    pub table_id: i32,
    pub team_id: i32,
}

impl From<&ProjectsTeamsRolePageProp> for PrimaryKey {
    fn from(prop: &ProjectsTeamsRolePageProp) -> Self {
        (prop.table_id, prop.team_id).into()
    }
}

#[function_component(ProjectsTeamsRolePage)]
pub fn projects_teams_role_page(props: &ProjectsTeamsRolePageProp) -> Html {
    let can_update = use_state(|| false);
    let can_admin = use_state(|| false);

    let can_update_callback = {
        let can_update = can_update.clone();
        Callback::from(move |value: bool| can_update.set(value))
    };
    let can_admin_callback = {
        let can_admin = can_admin.clone();
        Callback::from(move |value: bool| can_admin.set(value))
    };
    html! {
        <BasicPage<NestedProjectsTeamsRole> id={PrimaryKey::from(props)} can_update={can_update_callback} can_admin={can_admin_callback}>
            <span>{"No content available yet."}</span>
        </BasicPage<NestedProjectsTeamsRole>>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct ProjectsUsersRoleInvitationPageProp {
    pub table_id: i32,
    pub user_id: i32,
}

impl From<&ProjectsUsersRoleInvitationPageProp> for PrimaryKey {
    fn from(prop: &ProjectsUsersRoleInvitationPageProp) -> Self {
        (prop.table_id, prop.user_id).into()
    }
}

#[function_component(ProjectsUsersRoleInvitationPage)]
pub fn projects_users_role_invitation_page(props: &ProjectsUsersRoleInvitationPageProp) -> Html {
    let can_update = use_state(|| false);
    let can_admin = use_state(|| false);

    let can_update_callback = {
        let can_update = can_update.clone();
        Callback::from(move |value: bool| can_update.set(value))
    };
    let can_admin_callback = {
        let can_admin = can_admin.clone();
        Callback::from(move |value: bool| can_admin.set(value))
    };
    html! {
        <BasicPage<NestedProjectsUsersRoleInvitation> id={PrimaryKey::from(props)} can_update={can_update_callback} can_admin={can_admin_callback}>
            <span>{"No content available yet."}</span>
        </BasicPage<NestedProjectsUsersRoleInvitation>>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct ProjectsUsersRoleRequestPageProp {
    pub table_id: i32,
    pub user_id: i32,
}

impl From<&ProjectsUsersRoleRequestPageProp> for PrimaryKey {
    fn from(prop: &ProjectsUsersRoleRequestPageProp) -> Self {
        (prop.table_id, prop.user_id).into()
    }
}

#[function_component(ProjectsUsersRoleRequestPage)]
pub fn projects_users_role_request_page(props: &ProjectsUsersRoleRequestPageProp) -> Html {
    let can_update = use_state(|| false);
    let can_admin = use_state(|| false);

    let can_update_callback = {
        let can_update = can_update.clone();
        Callback::from(move |value: bool| can_update.set(value))
    };
    let can_admin_callback = {
        let can_admin = can_admin.clone();
        Callback::from(move |value: bool| can_admin.set(value))
    };
    html! {
        <BasicPage<NestedProjectsUsersRoleRequest> id={PrimaryKey::from(props)} can_update={can_update_callback} can_admin={can_admin_callback}>
            <span>{"No content available yet."}</span>
        </BasicPage<NestedProjectsUsersRoleRequest>>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct ProjectsUsersRolePageProp {
    pub table_id: i32,
    pub user_id: i32,
}

impl From<&ProjectsUsersRolePageProp> for PrimaryKey {
    fn from(prop: &ProjectsUsersRolePageProp) -> Self {
        (prop.table_id, prop.user_id).into()
    }
}

#[function_component(ProjectsUsersRolePage)]
pub fn projects_users_role_page(props: &ProjectsUsersRolePageProp) -> Html {
    let can_update = use_state(|| false);
    let can_admin = use_state(|| false);

    let can_update_callback = {
        let can_update = can_update.clone();
        Callback::from(move |value: bool| can_update.set(value))
    };
    let can_admin_callback = {
        let can_admin = can_admin.clone();
        Callback::from(move |value: bool| can_admin.set(value))
    };
    html! {
        <BasicPage<NestedProjectsUsersRole> id={PrimaryKey::from(props)} can_update={can_update_callback} can_admin={can_admin_callback}>
            <span>{"No content available yet."}</span>
        </BasicPage<NestedProjectsUsersRole>>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct RankPageProp {
    pub id: i32,
}

impl From<&RankPageProp> for PrimaryKey {
    fn from(prop: &RankPageProp) -> Self {
        prop.id.into()
    }
}

impl RankPageProp {
    fn filter_taxa_by_ott_rank_id(&self) -> TaxonFilter {
        let mut filter = TaxonFilter::default();
        filter.ott_rank_id = Some(self.id);
        filter
    }
}

#[function_component(RankPage)]
pub fn rank_page(props: &RankPageProp) -> Html {
    let can_update = use_state(|| false);
    let can_admin = use_state(|| false);

    let can_update_callback = {
        let can_update = can_update.clone();
        Callback::from(move |value: bool| can_update.set(value))
    };
    let can_admin_callback = {
        let can_admin = can_admin.clone();
        Callback::from(move |value: bool| can_admin.set(value))
    };
    html! {
        <BasicPage<NestedRank> id={PrimaryKey::from(props)} can_update={can_update_callback} can_admin={can_admin_callback}>
            // Linked with foreign key taxa.ott_rank_id
            <BasicList<NestedTaxon> column_name={"ott_rank_id"} filters={props.filter_taxa_by_ott_rank_id()} can_create={*can_update} can_truncate={*can_admin}/>
        </BasicPage<NestedRank>>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct SampleContainerPageProp {
    pub id: i32,
}

impl From<&SampleContainerPageProp> for PrimaryKey {
    fn from(prop: &SampleContainerPageProp) -> Self {
        prop.id.into()
    }
}

impl SampleContainerPageProp {
    fn filter_samples_by_container_id(&self) -> SampleFilter {
        let mut filter = SampleFilter::default();
        filter.container_id = Some(self.id);
        filter
    }
}

#[function_component(SampleContainerPage)]
pub fn sample_container_page(props: &SampleContainerPageProp) -> Html {
    let can_update = use_state(|| false);
    let can_admin = use_state(|| false);

    let can_update_callback = {
        let can_update = can_update.clone();
        Callback::from(move |value: bool| can_update.set(value))
    };
    let can_admin_callback = {
        let can_admin = can_admin.clone();
        Callback::from(move |value: bool| can_admin.set(value))
    };
    html! {
        <BasicPage<NestedSampleContainer> id={PrimaryKey::from(props)} can_update={can_update_callback} can_admin={can_admin_callback}>
            // Linked with foreign key samples.container_id
            <BasicList<NestedSample> column_name={"container_id"} filters={props.filter_samples_by_container_id()} can_create={*can_update} can_truncate={*can_admin}/>
        </BasicPage<NestedSampleContainer>>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct SampleStatePageProp {
    pub id: i32,
}

impl From<&SampleStatePageProp> for PrimaryKey {
    fn from(prop: &SampleStatePageProp) -> Self {
        prop.id.into()
    }
}

impl SampleStatePageProp {
    fn filter_samples_by_state_id(&self) -> SampleFilter {
        let mut filter = SampleFilter::default();
        filter.state_id = Some(self.id);
        filter
    }
}

#[function_component(SampleStatePage)]
pub fn sample_state_page(props: &SampleStatePageProp) -> Html {
    let can_update = use_state(|| false);
    let can_admin = use_state(|| false);

    let can_update_callback = {
        let can_update = can_update.clone();
        Callback::from(move |value: bool| can_update.set(value))
    };
    let can_admin_callback = {
        let can_admin = can_admin.clone();
        Callback::from(move |value: bool| can_admin.set(value))
    };
    html! {
        <BasicPage<NestedSampleState> id={PrimaryKey::from(props)} can_update={can_update_callback} can_admin={can_admin_callback}>
            // Linked with foreign key samples.state_id
            <BasicList<NestedSample> column_name={"state_id"} filters={props.filter_samples_by_state_id()} can_create={*can_update} can_truncate={*can_admin}/>
        </BasicPage<NestedSampleState>>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct SampleTaxonPageProp {
    pub sample_id: uuid::Uuid,
    pub taxon_id: i32,
}

impl From<&SampleTaxonPageProp> for PrimaryKey {
    fn from(prop: &SampleTaxonPageProp) -> Self {
        (prop.sample_id, prop.taxon_id).into()
    }
}

#[function_component(SampleTaxonPage)]
pub fn sample_taxon_page(props: &SampleTaxonPageProp) -> Html {
    let can_update = use_state(|| false);
    let can_admin = use_state(|| false);

    let can_update_callback = {
        let can_update = can_update.clone();
        Callback::from(move |value: bool| can_update.set(value))
    };
    let can_admin_callback = {
        let can_admin = can_admin.clone();
        Callback::from(move |value: bool| can_admin.set(value))
    };
    html! {
        <BasicPage<NestedSampleTaxon> id={PrimaryKey::from(props)} can_update={can_update_callback} can_admin={can_admin_callback}>
            <span>{"No content available yet."}</span>
        </BasicPage<NestedSampleTaxon>>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct SamplePageProp {
    pub id: uuid::Uuid,
}

impl From<&SamplePageProp> for PrimaryKey {
    fn from(prop: &SamplePageProp) -> Self {
        prop.id.into()
    }
}

impl SamplePageProp {
    fn filter_spectra_collections_by_sample_id(&self) -> SpectraCollectionFilter {
        let mut filter = SpectraCollectionFilter::default();
        filter.sample_id = Some(self.id);
        filter
    }
    fn filter_derived_samples_by_parent_sample_id(&self) -> DerivedSampleFilter {
        let mut filter = DerivedSampleFilter::default();
        filter.parent_sample_id = Some(self.id);
        filter
    }
    fn filter_derived_samples_by_child_sample_id(&self) -> DerivedSampleFilter {
        let mut filter = DerivedSampleFilter::default();
        filter.child_sample_id = Some(self.id);
        filter
    }
    fn filter_organisms_by_sample_id(&self) -> OrganismFilter {
        let mut filter = OrganismFilter::default();
        filter.sample_id = Some(self.id);
        filter
    }
    fn filter_sample_taxa_by_sample_id(&self) -> SampleTaxonFilter {
        let mut filter = SampleTaxonFilter::default();
        filter.sample_id = Some(self.id);
        filter
    }
    fn filter_observations_by_sample_id(&self) -> ObservationFilter {
        let mut filter = ObservationFilter::default();
        filter.sample_id = Some(self.id);
        filter
    }
}

#[function_component(SamplePage)]
pub fn sample_page(props: &SamplePageProp) -> Html {
    let can_update = use_state(|| false);
    let can_admin = use_state(|| false);

    let can_update_callback = {
        let can_update = can_update.clone();
        Callback::from(move |value: bool| can_update.set(value))
    };
    let can_admin_callback = {
        let can_admin = can_admin.clone();
        Callback::from(move |value: bool| can_admin.set(value))
    };
    html! {
        <BasicPage<NestedSample> id={PrimaryKey::from(props)} can_update={can_update_callback} can_admin={can_admin_callback}>
            // Linked with foreign key spectra_collections.sample_id
            <BasicList<NestedSpectraCollection> column_name={"sample_id"} filters={props.filter_spectra_collections_by_sample_id()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key derived_samples.parent_sample_id
            <BasicList<NestedDerivedSample> column_name={"parent_sample_id"} filters={props.filter_derived_samples_by_parent_sample_id()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key derived_samples.child_sample_id
            <BasicList<NestedDerivedSample> column_name={"child_sample_id"} filters={props.filter_derived_samples_by_child_sample_id()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key organisms.sample_id
            <BasicList<NestedOrganism> column_name={"sample_id"} filters={props.filter_organisms_by_sample_id()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key sample_taxa.sample_id
            <BasicList<NestedSampleTaxon> column_name={"sample_id"} filters={props.filter_sample_taxa_by_sample_id()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key observations.sample_id
            <BasicList<NestedObservation> column_name={"sample_id"} filters={props.filter_observations_by_sample_id()} can_create={*can_update} can_truncate={*can_admin}/>
        </BasicPage<NestedSample>>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct SpectraCollectionPageProp {
    pub id: i32,
}

impl From<&SpectraCollectionPageProp> for PrimaryKey {
    fn from(prop: &SpectraCollectionPageProp) -> Self {
        prop.id.into()
    }
}

impl SpectraCollectionPageProp {
    fn filter_spectra_by_spectra_collection_id(&self) -> SpectrumFilter {
        let mut filter = SpectrumFilter::default();
        filter.spectra_collection_id = Some(self.id);
        filter
    }
}

#[function_component(SpectraCollectionPage)]
pub fn spectra_collection_page(props: &SpectraCollectionPageProp) -> Html {
    let can_update = use_state(|| false);
    let can_admin = use_state(|| false);

    let can_update_callback = {
        let can_update = can_update.clone();
        Callback::from(move |value: bool| can_update.set(value))
    };
    let can_admin_callback = {
        let can_admin = can_admin.clone();
        Callback::from(move |value: bool| can_admin.set(value))
    };
    html! {
        <BasicPage<NestedSpectraCollection> id={PrimaryKey::from(props)} can_update={can_update_callback} can_admin={can_admin_callback}>
            <span>{"No content available yet."}</span>
        </BasicPage<NestedSpectraCollection>>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct TaxonPageProp {
    pub id: i32,
}

impl From<&TaxonPageProp> for PrimaryKey {
    fn from(prop: &TaxonPageProp) -> Self {
        prop.id.into()
    }
}

impl TaxonPageProp {
    fn filter_taxa_by_domain_id(&self) -> TaxonFilter {
        let mut filter = TaxonFilter::default();
        filter.domain_id = Some(self.id);
        filter
    }
    fn filter_taxa_by_kingdom_id(&self) -> TaxonFilter {
        let mut filter = TaxonFilter::default();
        filter.kingdom_id = Some(self.id);
        filter
    }
    fn filter_taxa_by_phylum_id(&self) -> TaxonFilter {
        let mut filter = TaxonFilter::default();
        filter.phylum_id = Some(self.id);
        filter
    }
    fn filter_taxa_by_class_id(&self) -> TaxonFilter {
        let mut filter = TaxonFilter::default();
        filter.class_id = Some(self.id);
        filter
    }
    fn filter_taxa_by_order_id(&self) -> TaxonFilter {
        let mut filter = TaxonFilter::default();
        filter.order_id = Some(self.id);
        filter
    }
    fn filter_taxa_by_family_id(&self) -> TaxonFilter {
        let mut filter = TaxonFilter::default();
        filter.family_id = Some(self.id);
        filter
    }
    fn filter_taxa_by_genus_id(&self) -> TaxonFilter {
        let mut filter = TaxonFilter::default();
        filter.genus_id = Some(self.id);
        filter
    }
    fn filter_taxa_by_parent_id(&self) -> TaxonFilter {
        let mut filter = TaxonFilter::default();
        filter.parent_id = Some(self.id);
        filter
    }
    fn filter_sample_taxa_by_taxon_id(&self) -> SampleTaxonFilter {
        let mut filter = SampleTaxonFilter::default();
        filter.taxon_id = Some(self.id);
        filter
    }
    fn filter_organism_taxa_by_taxon_id(&self) -> OrganismTaxonFilter {
        let mut filter = OrganismTaxonFilter::default();
        filter.taxon_id = Some(self.id);
        filter
    }
}

#[function_component(TaxonPage)]
pub fn taxon_page(props: &TaxonPageProp) -> Html {
    let can_update = use_state(|| false);
    let can_admin = use_state(|| false);

    let can_update_callback = {
        let can_update = can_update.clone();
        Callback::from(move |value: bool| can_update.set(value))
    };
    let can_admin_callback = {
        let can_admin = can_admin.clone();
        Callback::from(move |value: bool| can_admin.set(value))
    };
    html! {
        <BasicPage<NestedTaxon> id={PrimaryKey::from(props)} can_update={can_update_callback} can_admin={can_admin_callback}>
            // Linked with foreign key taxa.domain_id
            <BasicList<NestedTaxon> column_name={"domain_id"} filters={props.filter_taxa_by_domain_id()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key taxa.kingdom_id
            <BasicList<NestedTaxon> column_name={"kingdom_id"} filters={props.filter_taxa_by_kingdom_id()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key taxa.phylum_id
            <BasicList<NestedTaxon> column_name={"phylum_id"} filters={props.filter_taxa_by_phylum_id()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key taxa.class_id
            <BasicList<NestedTaxon> column_name={"class_id"} filters={props.filter_taxa_by_class_id()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key taxa.order_id
            <BasicList<NestedTaxon> column_name={"order_id"} filters={props.filter_taxa_by_order_id()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key taxa.family_id
            <BasicList<NestedTaxon> column_name={"family_id"} filters={props.filter_taxa_by_family_id()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key taxa.genus_id
            <BasicList<NestedTaxon> column_name={"genus_id"} filters={props.filter_taxa_by_genus_id()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key taxa.parent_id
            <BasicList<NestedTaxon> column_name={"parent_id"} filters={props.filter_taxa_by_parent_id()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key sample_taxa.taxon_id
            <BasicList<NestedSampleTaxon> column_name={"taxon_id"} filters={props.filter_sample_taxa_by_taxon_id()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key organism_taxa.taxon_id
            <BasicList<NestedOrganismTaxon> column_name={"taxon_id"} filters={props.filter_organism_taxa_by_taxon_id()} can_create={*can_update} can_truncate={*can_admin}/>
        </BasicPage<NestedTaxon>>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct TeamPageProp {
    pub id: i32,
}

impl From<&TeamPageProp> for PrimaryKey {
    fn from(prop: &TeamPageProp) -> Self {
        prop.id.into()
    }
}

impl TeamPageProp {
    fn filter_teams_by_parent_team_id(&self) -> TeamFilter {
        let mut filter = TeamFilter::default();
        filter.parent_team_id = Some(self.id);
        filter
    }
    fn filter_teams_teams_role_invitations_by_table_id(&self) -> TeamsTeamsRoleInvitationFilter {
        let mut filter = TeamsTeamsRoleInvitationFilter::default();
        filter.table_id = Some(self.id);
        filter
    }
    fn filter_teams_teams_role_invitations_by_team_id(&self) -> TeamsTeamsRoleInvitationFilter {
        let mut filter = TeamsTeamsRoleInvitationFilter::default();
        filter.team_id = Some(self.id);
        filter
    }
    fn filter_teams_users_role_invitations_by_table_id(&self) -> TeamsUsersRoleInvitationFilter {
        let mut filter = TeamsUsersRoleInvitationFilter::default();
        filter.table_id = Some(self.id);
        filter
    }
    fn filter_teams_users_role_requests_by_table_id(&self) -> TeamsUsersRoleRequestFilter {
        let mut filter = TeamsUsersRoleRequestFilter::default();
        filter.table_id = Some(self.id);
        filter
    }
    fn filter_teams_users_roles_by_table_id(&self) -> TeamsUsersRoleFilter {
        let mut filter = TeamsUsersRoleFilter::default();
        filter.table_id = Some(self.id);
        filter
    }
    fn filter_projects_teams_role_invitations_by_team_id(
        &self,
    ) -> ProjectsTeamsRoleInvitationFilter {
        let mut filter = ProjectsTeamsRoleInvitationFilter::default();
        filter.team_id = Some(self.id);
        filter
    }
    fn filter_projects_teams_role_requests_by_team_id(&self) -> ProjectsTeamsRoleRequestFilter {
        let mut filter = ProjectsTeamsRoleRequestFilter::default();
        filter.team_id = Some(self.id);
        filter
    }
    fn filter_projects_teams_roles_by_team_id(&self) -> ProjectsTeamsRoleFilter {
        let mut filter = ProjectsTeamsRoleFilter::default();
        filter.team_id = Some(self.id);
        filter
    }
}

#[function_component(TeamPage)]
pub fn team_page(props: &TeamPageProp) -> Html {
    let can_update = use_state(|| false);
    let can_admin = use_state(|| false);

    let can_update_callback = {
        let can_update = can_update.clone();
        Callback::from(move |value: bool| can_update.set(value))
    };
    let can_admin_callback = {
        let can_admin = can_admin.clone();
        Callback::from(move |value: bool| can_admin.set(value))
    };
    html! {
        <BasicPage<NestedTeam> id={PrimaryKey::from(props)} can_update={can_update_callback} can_admin={can_admin_callback}>
            // Linked with foreign key teams.parent_team_id
            <BasicList<NestedTeam> column_name={"parent_team_id"} filters={props.filter_teams_by_parent_team_id()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key teams_teams_role_invitations.table_id
            <BasicList<NestedTeamsTeamsRoleInvitation> column_name={"table_id"} filters={props.filter_teams_teams_role_invitations_by_table_id()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key teams_teams_role_invitations.team_id
            <BasicList<NestedTeamsTeamsRoleInvitation> column_name={"team_id"} filters={props.filter_teams_teams_role_invitations_by_team_id()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key teams_users_role_invitations.table_id
            <BasicList<NestedTeamsUsersRoleInvitation> column_name={"table_id"} filters={props.filter_teams_users_role_invitations_by_table_id()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key teams_users_role_requests.table_id
            <BasicList<NestedTeamsUsersRoleRequest> column_name={"table_id"} filters={props.filter_teams_users_role_requests_by_table_id()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key teams_users_roles.table_id
            <BasicList<NestedTeamsUsersRole> column_name={"table_id"} filters={props.filter_teams_users_roles_by_table_id()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key projects_teams_role_invitations.team_id
            <BasicList<NestedProjectsTeamsRoleInvitation> column_name={"team_id"} filters={props.filter_projects_teams_role_invitations_by_team_id()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key projects_teams_role_requests.team_id
            <BasicList<NestedProjectsTeamsRoleRequest> column_name={"team_id"} filters={props.filter_projects_teams_role_requests_by_team_id()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key projects_teams_roles.team_id
            <BasicList<NestedProjectsTeamsRole> column_name={"team_id"} filters={props.filter_projects_teams_roles_by_team_id()} can_create={*can_update} can_truncate={*can_admin}/>
        </BasicPage<NestedTeam>>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct TeamsTeamsRoleInvitationPageProp {
    pub table_id: i32,
    pub team_id: i32,
}

impl From<&TeamsTeamsRoleInvitationPageProp> for PrimaryKey {
    fn from(prop: &TeamsTeamsRoleInvitationPageProp) -> Self {
        (prop.table_id, prop.team_id).into()
    }
}

#[function_component(TeamsTeamsRoleInvitationPage)]
pub fn teams_teams_role_invitation_page(props: &TeamsTeamsRoleInvitationPageProp) -> Html {
    let can_update = use_state(|| false);
    let can_admin = use_state(|| false);

    let can_update_callback = {
        let can_update = can_update.clone();
        Callback::from(move |value: bool| can_update.set(value))
    };
    let can_admin_callback = {
        let can_admin = can_admin.clone();
        Callback::from(move |value: bool| can_admin.set(value))
    };
    html! {
        <BasicPage<NestedTeamsTeamsRoleInvitation> id={PrimaryKey::from(props)} can_update={can_update_callback} can_admin={can_admin_callback}>
            <span>{"No content available yet."}</span>
        </BasicPage<NestedTeamsTeamsRoleInvitation>>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct TeamsUsersRoleInvitationPageProp {
    pub table_id: i32,
    pub user_id: i32,
}

impl From<&TeamsUsersRoleInvitationPageProp> for PrimaryKey {
    fn from(prop: &TeamsUsersRoleInvitationPageProp) -> Self {
        (prop.table_id, prop.user_id).into()
    }
}

#[function_component(TeamsUsersRoleInvitationPage)]
pub fn teams_users_role_invitation_page(props: &TeamsUsersRoleInvitationPageProp) -> Html {
    let can_update = use_state(|| false);
    let can_admin = use_state(|| false);

    let can_update_callback = {
        let can_update = can_update.clone();
        Callback::from(move |value: bool| can_update.set(value))
    };
    let can_admin_callback = {
        let can_admin = can_admin.clone();
        Callback::from(move |value: bool| can_admin.set(value))
    };
    html! {
        <BasicPage<NestedTeamsUsersRoleInvitation> id={PrimaryKey::from(props)} can_update={can_update_callback} can_admin={can_admin_callback}>
            <span>{"No content available yet."}</span>
        </BasicPage<NestedTeamsUsersRoleInvitation>>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct TeamsUsersRoleRequestPageProp {
    pub table_id: i32,
    pub user_id: i32,
}

impl From<&TeamsUsersRoleRequestPageProp> for PrimaryKey {
    fn from(prop: &TeamsUsersRoleRequestPageProp) -> Self {
        (prop.table_id, prop.user_id).into()
    }
}

#[function_component(TeamsUsersRoleRequestPage)]
pub fn teams_users_role_request_page(props: &TeamsUsersRoleRequestPageProp) -> Html {
    let can_update = use_state(|| false);
    let can_admin = use_state(|| false);

    let can_update_callback = {
        let can_update = can_update.clone();
        Callback::from(move |value: bool| can_update.set(value))
    };
    let can_admin_callback = {
        let can_admin = can_admin.clone();
        Callback::from(move |value: bool| can_admin.set(value))
    };
    html! {
        <BasicPage<NestedTeamsUsersRoleRequest> id={PrimaryKey::from(props)} can_update={can_update_callback} can_admin={can_admin_callback}>
            <span>{"No content available yet."}</span>
        </BasicPage<NestedTeamsUsersRoleRequest>>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct TeamsUsersRolePageProp {
    pub table_id: i32,
    pub user_id: i32,
}

impl From<&TeamsUsersRolePageProp> for PrimaryKey {
    fn from(prop: &TeamsUsersRolePageProp) -> Self {
        (prop.table_id, prop.user_id).into()
    }
}

#[function_component(TeamsUsersRolePage)]
pub fn teams_users_role_page(props: &TeamsUsersRolePageProp) -> Html {
    let can_update = use_state(|| false);
    let can_admin = use_state(|| false);

    let can_update_callback = {
        let can_update = can_update.clone();
        Callback::from(move |value: bool| can_update.set(value))
    };
    let can_admin_callback = {
        let can_admin = can_admin.clone();
        Callback::from(move |value: bool| can_admin.set(value))
    };
    html! {
        <BasicPage<NestedTeamsUsersRole> id={PrimaryKey::from(props)} can_update={can_update_callback} can_admin={can_admin_callback}>
            <span>{"No content available yet."}</span>
        </BasicPage<NestedTeamsUsersRole>>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct UserPageProp {
    pub id: i32,
}

impl From<&UserPageProp> for PrimaryKey {
    fn from(prop: &UserPageProp) -> Self {
        prop.id.into()
    }
}

impl UserPageProp {
    fn filter_users_users_role_invitations_by_table_id(&self) -> UsersUsersRoleInvitationFilter {
        let mut filter = UsersUsersRoleInvitationFilter::default();
        filter.table_id = Some(self.id);
        filter
    }
    fn filter_users_users_role_invitations_by_user_id(&self) -> UsersUsersRoleInvitationFilter {
        let mut filter = UsersUsersRoleInvitationFilter::default();
        filter.user_id = Some(self.id);
        filter
    }
    fn filter_users_users_role_invitations_by_created_by(&self) -> UsersUsersRoleInvitationFilter {
        let mut filter = UsersUsersRoleInvitationFilter::default();
        filter.created_by = Some(self.id);
        filter
    }
    fn filter_users_users_role_requests_by_table_id(&self) -> UsersUsersRoleRequestFilter {
        let mut filter = UsersUsersRoleRequestFilter::default();
        filter.table_id = Some(self.id);
        filter
    }
    fn filter_users_users_role_requests_by_user_id(&self) -> UsersUsersRoleRequestFilter {
        let mut filter = UsersUsersRoleRequestFilter::default();
        filter.user_id = Some(self.id);
        filter
    }
    fn filter_users_users_role_requests_by_created_by(&self) -> UsersUsersRoleRequestFilter {
        let mut filter = UsersUsersRoleRequestFilter::default();
        filter.created_by = Some(self.id);
        filter
    }
    fn filter_users_users_roles_by_table_id(&self) -> UsersUsersRoleFilter {
        let mut filter = UsersUsersRoleFilter::default();
        filter.table_id = Some(self.id);
        filter
    }
    fn filter_users_users_roles_by_user_id(&self) -> UsersUsersRoleFilter {
        let mut filter = UsersUsersRoleFilter::default();
        filter.user_id = Some(self.id);
        filter
    }
    fn filter_users_users_roles_by_created_by(&self) -> UsersUsersRoleFilter {
        let mut filter = UsersUsersRoleFilter::default();
        filter.created_by = Some(self.id);
        filter
    }
    fn filter_projects_by_created_by(&self) -> ProjectFilter {
        let mut filter = ProjectFilter::default();
        filter.created_by = Some(self.id);
        filter
    }
    fn filter_projects_by_updated_by(&self) -> ProjectFilter {
        let mut filter = ProjectFilter::default();
        filter.updated_by = Some(self.id);
        filter
    }
    fn filter_projects_users_role_invitations_by_user_id(
        &self,
    ) -> ProjectsUsersRoleInvitationFilter {
        let mut filter = ProjectsUsersRoleInvitationFilter::default();
        filter.user_id = Some(self.id);
        filter
    }
    fn filter_projects_users_role_invitations_by_created_by(
        &self,
    ) -> ProjectsUsersRoleInvitationFilter {
        let mut filter = ProjectsUsersRoleInvitationFilter::default();
        filter.created_by = Some(self.id);
        filter
    }
    fn filter_projects_users_role_requests_by_user_id(&self) -> ProjectsUsersRoleRequestFilter {
        let mut filter = ProjectsUsersRoleRequestFilter::default();
        filter.user_id = Some(self.id);
        filter
    }
    fn filter_projects_users_role_requests_by_created_by(&self) -> ProjectsUsersRoleRequestFilter {
        let mut filter = ProjectsUsersRoleRequestFilter::default();
        filter.created_by = Some(self.id);
        filter
    }
    fn filter_projects_users_roles_by_user_id(&self) -> ProjectsUsersRoleFilter {
        let mut filter = ProjectsUsersRoleFilter::default();
        filter.user_id = Some(self.id);
        filter
    }
    fn filter_projects_users_roles_by_created_by(&self) -> ProjectsUsersRoleFilter {
        let mut filter = ProjectsUsersRoleFilter::default();
        filter.created_by = Some(self.id);
        filter
    }
    fn filter_sample_containers_by_created_by(&self) -> SampleContainerFilter {
        let mut filter = SampleContainerFilter::default();
        filter.created_by = Some(self.id);
        filter
    }
    fn filter_sample_containers_by_updated_by(&self) -> SampleContainerFilter {
        let mut filter = SampleContainerFilter::default();
        filter.updated_by = Some(self.id);
        filter
    }
    fn filter_samples_by_created_by(&self) -> SampleFilter {
        let mut filter = SampleFilter::default();
        filter.created_by = Some(self.id);
        filter
    }
    fn filter_samples_by_sampled_by(&self) -> SampleFilter {
        let mut filter = SampleFilter::default();
        filter.sampled_by = Some(self.id);
        filter
    }
    fn filter_samples_by_updated_by(&self) -> SampleFilter {
        let mut filter = SampleFilter::default();
        filter.updated_by = Some(self.id);
        filter
    }
    fn filter_spectra_collections_by_created_by(&self) -> SpectraCollectionFilter {
        let mut filter = SpectraCollectionFilter::default();
        filter.created_by = Some(self.id);
        filter
    }
    fn filter_spectra_collections_by_updated_by(&self) -> SpectraCollectionFilter {
        let mut filter = SpectraCollectionFilter::default();
        filter.updated_by = Some(self.id);
        filter
    }
    fn filter_teams_by_created_by(&self) -> TeamFilter {
        let mut filter = TeamFilter::default();
        filter.created_by = Some(self.id);
        filter
    }
    fn filter_teams_by_updated_by(&self) -> TeamFilter {
        let mut filter = TeamFilter::default();
        filter.updated_by = Some(self.id);
        filter
    }
    fn filter_teams_teams_role_invitations_by_created_by(&self) -> TeamsTeamsRoleInvitationFilter {
        let mut filter = TeamsTeamsRoleInvitationFilter::default();
        filter.created_by = Some(self.id);
        filter
    }
    fn filter_teams_users_role_invitations_by_user_id(&self) -> TeamsUsersRoleInvitationFilter {
        let mut filter = TeamsUsersRoleInvitationFilter::default();
        filter.user_id = Some(self.id);
        filter
    }
    fn filter_teams_users_role_invitations_by_created_by(&self) -> TeamsUsersRoleInvitationFilter {
        let mut filter = TeamsUsersRoleInvitationFilter::default();
        filter.created_by = Some(self.id);
        filter
    }
    fn filter_teams_users_role_requests_by_user_id(&self) -> TeamsUsersRoleRequestFilter {
        let mut filter = TeamsUsersRoleRequestFilter::default();
        filter.user_id = Some(self.id);
        filter
    }
    fn filter_teams_users_role_requests_by_created_by(&self) -> TeamsUsersRoleRequestFilter {
        let mut filter = TeamsUsersRoleRequestFilter::default();
        filter.created_by = Some(self.id);
        filter
    }
    fn filter_teams_users_roles_by_user_id(&self) -> TeamsUsersRoleFilter {
        let mut filter = TeamsUsersRoleFilter::default();
        filter.user_id = Some(self.id);
        filter
    }
    fn filter_teams_users_roles_by_created_by(&self) -> TeamsUsersRoleFilter {
        let mut filter = TeamsUsersRoleFilter::default();
        filter.created_by = Some(self.id);
        filter
    }
    fn filter_derived_samples_by_created_by(&self) -> DerivedSampleFilter {
        let mut filter = DerivedSampleFilter::default();
        filter.created_by = Some(self.id);
        filter
    }
    fn filter_derived_samples_by_updated_by(&self) -> DerivedSampleFilter {
        let mut filter = DerivedSampleFilter::default();
        filter.updated_by = Some(self.id);
        filter
    }
    fn filter_nameplates_by_created_by(&self) -> NameplateFilter {
        let mut filter = NameplateFilter::default();
        filter.created_by = Some(self.id);
        filter
    }
    fn filter_nameplates_by_updated_by(&self) -> NameplateFilter {
        let mut filter = NameplateFilter::default();
        filter.updated_by = Some(self.id);
        filter
    }
    fn filter_organisms_by_created_by(&self) -> OrganismFilter {
        let mut filter = OrganismFilter::default();
        filter.created_by = Some(self.id);
        filter
    }
    fn filter_organisms_by_updated_by(&self) -> OrganismFilter {
        let mut filter = OrganismFilter::default();
        filter.updated_by = Some(self.id);
        filter
    }
    fn filter_projects_teams_role_invitations_by_created_by(
        &self,
    ) -> ProjectsTeamsRoleInvitationFilter {
        let mut filter = ProjectsTeamsRoleInvitationFilter::default();
        filter.created_by = Some(self.id);
        filter
    }
    fn filter_projects_teams_role_requests_by_created_by(&self) -> ProjectsTeamsRoleRequestFilter {
        let mut filter = ProjectsTeamsRoleRequestFilter::default();
        filter.created_by = Some(self.id);
        filter
    }
    fn filter_projects_teams_roles_by_created_by(&self) -> ProjectsTeamsRoleFilter {
        let mut filter = ProjectsTeamsRoleFilter::default();
        filter.created_by = Some(self.id);
        filter
    }
    fn filter_sample_taxa_by_created_by(&self) -> SampleTaxonFilter {
        let mut filter = SampleTaxonFilter::default();
        filter.created_by = Some(self.id);
        filter
    }
    fn filter_observations_by_created_by(&self) -> ObservationFilter {
        let mut filter = ObservationFilter::default();
        filter.created_by = Some(self.id);
        filter
    }
    fn filter_observations_by_updated_by(&self) -> ObservationFilter {
        let mut filter = ObservationFilter::default();
        filter.updated_by = Some(self.id);
        filter
    }
    fn filter_organism_taxa_by_created_by(&self) -> OrganismTaxonFilter {
        let mut filter = OrganismTaxonFilter::default();
        filter.created_by = Some(self.id);
        filter
    }
}

#[function_component(UserPage)]
pub fn user_page(props: &UserPageProp) -> Html {
    let can_update = use_state(|| false);
    let can_admin = use_state(|| false);

    let can_update_callback = {
        let can_update = can_update.clone();
        Callback::from(move |value: bool| can_update.set(value))
    };
    let can_admin_callback = {
        let can_admin = can_admin.clone();
        Callback::from(move |value: bool| can_admin.set(value))
    };
    html! {
        <BasicPage<NestedUser> id={PrimaryKey::from(props)} can_update={can_update_callback} can_admin={can_admin_callback}>
            // Linked with foreign key users_users_role_invitations.table_id
            <BasicList<NestedUsersUsersRoleInvitation> column_name={"table_id"} filters={props.filter_users_users_role_invitations_by_table_id()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key users_users_role_invitations.user_id
            <BasicList<NestedUsersUsersRoleInvitation> column_name={"user_id"} filters={props.filter_users_users_role_invitations_by_user_id()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key users_users_role_invitations.created_by
            <BasicList<NestedUsersUsersRoleInvitation> column_name={"created_by"} filters={props.filter_users_users_role_invitations_by_created_by()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key users_users_role_requests.table_id
            <BasicList<NestedUsersUsersRoleRequest> column_name={"table_id"} filters={props.filter_users_users_role_requests_by_table_id()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key users_users_role_requests.user_id
            <BasicList<NestedUsersUsersRoleRequest> column_name={"user_id"} filters={props.filter_users_users_role_requests_by_user_id()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key users_users_role_requests.created_by
            <BasicList<NestedUsersUsersRoleRequest> column_name={"created_by"} filters={props.filter_users_users_role_requests_by_created_by()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key users_users_roles.table_id
            <BasicList<NestedUsersUsersRole> column_name={"table_id"} filters={props.filter_users_users_roles_by_table_id()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key users_users_roles.user_id
            <BasicList<NestedUsersUsersRole> column_name={"user_id"} filters={props.filter_users_users_roles_by_user_id()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key users_users_roles.created_by
            <BasicList<NestedUsersUsersRole> column_name={"created_by"} filters={props.filter_users_users_roles_by_created_by()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key projects.created_by
            <BasicList<NestedProject> column_name={"created_by"} filters={props.filter_projects_by_created_by()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key projects.updated_by
            <BasicList<NestedProject> column_name={"updated_by"} filters={props.filter_projects_by_updated_by()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key projects_users_role_invitations.user_id
            <BasicList<NestedProjectsUsersRoleInvitation> column_name={"user_id"} filters={props.filter_projects_users_role_invitations_by_user_id()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key projects_users_role_invitations.created_by
            <BasicList<NestedProjectsUsersRoleInvitation> column_name={"created_by"} filters={props.filter_projects_users_role_invitations_by_created_by()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key projects_users_role_requests.user_id
            <BasicList<NestedProjectsUsersRoleRequest> column_name={"user_id"} filters={props.filter_projects_users_role_requests_by_user_id()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key projects_users_role_requests.created_by
            <BasicList<NestedProjectsUsersRoleRequest> column_name={"created_by"} filters={props.filter_projects_users_role_requests_by_created_by()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key projects_users_roles.user_id
            <BasicList<NestedProjectsUsersRole> column_name={"user_id"} filters={props.filter_projects_users_roles_by_user_id()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key projects_users_roles.created_by
            <BasicList<NestedProjectsUsersRole> column_name={"created_by"} filters={props.filter_projects_users_roles_by_created_by()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key sample_containers.created_by
            <BasicList<NestedSampleContainer> column_name={"created_by"} filters={props.filter_sample_containers_by_created_by()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key sample_containers.updated_by
            <BasicList<NestedSampleContainer> column_name={"updated_by"} filters={props.filter_sample_containers_by_updated_by()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key samples.created_by
            <BasicList<NestedSample> column_name={"created_by"} filters={props.filter_samples_by_created_by()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key samples.sampled_by
            <BasicList<NestedSample> column_name={"sampled_by"} filters={props.filter_samples_by_sampled_by()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key samples.updated_by
            <BasicList<NestedSample> column_name={"updated_by"} filters={props.filter_samples_by_updated_by()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key spectra_collections.created_by
            <BasicList<NestedSpectraCollection> column_name={"created_by"} filters={props.filter_spectra_collections_by_created_by()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key spectra_collections.updated_by
            <BasicList<NestedSpectraCollection> column_name={"updated_by"} filters={props.filter_spectra_collections_by_updated_by()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key teams.created_by
            <BasicList<NestedTeam> column_name={"created_by"} filters={props.filter_teams_by_created_by()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key teams.updated_by
            <BasicList<NestedTeam> column_name={"updated_by"} filters={props.filter_teams_by_updated_by()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key teams_teams_role_invitations.created_by
            <BasicList<NestedTeamsTeamsRoleInvitation> column_name={"created_by"} filters={props.filter_teams_teams_role_invitations_by_created_by()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key teams_users_role_invitations.user_id
            <BasicList<NestedTeamsUsersRoleInvitation> column_name={"user_id"} filters={props.filter_teams_users_role_invitations_by_user_id()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key teams_users_role_invitations.created_by
            <BasicList<NestedTeamsUsersRoleInvitation> column_name={"created_by"} filters={props.filter_teams_users_role_invitations_by_created_by()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key teams_users_role_requests.user_id
            <BasicList<NestedTeamsUsersRoleRequest> column_name={"user_id"} filters={props.filter_teams_users_role_requests_by_user_id()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key teams_users_role_requests.created_by
            <BasicList<NestedTeamsUsersRoleRequest> column_name={"created_by"} filters={props.filter_teams_users_role_requests_by_created_by()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key teams_users_roles.user_id
            <BasicList<NestedTeamsUsersRole> column_name={"user_id"} filters={props.filter_teams_users_roles_by_user_id()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key teams_users_roles.created_by
            <BasicList<NestedTeamsUsersRole> column_name={"created_by"} filters={props.filter_teams_users_roles_by_created_by()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key derived_samples.created_by
            <BasicList<NestedDerivedSample> column_name={"created_by"} filters={props.filter_derived_samples_by_created_by()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key derived_samples.updated_by
            <BasicList<NestedDerivedSample> column_name={"updated_by"} filters={props.filter_derived_samples_by_updated_by()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key nameplates.created_by
            <BasicList<NestedNameplate> column_name={"created_by"} filters={props.filter_nameplates_by_created_by()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key nameplates.updated_by
            <BasicList<NestedNameplate> column_name={"updated_by"} filters={props.filter_nameplates_by_updated_by()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key organisms.created_by
            <BasicList<NestedOrganism> column_name={"created_by"} filters={props.filter_organisms_by_created_by()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key organisms.updated_by
            <BasicList<NestedOrganism> column_name={"updated_by"} filters={props.filter_organisms_by_updated_by()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key projects_teams_role_invitations.created_by
            <BasicList<NestedProjectsTeamsRoleInvitation> column_name={"created_by"} filters={props.filter_projects_teams_role_invitations_by_created_by()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key projects_teams_role_requests.created_by
            <BasicList<NestedProjectsTeamsRoleRequest> column_name={"created_by"} filters={props.filter_projects_teams_role_requests_by_created_by()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key projects_teams_roles.created_by
            <BasicList<NestedProjectsTeamsRole> column_name={"created_by"} filters={props.filter_projects_teams_roles_by_created_by()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key sample_taxa.created_by
            <BasicList<NestedSampleTaxon> column_name={"created_by"} filters={props.filter_sample_taxa_by_created_by()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key observations.created_by
            <BasicList<NestedObservation> column_name={"created_by"} filters={props.filter_observations_by_created_by()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key observations.updated_by
            <BasicList<NestedObservation> column_name={"updated_by"} filters={props.filter_observations_by_updated_by()} can_create={*can_update} can_truncate={*can_admin}/>
            // Linked with foreign key organism_taxa.created_by
            <BasicList<NestedOrganismTaxon> column_name={"created_by"} filters={props.filter_organism_taxa_by_created_by()} can_create={*can_update} can_truncate={*can_admin}/>
        </BasicPage<NestedUser>>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct UsersUsersRoleInvitationPageProp {
    pub table_id: i32,
    pub user_id: i32,
}

impl From<&UsersUsersRoleInvitationPageProp> for PrimaryKey {
    fn from(prop: &UsersUsersRoleInvitationPageProp) -> Self {
        (prop.table_id, prop.user_id).into()
    }
}

#[function_component(UsersUsersRoleInvitationPage)]
pub fn users_users_role_invitation_page(props: &UsersUsersRoleInvitationPageProp) -> Html {
    let can_update = use_state(|| false);
    let can_admin = use_state(|| false);

    let can_update_callback = {
        let can_update = can_update.clone();
        Callback::from(move |value: bool| can_update.set(value))
    };
    let can_admin_callback = {
        let can_admin = can_admin.clone();
        Callback::from(move |value: bool| can_admin.set(value))
    };
    html! {
        <BasicPage<NestedUsersUsersRoleInvitation> id={PrimaryKey::from(props)} can_update={can_update_callback} can_admin={can_admin_callback}>
            <span>{"No content available yet."}</span>
        </BasicPage<NestedUsersUsersRoleInvitation>>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct UsersUsersRoleRequestPageProp {
    pub table_id: i32,
    pub user_id: i32,
}

impl From<&UsersUsersRoleRequestPageProp> for PrimaryKey {
    fn from(prop: &UsersUsersRoleRequestPageProp) -> Self {
        (prop.table_id, prop.user_id).into()
    }
}

#[function_component(UsersUsersRoleRequestPage)]
pub fn users_users_role_request_page(props: &UsersUsersRoleRequestPageProp) -> Html {
    let can_update = use_state(|| false);
    let can_admin = use_state(|| false);

    let can_update_callback = {
        let can_update = can_update.clone();
        Callback::from(move |value: bool| can_update.set(value))
    };
    let can_admin_callback = {
        let can_admin = can_admin.clone();
        Callback::from(move |value: bool| can_admin.set(value))
    };
    html! {
        <BasicPage<NestedUsersUsersRoleRequest> id={PrimaryKey::from(props)} can_update={can_update_callback} can_admin={can_admin_callback}>
            <span>{"No content available yet."}</span>
        </BasicPage<NestedUsersUsersRoleRequest>>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct UsersUsersRolePageProp {
    pub table_id: i32,
    pub user_id: i32,
}

impl From<&UsersUsersRolePageProp> for PrimaryKey {
    fn from(prop: &UsersUsersRolePageProp) -> Self {
        (prop.table_id, prop.user_id).into()
    }
}

#[function_component(UsersUsersRolePage)]
pub fn users_users_role_page(props: &UsersUsersRolePageProp) -> Html {
    let can_update = use_state(|| false);
    let can_admin = use_state(|| false);

    let can_update_callback = {
        let can_update = can_update.clone();
        Callback::from(move |value: bool| can_update.set(value))
    };
    let can_admin_callback = {
        let can_admin = can_admin.clone();
        Callback::from(move |value: bool| can_admin.set(value))
    };
    html! {
        <BasicPage<NestedUsersUsersRole> id={PrimaryKey::from(props)} can_update={can_update_callback} can_admin={can_admin_callback}>
            <span>{"No content available yet."}</span>
        </BasicPage<NestedUsersUsersRole>>
    }
}
