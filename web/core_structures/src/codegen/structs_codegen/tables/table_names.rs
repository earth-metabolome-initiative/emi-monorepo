#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TableName {
    Address,
    AliquotingInstrumentModel,
    AliquotingProcedureModel,
    BallMillProcedureModel,
    Brand,
    CentrifugeProcedureModel,
    City,
    Color,
    CommercialProductLot,
    CommercialProduct,
    CommercialReagent,
    ContainerModel,
    Container,
    Country,
    DisposalProcedureModel,
    Document,
    EmailProvider,
    FractioningProcedureModel,
    FreezeDryingProcedureModel,
    FreezingProcedureModel,
    InstrumentModel,
    InstrumentState,
    Instrument,
    LoginProvider,
    Material,
    MixCountableProcedureModel,
    MixSolidProcedureModel,
    NextProcedureModel,
    ObservationSubject,
    OrganismObservation,
    OrganismTaxon,
    Organism,
    Organization,
    PackagingModel,
    PackagingProcedureModel,
    ParentProcedureModel,
    PermanenceCategory,
    PouringProcedureModel,
    ProcedureModelTrackable,
    ProcedureModel,
    ProcedureTrackable,
    Procedure,
    Processable,
    ProjectState,
    Project,
    Rank,
    Reagent,
    Role,
    Room,
    SampleState,
    SamplingProcedureModel,
    ShakingProcedureModel,
    SharedProcedureModelTrackable,
    SpatialRefSy,
    Spectrum,
    SpectraCollection,
    Taxon,
    TeamMember,
    TeamProject,
    TeamState,
    Team,
    TemporaryUser,
    TrackableLocation,
    Trackable,
    Unit,
    UserEmail,
    UserOrganization,
    User,
    VolumetricProcessable,
    WeighingInstrumentModel,
    WeighingProcedureModel,
    WeighingProcedure,
}
impl core::fmt::Display for TableName {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            TableName::Address => write!(f, stringify!(Address)),
            TableName::AliquotingInstrumentModel => {
                write!(f, stringify!(AliquotingInstrumentModel))
            }
            TableName::AliquotingProcedureModel => {
                write!(f, stringify!(AliquotingProcedureModel))
            }
            TableName::BallMillProcedureModel => {
                write!(f, stringify!(BallMillProcedureModel))
            }
            TableName::Brand => write!(f, stringify!(Brand)),
            TableName::CentrifugeProcedureModel => {
                write!(f, stringify!(CentrifugeProcedureModel))
            }
            TableName::City => write!(f, stringify!(City)),
            TableName::Color => write!(f, stringify!(Color)),
            TableName::CommercialProductLot => {
                write!(f, stringify!(CommercialProductLot))
            }
            TableName::CommercialProduct => write!(f, stringify!(CommercialProduct)),
            TableName::CommercialReagent => write!(f, stringify!(CommercialReagent)),
            TableName::ContainerModel => write!(f, stringify!(ContainerModel)),
            TableName::Container => write!(f, stringify!(Container)),
            TableName::Country => write!(f, stringify!(Country)),
            TableName::DisposalProcedureModel => {
                write!(f, stringify!(DisposalProcedureModel))
            }
            TableName::Document => write!(f, stringify!(Document)),
            TableName::EmailProvider => write!(f, stringify!(EmailProvider)),
            TableName::FractioningProcedureModel => {
                write!(f, stringify!(FractioningProcedureModel))
            }
            TableName::FreezeDryingProcedureModel => {
                write!(f, stringify!(FreezeDryingProcedureModel))
            }
            TableName::FreezingProcedureModel => {
                write!(f, stringify!(FreezingProcedureModel))
            }
            TableName::InstrumentModel => write!(f, stringify!(InstrumentModel)),
            TableName::InstrumentState => write!(f, stringify!(InstrumentState)),
            TableName::Instrument => write!(f, stringify!(Instrument)),
            TableName::LoginProvider => write!(f, stringify!(LoginProvider)),
            TableName::Material => write!(f, stringify!(Material)),
            TableName::MixCountableProcedureModel => {
                write!(f, stringify!(MixCountableProcedureModel))
            }
            TableName::MixSolidProcedureModel => {
                write!(f, stringify!(MixSolidProcedureModel))
            }
            TableName::NextProcedureModel => write!(f, stringify!(NextProcedureModel)),
            TableName::ObservationSubject => write!(f, stringify!(ObservationSubject)),
            TableName::OrganismObservation => write!(f, stringify!(OrganismObservation)),
            TableName::OrganismTaxon => write!(f, stringify!(OrganismTaxon)),
            TableName::Organism => write!(f, stringify!(Organism)),
            TableName::Organization => write!(f, stringify!(Organization)),
            TableName::PackagingModel => write!(f, stringify!(PackagingModel)),
            TableName::PackagingProcedureModel => {
                write!(f, stringify!(PackagingProcedureModel))
            }
            TableName::ParentProcedureModel => {
                write!(f, stringify!(ParentProcedureModel))
            }
            TableName::PermanenceCategory => write!(f, stringify!(PermanenceCategory)),
            TableName::PouringProcedureModel => {
                write!(f, stringify!(PouringProcedureModel))
            }
            TableName::ProcedureModelTrackable => {
                write!(f, stringify!(ProcedureModelTrackable))
            }
            TableName::ProcedureModel => write!(f, stringify!(ProcedureModel)),
            TableName::ProcedureTrackable => write!(f, stringify!(ProcedureTrackable)),
            TableName::Procedure => write!(f, stringify!(Procedure)),
            TableName::Processable => write!(f, stringify!(Processable)),
            TableName::ProjectState => write!(f, stringify!(ProjectState)),
            TableName::Project => write!(f, stringify!(Project)),
            TableName::Rank => write!(f, stringify!(Rank)),
            TableName::Reagent => write!(f, stringify!(Reagent)),
            TableName::Role => write!(f, stringify!(Role)),
            TableName::Room => write!(f, stringify!(Room)),
            TableName::SampleState => write!(f, stringify!(SampleState)),
            TableName::SamplingProcedureModel => {
                write!(f, stringify!(SamplingProcedureModel))
            }
            TableName::ShakingProcedureModel => {
                write!(f, stringify!(ShakingProcedureModel))
            }
            TableName::SharedProcedureModelTrackable => {
                write!(f, stringify!(SharedProcedureModelTrackable))
            }
            TableName::SpatialRefSy => write!(f, stringify!(SpatialRefSy)),
            TableName::Spectrum => write!(f, stringify!(Spectrum)),
            TableName::SpectraCollection => write!(f, stringify!(SpectraCollection)),
            TableName::Taxon => write!(f, stringify!(Taxon)),
            TableName::TeamMember => write!(f, stringify!(TeamMember)),
            TableName::TeamProject => write!(f, stringify!(TeamProject)),
            TableName::TeamState => write!(f, stringify!(TeamState)),
            TableName::Team => write!(f, stringify!(Team)),
            TableName::TemporaryUser => write!(f, stringify!(TemporaryUser)),
            TableName::TrackableLocation => write!(f, stringify!(TrackableLocation)),
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
            TableName::WeighingProcedureModel => {
                write!(f, stringify!(WeighingProcedureModel))
            }
            TableName::WeighingProcedure => write!(f, stringify!(WeighingProcedure)),
        }
    }
}
