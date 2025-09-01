#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TablePrimaryKey {
    AliquotingProcedureTemplate(i32),
    BallMillProcedureTemplate(i32),
    CappingProcedureTemplate(i32),
    CentrifugeProcedureTemplate(i32),
    DisposalProcedureTemplate(i32),
    FractioningProcedureTemplate(i32),
    FreezeDryingProcedureTemplate(i32),
    FreezingProcedureTemplate(i32),
    GeolocationProcedureTemplate(i32),
    PackagingProcedureTemplate(i32),
    PhotographProcedureTemplate(i32),
    PouringProcedureTemplate(i32),
    ProcedureTemplate(i32),
    StorageProcedureTemplate(i32),
    SupernatantProcedureTemplate(i32),
    WeighingProcedureTemplate(i32),
    AliquotingProcedure(::rosetta_uuid::Uuid),
    BallMillProcedure(::rosetta_uuid::Uuid),
    CappingProcedure(::rosetta_uuid::Uuid),
    CentrifugeProcedure(::rosetta_uuid::Uuid),
    DisposalProcedure(::rosetta_uuid::Uuid),
    FractioningProcedure(::rosetta_uuid::Uuid),
    FreezeDryingProcedure(::rosetta_uuid::Uuid),
    FreezingProcedure(::rosetta_uuid::Uuid),
    GeolocationProcedure(::rosetta_uuid::Uuid),
    PackagingProcedure(::rosetta_uuid::Uuid),
    PhotographProcedure(::rosetta_uuid::Uuid),
    PouringProcedure(::rosetta_uuid::Uuid),
    Procedure(::rosetta_uuid::Uuid),
    StorageProcedure(::rosetta_uuid::Uuid),
    SupernatantProcedure(::rosetta_uuid::Uuid),
    WeighingProcedure(::rosetta_uuid::Uuid),
    Address(i32),
    AssetCompatibilityRule((i32, i32)),
    AssetModelAncestor((i32, i32)),
    AssetModel(i32),
    Asset(::rosetta_uuid::Uuid),
    BallMillMachineModel(i32),
    BallMillMachine(::rosetta_uuid::Uuid),
    BeadsModel(i32),
    Brand(i32),
    CameraModel(i32),
    Camera(::rosetta_uuid::Uuid),
    CapsModel(i32),
    CentrifugeModel(i32),
    Centrifuge(::rosetta_uuid::Uuid),
    City(i32),
    Color(i16),
    CommercialBallMillMachineLot(i32),
    CommercialBallMillMachineModel(i32),
    CommercialBeadsLot(i32),
    CommercialBeadsModel(i32),
    CommercialCameraLot(i32),
    CommercialCameraModel(i32),
    CommercialCentrifugeLot(i32),
    CommercialCentrifugeModel(i32),
    CommercialFreezeDryerLot(i32),
    CommercialFreezeDryerModel(i32),
    CommercialFreezerLot(i32),
    CommercialFreezerModel(i32),
    CommercialPackagingLot(i32),
    CommercialPackagingModel(i32),
    CommercialPipetteLot(i32),
    CommercialPipetteModel(i32),
    CommercialPipetteTipLot(i32),
    CommercialPipetteTipModel(i32),
    CommercialPositioningDeviceLot(i32),
    CommercialPositioningDeviceModel(i32),
    CommercialProductLot(i32),
    CommercialProduct(i32),
    CommercialVolumeMeasuringDeviceLot(i32),
    CommercialVolumeMeasuringDeviceModel(i32),
    CommercialWeighingDeviceLot(i32),
    CommercialWeighingDeviceModel(i32),
    ContainerCompatibilityRule((i32, i32)),
    ContainerModel(i32),
    Container(::rosetta_uuid::Uuid),
    Country(::iso_codes::CountryCode),
    DigitalAssetModel(i32),
    DigitalAsset(::rosetta_uuid::Uuid),
    Document(::rosetta_uuid::Uuid),
    EmailProvider((i32, i16)),
    FreezeDryerModel(i32),
    FreezeDryer(::rosetta_uuid::Uuid),
    FreezerModel(i32),
    Freezer(::rosetta_uuid::Uuid),
    InstrumentState(i16),
    LoginProvider(i16),
    Material(i16),
    NextProcedureTemplate((i32, i32, i32)),
    ObservationSubject(i16),
    OrganismTaxon((::rosetta_uuid::Uuid, i32)),
    Organism(::rosetta_uuid::Uuid),
    Organization(i16),
    PackagingModel(i32),
    ParentProcedureTemplate((i32, i32)),
    PermanenceCategory(i16),
    PhoneModel(i32),
    PhysicalAssetModel(i32),
    PhysicalAsset(::rosetta_uuid::Uuid),
    PipetteModel(i32),
    PipetteTipModel(i32),
    Pipette(::rosetta_uuid::Uuid),
    PositioningDeviceModel(i32),
    PositioningDevice(::rosetta_uuid::Uuid),
    ProcedureAsset((::rosetta_uuid::Uuid, i32)),
    ProcedureTemplateAssetModel(i32),
    ProjectState(i16),
    Project(i32),
    Rank(i16),
    ReagentModel(i32),
    Role(i16),
    Room(i32),
    SampleState(i16),
    SharedProcedureTemplateAssetModel((i32, i32)),
    SpatialRefSy(i32),
    Spectrum(::rosetta_uuid::Uuid),
    SpectraCollection(::rosetta_uuid::Uuid),
    Taxon(i32),
    TeamMember((i32, i32)),
    TeamProject((i32, i32)),
    TeamState(i16),
    Team(i32),
    TemporaryUser(i32),
    Unit(i16),
    UserEmail(i32),
    UserOrganization((i32, i16)),
    User(i32),
    VolumeMeasuringDeviceModel(i32),
    VolumeMeasuringDevice(::rosetta_uuid::Uuid),
    VolumetricContainerModel(i32),
    VolumetricContainer(::rosetta_uuid::Uuid),
    WeighingDeviceModel(i32),
    WeighingDevice(::rosetta_uuid::Uuid),
}
impl web_common_traits::prelude::Tabular for TablePrimaryKey {
    type TableName = crate::codegen::tables::table_names::TableName;
    fn table_name(&self) -> Self::TableName {
        match self {
            TablePrimaryKey::AliquotingProcedureTemplate(_) => {
                crate::codegen::tables::table_names::TableName::AliquotingProcedureTemplate
            }
            TablePrimaryKey::BallMillProcedureTemplate(_) => {
                crate::codegen::tables::table_names::TableName::BallMillProcedureTemplate
            }
            TablePrimaryKey::CappingProcedureTemplate(_) => {
                crate::codegen::tables::table_names::TableName::CappingProcedureTemplate
            }
            TablePrimaryKey::CentrifugeProcedureTemplate(_) => {
                crate::codegen::tables::table_names::TableName::CentrifugeProcedureTemplate
            }
            TablePrimaryKey::DisposalProcedureTemplate(_) => {
                crate::codegen::tables::table_names::TableName::DisposalProcedureTemplate
            }
            TablePrimaryKey::FractioningProcedureTemplate(_) => {
                crate::codegen::tables::table_names::TableName::FractioningProcedureTemplate
            }
            TablePrimaryKey::FreezeDryingProcedureTemplate(_) => {
                crate::codegen::tables::table_names::TableName::FreezeDryingProcedureTemplate
            }
            TablePrimaryKey::FreezingProcedureTemplate(_) => {
                crate::codegen::tables::table_names::TableName::FreezingProcedureTemplate
            }
            TablePrimaryKey::GeolocationProcedureTemplate(_) => {
                crate::codegen::tables::table_names::TableName::GeolocationProcedureTemplate
            }
            TablePrimaryKey::PackagingProcedureTemplate(_) => {
                crate::codegen::tables::table_names::TableName::PackagingProcedureTemplate
            }
            TablePrimaryKey::PhotographProcedureTemplate(_) => {
                crate::codegen::tables::table_names::TableName::PhotographProcedureTemplate
            }
            TablePrimaryKey::PouringProcedureTemplate(_) => {
                crate::codegen::tables::table_names::TableName::PouringProcedureTemplate
            }
            TablePrimaryKey::ProcedureTemplate(_) => {
                crate::codegen::tables::table_names::TableName::ProcedureTemplate
            }
            TablePrimaryKey::StorageProcedureTemplate(_) => {
                crate::codegen::tables::table_names::TableName::StorageProcedureTemplate
            }
            TablePrimaryKey::SupernatantProcedureTemplate(_) => {
                crate::codegen::tables::table_names::TableName::SupernatantProcedureTemplate
            }
            TablePrimaryKey::WeighingProcedureTemplate(_) => {
                crate::codegen::tables::table_names::TableName::WeighingProcedureTemplate
            }
            TablePrimaryKey::AliquotingProcedure(_) => {
                crate::codegen::tables::table_names::TableName::AliquotingProcedure
            }
            TablePrimaryKey::BallMillProcedure(_) => {
                crate::codegen::tables::table_names::TableName::BallMillProcedure
            }
            TablePrimaryKey::CappingProcedure(_) => {
                crate::codegen::tables::table_names::TableName::CappingProcedure
            }
            TablePrimaryKey::CentrifugeProcedure(_) => {
                crate::codegen::tables::table_names::TableName::CentrifugeProcedure
            }
            TablePrimaryKey::DisposalProcedure(_) => {
                crate::codegen::tables::table_names::TableName::DisposalProcedure
            }
            TablePrimaryKey::FractioningProcedure(_) => {
                crate::codegen::tables::table_names::TableName::FractioningProcedure
            }
            TablePrimaryKey::FreezeDryingProcedure(_) => {
                crate::codegen::tables::table_names::TableName::FreezeDryingProcedure
            }
            TablePrimaryKey::FreezingProcedure(_) => {
                crate::codegen::tables::table_names::TableName::FreezingProcedure
            }
            TablePrimaryKey::GeolocationProcedure(_) => {
                crate::codegen::tables::table_names::TableName::GeolocationProcedure
            }
            TablePrimaryKey::PackagingProcedure(_) => {
                crate::codegen::tables::table_names::TableName::PackagingProcedure
            }
            TablePrimaryKey::PhotographProcedure(_) => {
                crate::codegen::tables::table_names::TableName::PhotographProcedure
            }
            TablePrimaryKey::PouringProcedure(_) => {
                crate::codegen::tables::table_names::TableName::PouringProcedure
            }
            TablePrimaryKey::Procedure(_) => {
                crate::codegen::tables::table_names::TableName::Procedure
            }
            TablePrimaryKey::StorageProcedure(_) => {
                crate::codegen::tables::table_names::TableName::StorageProcedure
            }
            TablePrimaryKey::SupernatantProcedure(_) => {
                crate::codegen::tables::table_names::TableName::SupernatantProcedure
            }
            TablePrimaryKey::WeighingProcedure(_) => {
                crate::codegen::tables::table_names::TableName::WeighingProcedure
            }
            TablePrimaryKey::Address(_) => crate::codegen::tables::table_names::TableName::Address,
            TablePrimaryKey::AssetCompatibilityRule(_) => {
                crate::codegen::tables::table_names::TableName::AssetCompatibilityRule
            }
            TablePrimaryKey::AssetModelAncestor(_) => {
                crate::codegen::tables::table_names::TableName::AssetModelAncestor
            }
            TablePrimaryKey::AssetModel(_) => {
                crate::codegen::tables::table_names::TableName::AssetModel
            }
            TablePrimaryKey::Asset(_) => crate::codegen::tables::table_names::TableName::Asset,
            TablePrimaryKey::BallMillMachineModel(_) => {
                crate::codegen::tables::table_names::TableName::BallMillMachineModel
            }
            TablePrimaryKey::BallMillMachine(_) => {
                crate::codegen::tables::table_names::TableName::BallMillMachine
            }
            TablePrimaryKey::BeadsModel(_) => {
                crate::codegen::tables::table_names::TableName::BeadsModel
            }
            TablePrimaryKey::Brand(_) => crate::codegen::tables::table_names::TableName::Brand,
            TablePrimaryKey::CameraModel(_) => {
                crate::codegen::tables::table_names::TableName::CameraModel
            }
            TablePrimaryKey::Camera(_) => crate::codegen::tables::table_names::TableName::Camera,
            TablePrimaryKey::CapsModel(_) => {
                crate::codegen::tables::table_names::TableName::CapsModel
            }
            TablePrimaryKey::CentrifugeModel(_) => {
                crate::codegen::tables::table_names::TableName::CentrifugeModel
            }
            TablePrimaryKey::Centrifuge(_) => {
                crate::codegen::tables::table_names::TableName::Centrifuge
            }
            TablePrimaryKey::City(_) => crate::codegen::tables::table_names::TableName::City,
            TablePrimaryKey::Color(_) => crate::codegen::tables::table_names::TableName::Color,
            TablePrimaryKey::CommercialBallMillMachineLot(_) => {
                crate::codegen::tables::table_names::TableName::CommercialBallMillMachineLot
            }
            TablePrimaryKey::CommercialBallMillMachineModel(_) => {
                crate::codegen::tables::table_names::TableName::CommercialBallMillMachineModel
            }
            TablePrimaryKey::CommercialBeadsLot(_) => {
                crate::codegen::tables::table_names::TableName::CommercialBeadsLot
            }
            TablePrimaryKey::CommercialBeadsModel(_) => {
                crate::codegen::tables::table_names::TableName::CommercialBeadsModel
            }
            TablePrimaryKey::CommercialCameraLot(_) => {
                crate::codegen::tables::table_names::TableName::CommercialCameraLot
            }
            TablePrimaryKey::CommercialCameraModel(_) => {
                crate::codegen::tables::table_names::TableName::CommercialCameraModel
            }
            TablePrimaryKey::CommercialCentrifugeLot(_) => {
                crate::codegen::tables::table_names::TableName::CommercialCentrifugeLot
            }
            TablePrimaryKey::CommercialCentrifugeModel(_) => {
                crate::codegen::tables::table_names::TableName::CommercialCentrifugeModel
            }
            TablePrimaryKey::CommercialFreezeDryerLot(_) => {
                crate::codegen::tables::table_names::TableName::CommercialFreezeDryerLot
            }
            TablePrimaryKey::CommercialFreezeDryerModel(_) => {
                crate::codegen::tables::table_names::TableName::CommercialFreezeDryerModel
            }
            TablePrimaryKey::CommercialFreezerLot(_) => {
                crate::codegen::tables::table_names::TableName::CommercialFreezerLot
            }
            TablePrimaryKey::CommercialFreezerModel(_) => {
                crate::codegen::tables::table_names::TableName::CommercialFreezerModel
            }
            TablePrimaryKey::CommercialPackagingLot(_) => {
                crate::codegen::tables::table_names::TableName::CommercialPackagingLot
            }
            TablePrimaryKey::CommercialPackagingModel(_) => {
                crate::codegen::tables::table_names::TableName::CommercialPackagingModel
            }
            TablePrimaryKey::CommercialPipetteLot(_) => {
                crate::codegen::tables::table_names::TableName::CommercialPipetteLot
            }
            TablePrimaryKey::CommercialPipetteModel(_) => {
                crate::codegen::tables::table_names::TableName::CommercialPipetteModel
            }
            TablePrimaryKey::CommercialPipetteTipLot(_) => {
                crate::codegen::tables::table_names::TableName::CommercialPipetteTipLot
            }
            TablePrimaryKey::CommercialPipetteTipModel(_) => {
                crate::codegen::tables::table_names::TableName::CommercialPipetteTipModel
            }
            TablePrimaryKey::CommercialPositioningDeviceLot(_) => {
                crate::codegen::tables::table_names::TableName::CommercialPositioningDeviceLot
            }
            TablePrimaryKey::CommercialPositioningDeviceModel(_) => {
                crate::codegen::tables::table_names::TableName::CommercialPositioningDeviceModel
            }
            TablePrimaryKey::CommercialProductLot(_) => {
                crate::codegen::tables::table_names::TableName::CommercialProductLot
            }
            TablePrimaryKey::CommercialProduct(_) => {
                crate::codegen::tables::table_names::TableName::CommercialProduct
            }
            TablePrimaryKey::CommercialVolumeMeasuringDeviceLot(_) => {
                crate::codegen::tables::table_names::TableName::CommercialVolumeMeasuringDeviceLot
            }
            TablePrimaryKey::CommercialVolumeMeasuringDeviceModel(_) => {
                crate::codegen::tables::table_names::TableName::CommercialVolumeMeasuringDeviceModel
            }
            TablePrimaryKey::CommercialWeighingDeviceLot(_) => {
                crate::codegen::tables::table_names::TableName::CommercialWeighingDeviceLot
            }
            TablePrimaryKey::CommercialWeighingDeviceModel(_) => {
                crate::codegen::tables::table_names::TableName::CommercialWeighingDeviceModel
            }
            TablePrimaryKey::ContainerCompatibilityRule(_) => {
                crate::codegen::tables::table_names::TableName::ContainerCompatibilityRule
            }
            TablePrimaryKey::ContainerModel(_) => {
                crate::codegen::tables::table_names::TableName::ContainerModel
            }
            TablePrimaryKey::Container(_) => {
                crate::codegen::tables::table_names::TableName::Container
            }
            TablePrimaryKey::Country(_) => crate::codegen::tables::table_names::TableName::Country,
            TablePrimaryKey::DigitalAssetModel(_) => {
                crate::codegen::tables::table_names::TableName::DigitalAssetModel
            }
            TablePrimaryKey::DigitalAsset(_) => {
                crate::codegen::tables::table_names::TableName::DigitalAsset
            }
            TablePrimaryKey::Document(_) => {
                crate::codegen::tables::table_names::TableName::Document
            }
            TablePrimaryKey::EmailProvider(_) => {
                crate::codegen::tables::table_names::TableName::EmailProvider
            }
            TablePrimaryKey::FreezeDryerModel(_) => {
                crate::codegen::tables::table_names::TableName::FreezeDryerModel
            }
            TablePrimaryKey::FreezeDryer(_) => {
                crate::codegen::tables::table_names::TableName::FreezeDryer
            }
            TablePrimaryKey::FreezerModel(_) => {
                crate::codegen::tables::table_names::TableName::FreezerModel
            }
            TablePrimaryKey::Freezer(_) => crate::codegen::tables::table_names::TableName::Freezer,
            TablePrimaryKey::InstrumentState(_) => {
                crate::codegen::tables::table_names::TableName::InstrumentState
            }
            TablePrimaryKey::LoginProvider(_) => {
                crate::codegen::tables::table_names::TableName::LoginProvider
            }
            TablePrimaryKey::Material(_) => {
                crate::codegen::tables::table_names::TableName::Material
            }
            TablePrimaryKey::NextProcedureTemplate(_) => {
                crate::codegen::tables::table_names::TableName::NextProcedureTemplate
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
            TablePrimaryKey::PackagingModel(_) => {
                crate::codegen::tables::table_names::TableName::PackagingModel
            }
            TablePrimaryKey::ParentProcedureTemplate(_) => {
                crate::codegen::tables::table_names::TableName::ParentProcedureTemplate
            }
            TablePrimaryKey::PermanenceCategory(_) => {
                crate::codegen::tables::table_names::TableName::PermanenceCategory
            }
            TablePrimaryKey::PhoneModel(_) => {
                crate::codegen::tables::table_names::TableName::PhoneModel
            }
            TablePrimaryKey::PhysicalAssetModel(_) => {
                crate::codegen::tables::table_names::TableName::PhysicalAssetModel
            }
            TablePrimaryKey::PhysicalAsset(_) => {
                crate::codegen::tables::table_names::TableName::PhysicalAsset
            }
            TablePrimaryKey::PipetteModel(_) => {
                crate::codegen::tables::table_names::TableName::PipetteModel
            }
            TablePrimaryKey::PipetteTipModel(_) => {
                crate::codegen::tables::table_names::TableName::PipetteTipModel
            }
            TablePrimaryKey::Pipette(_) => crate::codegen::tables::table_names::TableName::Pipette,
            TablePrimaryKey::PositioningDeviceModel(_) => {
                crate::codegen::tables::table_names::TableName::PositioningDeviceModel
            }
            TablePrimaryKey::PositioningDevice(_) => {
                crate::codegen::tables::table_names::TableName::PositioningDevice
            }
            TablePrimaryKey::ProcedureAsset(_) => {
                crate::codegen::tables::table_names::TableName::ProcedureAsset
            }
            TablePrimaryKey::ProcedureTemplateAssetModel(_) => {
                crate::codegen::tables::table_names::TableName::ProcedureTemplateAssetModel
            }
            TablePrimaryKey::ProjectState(_) => {
                crate::codegen::tables::table_names::TableName::ProjectState
            }
            TablePrimaryKey::Project(_) => crate::codegen::tables::table_names::TableName::Project,
            TablePrimaryKey::Rank(_) => crate::codegen::tables::table_names::TableName::Rank,
            TablePrimaryKey::ReagentModel(_) => {
                crate::codegen::tables::table_names::TableName::ReagentModel
            }
            TablePrimaryKey::Role(_) => crate::codegen::tables::table_names::TableName::Role,
            TablePrimaryKey::Room(_) => crate::codegen::tables::table_names::TableName::Room,
            TablePrimaryKey::SampleState(_) => {
                crate::codegen::tables::table_names::TableName::SampleState
            }
            TablePrimaryKey::SharedProcedureTemplateAssetModel(_) => {
                crate::codegen::tables::table_names::TableName::SharedProcedureTemplateAssetModel
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
            TablePrimaryKey::Unit(_) => crate::codegen::tables::table_names::TableName::Unit,
            TablePrimaryKey::UserEmail(_) => {
                crate::codegen::tables::table_names::TableName::UserEmail
            }
            TablePrimaryKey::UserOrganization(_) => {
                crate::codegen::tables::table_names::TableName::UserOrganization
            }
            TablePrimaryKey::User(_) => crate::codegen::tables::table_names::TableName::User,
            TablePrimaryKey::VolumeMeasuringDeviceModel(_) => {
                crate::codegen::tables::table_names::TableName::VolumeMeasuringDeviceModel
            }
            TablePrimaryKey::VolumeMeasuringDevice(_) => {
                crate::codegen::tables::table_names::TableName::VolumeMeasuringDevice
            }
            TablePrimaryKey::VolumetricContainerModel(_) => {
                crate::codegen::tables::table_names::TableName::VolumetricContainerModel
            }
            TablePrimaryKey::VolumetricContainer(_) => {
                crate::codegen::tables::table_names::TableName::VolumetricContainer
            }
            TablePrimaryKey::WeighingDeviceModel(_) => {
                crate::codegen::tables::table_names::TableName::WeighingDeviceModel
            }
            TablePrimaryKey::WeighingDevice(_) => {
                crate::codegen::tables::table_names::TableName::WeighingDevice
            }
        }
    }
}
