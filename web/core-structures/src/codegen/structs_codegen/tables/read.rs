use web_common_traits::crud::ReadAll;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ReadError {
    ConversionError,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum F2BReadAll {
    Address(ReadAll<crate::codegen::structs_codegen::tables::addresses::Address>),
    AliquotingInstrumentModel(
        ReadAll<
            crate::codegen::structs_codegen::tables::aliquoting_instrument_models::AliquotingInstrumentModel,
        >,
    ),
    AliquotingStepModel(
        ReadAll<
            crate::codegen::structs_codegen::tables::aliquoting_step_models::AliquotingStepModel,
        >,
    ),
    AliquotingStep(
        ReadAll<
            crate::codegen::structs_codegen::tables::aliquoting_steps::AliquotingStep,
        >,
    ),
    BrandState(
        ReadAll<crate::codegen::structs_codegen::tables::brand_states::BrandState>,
    ),
    Brand(ReadAll<crate::codegen::structs_codegen::tables::brands::Brand>),
    City(ReadAll<crate::codegen::structs_codegen::tables::cities::City>),
    Color(ReadAll<crate::codegen::structs_codegen::tables::colors::Color>),
    CommercialProduct(
        ReadAll<
            crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct,
        >,
    ),
    CommercialReagentModel(
        ReadAll<
            crate::codegen::structs_codegen::tables::commercial_reagent_models::CommercialReagentModel,
        >,
    ),
    ContainerCategory(
        ReadAll<
            crate::codegen::structs_codegen::tables::container_categories::ContainerCategory,
        >,
    ),
    ContainerModel(
        ReadAll<
            crate::codegen::structs_codegen::tables::container_models::ContainerModel,
        >,
    ),
    Country(ReadAll<crate::codegen::structs_codegen::tables::countries::Country>),
    DocumentFormat(
        ReadAll<
            crate::codegen::structs_codegen::tables::document_formats::DocumentFormat,
        >,
    ),
    EmailProvider(
        ReadAll<crate::codegen::structs_codegen::tables::email_providers::EmailProvider>,
    ),
    FractioningStepModel(
        ReadAll<
            crate::codegen::structs_codegen::tables::fractioning_step_models::FractioningStepModel,
        >,
    ),
    FractioningStep(
        ReadAll<
            crate::codegen::structs_codegen::tables::fractioning_steps::FractioningStep,
        >,
    ),
    FreezeDryingStepModel(
        ReadAll<
            crate::codegen::structs_codegen::tables::freeze_drying_step_models::FreezeDryingStepModel,
        >,
    ),
    GrindingStepModel(
        ReadAll<
            crate::codegen::structs_codegen::tables::grinding_step_models::GrindingStepModel,
        >,
    ),
    Icon(ReadAll<crate::codegen::structs_codegen::tables::icons::Icon>),
    InstrumentCategory(
        ReadAll<
            crate::codegen::structs_codegen::tables::instrument_categories::InstrumentCategory,
        >,
    ),
    InstrumentLocation(
        ReadAll<
            crate::codegen::structs_codegen::tables::instrument_locations::InstrumentLocation,
        >,
    ),
    InstrumentModelCategory(
        ReadAll<
            crate::codegen::structs_codegen::tables::instrument_model_categories::InstrumentModelCategory,
        >,
    ),
    InstrumentModel(
        ReadAll<
            crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel,
        >,
    ),
    InstrumentState(
        ReadAll<
            crate::codegen::structs_codegen::tables::instrument_states::InstrumentState,
        >,
    ),
    Instrument(
        ReadAll<crate::codegen::structs_codegen::tables::instruments::Instrument>,
    ),
    LoginProvider(
        ReadAll<crate::codegen::structs_codegen::tables::login_providers::LoginProvider>,
    ),
    Material(ReadAll<crate::codegen::structs_codegen::tables::materials::Material>),
    NameplateCategory(
        ReadAll<
            crate::codegen::structs_codegen::tables::nameplate_categories::NameplateCategory,
        >,
    ),
    NameplateModel(
        ReadAll<
            crate::codegen::structs_codegen::tables::nameplate_models::NameplateModel,
        >,
    ),
    ObservationSubject(
        ReadAll<
            crate::codegen::structs_codegen::tables::observation_subjects::ObservationSubject,
        >,
    ),
    OrganismObservation(
        ReadAll<
            crate::codegen::structs_codegen::tables::organism_observations::OrganismObservation,
        >,
    ),
    OrganismSamplingStepModel(
        ReadAll<
            crate::codegen::structs_codegen::tables::organism_sampling_step_models::OrganismSamplingStepModel,
        >,
    ),
    OrganismTaxon(
        ReadAll<crate::codegen::structs_codegen::tables::organism_taxa::OrganismTaxon>,
    ),
    Organism(ReadAll<crate::codegen::structs_codegen::tables::organisms::Organism>),
    Organization(
        ReadAll<crate::codegen::structs_codegen::tables::organizations::Organization>,
    ),
    PackagingModel(
        ReadAll<
            crate::codegen::structs_codegen::tables::packaging_models::PackagingModel,
        >,
    ),
    PackagingStepModel(
        ReadAll<
            crate::codegen::structs_codegen::tables::packaging_step_models::PackagingStepModel,
        >,
    ),
    PermanenceCategory(
        ReadAll<
            crate::codegen::structs_codegen::tables::permanence_categories::PermanenceCategory,
        >,
    ),
    Photograph(
        ReadAll<crate::codegen::structs_codegen::tables::photographs::Photograph>,
    ),
    ProcedureModelContainerCategory(
        ReadAll<
            crate::codegen::structs_codegen::tables::procedure_model_container_categories::ProcedureModelContainerCategory,
        >,
    ),
    ProcedureModelInstrumentCategory(
        ReadAll<
            crate::codegen::structs_codegen::tables::procedure_model_instrument_categories::ProcedureModelInstrumentCategory,
        >,
    ),
    ProcedureModelNameplateCategory(
        ReadAll<
            crate::codegen::structs_codegen::tables::procedure_model_nameplate_categories::ProcedureModelNameplateCategory,
        >,
    ),
    ProcedureModelToolCategory(
        ReadAll<
            crate::codegen::structs_codegen::tables::procedure_model_tool_categories::ProcedureModelToolCategory,
        >,
    ),
    ProcedureModel(
        ReadAll<
            crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel,
        >,
    ),
    ProcedureStepModel(
        ReadAll<
            crate::codegen::structs_codegen::tables::procedure_step_models::ProcedureStepModel,
        >,
    ),
    Procedure(ReadAll<crate::codegen::structs_codegen::tables::procedures::Procedure>),
    Processable(
        ReadAll<crate::codegen::structs_codegen::tables::processables::Processable>,
    ),
    ProcessingStep(
        ReadAll<
            crate::codegen::structs_codegen::tables::processing_steps::ProcessingStep,
        >,
    ),
    ProjectState(
        ReadAll<crate::codegen::structs_codegen::tables::project_states::ProjectState>,
    ),
    ProjectWorkflowModel(
        ReadAll<
            crate::codegen::structs_codegen::tables::project_workflow_models::ProjectWorkflowModel,
        >,
    ),
    Project(ReadAll<crate::codegen::structs_codegen::tables::projects::Project>),
    Rank(ReadAll<crate::codegen::structs_codegen::tables::ranks::Rank>),
    Role(ReadAll<crate::codegen::structs_codegen::tables::roles::Role>),
    Room(ReadAll<crate::codegen::structs_codegen::tables::rooms::Room>),
    SampleState(
        ReadAll<crate::codegen::structs_codegen::tables::sample_states::SampleState>,
    ),
    SamplingStepModel(
        ReadAll<
            crate::codegen::structs_codegen::tables::sampling_step_models::SamplingStepModel,
        >,
    ),
    SamplingStep(
        ReadAll<crate::codegen::structs_codegen::tables::sampling_steps::SamplingStep>,
    ),
    SpatialRefSy(
        ReadAll<crate::codegen::structs_codegen::tables::spatial_ref_sys::SpatialRefSy>,
    ),
    Spectrum(ReadAll<crate::codegen::structs_codegen::tables::spectra::Spectrum>),
    SpectraCollection(
        ReadAll<
            crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection,
        >,
    ),
    StepContainerModel(
        ReadAll<
            crate::codegen::structs_codegen::tables::step_container_models::StepContainerModel,
        >,
    ),
    StepInstrument(
        ReadAll<
            crate::codegen::structs_codegen::tables::step_instruments::StepInstrument,
        >,
    ),
    StepModelCategory(
        ReadAll<
            crate::codegen::structs_codegen::tables::step_model_categories::StepModelCategory,
        >,
    ),
    StepModelContainerCategory(
        ReadAll<
            crate::codegen::structs_codegen::tables::step_model_container_categories::StepModelContainerCategory,
        >,
    ),
    StepModelInstrumentCategory(
        ReadAll<
            crate::codegen::structs_codegen::tables::step_model_instrument_categories::StepModelInstrumentCategory,
        >,
    ),
    StepModelInstrumentModel(
        ReadAll<
            crate::codegen::structs_codegen::tables::step_model_instrument_models::StepModelInstrumentModel,
        >,
    ),
    StepModelInstrument(
        ReadAll<
            crate::codegen::structs_codegen::tables::step_model_instruments::StepModelInstrument,
        >,
    ),
    StepModelNameplateCategory(
        ReadAll<
            crate::codegen::structs_codegen::tables::step_model_nameplate_categories::StepModelNameplateCategory,
        >,
    ),
    StepModelToolCategory(
        ReadAll<
            crate::codegen::structs_codegen::tables::step_model_tool_categories::StepModelToolCategory,
        >,
    ),
    StepModel(ReadAll<crate::codegen::structs_codegen::tables::step_models::StepModel>),
    StepNameplateModel(
        ReadAll<
            crate::codegen::structs_codegen::tables::step_nameplate_models::StepNameplateModel,
        >,
    ),
    StepStorageContainer(
        ReadAll<
            crate::codegen::structs_codegen::tables::step_storage_containers::StepStorageContainer,
        >,
    ),
    StepToolModel(
        ReadAll<crate::codegen::structs_codegen::tables::step_tool_models::StepToolModel>,
    ),
    Step(ReadAll<crate::codegen::structs_codegen::tables::steps::Step>),
    StorageContainer(
        ReadAll<
            crate::codegen::structs_codegen::tables::storage_containers::StorageContainer,
        >,
    ),
    Taxon(ReadAll<crate::codegen::structs_codegen::tables::taxa::Taxon>),
    TeamMember(
        ReadAll<crate::codegen::structs_codegen::tables::team_members::TeamMember>,
    ),
    TeamProject(
        ReadAll<crate::codegen::structs_codegen::tables::team_projects::TeamProject>,
    ),
    TeamState(ReadAll<crate::codegen::structs_codegen::tables::team_states::TeamState>),
    Team(ReadAll<crate::codegen::structs_codegen::tables::teams::Team>),
    ToolCategory(
        ReadAll<crate::codegen::structs_codegen::tables::tool_categories::ToolCategory>,
    ),
    ToolModel(ReadAll<crate::codegen::structs_codegen::tables::tool_models::ToolModel>),
    TrackableLocation(
        ReadAll<
            crate::codegen::structs_codegen::tables::trackable_locations::TrackableLocation,
        >,
    ),
    TrackableState(
        ReadAll<
            crate::codegen::structs_codegen::tables::trackable_states::TrackableState,
        >,
    ),
    Trackable(ReadAll<crate::codegen::structs_codegen::tables::trackables::Trackable>),
    Unit(ReadAll<crate::codegen::structs_codegen::tables::units::Unit>),
    UserEmail(ReadAll<crate::codegen::structs_codegen::tables::user_emails::UserEmail>),
    UserOrganization(
        ReadAll<
            crate::codegen::structs_codegen::tables::user_organizations::UserOrganization,
        >,
    ),
    User(ReadAll<crate::codegen::structs_codegen::tables::users::User>),
    WeighingInstrumentModel(
        ReadAll<
            crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel,
        >,
    ),
    WeighingStepModel(
        ReadAll<
            crate::codegen::structs_codegen::tables::weighing_step_models::WeighingStepModel,
        >,
    ),
    WeighingStep(
        ReadAll<crate::codegen::structs_codegen::tables::weighing_steps::WeighingStep>,
    ),
}
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum B2FReadAll {
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
    BrandState(Vec<crate::codegen::structs_codegen::tables::brand_states::BrandState>),
    Brand(Vec<crate::codegen::structs_codegen::tables::brands::Brand>),
    City(Vec<crate::codegen::structs_codegen::tables::cities::City>),
    Color(Vec<crate::codegen::structs_codegen::tables::colors::Color>),
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
    ContainerCategory(
        Vec<
            crate::codegen::structs_codegen::tables::container_categories::ContainerCategory,
        >,
    ),
    ContainerModel(
        Vec<crate::codegen::structs_codegen::tables::container_models::ContainerModel>,
    ),
    Country(Vec<crate::codegen::structs_codegen::tables::countries::Country>),
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
    GrindingStepModel(
        Vec<
            crate::codegen::structs_codegen::tables::grinding_step_models::GrindingStepModel,
        >,
    ),
    Icon(Vec<crate::codegen::structs_codegen::tables::icons::Icon>),
    InstrumentCategory(
        Vec<
            crate::codegen::structs_codegen::tables::instrument_categories::InstrumentCategory,
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
    NameplateCategory(
        Vec<
            crate::codegen::structs_codegen::tables::nameplate_categories::NameplateCategory,
        >,
    ),
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
    ToolCategory(
        Vec<crate::codegen::structs_codegen::tables::tool_categories::ToolCategory>,
    ),
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
impl From<ReadAll<crate::codegen::structs_codegen::tables::addresses::Address>> for F2BReadAll {
    fn from(value: ReadAll<crate::codegen::structs_codegen::tables::addresses::Address>) -> Self {
        F2BReadAll::Address(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::addresses::Address>> for B2FReadAll {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::addresses::Address>) -> Self {
        B2FReadAll::Address(value)
    }
}
impl TryFrom<B2FReadAll> for Vec<crate::codegen::structs_codegen::tables::addresses::Address> {
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::Address(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<
    ReadAll<
        crate::codegen::structs_codegen::tables::aliquoting_instrument_models::AliquotingInstrumentModel,
    >,
> for F2BReadAll {
    fn from(
        value: ReadAll<
            crate::codegen::structs_codegen::tables::aliquoting_instrument_models::AliquotingInstrumentModel,
        >,
    ) -> Self {
        F2BReadAll::AliquotingInstrumentModel(value)
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::aliquoting_instrument_models::AliquotingInstrumentModel,
    >,
> for B2FReadAll {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::aliquoting_instrument_models::AliquotingInstrumentModel,
        >,
    ) -> Self {
        B2FReadAll::AliquotingInstrumentModel(value)
    }
}
impl TryFrom<B2FReadAll>
for Vec<
    crate::codegen::structs_codegen::tables::aliquoting_instrument_models::AliquotingInstrumentModel,
> {
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::AliquotingInstrumentModel(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl
    From<
        ReadAll<
            crate::codegen::structs_codegen::tables::aliquoting_step_models::AliquotingStepModel,
        >,
    > for F2BReadAll
{
    fn from(
        value: ReadAll<
            crate::codegen::structs_codegen::tables::aliquoting_step_models::AliquotingStepModel,
        >,
    ) -> Self {
        F2BReadAll::AliquotingStepModel(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::aliquoting_step_models::AliquotingStepModel>>
    for B2FReadAll
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::aliquoting_step_models::AliquotingStepModel,
        >,
    ) -> Self {
        B2FReadAll::AliquotingStepModel(value)
    }
}
impl TryFrom<B2FReadAll>
    for Vec<crate::codegen::structs_codegen::tables::aliquoting_step_models::AliquotingStepModel>
{
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::AliquotingStepModel(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<ReadAll<crate::codegen::structs_codegen::tables::aliquoting_steps::AliquotingStep>>
    for F2BReadAll
{
    fn from(
        value: ReadAll<crate::codegen::structs_codegen::tables::aliquoting_steps::AliquotingStep>,
    ) -> Self {
        F2BReadAll::AliquotingStep(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::aliquoting_steps::AliquotingStep>>
    for B2FReadAll
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::aliquoting_steps::AliquotingStep>,
    ) -> Self {
        B2FReadAll::AliquotingStep(value)
    }
}
impl TryFrom<B2FReadAll>
    for Vec<crate::codegen::structs_codegen::tables::aliquoting_steps::AliquotingStep>
{
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::AliquotingStep(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<ReadAll<crate::codegen::structs_codegen::tables::brand_states::BrandState>>
    for F2BReadAll
{
    fn from(
        value: ReadAll<crate::codegen::structs_codegen::tables::brand_states::BrandState>,
    ) -> Self {
        F2BReadAll::BrandState(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::brand_states::BrandState>> for B2FReadAll {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::brand_states::BrandState>) -> Self {
        B2FReadAll::BrandState(value)
    }
}
impl TryFrom<B2FReadAll>
    for Vec<crate::codegen::structs_codegen::tables::brand_states::BrandState>
{
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::BrandState(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<ReadAll<crate::codegen::structs_codegen::tables::brands::Brand>> for F2BReadAll {
    fn from(value: ReadAll<crate::codegen::structs_codegen::tables::brands::Brand>) -> Self {
        F2BReadAll::Brand(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::brands::Brand>> for B2FReadAll {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::brands::Brand>) -> Self {
        B2FReadAll::Brand(value)
    }
}
impl TryFrom<B2FReadAll> for Vec<crate::codegen::structs_codegen::tables::brands::Brand> {
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::Brand(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<ReadAll<crate::codegen::structs_codegen::tables::cities::City>> for F2BReadAll {
    fn from(value: ReadAll<crate::codegen::structs_codegen::tables::cities::City>) -> Self {
        F2BReadAll::City(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::cities::City>> for B2FReadAll {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::cities::City>) -> Self {
        B2FReadAll::City(value)
    }
}
impl TryFrom<B2FReadAll> for Vec<crate::codegen::structs_codegen::tables::cities::City> {
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::City(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<ReadAll<crate::codegen::structs_codegen::tables::colors::Color>> for F2BReadAll {
    fn from(value: ReadAll<crate::codegen::structs_codegen::tables::colors::Color>) -> Self {
        F2BReadAll::Color(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::colors::Color>> for B2FReadAll {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::colors::Color>) -> Self {
        B2FReadAll::Color(value)
    }
}
impl TryFrom<B2FReadAll> for Vec<crate::codegen::structs_codegen::tables::colors::Color> {
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::Color(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<ReadAll<crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct>>
    for F2BReadAll
{
    fn from(
        value: ReadAll<
            crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct,
        >,
    ) -> Self {
        F2BReadAll::CommercialProduct(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct>>
    for B2FReadAll
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct>,
    ) -> Self {
        B2FReadAll::CommercialProduct(value)
    }
}
impl TryFrom<B2FReadAll>
    for Vec<crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct>
{
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::CommercialProduct(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<
    ReadAll<
        crate::codegen::structs_codegen::tables::commercial_reagent_models::CommercialReagentModel,
    >,
> for F2BReadAll {
    fn from(
        value: ReadAll<
            crate::codegen::structs_codegen::tables::commercial_reagent_models::CommercialReagentModel,
        >,
    ) -> Self {
        F2BReadAll::CommercialReagentModel(value)
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::commercial_reagent_models::CommercialReagentModel,
    >,
> for B2FReadAll {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::commercial_reagent_models::CommercialReagentModel,
        >,
    ) -> Self {
        B2FReadAll::CommercialReagentModel(value)
    }
}
impl TryFrom<B2FReadAll>
    for Vec<
        crate::codegen::structs_codegen::tables::commercial_reagent_models::CommercialReagentModel,
    >
{
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::CommercialReagentModel(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<ReadAll<crate::codegen::structs_codegen::tables::container_categories::ContainerCategory>>
    for F2BReadAll
{
    fn from(
        value: ReadAll<
            crate::codegen::structs_codegen::tables::container_categories::ContainerCategory,
        >,
    ) -> Self {
        F2BReadAll::ContainerCategory(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::container_categories::ContainerCategory>>
    for B2FReadAll
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::container_categories::ContainerCategory,
        >,
    ) -> Self {
        B2FReadAll::ContainerCategory(value)
    }
}
impl TryFrom<B2FReadAll>
    for Vec<crate::codegen::structs_codegen::tables::container_categories::ContainerCategory>
{
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::ContainerCategory(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<ReadAll<crate::codegen::structs_codegen::tables::container_models::ContainerModel>>
    for F2BReadAll
{
    fn from(
        value: ReadAll<crate::codegen::structs_codegen::tables::container_models::ContainerModel>,
    ) -> Self {
        F2BReadAll::ContainerModel(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::container_models::ContainerModel>>
    for B2FReadAll
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::container_models::ContainerModel>,
    ) -> Self {
        B2FReadAll::ContainerModel(value)
    }
}
impl TryFrom<B2FReadAll>
    for Vec<crate::codegen::structs_codegen::tables::container_models::ContainerModel>
{
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::ContainerModel(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<ReadAll<crate::codegen::structs_codegen::tables::countries::Country>> for F2BReadAll {
    fn from(value: ReadAll<crate::codegen::structs_codegen::tables::countries::Country>) -> Self {
        F2BReadAll::Country(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::countries::Country>> for B2FReadAll {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::countries::Country>) -> Self {
        B2FReadAll::Country(value)
    }
}
impl TryFrom<B2FReadAll> for Vec<crate::codegen::structs_codegen::tables::countries::Country> {
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::Country(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<ReadAll<crate::codegen::structs_codegen::tables::document_formats::DocumentFormat>>
    for F2BReadAll
{
    fn from(
        value: ReadAll<crate::codegen::structs_codegen::tables::document_formats::DocumentFormat>,
    ) -> Self {
        F2BReadAll::DocumentFormat(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::document_formats::DocumentFormat>>
    for B2FReadAll
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::document_formats::DocumentFormat>,
    ) -> Self {
        B2FReadAll::DocumentFormat(value)
    }
}
impl TryFrom<B2FReadAll>
    for Vec<crate::codegen::structs_codegen::tables::document_formats::DocumentFormat>
{
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::DocumentFormat(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<ReadAll<crate::codegen::structs_codegen::tables::email_providers::EmailProvider>>
    for F2BReadAll
{
    fn from(
        value: ReadAll<crate::codegen::structs_codegen::tables::email_providers::EmailProvider>,
    ) -> Self {
        F2BReadAll::EmailProvider(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::email_providers::EmailProvider>>
    for B2FReadAll
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::email_providers::EmailProvider>,
    ) -> Self {
        B2FReadAll::EmailProvider(value)
    }
}
impl TryFrom<B2FReadAll>
    for Vec<crate::codegen::structs_codegen::tables::email_providers::EmailProvider>
{
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::EmailProvider(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl
    From<
        ReadAll<
            crate::codegen::structs_codegen::tables::fractioning_step_models::FractioningStepModel,
        >,
    > for F2BReadAll
{
    fn from(
        value: ReadAll<
            crate::codegen::structs_codegen::tables::fractioning_step_models::FractioningStepModel,
        >,
    ) -> Self {
        F2BReadAll::FractioningStepModel(value)
    }
}
impl
    From<
        Vec<crate::codegen::structs_codegen::tables::fractioning_step_models::FractioningStepModel>,
    > for B2FReadAll
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::fractioning_step_models::FractioningStepModel,
        >,
    ) -> Self {
        B2FReadAll::FractioningStepModel(value)
    }
}
impl TryFrom<B2FReadAll>
    for Vec<crate::codegen::structs_codegen::tables::fractioning_step_models::FractioningStepModel>
{
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::FractioningStepModel(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<ReadAll<crate::codegen::structs_codegen::tables::fractioning_steps::FractioningStep>>
    for F2BReadAll
{
    fn from(
        value: ReadAll<crate::codegen::structs_codegen::tables::fractioning_steps::FractioningStep>,
    ) -> Self {
        F2BReadAll::FractioningStep(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::fractioning_steps::FractioningStep>>
    for B2FReadAll
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::fractioning_steps::FractioningStep>,
    ) -> Self {
        B2FReadAll::FractioningStep(value)
    }
}
impl TryFrom<B2FReadAll>
    for Vec<crate::codegen::structs_codegen::tables::fractioning_steps::FractioningStep>
{
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::FractioningStep(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<
    ReadAll<
        crate::codegen::structs_codegen::tables::freeze_drying_step_models::FreezeDryingStepModel,
    >,
> for F2BReadAll {
    fn from(
        value: ReadAll<
            crate::codegen::structs_codegen::tables::freeze_drying_step_models::FreezeDryingStepModel,
        >,
    ) -> Self {
        F2BReadAll::FreezeDryingStepModel(value)
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::freeze_drying_step_models::FreezeDryingStepModel,
    >,
> for B2FReadAll {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::freeze_drying_step_models::FreezeDryingStepModel,
        >,
    ) -> Self {
        B2FReadAll::FreezeDryingStepModel(value)
    }
}
impl TryFrom<B2FReadAll>
    for Vec<
        crate::codegen::structs_codegen::tables::freeze_drying_step_models::FreezeDryingStepModel,
    >
{
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::FreezeDryingStepModel(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<ReadAll<crate::codegen::structs_codegen::tables::grinding_step_models::GrindingStepModel>>
    for F2BReadAll
{
    fn from(
        value: ReadAll<
            crate::codegen::structs_codegen::tables::grinding_step_models::GrindingStepModel,
        >,
    ) -> Self {
        F2BReadAll::GrindingStepModel(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::grinding_step_models::GrindingStepModel>>
    for B2FReadAll
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::grinding_step_models::GrindingStepModel,
        >,
    ) -> Self {
        B2FReadAll::GrindingStepModel(value)
    }
}
impl TryFrom<B2FReadAll>
    for Vec<crate::codegen::structs_codegen::tables::grinding_step_models::GrindingStepModel>
{
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::GrindingStepModel(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<ReadAll<crate::codegen::structs_codegen::tables::icons::Icon>> for F2BReadAll {
    fn from(value: ReadAll<crate::codegen::structs_codegen::tables::icons::Icon>) -> Self {
        F2BReadAll::Icon(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::icons::Icon>> for B2FReadAll {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::icons::Icon>) -> Self {
        B2FReadAll::Icon(value)
    }
}
impl TryFrom<B2FReadAll> for Vec<crate::codegen::structs_codegen::tables::icons::Icon> {
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::Icon(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl
    From<
        ReadAll<crate::codegen::structs_codegen::tables::instrument_categories::InstrumentCategory>,
    > for F2BReadAll
{
    fn from(
        value: ReadAll<
            crate::codegen::structs_codegen::tables::instrument_categories::InstrumentCategory,
        >,
    ) -> Self {
        F2BReadAll::InstrumentCategory(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::instrument_categories::InstrumentCategory>>
    for B2FReadAll
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::instrument_categories::InstrumentCategory,
        >,
    ) -> Self {
        B2FReadAll::InstrumentCategory(value)
    }
}
impl TryFrom<B2FReadAll>
    for Vec<crate::codegen::structs_codegen::tables::instrument_categories::InstrumentCategory>
{
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::InstrumentCategory(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl
    From<ReadAll<crate::codegen::structs_codegen::tables::instrument_locations::InstrumentLocation>>
    for F2BReadAll
{
    fn from(
        value: ReadAll<
            crate::codegen::structs_codegen::tables::instrument_locations::InstrumentLocation,
        >,
    ) -> Self {
        F2BReadAll::InstrumentLocation(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::instrument_locations::InstrumentLocation>>
    for B2FReadAll
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::instrument_locations::InstrumentLocation,
        >,
    ) -> Self {
        B2FReadAll::InstrumentLocation(value)
    }
}
impl TryFrom<B2FReadAll>
    for Vec<crate::codegen::structs_codegen::tables::instrument_locations::InstrumentLocation>
{
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::InstrumentLocation(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<
    ReadAll<
        crate::codegen::structs_codegen::tables::instrument_model_categories::InstrumentModelCategory,
    >,
> for F2BReadAll {
    fn from(
        value: ReadAll<
            crate::codegen::structs_codegen::tables::instrument_model_categories::InstrumentModelCategory,
        >,
    ) -> Self {
        F2BReadAll::InstrumentModelCategory(value)
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::instrument_model_categories::InstrumentModelCategory,
    >,
> for B2FReadAll {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::instrument_model_categories::InstrumentModelCategory,
        >,
    ) -> Self {
        B2FReadAll::InstrumentModelCategory(value)
    }
}
impl TryFrom<B2FReadAll>
for Vec<
    crate::codegen::structs_codegen::tables::instrument_model_categories::InstrumentModelCategory,
> {
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::InstrumentModelCategory(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<ReadAll<crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel>>
    for F2BReadAll
{
    fn from(
        value: ReadAll<crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel>,
    ) -> Self {
        F2BReadAll::InstrumentModel(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel>>
    for B2FReadAll
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel>,
    ) -> Self {
        B2FReadAll::InstrumentModel(value)
    }
}
impl TryFrom<B2FReadAll>
    for Vec<crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel>
{
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::InstrumentModel(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<ReadAll<crate::codegen::structs_codegen::tables::instrument_states::InstrumentState>>
    for F2BReadAll
{
    fn from(
        value: ReadAll<crate::codegen::structs_codegen::tables::instrument_states::InstrumentState>,
    ) -> Self {
        F2BReadAll::InstrumentState(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::instrument_states::InstrumentState>>
    for B2FReadAll
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::instrument_states::InstrumentState>,
    ) -> Self {
        B2FReadAll::InstrumentState(value)
    }
}
impl TryFrom<B2FReadAll>
    for Vec<crate::codegen::structs_codegen::tables::instrument_states::InstrumentState>
{
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::InstrumentState(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<ReadAll<crate::codegen::structs_codegen::tables::instruments::Instrument>>
    for F2BReadAll
{
    fn from(
        value: ReadAll<crate::codegen::structs_codegen::tables::instruments::Instrument>,
    ) -> Self {
        F2BReadAll::Instrument(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::instruments::Instrument>> for B2FReadAll {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::instruments::Instrument>) -> Self {
        B2FReadAll::Instrument(value)
    }
}
impl TryFrom<B2FReadAll> for Vec<crate::codegen::structs_codegen::tables::instruments::Instrument> {
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::Instrument(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<ReadAll<crate::codegen::structs_codegen::tables::login_providers::LoginProvider>>
    for F2BReadAll
{
    fn from(
        value: ReadAll<crate::codegen::structs_codegen::tables::login_providers::LoginProvider>,
    ) -> Self {
        F2BReadAll::LoginProvider(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::login_providers::LoginProvider>>
    for B2FReadAll
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::login_providers::LoginProvider>,
    ) -> Self {
        B2FReadAll::LoginProvider(value)
    }
}
impl TryFrom<B2FReadAll>
    for Vec<crate::codegen::structs_codegen::tables::login_providers::LoginProvider>
{
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::LoginProvider(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<ReadAll<crate::codegen::structs_codegen::tables::materials::Material>> for F2BReadAll {
    fn from(value: ReadAll<crate::codegen::structs_codegen::tables::materials::Material>) -> Self {
        F2BReadAll::Material(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::materials::Material>> for B2FReadAll {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::materials::Material>) -> Self {
        B2FReadAll::Material(value)
    }
}
impl TryFrom<B2FReadAll> for Vec<crate::codegen::structs_codegen::tables::materials::Material> {
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::Material(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<ReadAll<crate::codegen::structs_codegen::tables::nameplate_categories::NameplateCategory>>
    for F2BReadAll
{
    fn from(
        value: ReadAll<
            crate::codegen::structs_codegen::tables::nameplate_categories::NameplateCategory,
        >,
    ) -> Self {
        F2BReadAll::NameplateCategory(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::nameplate_categories::NameplateCategory>>
    for B2FReadAll
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::nameplate_categories::NameplateCategory,
        >,
    ) -> Self {
        B2FReadAll::NameplateCategory(value)
    }
}
impl TryFrom<B2FReadAll>
    for Vec<crate::codegen::structs_codegen::tables::nameplate_categories::NameplateCategory>
{
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::NameplateCategory(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<ReadAll<crate::codegen::structs_codegen::tables::nameplate_models::NameplateModel>>
    for F2BReadAll
{
    fn from(
        value: ReadAll<crate::codegen::structs_codegen::tables::nameplate_models::NameplateModel>,
    ) -> Self {
        F2BReadAll::NameplateModel(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::nameplate_models::NameplateModel>>
    for B2FReadAll
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::nameplate_models::NameplateModel>,
    ) -> Self {
        B2FReadAll::NameplateModel(value)
    }
}
impl TryFrom<B2FReadAll>
    for Vec<crate::codegen::structs_codegen::tables::nameplate_models::NameplateModel>
{
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::NameplateModel(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl
    From<ReadAll<crate::codegen::structs_codegen::tables::observation_subjects::ObservationSubject>>
    for F2BReadAll
{
    fn from(
        value: ReadAll<
            crate::codegen::structs_codegen::tables::observation_subjects::ObservationSubject,
        >,
    ) -> Self {
        F2BReadAll::ObservationSubject(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::observation_subjects::ObservationSubject>>
    for B2FReadAll
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::observation_subjects::ObservationSubject,
        >,
    ) -> Self {
        B2FReadAll::ObservationSubject(value)
    }
}
impl TryFrom<B2FReadAll>
    for Vec<crate::codegen::structs_codegen::tables::observation_subjects::ObservationSubject>
{
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::ObservationSubject(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl
    From<
        ReadAll<
            crate::codegen::structs_codegen::tables::organism_observations::OrganismObservation,
        >,
    > for F2BReadAll
{
    fn from(
        value: ReadAll<
            crate::codegen::structs_codegen::tables::organism_observations::OrganismObservation,
        >,
    ) -> Self {
        F2BReadAll::OrganismObservation(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::organism_observations::OrganismObservation>>
    for B2FReadAll
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::organism_observations::OrganismObservation,
        >,
    ) -> Self {
        B2FReadAll::OrganismObservation(value)
    }
}
impl TryFrom<B2FReadAll>
    for Vec<crate::codegen::structs_codegen::tables::organism_observations::OrganismObservation>
{
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::OrganismObservation(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<
    ReadAll<
        crate::codegen::structs_codegen::tables::organism_sampling_step_models::OrganismSamplingStepModel,
    >,
> for F2BReadAll {
    fn from(
        value: ReadAll<
            crate::codegen::structs_codegen::tables::organism_sampling_step_models::OrganismSamplingStepModel,
        >,
    ) -> Self {
        F2BReadAll::OrganismSamplingStepModel(value)
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::organism_sampling_step_models::OrganismSamplingStepModel,
    >,
> for B2FReadAll {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::organism_sampling_step_models::OrganismSamplingStepModel,
        >,
    ) -> Self {
        B2FReadAll::OrganismSamplingStepModel(value)
    }
}
impl TryFrom<B2FReadAll>
for Vec<
    crate::codegen::structs_codegen::tables::organism_sampling_step_models::OrganismSamplingStepModel,
> {
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::OrganismSamplingStepModel(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<ReadAll<crate::codegen::structs_codegen::tables::organism_taxa::OrganismTaxon>>
    for F2BReadAll
{
    fn from(
        value: ReadAll<crate::codegen::structs_codegen::tables::organism_taxa::OrganismTaxon>,
    ) -> Self {
        F2BReadAll::OrganismTaxon(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::organism_taxa::OrganismTaxon>>
    for B2FReadAll
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::organism_taxa::OrganismTaxon>,
    ) -> Self {
        B2FReadAll::OrganismTaxon(value)
    }
}
impl TryFrom<B2FReadAll>
    for Vec<crate::codegen::structs_codegen::tables::organism_taxa::OrganismTaxon>
{
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::OrganismTaxon(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<ReadAll<crate::codegen::structs_codegen::tables::organisms::Organism>> for F2BReadAll {
    fn from(value: ReadAll<crate::codegen::structs_codegen::tables::organisms::Organism>) -> Self {
        F2BReadAll::Organism(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::organisms::Organism>> for B2FReadAll {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::organisms::Organism>) -> Self {
        B2FReadAll::Organism(value)
    }
}
impl TryFrom<B2FReadAll> for Vec<crate::codegen::structs_codegen::tables::organisms::Organism> {
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::Organism(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<ReadAll<crate::codegen::structs_codegen::tables::organizations::Organization>>
    for F2BReadAll
{
    fn from(
        value: ReadAll<crate::codegen::structs_codegen::tables::organizations::Organization>,
    ) -> Self {
        F2BReadAll::Organization(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::organizations::Organization>>
    for B2FReadAll
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::organizations::Organization>,
    ) -> Self {
        B2FReadAll::Organization(value)
    }
}
impl TryFrom<B2FReadAll>
    for Vec<crate::codegen::structs_codegen::tables::organizations::Organization>
{
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::Organization(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<ReadAll<crate::codegen::structs_codegen::tables::packaging_models::PackagingModel>>
    for F2BReadAll
{
    fn from(
        value: ReadAll<crate::codegen::structs_codegen::tables::packaging_models::PackagingModel>,
    ) -> Self {
        F2BReadAll::PackagingModel(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::packaging_models::PackagingModel>>
    for B2FReadAll
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::packaging_models::PackagingModel>,
    ) -> Self {
        B2FReadAll::PackagingModel(value)
    }
}
impl TryFrom<B2FReadAll>
    for Vec<crate::codegen::structs_codegen::tables::packaging_models::PackagingModel>
{
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::PackagingModel(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl
    From<
        ReadAll<crate::codegen::structs_codegen::tables::packaging_step_models::PackagingStepModel>,
    > for F2BReadAll
{
    fn from(
        value: ReadAll<
            crate::codegen::structs_codegen::tables::packaging_step_models::PackagingStepModel,
        >,
    ) -> Self {
        F2BReadAll::PackagingStepModel(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::packaging_step_models::PackagingStepModel>>
    for B2FReadAll
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::packaging_step_models::PackagingStepModel,
        >,
    ) -> Self {
        B2FReadAll::PackagingStepModel(value)
    }
}
impl TryFrom<B2FReadAll>
    for Vec<crate::codegen::structs_codegen::tables::packaging_step_models::PackagingStepModel>
{
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::PackagingStepModel(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl
    From<
        ReadAll<crate::codegen::structs_codegen::tables::permanence_categories::PermanenceCategory>,
    > for F2BReadAll
{
    fn from(
        value: ReadAll<
            crate::codegen::structs_codegen::tables::permanence_categories::PermanenceCategory,
        >,
    ) -> Self {
        F2BReadAll::PermanenceCategory(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::permanence_categories::PermanenceCategory>>
    for B2FReadAll
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::permanence_categories::PermanenceCategory,
        >,
    ) -> Self {
        B2FReadAll::PermanenceCategory(value)
    }
}
impl TryFrom<B2FReadAll>
    for Vec<crate::codegen::structs_codegen::tables::permanence_categories::PermanenceCategory>
{
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::PermanenceCategory(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<ReadAll<crate::codegen::structs_codegen::tables::photographs::Photograph>>
    for F2BReadAll
{
    fn from(
        value: ReadAll<crate::codegen::structs_codegen::tables::photographs::Photograph>,
    ) -> Self {
        F2BReadAll::Photograph(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::photographs::Photograph>> for B2FReadAll {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::photographs::Photograph>) -> Self {
        B2FReadAll::Photograph(value)
    }
}
impl TryFrom<B2FReadAll> for Vec<crate::codegen::structs_codegen::tables::photographs::Photograph> {
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::Photograph(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<
    ReadAll<
        crate::codegen::structs_codegen::tables::procedure_model_container_categories::ProcedureModelContainerCategory,
    >,
> for F2BReadAll {
    fn from(
        value: ReadAll<
            crate::codegen::structs_codegen::tables::procedure_model_container_categories::ProcedureModelContainerCategory,
        >,
    ) -> Self {
        F2BReadAll::ProcedureModelContainerCategory(value)
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::procedure_model_container_categories::ProcedureModelContainerCategory,
    >,
> for B2FReadAll {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::procedure_model_container_categories::ProcedureModelContainerCategory,
        >,
    ) -> Self {
        B2FReadAll::ProcedureModelContainerCategory(value)
    }
}
impl TryFrom<B2FReadAll>
for Vec<
    crate::codegen::structs_codegen::tables::procedure_model_container_categories::ProcedureModelContainerCategory,
> {
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::ProcedureModelContainerCategory(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<
    ReadAll<
        crate::codegen::structs_codegen::tables::procedure_model_instrument_categories::ProcedureModelInstrumentCategory,
    >,
> for F2BReadAll {
    fn from(
        value: ReadAll<
            crate::codegen::structs_codegen::tables::procedure_model_instrument_categories::ProcedureModelInstrumentCategory,
        >,
    ) -> Self {
        F2BReadAll::ProcedureModelInstrumentCategory(value)
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::procedure_model_instrument_categories::ProcedureModelInstrumentCategory,
    >,
> for B2FReadAll {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::procedure_model_instrument_categories::ProcedureModelInstrumentCategory,
        >,
    ) -> Self {
        B2FReadAll::ProcedureModelInstrumentCategory(value)
    }
}
impl TryFrom<B2FReadAll>
for Vec<
    crate::codegen::structs_codegen::tables::procedure_model_instrument_categories::ProcedureModelInstrumentCategory,
> {
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::ProcedureModelInstrumentCategory(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<
    ReadAll<
        crate::codegen::structs_codegen::tables::procedure_model_nameplate_categories::ProcedureModelNameplateCategory,
    >,
> for F2BReadAll {
    fn from(
        value: ReadAll<
            crate::codegen::structs_codegen::tables::procedure_model_nameplate_categories::ProcedureModelNameplateCategory,
        >,
    ) -> Self {
        F2BReadAll::ProcedureModelNameplateCategory(value)
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::procedure_model_nameplate_categories::ProcedureModelNameplateCategory,
    >,
> for B2FReadAll {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::procedure_model_nameplate_categories::ProcedureModelNameplateCategory,
        >,
    ) -> Self {
        B2FReadAll::ProcedureModelNameplateCategory(value)
    }
}
impl TryFrom<B2FReadAll>
for Vec<
    crate::codegen::structs_codegen::tables::procedure_model_nameplate_categories::ProcedureModelNameplateCategory,
> {
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::ProcedureModelNameplateCategory(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<
    ReadAll<
        crate::codegen::structs_codegen::tables::procedure_model_tool_categories::ProcedureModelToolCategory,
    >,
> for F2BReadAll {
    fn from(
        value: ReadAll<
            crate::codegen::structs_codegen::tables::procedure_model_tool_categories::ProcedureModelToolCategory,
        >,
    ) -> Self {
        F2BReadAll::ProcedureModelToolCategory(value)
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::procedure_model_tool_categories::ProcedureModelToolCategory,
    >,
> for B2FReadAll {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::procedure_model_tool_categories::ProcedureModelToolCategory,
        >,
    ) -> Self {
        B2FReadAll::ProcedureModelToolCategory(value)
    }
}
impl TryFrom<B2FReadAll>
for Vec<
    crate::codegen::structs_codegen::tables::procedure_model_tool_categories::ProcedureModelToolCategory,
> {
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::ProcedureModelToolCategory(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<ReadAll<crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel>>
    for F2BReadAll
{
    fn from(
        value: ReadAll<crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel>,
    ) -> Self {
        F2BReadAll::ProcedureModel(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel>>
    for B2FReadAll
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel>,
    ) -> Self {
        B2FReadAll::ProcedureModel(value)
    }
}
impl TryFrom<B2FReadAll>
    for Vec<crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel>
{
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::ProcedureModel(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl
    From<
        ReadAll<crate::codegen::structs_codegen::tables::procedure_step_models::ProcedureStepModel>,
    > for F2BReadAll
{
    fn from(
        value: ReadAll<
            crate::codegen::structs_codegen::tables::procedure_step_models::ProcedureStepModel,
        >,
    ) -> Self {
        F2BReadAll::ProcedureStepModel(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::procedure_step_models::ProcedureStepModel>>
    for B2FReadAll
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::procedure_step_models::ProcedureStepModel,
        >,
    ) -> Self {
        B2FReadAll::ProcedureStepModel(value)
    }
}
impl TryFrom<B2FReadAll>
    for Vec<crate::codegen::structs_codegen::tables::procedure_step_models::ProcedureStepModel>
{
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::ProcedureStepModel(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<ReadAll<crate::codegen::structs_codegen::tables::procedures::Procedure>> for F2BReadAll {
    fn from(
        value: ReadAll<crate::codegen::structs_codegen::tables::procedures::Procedure>,
    ) -> Self {
        F2BReadAll::Procedure(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::procedures::Procedure>> for B2FReadAll {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::procedures::Procedure>) -> Self {
        B2FReadAll::Procedure(value)
    }
}
impl TryFrom<B2FReadAll> for Vec<crate::codegen::structs_codegen::tables::procedures::Procedure> {
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::Procedure(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<ReadAll<crate::codegen::structs_codegen::tables::processables::Processable>>
    for F2BReadAll
{
    fn from(
        value: ReadAll<crate::codegen::structs_codegen::tables::processables::Processable>,
    ) -> Self {
        F2BReadAll::Processable(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::processables::Processable>> for B2FReadAll {
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::processables::Processable>,
    ) -> Self {
        B2FReadAll::Processable(value)
    }
}
impl TryFrom<B2FReadAll>
    for Vec<crate::codegen::structs_codegen::tables::processables::Processable>
{
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::Processable(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<ReadAll<crate::codegen::structs_codegen::tables::processing_steps::ProcessingStep>>
    for F2BReadAll
{
    fn from(
        value: ReadAll<crate::codegen::structs_codegen::tables::processing_steps::ProcessingStep>,
    ) -> Self {
        F2BReadAll::ProcessingStep(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::processing_steps::ProcessingStep>>
    for B2FReadAll
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::processing_steps::ProcessingStep>,
    ) -> Self {
        B2FReadAll::ProcessingStep(value)
    }
}
impl TryFrom<B2FReadAll>
    for Vec<crate::codegen::structs_codegen::tables::processing_steps::ProcessingStep>
{
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::ProcessingStep(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<ReadAll<crate::codegen::structs_codegen::tables::project_states::ProjectState>>
    for F2BReadAll
{
    fn from(
        value: ReadAll<crate::codegen::structs_codegen::tables::project_states::ProjectState>,
    ) -> Self {
        F2BReadAll::ProjectState(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::project_states::ProjectState>>
    for B2FReadAll
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::project_states::ProjectState>,
    ) -> Self {
        B2FReadAll::ProjectState(value)
    }
}
impl TryFrom<B2FReadAll>
    for Vec<crate::codegen::structs_codegen::tables::project_states::ProjectState>
{
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::ProjectState(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl
    From<
        ReadAll<
            crate::codegen::structs_codegen::tables::project_workflow_models::ProjectWorkflowModel,
        >,
    > for F2BReadAll
{
    fn from(
        value: ReadAll<
            crate::codegen::structs_codegen::tables::project_workflow_models::ProjectWorkflowModel,
        >,
    ) -> Self {
        F2BReadAll::ProjectWorkflowModel(value)
    }
}
impl
    From<
        Vec<crate::codegen::structs_codegen::tables::project_workflow_models::ProjectWorkflowModel>,
    > for B2FReadAll
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::project_workflow_models::ProjectWorkflowModel,
        >,
    ) -> Self {
        B2FReadAll::ProjectWorkflowModel(value)
    }
}
impl TryFrom<B2FReadAll>
    for Vec<crate::codegen::structs_codegen::tables::project_workflow_models::ProjectWorkflowModel>
{
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::ProjectWorkflowModel(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<ReadAll<crate::codegen::structs_codegen::tables::projects::Project>> for F2BReadAll {
    fn from(value: ReadAll<crate::codegen::structs_codegen::tables::projects::Project>) -> Self {
        F2BReadAll::Project(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::projects::Project>> for B2FReadAll {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::projects::Project>) -> Self {
        B2FReadAll::Project(value)
    }
}
impl TryFrom<B2FReadAll> for Vec<crate::codegen::structs_codegen::tables::projects::Project> {
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::Project(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<ReadAll<crate::codegen::structs_codegen::tables::ranks::Rank>> for F2BReadAll {
    fn from(value: ReadAll<crate::codegen::structs_codegen::tables::ranks::Rank>) -> Self {
        F2BReadAll::Rank(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::ranks::Rank>> for B2FReadAll {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::ranks::Rank>) -> Self {
        B2FReadAll::Rank(value)
    }
}
impl TryFrom<B2FReadAll> for Vec<crate::codegen::structs_codegen::tables::ranks::Rank> {
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::Rank(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<ReadAll<crate::codegen::structs_codegen::tables::roles::Role>> for F2BReadAll {
    fn from(value: ReadAll<crate::codegen::structs_codegen::tables::roles::Role>) -> Self {
        F2BReadAll::Role(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::roles::Role>> for B2FReadAll {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::roles::Role>) -> Self {
        B2FReadAll::Role(value)
    }
}
impl TryFrom<B2FReadAll> for Vec<crate::codegen::structs_codegen::tables::roles::Role> {
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::Role(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<ReadAll<crate::codegen::structs_codegen::tables::rooms::Room>> for F2BReadAll {
    fn from(value: ReadAll<crate::codegen::structs_codegen::tables::rooms::Room>) -> Self {
        F2BReadAll::Room(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::rooms::Room>> for B2FReadAll {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::rooms::Room>) -> Self {
        B2FReadAll::Room(value)
    }
}
impl TryFrom<B2FReadAll> for Vec<crate::codegen::structs_codegen::tables::rooms::Room> {
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::Room(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<ReadAll<crate::codegen::structs_codegen::tables::sample_states::SampleState>>
    for F2BReadAll
{
    fn from(
        value: ReadAll<crate::codegen::structs_codegen::tables::sample_states::SampleState>,
    ) -> Self {
        F2BReadAll::SampleState(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::sample_states::SampleState>> for B2FReadAll {
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::sample_states::SampleState>,
    ) -> Self {
        B2FReadAll::SampleState(value)
    }
}
impl TryFrom<B2FReadAll>
    for Vec<crate::codegen::structs_codegen::tables::sample_states::SampleState>
{
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::SampleState(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<ReadAll<crate::codegen::structs_codegen::tables::sampling_step_models::SamplingStepModel>>
    for F2BReadAll
{
    fn from(
        value: ReadAll<
            crate::codegen::structs_codegen::tables::sampling_step_models::SamplingStepModel,
        >,
    ) -> Self {
        F2BReadAll::SamplingStepModel(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::sampling_step_models::SamplingStepModel>>
    for B2FReadAll
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::sampling_step_models::SamplingStepModel,
        >,
    ) -> Self {
        B2FReadAll::SamplingStepModel(value)
    }
}
impl TryFrom<B2FReadAll>
    for Vec<crate::codegen::structs_codegen::tables::sampling_step_models::SamplingStepModel>
{
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::SamplingStepModel(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<ReadAll<crate::codegen::structs_codegen::tables::sampling_steps::SamplingStep>>
    for F2BReadAll
{
    fn from(
        value: ReadAll<crate::codegen::structs_codegen::tables::sampling_steps::SamplingStep>,
    ) -> Self {
        F2BReadAll::SamplingStep(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::sampling_steps::SamplingStep>>
    for B2FReadAll
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::sampling_steps::SamplingStep>,
    ) -> Self {
        B2FReadAll::SamplingStep(value)
    }
}
impl TryFrom<B2FReadAll>
    for Vec<crate::codegen::structs_codegen::tables::sampling_steps::SamplingStep>
{
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::SamplingStep(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<ReadAll<crate::codegen::structs_codegen::tables::spatial_ref_sys::SpatialRefSy>>
    for F2BReadAll
{
    fn from(
        value: ReadAll<crate::codegen::structs_codegen::tables::spatial_ref_sys::SpatialRefSy>,
    ) -> Self {
        F2BReadAll::SpatialRefSy(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::spatial_ref_sys::SpatialRefSy>>
    for B2FReadAll
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::spatial_ref_sys::SpatialRefSy>,
    ) -> Self {
        B2FReadAll::SpatialRefSy(value)
    }
}
impl TryFrom<B2FReadAll>
    for Vec<crate::codegen::structs_codegen::tables::spatial_ref_sys::SpatialRefSy>
{
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::SpatialRefSy(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<ReadAll<crate::codegen::structs_codegen::tables::spectra::Spectrum>> for F2BReadAll {
    fn from(value: ReadAll<crate::codegen::structs_codegen::tables::spectra::Spectrum>) -> Self {
        F2BReadAll::Spectrum(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::spectra::Spectrum>> for B2FReadAll {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::spectra::Spectrum>) -> Self {
        B2FReadAll::Spectrum(value)
    }
}
impl TryFrom<B2FReadAll> for Vec<crate::codegen::structs_codegen::tables::spectra::Spectrum> {
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::Spectrum(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<ReadAll<crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection>>
    for F2BReadAll
{
    fn from(
        value: ReadAll<
            crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection,
        >,
    ) -> Self {
        F2BReadAll::SpectraCollection(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection>>
    for B2FReadAll
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection>,
    ) -> Self {
        B2FReadAll::SpectraCollection(value)
    }
}
impl TryFrom<B2FReadAll>
    for Vec<crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection>
{
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::SpectraCollection(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl
    From<
        ReadAll<crate::codegen::structs_codegen::tables::step_container_models::StepContainerModel>,
    > for F2BReadAll
{
    fn from(
        value: ReadAll<
            crate::codegen::structs_codegen::tables::step_container_models::StepContainerModel,
        >,
    ) -> Self {
        F2BReadAll::StepContainerModel(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::step_container_models::StepContainerModel>>
    for B2FReadAll
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::step_container_models::StepContainerModel,
        >,
    ) -> Self {
        B2FReadAll::StepContainerModel(value)
    }
}
impl TryFrom<B2FReadAll>
    for Vec<crate::codegen::structs_codegen::tables::step_container_models::StepContainerModel>
{
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::StepContainerModel(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<ReadAll<crate::codegen::structs_codegen::tables::step_instruments::StepInstrument>>
    for F2BReadAll
{
    fn from(
        value: ReadAll<crate::codegen::structs_codegen::tables::step_instruments::StepInstrument>,
    ) -> Self {
        F2BReadAll::StepInstrument(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::step_instruments::StepInstrument>>
    for B2FReadAll
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::step_instruments::StepInstrument>,
    ) -> Self {
        B2FReadAll::StepInstrument(value)
    }
}
impl TryFrom<B2FReadAll>
    for Vec<crate::codegen::structs_codegen::tables::step_instruments::StepInstrument>
{
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::StepInstrument(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl
    From<ReadAll<crate::codegen::structs_codegen::tables::step_model_categories::StepModelCategory>>
    for F2BReadAll
{
    fn from(
        value: ReadAll<
            crate::codegen::structs_codegen::tables::step_model_categories::StepModelCategory,
        >,
    ) -> Self {
        F2BReadAll::StepModelCategory(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::step_model_categories::StepModelCategory>>
    for B2FReadAll
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::step_model_categories::StepModelCategory,
        >,
    ) -> Self {
        B2FReadAll::StepModelCategory(value)
    }
}
impl TryFrom<B2FReadAll>
    for Vec<crate::codegen::structs_codegen::tables::step_model_categories::StepModelCategory>
{
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::StepModelCategory(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<
    ReadAll<
        crate::codegen::structs_codegen::tables::step_model_container_categories::StepModelContainerCategory,
    >,
> for F2BReadAll {
    fn from(
        value: ReadAll<
            crate::codegen::structs_codegen::tables::step_model_container_categories::StepModelContainerCategory,
        >,
    ) -> Self {
        F2BReadAll::StepModelContainerCategory(value)
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::step_model_container_categories::StepModelContainerCategory,
    >,
> for B2FReadAll {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::step_model_container_categories::StepModelContainerCategory,
        >,
    ) -> Self {
        B2FReadAll::StepModelContainerCategory(value)
    }
}
impl TryFrom<B2FReadAll>
for Vec<
    crate::codegen::structs_codegen::tables::step_model_container_categories::StepModelContainerCategory,
> {
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::StepModelContainerCategory(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<
    ReadAll<
        crate::codegen::structs_codegen::tables::step_model_instrument_categories::StepModelInstrumentCategory,
    >,
> for F2BReadAll {
    fn from(
        value: ReadAll<
            crate::codegen::structs_codegen::tables::step_model_instrument_categories::StepModelInstrumentCategory,
        >,
    ) -> Self {
        F2BReadAll::StepModelInstrumentCategory(value)
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::step_model_instrument_categories::StepModelInstrumentCategory,
    >,
> for B2FReadAll {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::step_model_instrument_categories::StepModelInstrumentCategory,
        >,
    ) -> Self {
        B2FReadAll::StepModelInstrumentCategory(value)
    }
}
impl TryFrom<B2FReadAll>
for Vec<
    crate::codegen::structs_codegen::tables::step_model_instrument_categories::StepModelInstrumentCategory,
> {
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::StepModelInstrumentCategory(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<
    ReadAll<
        crate::codegen::structs_codegen::tables::step_model_instrument_models::StepModelInstrumentModel,
    >,
> for F2BReadAll {
    fn from(
        value: ReadAll<
            crate::codegen::structs_codegen::tables::step_model_instrument_models::StepModelInstrumentModel,
        >,
    ) -> Self {
        F2BReadAll::StepModelInstrumentModel(value)
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::step_model_instrument_models::StepModelInstrumentModel,
    >,
> for B2FReadAll {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::step_model_instrument_models::StepModelInstrumentModel,
        >,
    ) -> Self {
        B2FReadAll::StepModelInstrumentModel(value)
    }
}
impl TryFrom<B2FReadAll>
for Vec<
    crate::codegen::structs_codegen::tables::step_model_instrument_models::StepModelInstrumentModel,
> {
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::StepModelInstrumentModel(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl
    From<
        ReadAll<
            crate::codegen::structs_codegen::tables::step_model_instruments::StepModelInstrument,
        >,
    > for F2BReadAll
{
    fn from(
        value: ReadAll<
            crate::codegen::structs_codegen::tables::step_model_instruments::StepModelInstrument,
        >,
    ) -> Self {
        F2BReadAll::StepModelInstrument(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::step_model_instruments::StepModelInstrument>>
    for B2FReadAll
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::step_model_instruments::StepModelInstrument,
        >,
    ) -> Self {
        B2FReadAll::StepModelInstrument(value)
    }
}
impl TryFrom<B2FReadAll>
    for Vec<crate::codegen::structs_codegen::tables::step_model_instruments::StepModelInstrument>
{
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::StepModelInstrument(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<
    ReadAll<
        crate::codegen::structs_codegen::tables::step_model_nameplate_categories::StepModelNameplateCategory,
    >,
> for F2BReadAll {
    fn from(
        value: ReadAll<
            crate::codegen::structs_codegen::tables::step_model_nameplate_categories::StepModelNameplateCategory,
        >,
    ) -> Self {
        F2BReadAll::StepModelNameplateCategory(value)
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::step_model_nameplate_categories::StepModelNameplateCategory,
    >,
> for B2FReadAll {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::step_model_nameplate_categories::StepModelNameplateCategory,
        >,
    ) -> Self {
        B2FReadAll::StepModelNameplateCategory(value)
    }
}
impl TryFrom<B2FReadAll>
for Vec<
    crate::codegen::structs_codegen::tables::step_model_nameplate_categories::StepModelNameplateCategory,
> {
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::StepModelNameplateCategory(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<
    ReadAll<
        crate::codegen::structs_codegen::tables::step_model_tool_categories::StepModelToolCategory,
    >,
> for F2BReadAll {
    fn from(
        value: ReadAll<
            crate::codegen::structs_codegen::tables::step_model_tool_categories::StepModelToolCategory,
        >,
    ) -> Self {
        F2BReadAll::StepModelToolCategory(value)
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::step_model_tool_categories::StepModelToolCategory,
    >,
> for B2FReadAll {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::step_model_tool_categories::StepModelToolCategory,
        >,
    ) -> Self {
        B2FReadAll::StepModelToolCategory(value)
    }
}
impl TryFrom<B2FReadAll>
    for Vec<
        crate::codegen::structs_codegen::tables::step_model_tool_categories::StepModelToolCategory,
    >
{
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::StepModelToolCategory(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<ReadAll<crate::codegen::structs_codegen::tables::step_models::StepModel>> for F2BReadAll {
    fn from(
        value: ReadAll<crate::codegen::structs_codegen::tables::step_models::StepModel>,
    ) -> Self {
        F2BReadAll::StepModel(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::step_models::StepModel>> for B2FReadAll {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::step_models::StepModel>) -> Self {
        B2FReadAll::StepModel(value)
    }
}
impl TryFrom<B2FReadAll> for Vec<crate::codegen::structs_codegen::tables::step_models::StepModel> {
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::StepModel(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl
    From<
        ReadAll<crate::codegen::structs_codegen::tables::step_nameplate_models::StepNameplateModel>,
    > for F2BReadAll
{
    fn from(
        value: ReadAll<
            crate::codegen::structs_codegen::tables::step_nameplate_models::StepNameplateModel,
        >,
    ) -> Self {
        F2BReadAll::StepNameplateModel(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::step_nameplate_models::StepNameplateModel>>
    for B2FReadAll
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::step_nameplate_models::StepNameplateModel,
        >,
    ) -> Self {
        B2FReadAll::StepNameplateModel(value)
    }
}
impl TryFrom<B2FReadAll>
    for Vec<crate::codegen::structs_codegen::tables::step_nameplate_models::StepNameplateModel>
{
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::StepNameplateModel(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl
    From<
        ReadAll<
            crate::codegen::structs_codegen::tables::step_storage_containers::StepStorageContainer,
        >,
    > for F2BReadAll
{
    fn from(
        value: ReadAll<
            crate::codegen::structs_codegen::tables::step_storage_containers::StepStorageContainer,
        >,
    ) -> Self {
        F2BReadAll::StepStorageContainer(value)
    }
}
impl
    From<
        Vec<crate::codegen::structs_codegen::tables::step_storage_containers::StepStorageContainer>,
    > for B2FReadAll
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::step_storage_containers::StepStorageContainer,
        >,
    ) -> Self {
        B2FReadAll::StepStorageContainer(value)
    }
}
impl TryFrom<B2FReadAll>
    for Vec<crate::codegen::structs_codegen::tables::step_storage_containers::StepStorageContainer>
{
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::StepStorageContainer(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<ReadAll<crate::codegen::structs_codegen::tables::step_tool_models::StepToolModel>>
    for F2BReadAll
{
    fn from(
        value: ReadAll<crate::codegen::structs_codegen::tables::step_tool_models::StepToolModel>,
    ) -> Self {
        F2BReadAll::StepToolModel(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::step_tool_models::StepToolModel>>
    for B2FReadAll
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::step_tool_models::StepToolModel>,
    ) -> Self {
        B2FReadAll::StepToolModel(value)
    }
}
impl TryFrom<B2FReadAll>
    for Vec<crate::codegen::structs_codegen::tables::step_tool_models::StepToolModel>
{
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::StepToolModel(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<ReadAll<crate::codegen::structs_codegen::tables::steps::Step>> for F2BReadAll {
    fn from(value: ReadAll<crate::codegen::structs_codegen::tables::steps::Step>) -> Self {
        F2BReadAll::Step(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::steps::Step>> for B2FReadAll {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::steps::Step>) -> Self {
        B2FReadAll::Step(value)
    }
}
impl TryFrom<B2FReadAll> for Vec<crate::codegen::structs_codegen::tables::steps::Step> {
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::Step(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<ReadAll<crate::codegen::structs_codegen::tables::storage_containers::StorageContainer>>
    for F2BReadAll
{
    fn from(
        value: ReadAll<
            crate::codegen::structs_codegen::tables::storage_containers::StorageContainer,
        >,
    ) -> Self {
        F2BReadAll::StorageContainer(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::storage_containers::StorageContainer>>
    for B2FReadAll
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::storage_containers::StorageContainer>,
    ) -> Self {
        B2FReadAll::StorageContainer(value)
    }
}
impl TryFrom<B2FReadAll>
    for Vec<crate::codegen::structs_codegen::tables::storage_containers::StorageContainer>
{
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::StorageContainer(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<ReadAll<crate::codegen::structs_codegen::tables::taxa::Taxon>> for F2BReadAll {
    fn from(value: ReadAll<crate::codegen::structs_codegen::tables::taxa::Taxon>) -> Self {
        F2BReadAll::Taxon(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::taxa::Taxon>> for B2FReadAll {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::taxa::Taxon>) -> Self {
        B2FReadAll::Taxon(value)
    }
}
impl TryFrom<B2FReadAll> for Vec<crate::codegen::structs_codegen::tables::taxa::Taxon> {
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::Taxon(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<ReadAll<crate::codegen::structs_codegen::tables::team_members::TeamMember>>
    for F2BReadAll
{
    fn from(
        value: ReadAll<crate::codegen::structs_codegen::tables::team_members::TeamMember>,
    ) -> Self {
        F2BReadAll::TeamMember(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::team_members::TeamMember>> for B2FReadAll {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::team_members::TeamMember>) -> Self {
        B2FReadAll::TeamMember(value)
    }
}
impl TryFrom<B2FReadAll>
    for Vec<crate::codegen::structs_codegen::tables::team_members::TeamMember>
{
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::TeamMember(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<ReadAll<crate::codegen::structs_codegen::tables::team_projects::TeamProject>>
    for F2BReadAll
{
    fn from(
        value: ReadAll<crate::codegen::structs_codegen::tables::team_projects::TeamProject>,
    ) -> Self {
        F2BReadAll::TeamProject(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::team_projects::TeamProject>> for B2FReadAll {
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::team_projects::TeamProject>,
    ) -> Self {
        B2FReadAll::TeamProject(value)
    }
}
impl TryFrom<B2FReadAll>
    for Vec<crate::codegen::structs_codegen::tables::team_projects::TeamProject>
{
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::TeamProject(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<ReadAll<crate::codegen::structs_codegen::tables::team_states::TeamState>> for F2BReadAll {
    fn from(
        value: ReadAll<crate::codegen::structs_codegen::tables::team_states::TeamState>,
    ) -> Self {
        F2BReadAll::TeamState(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::team_states::TeamState>> for B2FReadAll {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::team_states::TeamState>) -> Self {
        B2FReadAll::TeamState(value)
    }
}
impl TryFrom<B2FReadAll> for Vec<crate::codegen::structs_codegen::tables::team_states::TeamState> {
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::TeamState(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<ReadAll<crate::codegen::structs_codegen::tables::teams::Team>> for F2BReadAll {
    fn from(value: ReadAll<crate::codegen::structs_codegen::tables::teams::Team>) -> Self {
        F2BReadAll::Team(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::teams::Team>> for B2FReadAll {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::teams::Team>) -> Self {
        B2FReadAll::Team(value)
    }
}
impl TryFrom<B2FReadAll> for Vec<crate::codegen::structs_codegen::tables::teams::Team> {
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::Team(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<ReadAll<crate::codegen::structs_codegen::tables::tool_categories::ToolCategory>>
    for F2BReadAll
{
    fn from(
        value: ReadAll<crate::codegen::structs_codegen::tables::tool_categories::ToolCategory>,
    ) -> Self {
        F2BReadAll::ToolCategory(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::tool_categories::ToolCategory>>
    for B2FReadAll
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::tool_categories::ToolCategory>,
    ) -> Self {
        B2FReadAll::ToolCategory(value)
    }
}
impl TryFrom<B2FReadAll>
    for Vec<crate::codegen::structs_codegen::tables::tool_categories::ToolCategory>
{
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::ToolCategory(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<ReadAll<crate::codegen::structs_codegen::tables::tool_models::ToolModel>> for F2BReadAll {
    fn from(
        value: ReadAll<crate::codegen::structs_codegen::tables::tool_models::ToolModel>,
    ) -> Self {
        F2BReadAll::ToolModel(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::tool_models::ToolModel>> for B2FReadAll {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::tool_models::ToolModel>) -> Self {
        B2FReadAll::ToolModel(value)
    }
}
impl TryFrom<B2FReadAll> for Vec<crate::codegen::structs_codegen::tables::tool_models::ToolModel> {
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::ToolModel(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<ReadAll<crate::codegen::structs_codegen::tables::trackable_locations::TrackableLocation>>
    for F2BReadAll
{
    fn from(
        value: ReadAll<
            crate::codegen::structs_codegen::tables::trackable_locations::TrackableLocation,
        >,
    ) -> Self {
        F2BReadAll::TrackableLocation(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::trackable_locations::TrackableLocation>>
    for B2FReadAll
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::trackable_locations::TrackableLocation>,
    ) -> Self {
        B2FReadAll::TrackableLocation(value)
    }
}
impl TryFrom<B2FReadAll>
    for Vec<crate::codegen::structs_codegen::tables::trackable_locations::TrackableLocation>
{
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::TrackableLocation(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<ReadAll<crate::codegen::structs_codegen::tables::trackable_states::TrackableState>>
    for F2BReadAll
{
    fn from(
        value: ReadAll<crate::codegen::structs_codegen::tables::trackable_states::TrackableState>,
    ) -> Self {
        F2BReadAll::TrackableState(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::trackable_states::TrackableState>>
    for B2FReadAll
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::trackable_states::TrackableState>,
    ) -> Self {
        B2FReadAll::TrackableState(value)
    }
}
impl TryFrom<B2FReadAll>
    for Vec<crate::codegen::structs_codegen::tables::trackable_states::TrackableState>
{
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::TrackableState(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<ReadAll<crate::codegen::structs_codegen::tables::trackables::Trackable>> for F2BReadAll {
    fn from(
        value: ReadAll<crate::codegen::structs_codegen::tables::trackables::Trackable>,
    ) -> Self {
        F2BReadAll::Trackable(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::trackables::Trackable>> for B2FReadAll {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::trackables::Trackable>) -> Self {
        B2FReadAll::Trackable(value)
    }
}
impl TryFrom<B2FReadAll> for Vec<crate::codegen::structs_codegen::tables::trackables::Trackable> {
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::Trackable(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<ReadAll<crate::codegen::structs_codegen::tables::units::Unit>> for F2BReadAll {
    fn from(value: ReadAll<crate::codegen::structs_codegen::tables::units::Unit>) -> Self {
        F2BReadAll::Unit(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::units::Unit>> for B2FReadAll {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::units::Unit>) -> Self {
        B2FReadAll::Unit(value)
    }
}
impl TryFrom<B2FReadAll> for Vec<crate::codegen::structs_codegen::tables::units::Unit> {
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::Unit(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<ReadAll<crate::codegen::structs_codegen::tables::user_emails::UserEmail>> for F2BReadAll {
    fn from(
        value: ReadAll<crate::codegen::structs_codegen::tables::user_emails::UserEmail>,
    ) -> Self {
        F2BReadAll::UserEmail(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::user_emails::UserEmail>> for B2FReadAll {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::user_emails::UserEmail>) -> Self {
        B2FReadAll::UserEmail(value)
    }
}
impl TryFrom<B2FReadAll> for Vec<crate::codegen::structs_codegen::tables::user_emails::UserEmail> {
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::UserEmail(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<ReadAll<crate::codegen::structs_codegen::tables::user_organizations::UserOrganization>>
    for F2BReadAll
{
    fn from(
        value: ReadAll<
            crate::codegen::structs_codegen::tables::user_organizations::UserOrganization,
        >,
    ) -> Self {
        F2BReadAll::UserOrganization(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::user_organizations::UserOrganization>>
    for B2FReadAll
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::user_organizations::UserOrganization>,
    ) -> Self {
        B2FReadAll::UserOrganization(value)
    }
}
impl TryFrom<B2FReadAll>
    for Vec<crate::codegen::structs_codegen::tables::user_organizations::UserOrganization>
{
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::UserOrganization(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<ReadAll<crate::codegen::structs_codegen::tables::users::User>> for F2BReadAll {
    fn from(value: ReadAll<crate::codegen::structs_codegen::tables::users::User>) -> Self {
        F2BReadAll::User(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::users::User>> for B2FReadAll {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::users::User>) -> Self {
        B2FReadAll::User(value)
    }
}
impl TryFrom<B2FReadAll> for Vec<crate::codegen::structs_codegen::tables::users::User> {
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::User(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<
    ReadAll<
        crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel,
    >,
> for F2BReadAll {
    fn from(
        value: ReadAll<
            crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel,
        >,
    ) -> Self {
        F2BReadAll::WeighingInstrumentModel(value)
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel,
    >,
> for B2FReadAll {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel,
        >,
    ) -> Self {
        B2FReadAll::WeighingInstrumentModel(value)
    }
}
impl TryFrom<B2FReadAll>
for Vec<
    crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel,
> {
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::WeighingInstrumentModel(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<ReadAll<crate::codegen::structs_codegen::tables::weighing_step_models::WeighingStepModel>>
    for F2BReadAll
{
    fn from(
        value: ReadAll<
            crate::codegen::structs_codegen::tables::weighing_step_models::WeighingStepModel,
        >,
    ) -> Self {
        F2BReadAll::WeighingStepModel(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::weighing_step_models::WeighingStepModel>>
    for B2FReadAll
{
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::weighing_step_models::WeighingStepModel,
        >,
    ) -> Self {
        B2FReadAll::WeighingStepModel(value)
    }
}
impl TryFrom<B2FReadAll>
    for Vec<crate::codegen::structs_codegen::tables::weighing_step_models::WeighingStepModel>
{
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::WeighingStepModel(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
impl From<ReadAll<crate::codegen::structs_codegen::tables::weighing_steps::WeighingStep>>
    for F2BReadAll
{
    fn from(
        value: ReadAll<crate::codegen::structs_codegen::tables::weighing_steps::WeighingStep>,
    ) -> Self {
        F2BReadAll::WeighingStep(value)
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::weighing_steps::WeighingStep>>
    for B2FReadAll
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::weighing_steps::WeighingStep>,
    ) -> Self {
        B2FReadAll::WeighingStep(value)
    }
}
impl TryFrom<B2FReadAll>
    for Vec<crate::codegen::structs_codegen::tables::weighing_steps::WeighingStep>
{
    type Error = ReadError;
    fn try_from(value: B2FReadAll) -> Result<Self, Self::Error> {
        match value {
            B2FReadAll::WeighingStep(v) => Ok(v),
            _ => Err(ReadError::ConversionError),
        }
    }
}
