#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TablePrimaryKey {
    Address(i32),
    AliquotingProcedureModel(i32),
    AliquotingProcedure(::rosetta_uuid::Uuid),
    BallMillMachineModel(::rosetta_uuid::Uuid),
    BallMillProcedureModel(i32),
    BinaryQuestionProcedureModel(i32),
    Brand(i32),
    CameraModel(::rosetta_uuid::Uuid),
    CappingProcedureModel(i32),
    CentrifugeModel(::rosetta_uuid::Uuid),
    CentrifugeProcedureModel(i32),
    City(i32),
    Color(i16),
    CommercialProductLot(::rosetta_uuid::Uuid),
    CommercialProduct(::rosetta_uuid::Uuid),
    CommercialReagent(::rosetta_uuid::Uuid),
    CompatibilityRule((::rosetta_uuid::Uuid, ::rosetta_uuid::Uuid)),
    ContainerModel(::rosetta_uuid::Uuid),
    Container(::rosetta_uuid::Uuid),
    Country(::iso_codes::CountryCode),
    DisposalProcedureModel(i32),
    Document(::rosetta_uuid::Uuid),
    EmailProvider((i32, i16)),
    FractioningProcedureModel(i32),
    FreezeDrierModel(::rosetta_uuid::Uuid),
    FreezeDryingProcedureModel(i32),
    FreezerModel(::rosetta_uuid::Uuid),
    FreezingProcedureModel(i32),
    GeolocationProcedureModel(i32),
    InstrumentModel(::rosetta_uuid::Uuid),
    InstrumentState(i16),
    LoginProvider(i16),
    Material(i16),
    MixingProcedureModel(i32),
    NextProcedureModel((i32, i32, i32)),
    ObservationSubject(i16),
    OrganismTaxon((::rosetta_uuid::Uuid, i32)),
    Organism(::rosetta_uuid::Uuid),
    Organization(i16),
    PackagingProcedureModel(i32),
    ParentProcedureModel((i32, i32)),
    PermanenceCategory(i16),
    PhoneModel(::rosetta_uuid::Uuid),
    PhotographProcedureModel(i32),
    PipetteModel(::rosetta_uuid::Uuid),
    PipetteTipModel(::rosetta_uuid::Uuid),
    PlacingProcedureModel(i32),
    PositioningDeviceModel(::rosetta_uuid::Uuid),
    PouringProcedureModel(i32),
    ProcedureModelTrackable(i32),
    ProcedureModel(i32),
    ProcedureTrackable((::rosetta_uuid::Uuid, ::rosetta_uuid::Uuid)),
    Procedure(::rosetta_uuid::Uuid),
    Processable(::rosetta_uuid::Uuid),
    ProjectState(i16),
    Project(i32),
    Rank(i16),
    Reagent(::rosetta_uuid::Uuid),
    Role(i16),
    Room(i32),
    SampleState(i16),
    SharedProcedureModelTrackable((i32, i32)),
    SpatialRefSy(i32),
    Spectrum(i32),
    SpectraCollection(i32),
    StorageProcedureModel(i32),
    SupernatantProcedureModel(i32),
    SupernatantProcedure(::rosetta_uuid::Uuid),
    Taxon(i32),
    TeamMember((i32, i32)),
    TeamProject((i32, i32)),
    TeamState(i16),
    Team(i32),
    TemporaryUser(i32),
    TrackableAncestor((::rosetta_uuid::Uuid, ::rosetta_uuid::Uuid)),
    TrackableLocation(::rosetta_uuid::Uuid),
    Trackable(::rosetta_uuid::Uuid),
    Unit(i16),
    UserEmail(i32),
    UserOrganization((i32, i16)),
    User(i32),
    VolumetricContainerModel(::rosetta_uuid::Uuid),
    VolumetricProcessable(::rosetta_uuid::Uuid),
    WeighingInstrumentModel(::rosetta_uuid::Uuid),
    WeighingProcedureModel(i32),
    WeighingProcedure(::rosetta_uuid::Uuid),
}
impl web_common_traits::prelude::Tabular for TablePrimaryKey {
    type TableName = crate::codegen::tables::table_names::TableName;
    fn table_name(&self) -> Self::TableName {
        match self {
            TablePrimaryKey::Address(_) => crate::codegen::tables::table_names::TableName::Address,
            TablePrimaryKey::AliquotingProcedureModel(_) => {
                crate::codegen::tables::table_names::TableName::AliquotingProcedureModel
            }
            TablePrimaryKey::AliquotingProcedure(_) => {
                crate::codegen::tables::table_names::TableName::AliquotingProcedure
            }
            TablePrimaryKey::BallMillMachineModel(_) => {
                crate::codegen::tables::table_names::TableName::BallMillMachineModel
            }
            TablePrimaryKey::BallMillProcedureModel(_) => {
                crate::codegen::tables::table_names::TableName::BallMillProcedureModel
            }
            TablePrimaryKey::BinaryQuestionProcedureModel(_) => {
                crate::codegen::tables::table_names::TableName::BinaryQuestionProcedureModel
            }
            TablePrimaryKey::Brand(_) => crate::codegen::tables::table_names::TableName::Brand,
            TablePrimaryKey::CameraModel(_) => {
                crate::codegen::tables::table_names::TableName::CameraModel
            }
            TablePrimaryKey::CappingProcedureModel(_) => {
                crate::codegen::tables::table_names::TableName::CappingProcedureModel
            }
            TablePrimaryKey::CentrifugeModel(_) => {
                crate::codegen::tables::table_names::TableName::CentrifugeModel
            }
            TablePrimaryKey::CentrifugeProcedureModel(_) => {
                crate::codegen::tables::table_names::TableName::CentrifugeProcedureModel
            }
            TablePrimaryKey::City(_) => crate::codegen::tables::table_names::TableName::City,
            TablePrimaryKey::Color(_) => crate::codegen::tables::table_names::TableName::Color,
            TablePrimaryKey::CommercialProductLot(_) => {
                crate::codegen::tables::table_names::TableName::CommercialProductLot
            }
            TablePrimaryKey::CommercialProduct(_) => {
                crate::codegen::tables::table_names::TableName::CommercialProduct
            }
            TablePrimaryKey::CommercialReagent(_) => {
                crate::codegen::tables::table_names::TableName::CommercialReagent
            }
            TablePrimaryKey::CompatibilityRule(_) => {
                crate::codegen::tables::table_names::TableName::CompatibilityRule
            }
            TablePrimaryKey::ContainerModel(_) => {
                crate::codegen::tables::table_names::TableName::ContainerModel
            }
            TablePrimaryKey::Container(_) => {
                crate::codegen::tables::table_names::TableName::Container
            }
            TablePrimaryKey::Country(_) => crate::codegen::tables::table_names::TableName::Country,
            TablePrimaryKey::DisposalProcedureModel(_) => {
                crate::codegen::tables::table_names::TableName::DisposalProcedureModel
            }
            TablePrimaryKey::Document(_) => {
                crate::codegen::tables::table_names::TableName::Document
            }
            TablePrimaryKey::EmailProvider(_) => {
                crate::codegen::tables::table_names::TableName::EmailProvider
            }
            TablePrimaryKey::FractioningProcedureModel(_) => {
                crate::codegen::tables::table_names::TableName::FractioningProcedureModel
            }
            TablePrimaryKey::FreezeDrierModel(_) => {
                crate::codegen::tables::table_names::TableName::FreezeDrierModel
            }
            TablePrimaryKey::FreezeDryingProcedureModel(_) => {
                crate::codegen::tables::table_names::TableName::FreezeDryingProcedureModel
            }
            TablePrimaryKey::FreezerModel(_) => {
                crate::codegen::tables::table_names::TableName::FreezerModel
            }
            TablePrimaryKey::FreezingProcedureModel(_) => {
                crate::codegen::tables::table_names::TableName::FreezingProcedureModel
            }
            TablePrimaryKey::GeolocationProcedureModel(_) => {
                crate::codegen::tables::table_names::TableName::GeolocationProcedureModel
            }
            TablePrimaryKey::InstrumentModel(_) => {
                crate::codegen::tables::table_names::TableName::InstrumentModel
            }
            TablePrimaryKey::InstrumentState(_) => {
                crate::codegen::tables::table_names::TableName::InstrumentState
            }
            TablePrimaryKey::LoginProvider(_) => {
                crate::codegen::tables::table_names::TableName::LoginProvider
            }
            TablePrimaryKey::Material(_) => {
                crate::codegen::tables::table_names::TableName::Material
            }
            TablePrimaryKey::MixingProcedureModel(_) => {
                crate::codegen::tables::table_names::TableName::MixingProcedureModel
            }
            TablePrimaryKey::NextProcedureModel(_) => {
                crate::codegen::tables::table_names::TableName::NextProcedureModel
            }
            TablePrimaryKey::ObservationSubject(_) => {
                crate::codegen::tables::table_names::TableName::ObservationSubject
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
            TablePrimaryKey::PackagingProcedureModel(_) => {
                crate::codegen::tables::table_names::TableName::PackagingProcedureModel
            }
            TablePrimaryKey::ParentProcedureModel(_) => {
                crate::codegen::tables::table_names::TableName::ParentProcedureModel
            }
            TablePrimaryKey::PermanenceCategory(_) => {
                crate::codegen::tables::table_names::TableName::PermanenceCategory
            }
            TablePrimaryKey::PhoneModel(_) => {
                crate::codegen::tables::table_names::TableName::PhoneModel
            }
            TablePrimaryKey::PhotographProcedureModel(_) => {
                crate::codegen::tables::table_names::TableName::PhotographProcedureModel
            }
            TablePrimaryKey::PipetteModel(_) => {
                crate::codegen::tables::table_names::TableName::PipetteModel
            }
            TablePrimaryKey::PipetteTipModel(_) => {
                crate::codegen::tables::table_names::TableName::PipetteTipModel
            }
            TablePrimaryKey::PlacingProcedureModel(_) => {
                crate::codegen::tables::table_names::TableName::PlacingProcedureModel
            }
            TablePrimaryKey::PositioningDeviceModel(_) => {
                crate::codegen::tables::table_names::TableName::PositioningDeviceModel
            }
            TablePrimaryKey::PouringProcedureModel(_) => {
                crate::codegen::tables::table_names::TableName::PouringProcedureModel
            }
            TablePrimaryKey::ProcedureModelTrackable(_) => {
                crate::codegen::tables::table_names::TableName::ProcedureModelTrackable
            }
            TablePrimaryKey::ProcedureModel(_) => {
                crate::codegen::tables::table_names::TableName::ProcedureModel
            }
            TablePrimaryKey::ProcedureTrackable(_) => {
                crate::codegen::tables::table_names::TableName::ProcedureTrackable
            }
            TablePrimaryKey::Procedure(_) => {
                crate::codegen::tables::table_names::TableName::Procedure
            }
            TablePrimaryKey::Processable(_) => {
                crate::codegen::tables::table_names::TableName::Processable
            }
            TablePrimaryKey::ProjectState(_) => {
                crate::codegen::tables::table_names::TableName::ProjectState
            }
            TablePrimaryKey::Project(_) => crate::codegen::tables::table_names::TableName::Project,
            TablePrimaryKey::Rank(_) => crate::codegen::tables::table_names::TableName::Rank,
            TablePrimaryKey::Reagent(_) => crate::codegen::tables::table_names::TableName::Reagent,
            TablePrimaryKey::Role(_) => crate::codegen::tables::table_names::TableName::Role,
            TablePrimaryKey::Room(_) => crate::codegen::tables::table_names::TableName::Room,
            TablePrimaryKey::SampleState(_) => {
                crate::codegen::tables::table_names::TableName::SampleState
            }
            TablePrimaryKey::SharedProcedureModelTrackable(_) => {
                crate::codegen::tables::table_names::TableName::SharedProcedureModelTrackable
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
            TablePrimaryKey::StorageProcedureModel(_) => {
                crate::codegen::tables::table_names::TableName::StorageProcedureModel
            }
            TablePrimaryKey::SupernatantProcedureModel(_) => {
                crate::codegen::tables::table_names::TableName::SupernatantProcedureModel
            }
            TablePrimaryKey::SupernatantProcedure(_) => {
                crate::codegen::tables::table_names::TableName::SupernatantProcedure
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
            TablePrimaryKey::TrackableAncestor(_) => {
                crate::codegen::tables::table_names::TableName::TrackableAncestor
            }
            TablePrimaryKey::TrackableLocation(_) => {
                crate::codegen::tables::table_names::TableName::TrackableLocation
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
            TablePrimaryKey::VolumetricContainerModel(_) => {
                crate::codegen::tables::table_names::TableName::VolumetricContainerModel
            }
            TablePrimaryKey::VolumetricProcessable(_) => {
                crate::codegen::tables::table_names::TableName::VolumetricProcessable
            }
            TablePrimaryKey::WeighingInstrumentModel(_) => {
                crate::codegen::tables::table_names::TableName::WeighingInstrumentModel
            }
            TablePrimaryKey::WeighingProcedureModel(_) => {
                crate::codegen::tables::table_names::TableName::WeighingProcedureModel
            }
            TablePrimaryKey::WeighingProcedure(_) => {
                crate::codegen::tables::table_names::TableName::WeighingProcedure
            }
        }
    }
}
