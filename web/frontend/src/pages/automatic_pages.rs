use yew::prelude::*;
use uuid::Uuid;
use web_common::database::*;
use crate::components::*;

#[derive(Clone, PartialEq, Properties)]
pub struct ObservationPageProp {
    pub id: Uuid,
}

impl From<&ObservationPageProp> for PrimaryKey {
    fn from(prop: &ObservationPageProp) -> Self {
        prop.id.into()
    }
}

impl ObservationPageProp {
}

#[function_component(ObservationPage)]
pub fn observation_page(props: &ObservationPageProp) -> Html {
    html! {
        <BasicPage<NestedObservation> id={PrimaryKey::from(props)}>
            <span>{"No content available yet."}</span>
        </BasicPage<NestedObservation>>
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
    fn filter_sample_containers_by_project_id(&self) -> SampleContainerFilter {
        let mut filter = SampleContainerFilter::default();
        filter.project_id = Some(self.id);
        filter
    }
    fn filter_sampled_individuals_by_project_id(&self) -> SampledIndividualFilter {
        let mut filter = SampledIndividualFilter::default();
        filter.project_id = Some(self.id);
        filter
    }
    fn filter_samples_by_project_id(&self) -> SampleFilter {
        let mut filter = SampleFilter::default();
        filter.project_id = Some(self.id);
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
    html! {
        <BasicPage<NestedProject> id={PrimaryKey::from(props)}>
            // Linked with foreign key projects.parent_project_id
            <BasicList<NestedProject> filters={props.filter_projects_by_parent_project_id()}/>
            // Linked with foreign key sample_containers.project_id
            <BasicList<NestedSampleContainer> filters={props.filter_sample_containers_by_project_id()}/>
            // Linked with foreign key sampled_individuals.project_id
            <BasicList<NestedSampledIndividual> filters={props.filter_sampled_individuals_by_project_id()}/>
            // Linked with foreign key samples.project_id
            <BasicList<NestedSample> filters={props.filter_samples_by_project_id()}/>
            // Linked with foreign key observations.project_id
            <BasicList<NestedObservation> filters={props.filter_observations_by_project_id()}/>
        </BasicPage<NestedProject>>
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
    html! {
        <BasicPage<NestedSampleContainer> id={PrimaryKey::from(props)}>
            // Linked with foreign key samples.container_id
            <BasicList<NestedSample> filters={props.filter_samples_by_container_id()}/>
        </BasicPage<NestedSampleContainer>>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct SampledIndividualPageProp {
    pub id: Uuid,
}

impl From<&SampledIndividualPageProp> for PrimaryKey {
    fn from(prop: &SampledIndividualPageProp) -> Self {
        prop.id.into()
    }
}

impl SampledIndividualPageProp {
    fn filter_observations_by_individual_id(&self) -> ObservationFilter {
        let mut filter = ObservationFilter::default();
        filter.individual_id = Some(self.id);
        filter
    }
    fn filter_sampled_individual_bio_ott_taxon_items_by_sampled_individual_id(&self) -> SampledIndividualBioOttTaxonItemFilter {
        let mut filter = SampledIndividualBioOttTaxonItemFilter::default();
        filter.sampled_individual_id = Some(self.id);
        filter
    }
}

#[function_component(SampledIndividualPage)]
pub fn sampled_individual_page(props: &SampledIndividualPageProp) -> Html {
    html! {
        <BasicPage<NestedSampledIndividual> id={PrimaryKey::from(props)}>
            // Linked with foreign key observations.individual_id
            <BasicList<NestedObservation> filters={props.filter_observations_by_individual_id()}/>
        </BasicPage<NestedSampledIndividual>>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct SamplePageProp {
    pub id: Uuid,
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
    fn filter_sample_bio_ott_taxon_items_by_sample_id(&self) -> SampleBioOttTaxonItemFilter {
        let mut filter = SampleBioOttTaxonItemFilter::default();
        filter.sample_id = Some(self.id);
        filter
    }
}

#[function_component(SamplePage)]
pub fn sample_page(props: &SamplePageProp) -> Html {
    html! {
        <BasicPage<NestedSample> id={PrimaryKey::from(props)}>
            // Linked with foreign key spectra_collections.sample_id
            <BasicList<NestedSpectraCollection> filters={props.filter_spectra_collections_by_sample_id()}/>
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
    fn filter_spectra_by_spectra_collection_id(&self) -> SpectraFilter {
        let mut filter = SpectraFilter::default();
        filter.spectra_collection_id = Some(self.id);
        filter
    }
}

#[function_component(SpectraCollectionPage)]
pub fn spectra_collection_page(props: &SpectraCollectionPageProp) -> Html {
    html! {
        <BasicPage<NestedSpectraCollection> id={PrimaryKey::from(props)}>
            // Linked with foreign key spectra.spectra_collection_id
            <BasicList<NestedSpectra> filters={props.filter_spectra_by_spectra_collection_id()}/>
        </BasicPage<NestedSpectraCollection>>
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
}

#[function_component(TeamPage)]
pub fn team_page(props: &TeamPageProp) -> Html {
    html! {
        <BasicPage<NestedTeam> id={PrimaryKey::from(props)}>
            // Linked with foreign key teams.parent_team_id
            <BasicList<NestedTeam> filters={props.filter_teams_by_parent_team_id()}/>
        </BasicPage<NestedTeam>>
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
    fn filter_derived_samples_by_created_by(&self) -> DerivedSampleFilter {
        let mut filter = DerivedSampleFilter::default();
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
    fn filter_sample_bio_ott_taxon_items_by_created_by(&self) -> SampleBioOttTaxonItemFilter {
        let mut filter = SampleBioOttTaxonItemFilter::default();
        filter.created_by = Some(self.id);
        filter
    }
    fn filter_sample_containers_by_created_by(&self) -> SampleContainerFilter {
        let mut filter = SampleContainerFilter::default();
        filter.created_by = Some(self.id);
        filter
    }
    fn filter_sampled_individual_bio_ott_taxon_items_by_created_by(&self) -> SampledIndividualBioOttTaxonItemFilter {
        let mut filter = SampledIndividualBioOttTaxonItemFilter::default();
        filter.created_by = Some(self.id);
        filter
    }
    fn filter_sampled_individuals_by_created_by(&self) -> SampledIndividualFilter {
        let mut filter = SampledIndividualFilter::default();
        filter.created_by = Some(self.id);
        filter
    }
    fn filter_sampled_individuals_by_updated_by(&self) -> SampledIndividualFilter {
        let mut filter = SampledIndividualFilter::default();
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
}

#[function_component(UserPage)]
pub fn user_page(props: &UserPageProp) -> Html {
    html! {
        <BasicPage<User> id={PrimaryKey::from(props)}>
            // Linked with foreign key observations.created_by
            <BasicList<NestedObservation> filters={props.filter_observations_by_created_by()}/>
            // Linked with foreign key observations.updated_by
            <BasicList<NestedObservation> filters={props.filter_observations_by_updated_by()}/>
            // Linked with foreign key projects.created_by
            <BasicList<NestedProject> filters={props.filter_projects_by_created_by()}/>
            // Linked with foreign key projects.updated_by
            <BasicList<NestedProject> filters={props.filter_projects_by_updated_by()}/>
            // Linked with foreign key sample_containers.created_by
            <BasicList<NestedSampleContainer> filters={props.filter_sample_containers_by_created_by()}/>
            // Linked with foreign key sampled_individuals.created_by
            <BasicList<NestedSampledIndividual> filters={props.filter_sampled_individuals_by_created_by()}/>
            // Linked with foreign key sampled_individuals.updated_by
            <BasicList<NestedSampledIndividual> filters={props.filter_sampled_individuals_by_updated_by()}/>
            // Linked with foreign key samples.created_by
            <BasicList<NestedSample> filters={props.filter_samples_by_created_by()}/>
            // Linked with foreign key samples.sampled_by
            <BasicList<NestedSample> filters={props.filter_samples_by_sampled_by()}/>
            // Linked with foreign key samples.updated_by
            <BasicList<NestedSample> filters={props.filter_samples_by_updated_by()}/>
            // Linked with foreign key spectra_collections.created_by
            <BasicList<NestedSpectraCollection> filters={props.filter_spectra_collections_by_created_by()}/>
            // Linked with foreign key spectra_collections.updated_by
            <BasicList<NestedSpectraCollection> filters={props.filter_spectra_collections_by_updated_by()}/>
            // Linked with foreign key teams.created_by
            <BasicList<NestedTeam> filters={props.filter_teams_by_created_by()}/>
            // Linked with foreign key teams.updated_by
            <BasicList<NestedTeam> filters={props.filter_teams_by_updated_by()}/>
        </BasicPage<User>>
    }
}

