mod addresses;
mod aliquoting_instrument_models;
mod aliquoting_step_models;
mod aliquoting_steps;
mod ball_mill_step_models;
mod ball_mill_steps;
mod brands;
mod centrifuge_step_models;
mod centrifuge_steps;
mod cities;
mod colors;
mod commercial_product_lots;
mod commercial_products;
mod commercial_reagent_models;
mod commercial_reagents;
mod container_models;
mod countries;
mod disposal_step_models;
mod disposal_steps;
mod document_formats;
mod email_providers;
mod fractioning_step_models;
mod fractioning_steps;
mod freeze_drying_step_models;
mod instrument_locations;
mod instrument_model_categories;
mod instrument_models;
mod instrument_states;
mod instruments;
mod into_iter;
mod len;
mod login_providers;
mod materials;
mod nameplate_models;
mod observation_subjects;
mod organism_observations;
mod organism_sampling_step_models;
mod organism_taxa;
mod organisms;
mod organizations;
mod packaging_models;
mod packaging_step_models;
mod permanence_categories;
mod photographs;
mod postgres_async_bounded_read_dispatch;
mod procedure_model_container_categories;
mod procedure_model_instrument_categories;
mod procedure_model_nameplate_categories;
mod procedure_model_tool_categories;
mod procedure_models;
mod procedure_step_models;
mod procedures;
mod processables;
mod processing_steps;
mod project_states;
mod project_workflow_models;
mod projects;
mod ranks;
mod reagents;
mod roles;
mod rooms;
mod sample_states;
mod sampling_step_models;
mod sampling_steps;
mod shaking_step_models;
mod shaking_steps;
mod spatial_ref_sys;
mod spectra;
mod spectra_collections;
mod sqlite_bounded_read_dispatch;
mod step_container_models;
mod step_instruments;
mod step_model_categories;
mod step_model_container_categories;
mod step_model_instrument_categories;
mod step_model_instrument_models;
mod step_model_instruments;
mod step_model_nameplate_categories;
mod step_model_tool_categories;
mod step_models;
mod step_nameplate_models;
mod step_storage_containers;
mod step_tool_models;
mod steps;
mod storage_containers;
mod tabular;
mod taxa;
mod team_members;
mod team_projects;
mod team_states;
mod teams;
mod tool_models;
mod trackable_locations;
mod trackable_states;
mod trackables;
mod units;
mod user_emails;
mod user_organizations;
mod users;
mod volumetric_processables;
mod weighing_instrument_models;
mod weighing_step_models;
mod weighing_steps;
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Rows {
    Address(Vec<crate::codegen::structs_codegen::tables::addresses::Address>),
    AliquotingInstrumentModel(
        Vec<
            crate::codegen::structs_codegen::tables::aliquoting_instrument_models::AliquotingInstrumentModel,
        >,
    ),
    AliquotingStepModel(
        Vec<
            crate::codegen::structs_codegen::tables::aliquoting_step_models::AliquotingStepModel,
        >,
    ),
    AliquotingStep(
        Vec<crate::codegen::structs_codegen::tables::aliquoting_steps::AliquotingStep>,
    ),
    BallMillStepModel(
        Vec<
            crate::codegen::structs_codegen::tables::ball_mill_step_models::BallMillStepModel,
        >,
    ),
    BallMillStep(
        Vec<crate::codegen::structs_codegen::tables::ball_mill_steps::BallMillStep>,
    ),
    Brand(Vec<crate::codegen::structs_codegen::tables::brands::Brand>),
    CentrifugeStepModel(
        Vec<
            crate::codegen::structs_codegen::tables::centrifuge_step_models::CentrifugeStepModel,
        >,
    ),
    CentrifugeStep(
        Vec<crate::codegen::structs_codegen::tables::centrifuge_steps::CentrifugeStep>,
    ),
    City(Vec<crate::codegen::structs_codegen::tables::cities::City>),
    Color(Vec<crate::codegen::structs_codegen::tables::colors::Color>),
    CommercialProductLot(
        Vec<
            crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot,
        >,
    ),
    CommercialProduct(
        Vec<
            crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct,
        >,
    ),
    CommercialReagentModel(
        Vec<
            crate::codegen::structs_codegen::tables::commercial_reagent_models::CommercialReagentModel,
        >,
    ),
    CommercialReagent(
        Vec<
            crate::codegen::structs_codegen::tables::commercial_reagents::CommercialReagent,
        >,
    ),
    ContainerModel(
        Vec<crate::codegen::structs_codegen::tables::container_models::ContainerModel>,
    ),
    Country(Vec<crate::codegen::structs_codegen::tables::countries::Country>),
    DisposalStepModel(
        Vec<
            crate::codegen::structs_codegen::tables::disposal_step_models::DisposalStepModel,
        >,
    ),
    DisposalStep(
        Vec<crate::codegen::structs_codegen::tables::disposal_steps::DisposalStep>,
    ),
    DocumentFormat(
        Vec<crate::codegen::structs_codegen::tables::document_formats::DocumentFormat>,
    ),
    EmailProvider(
        Vec<crate::codegen::structs_codegen::tables::email_providers::EmailProvider>,
    ),
    FractioningStepModel(
        Vec<
            crate::codegen::structs_codegen::tables::fractioning_step_models::FractioningStepModel,
        >,
    ),
    FractioningStep(
        Vec<crate::codegen::structs_codegen::tables::fractioning_steps::FractioningStep>,
    ),
    FreezeDryingStepModel(
        Vec<
            crate::codegen::structs_codegen::tables::freeze_drying_step_models::FreezeDryingStepModel,
        >,
    ),
    InstrumentLocation(
        Vec<
            crate::codegen::structs_codegen::tables::instrument_locations::InstrumentLocation,
        >,
    ),
    InstrumentModelCategory(
        Vec<
            crate::codegen::structs_codegen::tables::instrument_model_categories::InstrumentModelCategory,
        >,
    ),
    InstrumentModel(
        Vec<crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel>,
    ),
    InstrumentState(
        Vec<crate::codegen::structs_codegen::tables::instrument_states::InstrumentState>,
    ),
    Instrument(Vec<crate::codegen::structs_codegen::tables::instruments::Instrument>),
    LoginProvider(
        Vec<crate::codegen::structs_codegen::tables::login_providers::LoginProvider>,
    ),
    Material(Vec<crate::codegen::structs_codegen::tables::materials::Material>),
    NameplateModel(
        Vec<crate::codegen::structs_codegen::tables::nameplate_models::NameplateModel>,
    ),
    ObservationSubject(
        Vec<
            crate::codegen::structs_codegen::tables::observation_subjects::ObservationSubject,
        >,
    ),
    OrganismObservation(
        Vec<
            crate::codegen::structs_codegen::tables::organism_observations::OrganismObservation,
        >,
    ),
    OrganismSamplingStepModel(
        Vec<
            crate::codegen::structs_codegen::tables::organism_sampling_step_models::OrganismSamplingStepModel,
        >,
    ),
    OrganismTaxon(
        Vec<crate::codegen::structs_codegen::tables::organism_taxa::OrganismTaxon>,
    ),
    Organism(Vec<crate::codegen::structs_codegen::tables::organisms::Organism>),
    Organization(
        Vec<crate::codegen::structs_codegen::tables::organizations::Organization>,
    ),
    PackagingModel(
        Vec<crate::codegen::structs_codegen::tables::packaging_models::PackagingModel>,
    ),
    PackagingStepModel(
        Vec<
            crate::codegen::structs_codegen::tables::packaging_step_models::PackagingStepModel,
        >,
    ),
    PermanenceCategory(
        Vec<
            crate::codegen::structs_codegen::tables::permanence_categories::PermanenceCategory,
        >,
    ),
    Photograph(Vec<crate::codegen::structs_codegen::tables::photographs::Photograph>),
    ProcedureModelContainerCategory(
        Vec<
            crate::codegen::structs_codegen::tables::procedure_model_container_categories::ProcedureModelContainerCategory,
        >,
    ),
    ProcedureModelInstrumentCategory(
        Vec<
            crate::codegen::structs_codegen::tables::procedure_model_instrument_categories::ProcedureModelInstrumentCategory,
        >,
    ),
    ProcedureModelNameplateCategory(
        Vec<
            crate::codegen::structs_codegen::tables::procedure_model_nameplate_categories::ProcedureModelNameplateCategory,
        >,
    ),
    ProcedureModelToolCategory(
        Vec<
            crate::codegen::structs_codegen::tables::procedure_model_tool_categories::ProcedureModelToolCategory,
        >,
    ),
    ProcedureModel(
        Vec<crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel>,
    ),
    ProcedureStepModel(
        Vec<
            crate::codegen::structs_codegen::tables::procedure_step_models::ProcedureStepModel,
        >,
    ),
    Procedure(Vec<crate::codegen::structs_codegen::tables::procedures::Procedure>),
    Processable(Vec<crate::codegen::structs_codegen::tables::processables::Processable>),
    ProcessingStep(
        Vec<crate::codegen::structs_codegen::tables::processing_steps::ProcessingStep>,
    ),
    ProjectState(
        Vec<crate::codegen::structs_codegen::tables::project_states::ProjectState>,
    ),
    ProjectWorkflowModel(
        Vec<
            crate::codegen::structs_codegen::tables::project_workflow_models::ProjectWorkflowModel,
        >,
    ),
    Project(Vec<crate::codegen::structs_codegen::tables::projects::Project>),
    Rank(Vec<crate::codegen::structs_codegen::tables::ranks::Rank>),
    Reagent(Vec<crate::codegen::structs_codegen::tables::reagents::Reagent>),
    Role(Vec<crate::codegen::structs_codegen::tables::roles::Role>),
    Room(Vec<crate::codegen::structs_codegen::tables::rooms::Room>),
    SampleState(
        Vec<crate::codegen::structs_codegen::tables::sample_states::SampleState>,
    ),
    SamplingStepModel(
        Vec<
            crate::codegen::structs_codegen::tables::sampling_step_models::SamplingStepModel,
        >,
    ),
    SamplingStep(
        Vec<crate::codegen::structs_codegen::tables::sampling_steps::SamplingStep>,
    ),
    ShakingStepModel(
        Vec<
            crate::codegen::structs_codegen::tables::shaking_step_models::ShakingStepModel,
        >,
    ),
    ShakingStep(
        Vec<crate::codegen::structs_codegen::tables::shaking_steps::ShakingStep>,
    ),
    SpatialRefSy(
        Vec<crate::codegen::structs_codegen::tables::spatial_ref_sys::SpatialRefSy>,
    ),
    Spectrum(Vec<crate::codegen::structs_codegen::tables::spectra::Spectrum>),
    SpectraCollection(
        Vec<
            crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection,
        >,
    ),
    StepContainerModel(
        Vec<
            crate::codegen::structs_codegen::tables::step_container_models::StepContainerModel,
        >,
    ),
    StepInstrument(
        Vec<crate::codegen::structs_codegen::tables::step_instruments::StepInstrument>,
    ),
    StepModelCategory(
        Vec<
            crate::codegen::structs_codegen::tables::step_model_categories::StepModelCategory,
        >,
    ),
    StepModelContainerCategory(
        Vec<
            crate::codegen::structs_codegen::tables::step_model_container_categories::StepModelContainerCategory,
        >,
    ),
    StepModelInstrumentCategory(
        Vec<
            crate::codegen::structs_codegen::tables::step_model_instrument_categories::StepModelInstrumentCategory,
        >,
    ),
    StepModelInstrumentModel(
        Vec<
            crate::codegen::structs_codegen::tables::step_model_instrument_models::StepModelInstrumentModel,
        >,
    ),
    StepModelInstrument(
        Vec<
            crate::codegen::structs_codegen::tables::step_model_instruments::StepModelInstrument,
        >,
    ),
    StepModelNameplateCategory(
        Vec<
            crate::codegen::structs_codegen::tables::step_model_nameplate_categories::StepModelNameplateCategory,
        >,
    ),
    StepModelToolCategory(
        Vec<
            crate::codegen::structs_codegen::tables::step_model_tool_categories::StepModelToolCategory,
        >,
    ),
    StepModel(Vec<crate::codegen::structs_codegen::tables::step_models::StepModel>),
    StepNameplateModel(
        Vec<
            crate::codegen::structs_codegen::tables::step_nameplate_models::StepNameplateModel,
        >,
    ),
    StepStorageContainer(
        Vec<
            crate::codegen::structs_codegen::tables::step_storage_containers::StepStorageContainer,
        >,
    ),
    StepToolModel(
        Vec<crate::codegen::structs_codegen::tables::step_tool_models::StepToolModel>,
    ),
    Step(Vec<crate::codegen::structs_codegen::tables::steps::Step>),
    StorageContainer(
        Vec<
            crate::codegen::structs_codegen::tables::storage_containers::StorageContainer,
        >,
    ),
    Taxon(Vec<crate::codegen::structs_codegen::tables::taxa::Taxon>),
    TeamMember(Vec<crate::codegen::structs_codegen::tables::team_members::TeamMember>),
    TeamProject(
        Vec<crate::codegen::structs_codegen::tables::team_projects::TeamProject>,
    ),
    TeamState(Vec<crate::codegen::structs_codegen::tables::team_states::TeamState>),
    Team(Vec<crate::codegen::structs_codegen::tables::teams::Team>),
    ToolModel(Vec<crate::codegen::structs_codegen::tables::tool_models::ToolModel>),
    TrackableLocation(
        Vec<
            crate::codegen::structs_codegen::tables::trackable_locations::TrackableLocation,
        >,
    ),
    TrackableState(
        Vec<crate::codegen::structs_codegen::tables::trackable_states::TrackableState>,
    ),
    Trackable(Vec<crate::codegen::structs_codegen::tables::trackables::Trackable>),
    Unit(Vec<crate::codegen::structs_codegen::tables::units::Unit>),
    UserEmail(Vec<crate::codegen::structs_codegen::tables::user_emails::UserEmail>),
    UserOrganization(
        Vec<
            crate::codegen::structs_codegen::tables::user_organizations::UserOrganization,
        >,
    ),
    User(Vec<crate::codegen::structs_codegen::tables::users::User>),
    VolumetricProcessable(
        Vec<
            crate::codegen::structs_codegen::tables::volumetric_processables::VolumetricProcessable,
        >,
    ),
    WeighingInstrumentModel(
        Vec<
            crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel,
        >,
    ),
    WeighingStepModel(
        Vec<
            crate::codegen::structs_codegen::tables::weighing_step_models::WeighingStepModel,
        >,
    ),
    WeighingStep(
        Vec<crate::codegen::structs_codegen::tables::weighing_steps::WeighingStep>,
    ),
}
impl Rows {
    #[cfg(feature = "sqlite")]
    /// Executes the upsert operation and returns the result.
    pub fn sqlite_upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Rows, diesel::result::Error> {
        use web_common_traits::database::Upsertable;
        Ok(match self {
            Rows::Address(addresses) => {
                addresses
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::AliquotingInstrumentModel(aliquoting_instrument_models) => {
                aliquoting_instrument_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::AliquotingStepModel(aliquoting_step_models) => {
                aliquoting_step_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::AliquotingStep(aliquoting_steps) => {
                aliquoting_steps
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::BallMillStepModel(ball_mill_step_models) => {
                ball_mill_step_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::BallMillStep(ball_mill_steps) => {
                ball_mill_steps
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::Brand(brands) => {
                brands
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::CentrifugeStepModel(centrifuge_step_models) => {
                centrifuge_step_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::CentrifugeStep(centrifuge_steps) => {
                centrifuge_steps
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::City(cities) => {
                cities
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::Color(colors) => {
                colors
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::CommercialProductLot(commercial_product_lots) => {
                commercial_product_lots
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::CommercialProduct(commercial_products) => {
                commercial_products
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::CommercialReagentModel(commercial_reagent_models) => {
                commercial_reagent_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::CommercialReagent(commercial_reagents) => {
                commercial_reagents
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::ContainerModel(container_models) => {
                container_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::Country(countries) => {
                countries
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::DisposalStepModel(disposal_step_models) => {
                disposal_step_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::DisposalStep(disposal_steps) => {
                disposal_steps
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::DocumentFormat(document_formats) => {
                document_formats
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::EmailProvider(email_providers) => {
                email_providers
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::FractioningStepModel(fractioning_step_models) => {
                fractioning_step_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::FractioningStep(fractioning_steps) => {
                fractioning_steps
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::FreezeDryingStepModel(freeze_drying_step_models) => {
                freeze_drying_step_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::InstrumentLocation(instrument_locations) => {
                instrument_locations
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::InstrumentModelCategory(instrument_model_categories) => {
                instrument_model_categories
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::InstrumentModel(instrument_models) => {
                instrument_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::InstrumentState(instrument_states) => {
                instrument_states
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::Instrument(instruments) => {
                instruments
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::LoginProvider(login_providers) => {
                login_providers
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::Material(materials) => {
                materials
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::NameplateModel(nameplate_models) => {
                nameplate_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::ObservationSubject(observation_subjects) => {
                observation_subjects
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::OrganismObservation(organism_observations) => {
                organism_observations
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::OrganismSamplingStepModel(organism_sampling_step_models) => {
                organism_sampling_step_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::OrganismTaxon(organism_taxa) => {
                organism_taxa
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::Organism(organisms) => {
                organisms
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::Organization(organizations) => {
                organizations
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::PackagingModel(packaging_models) => {
                packaging_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::PackagingStepModel(packaging_step_models) => {
                packaging_step_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::PermanenceCategory(permanence_categories) => {
                permanence_categories
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::Photograph(photographs) => {
                photographs
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::ProcedureModelContainerCategory(procedure_model_container_categories) => {
                procedure_model_container_categories
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::ProcedureModelInstrumentCategory(procedure_model_instrument_categories) => {
                procedure_model_instrument_categories
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::ProcedureModelNameplateCategory(procedure_model_nameplate_categories) => {
                procedure_model_nameplate_categories
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::ProcedureModelToolCategory(procedure_model_tool_categories) => {
                procedure_model_tool_categories
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::ProcedureModel(procedure_models) => {
                procedure_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::ProcedureStepModel(procedure_step_models) => {
                procedure_step_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::Procedure(procedures) => {
                procedures
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::Processable(processables) => {
                processables
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::ProcessingStep(processing_steps) => {
                processing_steps
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::ProjectState(project_states) => {
                project_states
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::ProjectWorkflowModel(project_workflow_models) => {
                project_workflow_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::Project(projects) => {
                projects
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::Rank(ranks) => {
                ranks
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::Reagent(reagents) => {
                reagents
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::Role(roles) => {
                roles
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::Room(rooms) => {
                rooms
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::SampleState(sample_states) => {
                sample_states
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::SamplingStepModel(sampling_step_models) => {
                sampling_step_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::SamplingStep(sampling_steps) => {
                sampling_steps
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::ShakingStepModel(shaking_step_models) => {
                shaking_step_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::ShakingStep(shaking_steps) => {
                shaking_steps
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::SpatialRefSy(spatial_ref_sys) => {
                spatial_ref_sys
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::Spectrum(spectra) => {
                spectra
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::SpectraCollection(spectra_collections) => {
                spectra_collections
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::StepContainerModel(step_container_models) => {
                step_container_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::StepInstrument(step_instruments) => {
                step_instruments
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::StepModelCategory(step_model_categories) => {
                step_model_categories
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::StepModelContainerCategory(step_model_container_categories) => {
                step_model_container_categories
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::StepModelInstrumentCategory(step_model_instrument_categories) => {
                step_model_instrument_categories
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::StepModelInstrumentModel(step_model_instrument_models) => {
                step_model_instrument_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::StepModelInstrument(step_model_instruments) => {
                step_model_instruments
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::StepModelNameplateCategory(step_model_nameplate_categories) => {
                step_model_nameplate_categories
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::StepModelToolCategory(step_model_tool_categories) => {
                step_model_tool_categories
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::StepModel(step_models) => {
                step_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::StepNameplateModel(step_nameplate_models) => {
                step_nameplate_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::StepStorageContainer(step_storage_containers) => {
                step_storage_containers
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::StepToolModel(step_tool_models) => {
                step_tool_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::Step(steps) => {
                steps
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::StorageContainer(storage_containers) => {
                storage_containers
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::Taxon(taxa) => {
                taxa.iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::TeamMember(team_members) => {
                team_members
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::TeamProject(team_projects) => {
                team_projects
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::TeamState(team_states) => {
                team_states
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::Team(teams) => {
                teams
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::ToolModel(tool_models) => {
                tool_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::TrackableLocation(trackable_locations) => {
                trackable_locations
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::TrackableState(trackable_states) => {
                trackable_states
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::Trackable(trackables) => {
                trackables
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::Unit(units) => {
                units
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::UserEmail(user_emails) => {
                user_emails
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::UserOrganization(user_organizations) => {
                user_organizations
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::User(users) => {
                users
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::VolumetricProcessable(volumetric_processables) => {
                volumetric_processables
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::WeighingInstrumentModel(weighing_instrument_models) => {
                weighing_instrument_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::WeighingStepModel(weighing_step_models) => {
                weighing_step_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::WeighingStep(weighing_steps) => {
                weighing_steps
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
        })
    }
}
impl web_common_traits::prelude::Rows for Rows {
    type PrimaryKey = crate::codegen::tables::table_primary_keys::TablePrimaryKey;
    fn primary_keys(&self) -> Vec<Self::PrimaryKey> {
        match self {
            Rows::Address(addresses) => addresses.primary_keys(),
            Rows::AliquotingInstrumentModel(aliquoting_instrument_models) => {
                aliquoting_instrument_models.primary_keys()
            }
            Rows::AliquotingStepModel(aliquoting_step_models) => {
                aliquoting_step_models.primary_keys()
            }
            Rows::AliquotingStep(aliquoting_steps) => aliquoting_steps.primary_keys(),
            Rows::BallMillStepModel(ball_mill_step_models) => ball_mill_step_models.primary_keys(),
            Rows::BallMillStep(ball_mill_steps) => ball_mill_steps.primary_keys(),
            Rows::Brand(brands) => brands.primary_keys(),
            Rows::CentrifugeStepModel(centrifuge_step_models) => {
                centrifuge_step_models.primary_keys()
            }
            Rows::CentrifugeStep(centrifuge_steps) => centrifuge_steps.primary_keys(),
            Rows::City(cities) => cities.primary_keys(),
            Rows::Color(colors) => colors.primary_keys(),
            Rows::CommercialProductLot(commercial_product_lots) => {
                commercial_product_lots.primary_keys()
            }
            Rows::CommercialProduct(commercial_products) => commercial_products.primary_keys(),
            Rows::CommercialReagentModel(commercial_reagent_models) => {
                commercial_reagent_models.primary_keys()
            }
            Rows::CommercialReagent(commercial_reagents) => commercial_reagents.primary_keys(),
            Rows::ContainerModel(container_models) => container_models.primary_keys(),
            Rows::Country(countries) => countries.primary_keys(),
            Rows::DisposalStepModel(disposal_step_models) => disposal_step_models.primary_keys(),
            Rows::DisposalStep(disposal_steps) => disposal_steps.primary_keys(),
            Rows::DocumentFormat(document_formats) => document_formats.primary_keys(),
            Rows::EmailProvider(email_providers) => email_providers.primary_keys(),
            Rows::FractioningStepModel(fractioning_step_models) => {
                fractioning_step_models.primary_keys()
            }
            Rows::FractioningStep(fractioning_steps) => fractioning_steps.primary_keys(),
            Rows::FreezeDryingStepModel(freeze_drying_step_models) => {
                freeze_drying_step_models.primary_keys()
            }
            Rows::InstrumentLocation(instrument_locations) => instrument_locations.primary_keys(),
            Rows::InstrumentModelCategory(instrument_model_categories) => {
                instrument_model_categories.primary_keys()
            }
            Rows::InstrumentModel(instrument_models) => instrument_models.primary_keys(),
            Rows::InstrumentState(instrument_states) => instrument_states.primary_keys(),
            Rows::Instrument(instruments) => instruments.primary_keys(),
            Rows::LoginProvider(login_providers) => login_providers.primary_keys(),
            Rows::Material(materials) => materials.primary_keys(),
            Rows::NameplateModel(nameplate_models) => nameplate_models.primary_keys(),
            Rows::ObservationSubject(observation_subjects) => observation_subjects.primary_keys(),
            Rows::OrganismObservation(organism_observations) => {
                organism_observations.primary_keys()
            }
            Rows::OrganismSamplingStepModel(organism_sampling_step_models) => {
                organism_sampling_step_models.primary_keys()
            }
            Rows::OrganismTaxon(organism_taxa) => organism_taxa.primary_keys(),
            Rows::Organism(organisms) => organisms.primary_keys(),
            Rows::Organization(organizations) => organizations.primary_keys(),
            Rows::PackagingModel(packaging_models) => packaging_models.primary_keys(),
            Rows::PackagingStepModel(packaging_step_models) => packaging_step_models.primary_keys(),
            Rows::PermanenceCategory(permanence_categories) => permanence_categories.primary_keys(),
            Rows::Photograph(photographs) => photographs.primary_keys(),
            Rows::ProcedureModelContainerCategory(procedure_model_container_categories) => {
                procedure_model_container_categories.primary_keys()
            }
            Rows::ProcedureModelInstrumentCategory(procedure_model_instrument_categories) => {
                procedure_model_instrument_categories.primary_keys()
            }
            Rows::ProcedureModelNameplateCategory(procedure_model_nameplate_categories) => {
                procedure_model_nameplate_categories.primary_keys()
            }
            Rows::ProcedureModelToolCategory(procedure_model_tool_categories) => {
                procedure_model_tool_categories.primary_keys()
            }
            Rows::ProcedureModel(procedure_models) => procedure_models.primary_keys(),
            Rows::ProcedureStepModel(procedure_step_models) => procedure_step_models.primary_keys(),
            Rows::Procedure(procedures) => procedures.primary_keys(),
            Rows::Processable(processables) => processables.primary_keys(),
            Rows::ProcessingStep(processing_steps) => processing_steps.primary_keys(),
            Rows::ProjectState(project_states) => project_states.primary_keys(),
            Rows::ProjectWorkflowModel(project_workflow_models) => {
                project_workflow_models.primary_keys()
            }
            Rows::Project(projects) => projects.primary_keys(),
            Rows::Rank(ranks) => ranks.primary_keys(),
            Rows::Reagent(reagents) => reagents.primary_keys(),
            Rows::Role(roles) => roles.primary_keys(),
            Rows::Room(rooms) => rooms.primary_keys(),
            Rows::SampleState(sample_states) => sample_states.primary_keys(),
            Rows::SamplingStepModel(sampling_step_models) => sampling_step_models.primary_keys(),
            Rows::SamplingStep(sampling_steps) => sampling_steps.primary_keys(),
            Rows::ShakingStepModel(shaking_step_models) => shaking_step_models.primary_keys(),
            Rows::ShakingStep(shaking_steps) => shaking_steps.primary_keys(),
            Rows::SpatialRefSy(spatial_ref_sys) => spatial_ref_sys.primary_keys(),
            Rows::Spectrum(spectra) => spectra.primary_keys(),
            Rows::SpectraCollection(spectra_collections) => spectra_collections.primary_keys(),
            Rows::StepContainerModel(step_container_models) => step_container_models.primary_keys(),
            Rows::StepInstrument(step_instruments) => step_instruments.primary_keys(),
            Rows::StepModelCategory(step_model_categories) => step_model_categories.primary_keys(),
            Rows::StepModelContainerCategory(step_model_container_categories) => {
                step_model_container_categories.primary_keys()
            }
            Rows::StepModelInstrumentCategory(step_model_instrument_categories) => {
                step_model_instrument_categories.primary_keys()
            }
            Rows::StepModelInstrumentModel(step_model_instrument_models) => {
                step_model_instrument_models.primary_keys()
            }
            Rows::StepModelInstrument(step_model_instruments) => {
                step_model_instruments.primary_keys()
            }
            Rows::StepModelNameplateCategory(step_model_nameplate_categories) => {
                step_model_nameplate_categories.primary_keys()
            }
            Rows::StepModelToolCategory(step_model_tool_categories) => {
                step_model_tool_categories.primary_keys()
            }
            Rows::StepModel(step_models) => step_models.primary_keys(),
            Rows::StepNameplateModel(step_nameplate_models) => step_nameplate_models.primary_keys(),
            Rows::StepStorageContainer(step_storage_containers) => {
                step_storage_containers.primary_keys()
            }
            Rows::StepToolModel(step_tool_models) => step_tool_models.primary_keys(),
            Rows::Step(steps) => steps.primary_keys(),
            Rows::StorageContainer(storage_containers) => storage_containers.primary_keys(),
            Rows::Taxon(taxa) => taxa.primary_keys(),
            Rows::TeamMember(team_members) => team_members.primary_keys(),
            Rows::TeamProject(team_projects) => team_projects.primary_keys(),
            Rows::TeamState(team_states) => team_states.primary_keys(),
            Rows::Team(teams) => teams.primary_keys(),
            Rows::ToolModel(tool_models) => tool_models.primary_keys(),
            Rows::TrackableLocation(trackable_locations) => trackable_locations.primary_keys(),
            Rows::TrackableState(trackable_states) => trackable_states.primary_keys(),
            Rows::Trackable(trackables) => trackables.primary_keys(),
            Rows::Unit(units) => units.primary_keys(),
            Rows::UserEmail(user_emails) => user_emails.primary_keys(),
            Rows::UserOrganization(user_organizations) => user_organizations.primary_keys(),
            Rows::User(users) => users.primary_keys(),
            Rows::VolumetricProcessable(volumetric_processables) => {
                volumetric_processables.primary_keys()
            }
            Rows::WeighingInstrumentModel(weighing_instrument_models) => {
                weighing_instrument_models.primary_keys()
            }
            Rows::WeighingStepModel(weighing_step_models) => weighing_step_models.primary_keys(),
            Rows::WeighingStep(weighing_steps) => weighing_steps.primary_keys(),
        }
    }
}
