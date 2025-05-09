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
mod from_row;
mod instrument_locations;
mod instrument_model_categories;
mod instrument_models;
mod instrument_states;
mod instruments;
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
mod postgres_async_read_dispatch;
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
mod sqlite_read_dispatch;
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
pub enum Row {
    Address(crate::codegen::structs_codegen::tables::addresses::Address),
    AliquotingInstrumentModel(
        crate::codegen::structs_codegen::tables::aliquoting_instrument_models::AliquotingInstrumentModel,
    ),
    AliquotingStepModel(
        crate::codegen::structs_codegen::tables::aliquoting_step_models::AliquotingStepModel,
    ),
    AliquotingStep(
        crate::codegen::structs_codegen::tables::aliquoting_steps::AliquotingStep,
    ),
    BallMillStepModel(
        crate::codegen::structs_codegen::tables::ball_mill_step_models::BallMillStepModel,
    ),
    BallMillStep(crate::codegen::structs_codegen::tables::ball_mill_steps::BallMillStep),
    Brand(crate::codegen::structs_codegen::tables::brands::Brand),
    CentrifugeStepModel(
        crate::codegen::structs_codegen::tables::centrifuge_step_models::CentrifugeStepModel,
    ),
    CentrifugeStep(
        crate::codegen::structs_codegen::tables::centrifuge_steps::CentrifugeStep,
    ),
    City(crate::codegen::structs_codegen::tables::cities::City),
    Color(crate::codegen::structs_codegen::tables::colors::Color),
    CommercialProductLot(
        crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot,
    ),
    CommercialProduct(
        crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct,
    ),
    CommercialReagentModel(
        crate::codegen::structs_codegen::tables::commercial_reagent_models::CommercialReagentModel,
    ),
    CommercialReagent(
        crate::codegen::structs_codegen::tables::commercial_reagents::CommercialReagent,
    ),
    ContainerModel(
        crate::codegen::structs_codegen::tables::container_models::ContainerModel,
    ),
    Country(crate::codegen::structs_codegen::tables::countries::Country),
    DisposalStepModel(
        crate::codegen::structs_codegen::tables::disposal_step_models::DisposalStepModel,
    ),
    DisposalStep(crate::codegen::structs_codegen::tables::disposal_steps::DisposalStep),
    DocumentFormat(
        crate::codegen::structs_codegen::tables::document_formats::DocumentFormat,
    ),
    EmailProvider(
        crate::codegen::structs_codegen::tables::email_providers::EmailProvider,
    ),
    FractioningStepModel(
        crate::codegen::structs_codegen::tables::fractioning_step_models::FractioningStepModel,
    ),
    FractioningStep(
        crate::codegen::structs_codegen::tables::fractioning_steps::FractioningStep,
    ),
    FreezeDryingStepModel(
        crate::codegen::structs_codegen::tables::freeze_drying_step_models::FreezeDryingStepModel,
    ),
    InstrumentLocation(
        crate::codegen::structs_codegen::tables::instrument_locations::InstrumentLocation,
    ),
    InstrumentModelCategory(
        crate::codegen::structs_codegen::tables::instrument_model_categories::InstrumentModelCategory,
    ),
    InstrumentModel(
        crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel,
    ),
    InstrumentState(
        crate::codegen::structs_codegen::tables::instrument_states::InstrumentState,
    ),
    Instrument(crate::codegen::structs_codegen::tables::instruments::Instrument),
    LoginProvider(
        crate::codegen::structs_codegen::tables::login_providers::LoginProvider,
    ),
    Material(crate::codegen::structs_codegen::tables::materials::Material),
    NameplateModel(
        crate::codegen::structs_codegen::tables::nameplate_models::NameplateModel,
    ),
    ObservationSubject(
        crate::codegen::structs_codegen::tables::observation_subjects::ObservationSubject,
    ),
    OrganismObservation(
        crate::codegen::structs_codegen::tables::organism_observations::OrganismObservation,
    ),
    OrganismSamplingStepModel(
        crate::codegen::structs_codegen::tables::organism_sampling_step_models::OrganismSamplingStepModel,
    ),
    OrganismTaxon(crate::codegen::structs_codegen::tables::organism_taxa::OrganismTaxon),
    Organism(crate::codegen::structs_codegen::tables::organisms::Organism),
    Organization(crate::codegen::structs_codegen::tables::organizations::Organization),
    PackagingModel(
        crate::codegen::structs_codegen::tables::packaging_models::PackagingModel,
    ),
    PackagingStepModel(
        crate::codegen::structs_codegen::tables::packaging_step_models::PackagingStepModel,
    ),
    PermanenceCategory(
        crate::codegen::structs_codegen::tables::permanence_categories::PermanenceCategory,
    ),
    Photograph(crate::codegen::structs_codegen::tables::photographs::Photograph),
    ProcedureModelContainerCategory(
        crate::codegen::structs_codegen::tables::procedure_model_container_categories::ProcedureModelContainerCategory,
    ),
    ProcedureModelInstrumentCategory(
        crate::codegen::structs_codegen::tables::procedure_model_instrument_categories::ProcedureModelInstrumentCategory,
    ),
    ProcedureModelNameplateCategory(
        crate::codegen::structs_codegen::tables::procedure_model_nameplate_categories::ProcedureModelNameplateCategory,
    ),
    ProcedureModelToolCategory(
        crate::codegen::structs_codegen::tables::procedure_model_tool_categories::ProcedureModelToolCategory,
    ),
    ProcedureModel(
        crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel,
    ),
    ProcedureStepModel(
        crate::codegen::structs_codegen::tables::procedure_step_models::ProcedureStepModel,
    ),
    Procedure(crate::codegen::structs_codegen::tables::procedures::Procedure),
    Processable(crate::codegen::structs_codegen::tables::processables::Processable),
    ProcessingStep(
        crate::codegen::structs_codegen::tables::processing_steps::ProcessingStep,
    ),
    ProjectState(crate::codegen::structs_codegen::tables::project_states::ProjectState),
    ProjectWorkflowModel(
        crate::codegen::structs_codegen::tables::project_workflow_models::ProjectWorkflowModel,
    ),
    Project(crate::codegen::structs_codegen::tables::projects::Project),
    Rank(crate::codegen::structs_codegen::tables::ranks::Rank),
    Reagent(crate::codegen::structs_codegen::tables::reagents::Reagent),
    Role(crate::codegen::structs_codegen::tables::roles::Role),
    Room(crate::codegen::structs_codegen::tables::rooms::Room),
    SampleState(crate::codegen::structs_codegen::tables::sample_states::SampleState),
    SamplingStepModel(
        crate::codegen::structs_codegen::tables::sampling_step_models::SamplingStepModel,
    ),
    SamplingStep(crate::codegen::structs_codegen::tables::sampling_steps::SamplingStep),
    ShakingStepModel(
        crate::codegen::structs_codegen::tables::shaking_step_models::ShakingStepModel,
    ),
    ShakingStep(crate::codegen::structs_codegen::tables::shaking_steps::ShakingStep),
    SpatialRefSy(crate::codegen::structs_codegen::tables::spatial_ref_sys::SpatialRefSy),
    Spectrum(crate::codegen::structs_codegen::tables::spectra::Spectrum),
    SpectraCollection(
        crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection,
    ),
    StepContainerModel(
        crate::codegen::structs_codegen::tables::step_container_models::StepContainerModel,
    ),
    StepInstrument(
        crate::codegen::structs_codegen::tables::step_instruments::StepInstrument,
    ),
    StepModelCategory(
        crate::codegen::structs_codegen::tables::step_model_categories::StepModelCategory,
    ),
    StepModelContainerCategory(
        crate::codegen::structs_codegen::tables::step_model_container_categories::StepModelContainerCategory,
    ),
    StepModelInstrumentCategory(
        crate::codegen::structs_codegen::tables::step_model_instrument_categories::StepModelInstrumentCategory,
    ),
    StepModelInstrumentModel(
        crate::codegen::structs_codegen::tables::step_model_instrument_models::StepModelInstrumentModel,
    ),
    StepModelInstrument(
        crate::codegen::structs_codegen::tables::step_model_instruments::StepModelInstrument,
    ),
    StepModelNameplateCategory(
        crate::codegen::structs_codegen::tables::step_model_nameplate_categories::StepModelNameplateCategory,
    ),
    StepModelToolCategory(
        crate::codegen::structs_codegen::tables::step_model_tool_categories::StepModelToolCategory,
    ),
    StepModel(crate::codegen::structs_codegen::tables::step_models::StepModel),
    StepNameplateModel(
        crate::codegen::structs_codegen::tables::step_nameplate_models::StepNameplateModel,
    ),
    StepStorageContainer(
        crate::codegen::structs_codegen::tables::step_storage_containers::StepStorageContainer,
    ),
    StepToolModel(
        crate::codegen::structs_codegen::tables::step_tool_models::StepToolModel,
    ),
    Step(crate::codegen::structs_codegen::tables::steps::Step),
    StorageContainer(
        crate::codegen::structs_codegen::tables::storage_containers::StorageContainer,
    ),
    Taxon(crate::codegen::structs_codegen::tables::taxa::Taxon),
    TeamMember(crate::codegen::structs_codegen::tables::team_members::TeamMember),
    TeamProject(crate::codegen::structs_codegen::tables::team_projects::TeamProject),
    TeamState(crate::codegen::structs_codegen::tables::team_states::TeamState),
    Team(crate::codegen::structs_codegen::tables::teams::Team),
    ToolModel(crate::codegen::structs_codegen::tables::tool_models::ToolModel),
    TrackableLocation(
        crate::codegen::structs_codegen::tables::trackable_locations::TrackableLocation,
    ),
    TrackableState(
        crate::codegen::structs_codegen::tables::trackable_states::TrackableState,
    ),
    Trackable(crate::codegen::structs_codegen::tables::trackables::Trackable),
    Unit(crate::codegen::structs_codegen::tables::units::Unit),
    UserEmail(crate::codegen::structs_codegen::tables::user_emails::UserEmail),
    UserOrganization(
        crate::codegen::structs_codegen::tables::user_organizations::UserOrganization,
    ),
    User(crate::codegen::structs_codegen::tables::users::User),
    VolumetricProcessable(
        crate::codegen::structs_codegen::tables::volumetric_processables::VolumetricProcessable,
    ),
    WeighingInstrumentModel(
        crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel,
    ),
    WeighingStepModel(
        crate::codegen::structs_codegen::tables::weighing_step_models::WeighingStepModel,
    ),
    WeighingStep(crate::codegen::structs_codegen::tables::weighing_steps::WeighingStep),
}
impl Row {
    #[cfg(feature = "sqlite")]
    /// Executes the upsert operation and returns the result.
    pub fn sqlite_upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Row>, diesel::result::Error> {
        use web_common_traits::database::Upsertable;
        Ok(match self {
            Row::Address(addresses) => addresses.upsert(conn)?.map(Row::from),
            Row::AliquotingInstrumentModel(aliquoting_instrument_models) => {
                aliquoting_instrument_models.upsert(conn)?.map(Row::from)
            }
            Row::AliquotingStepModel(aliquoting_step_models) => {
                aliquoting_step_models.upsert(conn)?.map(Row::from)
            }
            Row::AliquotingStep(aliquoting_steps) => aliquoting_steps.upsert(conn)?.map(Row::from),
            Row::BallMillStepModel(ball_mill_step_models) => {
                ball_mill_step_models.upsert(conn)?.map(Row::from)
            }
            Row::BallMillStep(ball_mill_steps) => ball_mill_steps.upsert(conn)?.map(Row::from),
            Row::Brand(brands) => brands.upsert(conn)?.map(Row::from),
            Row::CentrifugeStepModel(centrifuge_step_models) => {
                centrifuge_step_models.upsert(conn)?.map(Row::from)
            }
            Row::CentrifugeStep(centrifuge_steps) => centrifuge_steps.upsert(conn)?.map(Row::from),
            Row::City(cities) => cities.upsert(conn)?.map(Row::from),
            Row::Color(colors) => colors.upsert(conn)?.map(Row::from),
            Row::CommercialProductLot(commercial_product_lots) => {
                commercial_product_lots.upsert(conn)?.map(Row::from)
            }
            Row::CommercialProduct(commercial_products) => {
                commercial_products.upsert(conn)?.map(Row::from)
            }
            Row::CommercialReagentModel(commercial_reagent_models) => {
                commercial_reagent_models.upsert(conn)?.map(Row::from)
            }
            Row::CommercialReagent(commercial_reagents) => {
                commercial_reagents.upsert(conn)?.map(Row::from)
            }
            Row::ContainerModel(container_models) => container_models.upsert(conn)?.map(Row::from),
            Row::Country(countries) => countries.upsert(conn)?.map(Row::from),
            Row::DisposalStepModel(disposal_step_models) => {
                disposal_step_models.upsert(conn)?.map(Row::from)
            }
            Row::DisposalStep(disposal_steps) => disposal_steps.upsert(conn)?.map(Row::from),
            Row::DocumentFormat(document_formats) => document_formats.upsert(conn)?.map(Row::from),
            Row::EmailProvider(email_providers) => email_providers.upsert(conn)?.map(Row::from),
            Row::FractioningStepModel(fractioning_step_models) => {
                fractioning_step_models.upsert(conn)?.map(Row::from)
            }
            Row::FractioningStep(fractioning_steps) => {
                fractioning_steps.upsert(conn)?.map(Row::from)
            }
            Row::FreezeDryingStepModel(freeze_drying_step_models) => {
                freeze_drying_step_models.upsert(conn)?.map(Row::from)
            }
            Row::InstrumentLocation(instrument_locations) => {
                instrument_locations.upsert(conn)?.map(Row::from)
            }
            Row::InstrumentModelCategory(instrument_model_categories) => {
                instrument_model_categories.upsert(conn)?.map(Row::from)
            }
            Row::InstrumentModel(instrument_models) => {
                instrument_models.upsert(conn)?.map(Row::from)
            }
            Row::InstrumentState(instrument_states) => {
                instrument_states.upsert(conn)?.map(Row::from)
            }
            Row::Instrument(instruments) => instruments.upsert(conn)?.map(Row::from),
            Row::LoginProvider(login_providers) => login_providers.upsert(conn)?.map(Row::from),
            Row::Material(materials) => materials.upsert(conn)?.map(Row::from),
            Row::NameplateModel(nameplate_models) => nameplate_models.upsert(conn)?.map(Row::from),
            Row::ObservationSubject(observation_subjects) => {
                observation_subjects.upsert(conn)?.map(Row::from)
            }
            Row::OrganismObservation(organism_observations) => {
                organism_observations.upsert(conn)?.map(Row::from)
            }
            Row::OrganismSamplingStepModel(organism_sampling_step_models) => {
                organism_sampling_step_models.upsert(conn)?.map(Row::from)
            }
            Row::OrganismTaxon(organism_taxa) => organism_taxa.upsert(conn)?.map(Row::from),
            Row::Organism(organisms) => organisms.upsert(conn)?.map(Row::from),
            Row::Organization(organizations) => organizations.upsert(conn)?.map(Row::from),
            Row::PackagingModel(packaging_models) => packaging_models.upsert(conn)?.map(Row::from),
            Row::PackagingStepModel(packaging_step_models) => {
                packaging_step_models.upsert(conn)?.map(Row::from)
            }
            Row::PermanenceCategory(permanence_categories) => {
                permanence_categories.upsert(conn)?.map(Row::from)
            }
            Row::Photograph(photographs) => photographs.upsert(conn)?.map(Row::from),
            Row::ProcedureModelContainerCategory(procedure_model_container_categories) => {
                procedure_model_container_categories.upsert(conn)?.map(Row::from)
            }
            Row::ProcedureModelInstrumentCategory(procedure_model_instrument_categories) => {
                procedure_model_instrument_categories.upsert(conn)?.map(Row::from)
            }
            Row::ProcedureModelNameplateCategory(procedure_model_nameplate_categories) => {
                procedure_model_nameplate_categories.upsert(conn)?.map(Row::from)
            }
            Row::ProcedureModelToolCategory(procedure_model_tool_categories) => {
                procedure_model_tool_categories.upsert(conn)?.map(Row::from)
            }
            Row::ProcedureModel(procedure_models) => procedure_models.upsert(conn)?.map(Row::from),
            Row::ProcedureStepModel(procedure_step_models) => {
                procedure_step_models.upsert(conn)?.map(Row::from)
            }
            Row::Procedure(procedures) => procedures.upsert(conn)?.map(Row::from),
            Row::Processable(processables) => processables.upsert(conn)?.map(Row::from),
            Row::ProcessingStep(processing_steps) => processing_steps.upsert(conn)?.map(Row::from),
            Row::ProjectState(project_states) => project_states.upsert(conn)?.map(Row::from),
            Row::ProjectWorkflowModel(project_workflow_models) => {
                project_workflow_models.upsert(conn)?.map(Row::from)
            }
            Row::Project(projects) => projects.upsert(conn)?.map(Row::from),
            Row::Rank(ranks) => ranks.upsert(conn)?.map(Row::from),
            Row::Reagent(reagents) => reagents.upsert(conn)?.map(Row::from),
            Row::Role(roles) => roles.upsert(conn)?.map(Row::from),
            Row::Room(rooms) => rooms.upsert(conn)?.map(Row::from),
            Row::SampleState(sample_states) => sample_states.upsert(conn)?.map(Row::from),
            Row::SamplingStepModel(sampling_step_models) => {
                sampling_step_models.upsert(conn)?.map(Row::from)
            }
            Row::SamplingStep(sampling_steps) => sampling_steps.upsert(conn)?.map(Row::from),
            Row::ShakingStepModel(shaking_step_models) => {
                shaking_step_models.upsert(conn)?.map(Row::from)
            }
            Row::ShakingStep(shaking_steps) => shaking_steps.upsert(conn)?.map(Row::from),
            Row::SpatialRefSy(spatial_ref_sys) => spatial_ref_sys.upsert(conn)?.map(Row::from),
            Row::Spectrum(spectra) => spectra.upsert(conn)?.map(Row::from),
            Row::SpectraCollection(spectra_collections) => {
                spectra_collections.upsert(conn)?.map(Row::from)
            }
            Row::StepContainerModel(step_container_models) => {
                step_container_models.upsert(conn)?.map(Row::from)
            }
            Row::StepInstrument(step_instruments) => step_instruments.upsert(conn)?.map(Row::from),
            Row::StepModelCategory(step_model_categories) => {
                step_model_categories.upsert(conn)?.map(Row::from)
            }
            Row::StepModelContainerCategory(step_model_container_categories) => {
                step_model_container_categories.upsert(conn)?.map(Row::from)
            }
            Row::StepModelInstrumentCategory(step_model_instrument_categories) => {
                step_model_instrument_categories.upsert(conn)?.map(Row::from)
            }
            Row::StepModelInstrumentModel(step_model_instrument_models) => {
                step_model_instrument_models.upsert(conn)?.map(Row::from)
            }
            Row::StepModelInstrument(step_model_instruments) => {
                step_model_instruments.upsert(conn)?.map(Row::from)
            }
            Row::StepModelNameplateCategory(step_model_nameplate_categories) => {
                step_model_nameplate_categories.upsert(conn)?.map(Row::from)
            }
            Row::StepModelToolCategory(step_model_tool_categories) => {
                step_model_tool_categories.upsert(conn)?.map(Row::from)
            }
            Row::StepModel(step_models) => step_models.upsert(conn)?.map(Row::from),
            Row::StepNameplateModel(step_nameplate_models) => {
                step_nameplate_models.upsert(conn)?.map(Row::from)
            }
            Row::StepStorageContainer(step_storage_containers) => {
                step_storage_containers.upsert(conn)?.map(Row::from)
            }
            Row::StepToolModel(step_tool_models) => step_tool_models.upsert(conn)?.map(Row::from),
            Row::Step(steps) => steps.upsert(conn)?.map(Row::from),
            Row::StorageContainer(storage_containers) => {
                storage_containers.upsert(conn)?.map(Row::from)
            }
            Row::Taxon(taxa) => taxa.upsert(conn)?.map(Row::from),
            Row::TeamMember(team_members) => team_members.upsert(conn)?.map(Row::from),
            Row::TeamProject(team_projects) => team_projects.upsert(conn)?.map(Row::from),
            Row::TeamState(team_states) => team_states.upsert(conn)?.map(Row::from),
            Row::Team(teams) => teams.upsert(conn)?.map(Row::from),
            Row::ToolModel(tool_models) => tool_models.upsert(conn)?.map(Row::from),
            Row::TrackableLocation(trackable_locations) => {
                trackable_locations.upsert(conn)?.map(Row::from)
            }
            Row::TrackableState(trackable_states) => trackable_states.upsert(conn)?.map(Row::from),
            Row::Trackable(trackables) => trackables.upsert(conn)?.map(Row::from),
            Row::Unit(units) => units.upsert(conn)?.map(Row::from),
            Row::UserEmail(user_emails) => user_emails.upsert(conn)?.map(Row::from),
            Row::UserOrganization(user_organizations) => {
                user_organizations.upsert(conn)?.map(Row::from)
            }
            Row::User(users) => users.upsert(conn)?.map(Row::from),
            Row::VolumetricProcessable(volumetric_processables) => {
                volumetric_processables.upsert(conn)?.map(Row::from)
            }
            Row::WeighingInstrumentModel(weighing_instrument_models) => {
                weighing_instrument_models.upsert(conn)?.map(Row::from)
            }
            Row::WeighingStepModel(weighing_step_models) => {
                weighing_step_models.upsert(conn)?.map(Row::from)
            }
            Row::WeighingStep(weighing_steps) => weighing_steps.upsert(conn)?.map(Row::from),
        })
    }
}
impl web_common_traits::prelude::Row for Row {
    type PrimaryKey = crate::codegen::tables::table_primary_keys::TablePrimaryKey;
    fn primary_key(&self) -> Self::PrimaryKey {
        match self {
            Row::Address(addresses) => addresses.primary_key(),
            Row::AliquotingInstrumentModel(aliquoting_instrument_models) => {
                aliquoting_instrument_models.primary_key()
            }
            Row::AliquotingStepModel(aliquoting_step_models) => {
                aliquoting_step_models.primary_key()
            }
            Row::AliquotingStep(aliquoting_steps) => aliquoting_steps.primary_key(),
            Row::BallMillStepModel(ball_mill_step_models) => ball_mill_step_models.primary_key(),
            Row::BallMillStep(ball_mill_steps) => ball_mill_steps.primary_key(),
            Row::Brand(brands) => brands.primary_key(),
            Row::CentrifugeStepModel(centrifuge_step_models) => {
                centrifuge_step_models.primary_key()
            }
            Row::CentrifugeStep(centrifuge_steps) => centrifuge_steps.primary_key(),
            Row::City(cities) => cities.primary_key(),
            Row::Color(colors) => colors.primary_key(),
            Row::CommercialProductLot(commercial_product_lots) => {
                commercial_product_lots.primary_key()
            }
            Row::CommercialProduct(commercial_products) => commercial_products.primary_key(),
            Row::CommercialReagentModel(commercial_reagent_models) => {
                commercial_reagent_models.primary_key()
            }
            Row::CommercialReagent(commercial_reagents) => commercial_reagents.primary_key(),
            Row::ContainerModel(container_models) => container_models.primary_key(),
            Row::Country(countries) => countries.primary_key(),
            Row::DisposalStepModel(disposal_step_models) => disposal_step_models.primary_key(),
            Row::DisposalStep(disposal_steps) => disposal_steps.primary_key(),
            Row::DocumentFormat(document_formats) => document_formats.primary_key(),
            Row::EmailProvider(email_providers) => email_providers.primary_key(),
            Row::FractioningStepModel(fractioning_step_models) => {
                fractioning_step_models.primary_key()
            }
            Row::FractioningStep(fractioning_steps) => fractioning_steps.primary_key(),
            Row::FreezeDryingStepModel(freeze_drying_step_models) => {
                freeze_drying_step_models.primary_key()
            }
            Row::InstrumentLocation(instrument_locations) => instrument_locations.primary_key(),
            Row::InstrumentModelCategory(instrument_model_categories) => {
                instrument_model_categories.primary_key()
            }
            Row::InstrumentModel(instrument_models) => instrument_models.primary_key(),
            Row::InstrumentState(instrument_states) => instrument_states.primary_key(),
            Row::Instrument(instruments) => instruments.primary_key(),
            Row::LoginProvider(login_providers) => login_providers.primary_key(),
            Row::Material(materials) => materials.primary_key(),
            Row::NameplateModel(nameplate_models) => nameplate_models.primary_key(),
            Row::ObservationSubject(observation_subjects) => observation_subjects.primary_key(),
            Row::OrganismObservation(organism_observations) => organism_observations.primary_key(),
            Row::OrganismSamplingStepModel(organism_sampling_step_models) => {
                organism_sampling_step_models.primary_key()
            }
            Row::OrganismTaxon(organism_taxa) => organism_taxa.primary_key(),
            Row::Organism(organisms) => organisms.primary_key(),
            Row::Organization(organizations) => organizations.primary_key(),
            Row::PackagingModel(packaging_models) => packaging_models.primary_key(),
            Row::PackagingStepModel(packaging_step_models) => packaging_step_models.primary_key(),
            Row::PermanenceCategory(permanence_categories) => permanence_categories.primary_key(),
            Row::Photograph(photographs) => photographs.primary_key(),
            Row::ProcedureModelContainerCategory(procedure_model_container_categories) => {
                procedure_model_container_categories.primary_key()
            }
            Row::ProcedureModelInstrumentCategory(procedure_model_instrument_categories) => {
                procedure_model_instrument_categories.primary_key()
            }
            Row::ProcedureModelNameplateCategory(procedure_model_nameplate_categories) => {
                procedure_model_nameplate_categories.primary_key()
            }
            Row::ProcedureModelToolCategory(procedure_model_tool_categories) => {
                procedure_model_tool_categories.primary_key()
            }
            Row::ProcedureModel(procedure_models) => procedure_models.primary_key(),
            Row::ProcedureStepModel(procedure_step_models) => procedure_step_models.primary_key(),
            Row::Procedure(procedures) => procedures.primary_key(),
            Row::Processable(processables) => processables.primary_key(),
            Row::ProcessingStep(processing_steps) => processing_steps.primary_key(),
            Row::ProjectState(project_states) => project_states.primary_key(),
            Row::ProjectWorkflowModel(project_workflow_models) => {
                project_workflow_models.primary_key()
            }
            Row::Project(projects) => projects.primary_key(),
            Row::Rank(ranks) => ranks.primary_key(),
            Row::Reagent(reagents) => reagents.primary_key(),
            Row::Role(roles) => roles.primary_key(),
            Row::Room(rooms) => rooms.primary_key(),
            Row::SampleState(sample_states) => sample_states.primary_key(),
            Row::SamplingStepModel(sampling_step_models) => sampling_step_models.primary_key(),
            Row::SamplingStep(sampling_steps) => sampling_steps.primary_key(),
            Row::ShakingStepModel(shaking_step_models) => shaking_step_models.primary_key(),
            Row::ShakingStep(shaking_steps) => shaking_steps.primary_key(),
            Row::SpatialRefSy(spatial_ref_sys) => spatial_ref_sys.primary_key(),
            Row::Spectrum(spectra) => spectra.primary_key(),
            Row::SpectraCollection(spectra_collections) => spectra_collections.primary_key(),
            Row::StepContainerModel(step_container_models) => step_container_models.primary_key(),
            Row::StepInstrument(step_instruments) => step_instruments.primary_key(),
            Row::StepModelCategory(step_model_categories) => step_model_categories.primary_key(),
            Row::StepModelContainerCategory(step_model_container_categories) => {
                step_model_container_categories.primary_key()
            }
            Row::StepModelInstrumentCategory(step_model_instrument_categories) => {
                step_model_instrument_categories.primary_key()
            }
            Row::StepModelInstrumentModel(step_model_instrument_models) => {
                step_model_instrument_models.primary_key()
            }
            Row::StepModelInstrument(step_model_instruments) => {
                step_model_instruments.primary_key()
            }
            Row::StepModelNameplateCategory(step_model_nameplate_categories) => {
                step_model_nameplate_categories.primary_key()
            }
            Row::StepModelToolCategory(step_model_tool_categories) => {
                step_model_tool_categories.primary_key()
            }
            Row::StepModel(step_models) => step_models.primary_key(),
            Row::StepNameplateModel(step_nameplate_models) => step_nameplate_models.primary_key(),
            Row::StepStorageContainer(step_storage_containers) => {
                step_storage_containers.primary_key()
            }
            Row::StepToolModel(step_tool_models) => step_tool_models.primary_key(),
            Row::Step(steps) => steps.primary_key(),
            Row::StorageContainer(storage_containers) => storage_containers.primary_key(),
            Row::Taxon(taxa) => taxa.primary_key(),
            Row::TeamMember(team_members) => team_members.primary_key(),
            Row::TeamProject(team_projects) => team_projects.primary_key(),
            Row::TeamState(team_states) => team_states.primary_key(),
            Row::Team(teams) => teams.primary_key(),
            Row::ToolModel(tool_models) => tool_models.primary_key(),
            Row::TrackableLocation(trackable_locations) => trackable_locations.primary_key(),
            Row::TrackableState(trackable_states) => trackable_states.primary_key(),
            Row::Trackable(trackables) => trackables.primary_key(),
            Row::Unit(units) => units.primary_key(),
            Row::UserEmail(user_emails) => user_emails.primary_key(),
            Row::UserOrganization(user_organizations) => user_organizations.primary_key(),
            Row::User(users) => users.primary_key(),
            Row::VolumetricProcessable(volumetric_processables) => {
                volumetric_processables.primary_key()
            }
            Row::WeighingInstrumentModel(weighing_instrument_models) => {
                weighing_instrument_models.primary_key()
            }
            Row::WeighingStepModel(weighing_step_models) => weighing_step_models.primary_key(),
            Row::WeighingStep(weighing_steps) => weighing_steps.primary_key(),
        }
    }
}
