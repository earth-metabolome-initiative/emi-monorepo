#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TableName {
    Address,
    AliquotingInstrumentModel,
    AliquotingStepModel,
    AliquotingStep,
    BallMillStepModel,
    BallMillStep,
    Brand,
    CentrifugeStepModel,
    CentrifugeStep,
    ChemicalEntity,
    City,
    Color,
    CommercialProductLot,
    CommercialProduct,
    CommercialReagent,
    ContainerModel,
    Country,
    DisposalStepModel,
    DisposalStep,
    DocumentFormat,
    Document,
    EmailProvider,
    FractioningStepModel,
    FractioningStep,
    FreezeDryingStepModel,
    InstrumentLocation,
    InstrumentModelCategory,
    InstrumentModel,
    InstrumentState,
    Instrument,
    LoginProvider,
    Material,
    NameplateModel,
    ObservationSubject,
    OrganismObservation,
    OrganismSamplingStepModel,
    OrganismTaxon,
    Organism,
    Organization,
    PackagingModel,
    PackagingStepModel,
    ParentProcedureModel,
    PermanenceCategory,
    ProcedureModelContainerCategory,
    ProcedureModelInstrumentCategory,
    ProcedureModelNameplateCategory,
    ProcedureModelToolCategory,
    ProcedureModel,
    Procedure,
    Processable,
    ProcessingStep,
    ProjectState,
    ProjectWorkflowModel,
    Project,
    Rank,
    Reagent,
    Role,
    Room,
    SampleState,
    SamplingStepModel,
    SamplingStep,
    ShakingStepModel,
    ShakingStep,
    SpatialRefSy,
    Spectrum,
    SpectraCollection,
    StepContainerModel,
    StepInstrument,
    StepModelContainerCategory,
    StepModelInstrumentCategory,
    StepModelInstrumentModel,
    StepModelInstrument,
    StepModelNameplateCategory,
    StepModelToolCategory,
    StepModelTrackableCategory,
    StepModel,
    StepNameplateModel,
    StepStorageContainer,
    StepToolModel,
    Step,
    StorageContainer,
    Taxon,
    TeamMember,
    TeamProject,
    TeamState,
    Team,
    TemporaryUser,
    ToolModel,
    TrackableCategory,
    TrackableLocation,
    TrackableState,
    Trackable,
    Unit,
    UserEmail,
    UserOrganization,
    User,
    VolumetricProcessable,
    WeighingInstrumentModel,
    WeighingStepModel,
    WeighingStep,
}
impl core::fmt::Display for TableName {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            TableName::Address => write!(f, stringify!(Address)),
            TableName::AliquotingInstrumentModel => {
                write!(f, stringify!(AliquotingInstrumentModel))
            }
            TableName::AliquotingStepModel => write!(f, stringify!(AliquotingStepModel)),
            TableName::AliquotingStep => write!(f, stringify!(AliquotingStep)),
            TableName::BallMillStepModel => write!(f, stringify!(BallMillStepModel)),
            TableName::BallMillStep => write!(f, stringify!(BallMillStep)),
            TableName::Brand => write!(f, stringify!(Brand)),
            TableName::CentrifugeStepModel => write!(f, stringify!(CentrifugeStepModel)),
            TableName::CentrifugeStep => write!(f, stringify!(CentrifugeStep)),
            TableName::ChemicalEntity => write!(f, stringify!(ChemicalEntity)),
            TableName::City => write!(f, stringify!(City)),
            TableName::Color => write!(f, stringify!(Color)),
            TableName::CommercialProductLot => {
                write!(f, stringify!(CommercialProductLot))
            }
            TableName::CommercialProduct => write!(f, stringify!(CommercialProduct)),
            TableName::CommercialReagent => write!(f, stringify!(CommercialReagent)),
            TableName::ContainerModel => write!(f, stringify!(ContainerModel)),
            TableName::Country => write!(f, stringify!(Country)),
            TableName::DisposalStepModel => write!(f, stringify!(DisposalStepModel)),
            TableName::DisposalStep => write!(f, stringify!(DisposalStep)),
            TableName::DocumentFormat => write!(f, stringify!(DocumentFormat)),
            TableName::Document => write!(f, stringify!(Document)),
            TableName::EmailProvider => write!(f, stringify!(EmailProvider)),
            TableName::FractioningStepModel => {
                write!(f, stringify!(FractioningStepModel))
            }
            TableName::FractioningStep => write!(f, stringify!(FractioningStep)),
            TableName::FreezeDryingStepModel => {
                write!(f, stringify!(FreezeDryingStepModel))
            }
            TableName::InstrumentLocation => write!(f, stringify!(InstrumentLocation)),
            TableName::InstrumentModelCategory => {
                write!(f, stringify!(InstrumentModelCategory))
            }
            TableName::InstrumentModel => write!(f, stringify!(InstrumentModel)),
            TableName::InstrumentState => write!(f, stringify!(InstrumentState)),
            TableName::Instrument => write!(f, stringify!(Instrument)),
            TableName::LoginProvider => write!(f, stringify!(LoginProvider)),
            TableName::Material => write!(f, stringify!(Material)),
            TableName::NameplateModel => write!(f, stringify!(NameplateModel)),
            TableName::ObservationSubject => write!(f, stringify!(ObservationSubject)),
            TableName::OrganismObservation => write!(f, stringify!(OrganismObservation)),
            TableName::OrganismSamplingStepModel => {
                write!(f, stringify!(OrganismSamplingStepModel))
            }
            TableName::OrganismTaxon => write!(f, stringify!(OrganismTaxon)),
            TableName::Organism => write!(f, stringify!(Organism)),
            TableName::Organization => write!(f, stringify!(Organization)),
            TableName::PackagingModel => write!(f, stringify!(PackagingModel)),
            TableName::PackagingStepModel => write!(f, stringify!(PackagingStepModel)),
            TableName::ParentProcedureModel => {
                write!(f, stringify!(ParentProcedureModel))
            }
            TableName::PermanenceCategory => write!(f, stringify!(PermanenceCategory)),
            TableName::ProcedureModelContainerCategory => {
                write!(f, stringify!(ProcedureModelContainerCategory))
            }
            TableName::ProcedureModelInstrumentCategory => {
                write!(f, stringify!(ProcedureModelInstrumentCategory))
            }
            TableName::ProcedureModelNameplateCategory => {
                write!(f, stringify!(ProcedureModelNameplateCategory))
            }
            TableName::ProcedureModelToolCategory => {
                write!(f, stringify!(ProcedureModelToolCategory))
            }
            TableName::ProcedureModel => write!(f, stringify!(ProcedureModel)),
            TableName::Procedure => write!(f, stringify!(Procedure)),
            TableName::Processable => write!(f, stringify!(Processable)),
            TableName::ProcessingStep => write!(f, stringify!(ProcessingStep)),
            TableName::ProjectState => write!(f, stringify!(ProjectState)),
            TableName::ProjectWorkflowModel => {
                write!(f, stringify!(ProjectWorkflowModel))
            }
            TableName::Project => write!(f, stringify!(Project)),
            TableName::Rank => write!(f, stringify!(Rank)),
            TableName::Reagent => write!(f, stringify!(Reagent)),
            TableName::Role => write!(f, stringify!(Role)),
            TableName::Room => write!(f, stringify!(Room)),
            TableName::SampleState => write!(f, stringify!(SampleState)),
            TableName::SamplingStepModel => write!(f, stringify!(SamplingStepModel)),
            TableName::SamplingStep => write!(f, stringify!(SamplingStep)),
            TableName::ShakingStepModel => write!(f, stringify!(ShakingStepModel)),
            TableName::ShakingStep => write!(f, stringify!(ShakingStep)),
            TableName::SpatialRefSy => write!(f, stringify!(SpatialRefSy)),
            TableName::Spectrum => write!(f, stringify!(Spectrum)),
            TableName::SpectraCollection => write!(f, stringify!(SpectraCollection)),
            TableName::StepContainerModel => write!(f, stringify!(StepContainerModel)),
            TableName::StepInstrument => write!(f, stringify!(StepInstrument)),
            TableName::StepModelContainerCategory => {
                write!(f, stringify!(StepModelContainerCategory))
            }
            TableName::StepModelInstrumentCategory => {
                write!(f, stringify!(StepModelInstrumentCategory))
            }
            TableName::StepModelInstrumentModel => {
                write!(f, stringify!(StepModelInstrumentModel))
            }
            TableName::StepModelInstrument => write!(f, stringify!(StepModelInstrument)),
            TableName::StepModelNameplateCategory => {
                write!(f, stringify!(StepModelNameplateCategory))
            }
            TableName::StepModelToolCategory => {
                write!(f, stringify!(StepModelToolCategory))
            }
            TableName::StepModelTrackableCategory => {
                write!(f, stringify!(StepModelTrackableCategory))
            }
            TableName::StepModel => write!(f, stringify!(StepModel)),
            TableName::StepNameplateModel => write!(f, stringify!(StepNameplateModel)),
            TableName::StepStorageContainer => {
                write!(f, stringify!(StepStorageContainer))
            }
            TableName::StepToolModel => write!(f, stringify!(StepToolModel)),
            TableName::Step => write!(f, stringify!(Step)),
            TableName::StorageContainer => write!(f, stringify!(StorageContainer)),
            TableName::Taxon => write!(f, stringify!(Taxon)),
            TableName::TeamMember => write!(f, stringify!(TeamMember)),
            TableName::TeamProject => write!(f, stringify!(TeamProject)),
            TableName::TeamState => write!(f, stringify!(TeamState)),
            TableName::Team => write!(f, stringify!(Team)),
            TableName::TemporaryUser => write!(f, stringify!(TemporaryUser)),
            TableName::ToolModel => write!(f, stringify!(ToolModel)),
            TableName::TrackableCategory => write!(f, stringify!(TrackableCategory)),
            TableName::TrackableLocation => write!(f, stringify!(TrackableLocation)),
            TableName::TrackableState => write!(f, stringify!(TrackableState)),
            TableName::Trackable => write!(f, stringify!(Trackable)),
            TableName::Unit => write!(f, stringify!(Unit)),
            TableName::UserEmail => write!(f, stringify!(UserEmail)),
            TableName::UserOrganization => write!(f, stringify!(UserOrganization)),
            TableName::User => write!(f, stringify!(User)),
            TableName::VolumetricProcessable => {
                write!(f, stringify!(VolumetricProcessable))
            }
            TableName::WeighingInstrumentModel => {
                write!(f, stringify!(WeighingInstrumentModel))
            }
            TableName::WeighingStepModel => write!(f, stringify!(WeighingStepModel)),
            TableName::WeighingStep => write!(f, stringify!(WeighingStep)),
        }
    }
}
