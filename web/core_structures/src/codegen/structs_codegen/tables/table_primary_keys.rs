#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TablePrimaryKey {
    Address(i32),
    AliquotingInstrumentModel(i32),
    AliquotingStepModel(i32),
    AliquotingStep(::rosetta_uuid::Uuid),
    BallMillStepModel(i32),
    BallMillStep(::rosetta_uuid::Uuid),
    Brand(i32),
    CentrifugeStepModel(i32),
    CentrifugeStep(::rosetta_uuid::Uuid),
    ChemicalEntity(i32),
    City(i32),
    Color(i16),
    CommercialProductLot(i32),
    CommercialProduct(i32),
    CommercialReagentModel(i32),
    CommercialReagent(::rosetta_uuid::Uuid),
    ContainerModel(i32),
    Country(::iso_codes::CountryCode),
    DisposalStepModel(i32),
    DisposalStep(::rosetta_uuid::Uuid),
    DocumentFormat(i16),
    Document(::rosetta_uuid::Uuid),
    EmailProvider((i32, i16)),
    FractioningStepModel(i32),
    FractioningStep(::rosetta_uuid::Uuid),
    FreezeDryingStepModel(i32),
    InstrumentLocation(i32),
    InstrumentModelCategory(i32),
    InstrumentModel(i32),
    InstrumentState(i16),
    Instrument(i32),
    LoginProvider(i16),
    Material(i16),
    NameplateModel(i32),
    ObservationSubject(i16),
    OrganismObservation(::rosetta_uuid::Uuid),
    OrganismSamplingStepModel(i32),
    OrganismTaxon((::rosetta_uuid::Uuid, i32)),
    Organism(::rosetta_uuid::Uuid),
    Organization(i16),
    PackagingModel(i32),
    PackagingStepModel(i32),
    PermanenceCategory(i16),
    ProcedureModelContainerCategory(i32),
    ProcedureModelInstrumentCategory(i32),
    ProcedureModelNameplateCategory(i32),
    ProcedureModel(i32),
    Procedure(i32),
    Processable(::rosetta_uuid::Uuid),
    ProcessingStep(::rosetta_uuid::Uuid),
    ProjectState(i16),
    ProjectWorkflowModel(i32),
    Project(i32),
    Rank(i16),
    Reagent(i32),
    Role(i16),
    Room(i32),
    SampleState(i16),
    SamplingStepModel(i32),
    SamplingStep(::rosetta_uuid::Uuid),
    ShakingStepModel(i32),
    ShakingStep(::rosetta_uuid::Uuid),
    SpatialRefSy(i32),
    Spectrum(i32),
    SpectraCollection(i32),
    StepContainerModel(::rosetta_uuid::Uuid),
    StepInstrument(::rosetta_uuid::Uuid),
    StepModelContainerCategory(i32),
    StepModelInstrumentCategory(i32),
    StepModelInstrumentModel(i32),
    StepModelInstrument(i32),
    StepModelNameplateCategory(i32),
    StepModelReagent(i32),
    StepModelToolCategory(i32),
    StepModel(i32),
    StepNameplateModel(::rosetta_uuid::Uuid),
    StepStorageContainer(::rosetta_uuid::Uuid),
    StepToolModel(::rosetta_uuid::Uuid),
    Step(::rosetta_uuid::Uuid),
    StorageContainer(::rosetta_uuid::Uuid),
    Taxon(i32),
    TeamMember((i32, i32)),
    TeamProject((i32, i32)),
    TeamState(i16),
    Team(i32),
    TemporaryUser(i32),
    ToolModel(i32),
    TrackableLocation(::rosetta_uuid::Uuid),
    TrackableState(i16),
    Trackable(::rosetta_uuid::Uuid),
    Unit(i16),
    UserEmail(i32),
    UserOrganization((i32, i16)),
    User(i32),
    VolumetricProcessable(::rosetta_uuid::Uuid),
    WeighingInstrumentModel(i32),
    WeighingStepModel(i32),
    WeighingStep(::rosetta_uuid::Uuid),
}
impl web_common_traits::prelude::Tabular for TablePrimaryKey {
    type TableName = crate::codegen::tables::table_names::TableName;
    fn table_name(&self) -> Self::TableName {
        match self {
            TablePrimaryKey::Address(_) => crate::codegen::tables::table_names::TableName::Address,
            TablePrimaryKey::AliquotingInstrumentModel(_) => {
                crate::codegen::tables::table_names::TableName::AliquotingInstrumentModel
            }
            TablePrimaryKey::AliquotingStepModel(_) => {
                crate::codegen::tables::table_names::TableName::AliquotingStepModel
            }
            TablePrimaryKey::AliquotingStep(_) => {
                crate::codegen::tables::table_names::TableName::AliquotingStep
            }
            TablePrimaryKey::BallMillStepModel(_) => {
                crate::codegen::tables::table_names::TableName::BallMillStepModel
            }
            TablePrimaryKey::BallMillStep(_) => {
                crate::codegen::tables::table_names::TableName::BallMillStep
            }
            TablePrimaryKey::Brand(_) => crate::codegen::tables::table_names::TableName::Brand,
            TablePrimaryKey::CentrifugeStepModel(_) => {
                crate::codegen::tables::table_names::TableName::CentrifugeStepModel
            }
            TablePrimaryKey::CentrifugeStep(_) => {
                crate::codegen::tables::table_names::TableName::CentrifugeStep
            }
            TablePrimaryKey::ChemicalEntity(_) => {
                crate::codegen::tables::table_names::TableName::ChemicalEntity
            }
            TablePrimaryKey::City(_) => crate::codegen::tables::table_names::TableName::City,
            TablePrimaryKey::Color(_) => crate::codegen::tables::table_names::TableName::Color,
            TablePrimaryKey::CommercialProductLot(_) => {
                crate::codegen::tables::table_names::TableName::CommercialProductLot
            }
            TablePrimaryKey::CommercialProduct(_) => {
                crate::codegen::tables::table_names::TableName::CommercialProduct
            }
            TablePrimaryKey::CommercialReagentModel(_) => {
                crate::codegen::tables::table_names::TableName::CommercialReagentModel
            }
            TablePrimaryKey::CommercialReagent(_) => {
                crate::codegen::tables::table_names::TableName::CommercialReagent
            }
            TablePrimaryKey::ContainerModel(_) => {
                crate::codegen::tables::table_names::TableName::ContainerModel
            }
            TablePrimaryKey::Country(_) => crate::codegen::tables::table_names::TableName::Country,
            TablePrimaryKey::DisposalStepModel(_) => {
                crate::codegen::tables::table_names::TableName::DisposalStepModel
            }
            TablePrimaryKey::DisposalStep(_) => {
                crate::codegen::tables::table_names::TableName::DisposalStep
            }
            TablePrimaryKey::DocumentFormat(_) => {
                crate::codegen::tables::table_names::TableName::DocumentFormat
            }
            TablePrimaryKey::Document(_) => {
                crate::codegen::tables::table_names::TableName::Document
            }
            TablePrimaryKey::EmailProvider(_) => {
                crate::codegen::tables::table_names::TableName::EmailProvider
            }
            TablePrimaryKey::FractioningStepModel(_) => {
                crate::codegen::tables::table_names::TableName::FractioningStepModel
            }
            TablePrimaryKey::FractioningStep(_) => {
                crate::codegen::tables::table_names::TableName::FractioningStep
            }
            TablePrimaryKey::FreezeDryingStepModel(_) => {
                crate::codegen::tables::table_names::TableName::FreezeDryingStepModel
            }
            TablePrimaryKey::InstrumentLocation(_) => {
                crate::codegen::tables::table_names::TableName::InstrumentLocation
            }
            TablePrimaryKey::InstrumentModelCategory(_) => {
                crate::codegen::tables::table_names::TableName::InstrumentModelCategory
            }
            TablePrimaryKey::InstrumentModel(_) => {
                crate::codegen::tables::table_names::TableName::InstrumentModel
            }
            TablePrimaryKey::InstrumentState(_) => {
                crate::codegen::tables::table_names::TableName::InstrumentState
            }
            TablePrimaryKey::Instrument(_) => {
                crate::codegen::tables::table_names::TableName::Instrument
            }
            TablePrimaryKey::LoginProvider(_) => {
                crate::codegen::tables::table_names::TableName::LoginProvider
            }
            TablePrimaryKey::Material(_) => {
                crate::codegen::tables::table_names::TableName::Material
            }
            TablePrimaryKey::NameplateModel(_) => {
                crate::codegen::tables::table_names::TableName::NameplateModel
            }
            TablePrimaryKey::ObservationSubject(_) => {
                crate::codegen::tables::table_names::TableName::ObservationSubject
            }
            TablePrimaryKey::OrganismObservation(_) => {
                crate::codegen::tables::table_names::TableName::OrganismObservation
            }
            TablePrimaryKey::OrganismSamplingStepModel(_) => {
                crate::codegen::tables::table_names::TableName::OrganismSamplingStepModel
            }
            TablePrimaryKey::OrganismTaxon(_) => {
                crate::codegen::tables::table_names::TableName::OrganismTaxon
            }
            TablePrimaryKey::Organism(_) => {
                crate::codegen::tables::table_names::TableName::Organism
            }
            TablePrimaryKey::Organization(_) => {
                crate::codegen::tables::table_names::TableName::Organization
            }
            TablePrimaryKey::PackagingModel(_) => {
                crate::codegen::tables::table_names::TableName::PackagingModel
            }
            TablePrimaryKey::PackagingStepModel(_) => {
                crate::codegen::tables::table_names::TableName::PackagingStepModel
            }
            TablePrimaryKey::PermanenceCategory(_) => {
                crate::codegen::tables::table_names::TableName::PermanenceCategory
            }
            TablePrimaryKey::ProcedureModelContainerCategory(_) => {
                crate::codegen::tables::table_names::TableName::ProcedureModelContainerCategory
            }
            TablePrimaryKey::ProcedureModelInstrumentCategory(_) => {
                crate::codegen::tables::table_names::TableName::ProcedureModelInstrumentCategory
            }
            TablePrimaryKey::ProcedureModelNameplateCategory(_) => {
                crate::codegen::tables::table_names::TableName::ProcedureModelNameplateCategory
            }
            TablePrimaryKey::ProcedureModel(_) => {
                crate::codegen::tables::table_names::TableName::ProcedureModel
            }
            TablePrimaryKey::Procedure(_) => {
                crate::codegen::tables::table_names::TableName::Procedure
            }
            TablePrimaryKey::Processable(_) => {
                crate::codegen::tables::table_names::TableName::Processable
            }
            TablePrimaryKey::ProcessingStep(_) => {
                crate::codegen::tables::table_names::TableName::ProcessingStep
            }
            TablePrimaryKey::ProjectState(_) => {
                crate::codegen::tables::table_names::TableName::ProjectState
            }
            TablePrimaryKey::ProjectWorkflowModel(_) => {
                crate::codegen::tables::table_names::TableName::ProjectWorkflowModel
            }
            TablePrimaryKey::Project(_) => crate::codegen::tables::table_names::TableName::Project,
            TablePrimaryKey::Rank(_) => crate::codegen::tables::table_names::TableName::Rank,
            TablePrimaryKey::Reagent(_) => crate::codegen::tables::table_names::TableName::Reagent,
            TablePrimaryKey::Role(_) => crate::codegen::tables::table_names::TableName::Role,
            TablePrimaryKey::Room(_) => crate::codegen::tables::table_names::TableName::Room,
            TablePrimaryKey::SampleState(_) => {
                crate::codegen::tables::table_names::TableName::SampleState
            }
            TablePrimaryKey::SamplingStepModel(_) => {
                crate::codegen::tables::table_names::TableName::SamplingStepModel
            }
            TablePrimaryKey::SamplingStep(_) => {
                crate::codegen::tables::table_names::TableName::SamplingStep
            }
            TablePrimaryKey::ShakingStepModel(_) => {
                crate::codegen::tables::table_names::TableName::ShakingStepModel
            }
            TablePrimaryKey::ShakingStep(_) => {
                crate::codegen::tables::table_names::TableName::ShakingStep
            }
            TablePrimaryKey::SpatialRefSy(_) => {
                crate::codegen::tables::table_names::TableName::SpatialRefSy
            }
            TablePrimaryKey::Spectrum(_) => {
                crate::codegen::tables::table_names::TableName::Spectrum
            }
            TablePrimaryKey::SpectraCollection(_) => {
                crate::codegen::tables::table_names::TableName::SpectraCollection
            }
            TablePrimaryKey::StepContainerModel(_) => {
                crate::codegen::tables::table_names::TableName::StepContainerModel
            }
            TablePrimaryKey::StepInstrument(_) => {
                crate::codegen::tables::table_names::TableName::StepInstrument
            }
            TablePrimaryKey::StepModelContainerCategory(_) => {
                crate::codegen::tables::table_names::TableName::StepModelContainerCategory
            }
            TablePrimaryKey::StepModelInstrumentCategory(_) => {
                crate::codegen::tables::table_names::TableName::StepModelInstrumentCategory
            }
            TablePrimaryKey::StepModelInstrumentModel(_) => {
                crate::codegen::tables::table_names::TableName::StepModelInstrumentModel
            }
            TablePrimaryKey::StepModelInstrument(_) => {
                crate::codegen::tables::table_names::TableName::StepModelInstrument
            }
            TablePrimaryKey::StepModelNameplateCategory(_) => {
                crate::codegen::tables::table_names::TableName::StepModelNameplateCategory
            }
            TablePrimaryKey::StepModelReagent(_) => {
                crate::codegen::tables::table_names::TableName::StepModelReagent
            }
            TablePrimaryKey::StepModelToolCategory(_) => {
                crate::codegen::tables::table_names::TableName::StepModelToolCategory
            }
            TablePrimaryKey::StepModel(_) => {
                crate::codegen::tables::table_names::TableName::StepModel
            }
            TablePrimaryKey::StepNameplateModel(_) => {
                crate::codegen::tables::table_names::TableName::StepNameplateModel
            }
            TablePrimaryKey::StepStorageContainer(_) => {
                crate::codegen::tables::table_names::TableName::StepStorageContainer
            }
            TablePrimaryKey::StepToolModel(_) => {
                crate::codegen::tables::table_names::TableName::StepToolModel
            }
            TablePrimaryKey::Step(_) => crate::codegen::tables::table_names::TableName::Step,
            TablePrimaryKey::StorageContainer(_) => {
                crate::codegen::tables::table_names::TableName::StorageContainer
            }
            TablePrimaryKey::Taxon(_) => crate::codegen::tables::table_names::TableName::Taxon,
            TablePrimaryKey::TeamMember(_) => {
                crate::codegen::tables::table_names::TableName::TeamMember
            }
            TablePrimaryKey::TeamProject(_) => {
                crate::codegen::tables::table_names::TableName::TeamProject
            }
            TablePrimaryKey::TeamState(_) => {
                crate::codegen::tables::table_names::TableName::TeamState
            }
            TablePrimaryKey::Team(_) => crate::codegen::tables::table_names::TableName::Team,
            TablePrimaryKey::TemporaryUser(_) => {
                crate::codegen::tables::table_names::TableName::TemporaryUser
            }
            TablePrimaryKey::ToolModel(_) => {
                crate::codegen::tables::table_names::TableName::ToolModel
            }
            TablePrimaryKey::TrackableLocation(_) => {
                crate::codegen::tables::table_names::TableName::TrackableLocation
            }
            TablePrimaryKey::TrackableState(_) => {
                crate::codegen::tables::table_names::TableName::TrackableState
            }
            TablePrimaryKey::Trackable(_) => {
                crate::codegen::tables::table_names::TableName::Trackable
            }
            TablePrimaryKey::Unit(_) => crate::codegen::tables::table_names::TableName::Unit,
            TablePrimaryKey::UserEmail(_) => {
                crate::codegen::tables::table_names::TableName::UserEmail
            }
            TablePrimaryKey::UserOrganization(_) => {
                crate::codegen::tables::table_names::TableName::UserOrganization
            }
            TablePrimaryKey::User(_) => crate::codegen::tables::table_names::TableName::User,
            TablePrimaryKey::VolumetricProcessable(_) => {
                crate::codegen::tables::table_names::TableName::VolumetricProcessable
            }
            TablePrimaryKey::WeighingInstrumentModel(_) => {
                crate::codegen::tables::table_names::TableName::WeighingInstrumentModel
            }
            TablePrimaryKey::WeighingStepModel(_) => {
                crate::codegen::tables::table_names::TableName::WeighingStepModel
            }
            TablePrimaryKey::WeighingStep(_) => {
                crate::codegen::tables::table_names::TableName::WeighingStep
            }
        }
    }
}
