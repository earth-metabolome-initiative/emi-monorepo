use yew::prelude::*;
use uuid::Uuid;
use web_common::database::*;
use crate::components::*;

#[derive(Clone, PartialEq, Properties)]
pub struct BioOttRankPageProp {
    pub id: i32,
}

impl From<&BioOttRankPageProp> for PrimaryKey {
    fn from(prop: &BioOttRankPageProp) -> Self {
        prop.id.into()
    }
}

impl BioOttRankPageProp {
    fn filter_bio_ott_taxon_items_by_ott_rank_id(&self) -> BioOttTaxonItemFilter {
        let mut filter = BioOttTaxonItemFilter::default();
        filter.ott_rank_id = Some(self.id);
        filter
    }
}

#[function_component(BioOttRankPage)]
pub fn bio_ott_rank_page(props: &BioOttRankPageProp) -> Html {
    html! {
        <BasicPage<NestedBioOttRank> id={PrimaryKey::from(props)}>
            // Linked with foreign key bio_ott_taxon_items.ott_rank_id
            <BasicList<NestedBioOttTaxonItem> filters={props.filter_bio_ott_taxon_items_by_ott_rank_id()}/>
        </BasicPage<NestedBioOttRank>>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct BioOttTaxonItemPageProp {
    pub id: i32,
}

impl From<&BioOttTaxonItemPageProp> for PrimaryKey {
    fn from(prop: &BioOttTaxonItemPageProp) -> Self {
        prop.id.into()
    }
}

impl BioOttTaxonItemPageProp {
    fn filter_bio_ott_taxon_items_by_domain_id(&self) -> BioOttTaxonItemFilter {
        let mut filter = BioOttTaxonItemFilter::default();
        filter.domain_id = Some(self.id);
        filter
    }
    fn filter_bio_ott_taxon_items_by_kingdom_id(&self) -> BioOttTaxonItemFilter {
        let mut filter = BioOttTaxonItemFilter::default();
        filter.kingdom_id = Some(self.id);
        filter
    }
    fn filter_bio_ott_taxon_items_by_phylum_id(&self) -> BioOttTaxonItemFilter {
        let mut filter = BioOttTaxonItemFilter::default();
        filter.phylum_id = Some(self.id);
        filter
    }
    fn filter_bio_ott_taxon_items_by_class_id(&self) -> BioOttTaxonItemFilter {
        let mut filter = BioOttTaxonItemFilter::default();
        filter.class_id = Some(self.id);
        filter
    }
    fn filter_bio_ott_taxon_items_by_order_id(&self) -> BioOttTaxonItemFilter {
        let mut filter = BioOttTaxonItemFilter::default();
        filter.order_id = Some(self.id);
        filter
    }
    fn filter_bio_ott_taxon_items_by_family_id(&self) -> BioOttTaxonItemFilter {
        let mut filter = BioOttTaxonItemFilter::default();
        filter.family_id = Some(self.id);
        filter
    }
    fn filter_bio_ott_taxon_items_by_genus_id(&self) -> BioOttTaxonItemFilter {
        let mut filter = BioOttTaxonItemFilter::default();
        filter.genus_id = Some(self.id);
        filter
    }
    fn filter_bio_ott_taxon_items_by_parent_id(&self) -> BioOttTaxonItemFilter {
        let mut filter = BioOttTaxonItemFilter::default();
        filter.parent_id = Some(self.id);
        filter
    }
    fn filter_sample_bio_ott_taxon_items_by_taxon_id(&self) -> SampleBioOttTaxonItemFilter {
        let mut filter = SampleBioOttTaxonItemFilter::default();
        filter.taxon_id = Some(self.id);
        filter
    }
    fn filter_sampled_individual_bio_ott_taxon_items_by_taxon_id(&self) -> SampledIndividualBioOttTaxonItemFilter {
        let mut filter = SampledIndividualBioOttTaxonItemFilter::default();
        filter.taxon_id = Some(self.id);
        filter
    }
}

#[function_component(BioOttTaxonItemPage)]
pub fn bio_ott_taxon_item_page(props: &BioOttTaxonItemPageProp) -> Html {
    html! {
        <BasicPage<NestedBioOttTaxonItem> id={PrimaryKey::from(props)}>
            // Linked with foreign key bio_ott_taxon_items.domain_id
            <BasicList<NestedBioOttTaxonItem> filters={props.filter_bio_ott_taxon_items_by_domain_id()}/>
            // Linked with foreign key bio_ott_taxon_items.kingdom_id
            <BasicList<NestedBioOttTaxonItem> filters={props.filter_bio_ott_taxon_items_by_kingdom_id()}/>
            // Linked with foreign key bio_ott_taxon_items.phylum_id
            <BasicList<NestedBioOttTaxonItem> filters={props.filter_bio_ott_taxon_items_by_phylum_id()}/>
            // Linked with foreign key bio_ott_taxon_items.class_id
            <BasicList<NestedBioOttTaxonItem> filters={props.filter_bio_ott_taxon_items_by_class_id()}/>
            // Linked with foreign key bio_ott_taxon_items.order_id
            <BasicList<NestedBioOttTaxonItem> filters={props.filter_bio_ott_taxon_items_by_order_id()}/>
            // Linked with foreign key bio_ott_taxon_items.family_id
            <BasicList<NestedBioOttTaxonItem> filters={props.filter_bio_ott_taxon_items_by_family_id()}/>
            // Linked with foreign key bio_ott_taxon_items.genus_id
            <BasicList<NestedBioOttTaxonItem> filters={props.filter_bio_ott_taxon_items_by_genus_id()}/>
            // Linked with foreign key bio_ott_taxon_items.parent_id
            <BasicList<NestedBioOttTaxonItem> filters={props.filter_bio_ott_taxon_items_by_parent_id()}/>
        </BasicPage<NestedBioOttTaxonItem>>
    }
}

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
    html! {
        <BasicPage<Country> id={PrimaryKey::from(props)}>
            // Linked with foreign key organizations.country_id
            <BasicList<NestedOrganization> filters={props.filter_organizations_by_country_id()}/>
        </BasicPage<Country>>
    }
}

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
pub struct OrganizationPageProp {
    pub id: i32,
}

impl From<&OrganizationPageProp> for PrimaryKey {
    fn from(prop: &OrganizationPageProp) -> Self {
        prop.id.into()
    }
}

impl OrganizationPageProp {
}

#[function_component(OrganizationPage)]
pub fn organization_page(props: &OrganizationPageProp) -> Html {
    html! {
        <BasicPage<NestedOrganization> id={PrimaryKey::from(props)}>
            <span>{"No content available yet."}</span>
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
pub struct SampleStatePageProp {
    pub id: i32,
}

impl From<&SampleStatePageProp> for PrimaryKey {
    fn from(prop: &SampleStatePageProp) -> Self {
        prop.id.into()
    }
}

impl SampleStatePageProp {
    fn filter_samples_by_state(&self) -> SampleFilter {
        let mut filter = SampleFilter::default();
        filter.state = Some(self.id);
        filter
    }
}

#[function_component(SampleStatePage)]
pub fn sample_state_page(props: &SampleStatePageProp) -> Html {
    html! {
        <BasicPage<NestedSampleState> id={PrimaryKey::from(props)}>
            // Linked with foreign key samples.state
            <BasicList<NestedSample> filters={props.filter_samples_by_state()}/>
        </BasicPage<NestedSampleState>>
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
pub struct SpectraPageProp {
    pub id: i32,
}

impl From<&SpectraPageProp> for PrimaryKey {
    fn from(prop: &SpectraPageProp) -> Self {
        prop.id.into()
    }
}

impl SpectraPageProp {
}

#[function_component(SpectraPage)]
pub fn spectra_page(props: &SpectraPageProp) -> Html {
    html! {
        <BasicPage<NestedSpectra> id={PrimaryKey::from(props)}>
            <span>{"No content available yet."}</span>
        </BasicPage<NestedSpectra>>
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
    fn filter_derived_samples_by_updated_by(&self) -> DerivedSampleFilter {
        let mut filter = DerivedSampleFilter::default();
        filter.updated_by = Some(self.id);
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
    fn filter_sample_containers_by_updated_by(&self) -> SampleContainerFilter {
        let mut filter = SampleContainerFilter::default();
        filter.updated_by = Some(self.id);
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
    fn filter_spectra_by_created_by(&self) -> SpectraFilter {
        let mut filter = SpectraFilter::default();
        filter.created_by = Some(self.id);
        filter
    }
    fn filter_spectra_by_updated_by(&self) -> SpectraFilter {
        let mut filter = SpectraFilter::default();
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
            // Linked with foreign key sample_containers.updated_by
            <BasicList<NestedSampleContainer> filters={props.filter_sample_containers_by_updated_by()}/>
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
            // Linked with foreign key spectra.created_by
            <BasicList<NestedSpectra> filters={props.filter_spectra_by_created_by()}/>
            // Linked with foreign key spectra.updated_by
            <BasicList<NestedSpectra> filters={props.filter_spectra_by_updated_by()}/>
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

