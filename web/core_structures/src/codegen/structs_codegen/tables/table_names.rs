#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TableName {
    AliquotingProcedureTemplate,
    BallMillProcedureTemplate,
    CappingProcedureTemplate,
    CentrifugeProcedureTemplate,
    DisposalProcedureTemplate,
    FractioningProcedureTemplate,
    FreezeDryingProcedureTemplate,
    FreezingProcedureTemplate,
    GeolocationProcedureTemplate,
    PackagingProcedureTemplate,
    PhotographProcedureTemplate,
    PouringProcedureTemplate,
    ProcedureTemplate,
    StorageProcedureTemplate,
    SupernatantProcedureTemplate,
    WeighingProcedureTemplate,
    AliquotingProcedure,
    BallMillProcedure,
    CappingProcedure,
    CentrifugeProcedure,
    DisposalProcedure,
    FractioningProcedure,
    FreezeDryingProcedure,
    FreezingProcedure,
    GeolocationProcedure,
    PackagingProcedure,
    PhotographProcedure,
    PouringProcedure,
    Procedure,
    StorageProcedure,
    SupernatantProcedure,
    WeighingProcedure,
    Address,
    AssetCompatibilityRule,
    AssetModelAncestor,
    AssetModel,
    Asset,
    BallMillMachineModel,
    BallMillMachine,
    BeadsModel,
    Brand,
    CameraModel,
    Camera,
    CapsModel,
    CentrifugeModel,
    Centrifuge,
    City,
    Color,
    CommercialBallMillMachineLot,
    CommercialBallMillMachineModel,
    CommercialBeadsLot,
    CommercialBeadsModel,
    CommercialCameraLot,
    CommercialCameraModel,
    CommercialCentrifugeLot,
    CommercialCentrifugeModel,
    CommercialFreezeDryerLot,
    CommercialFreezeDryerModel,
    CommercialFreezerLot,
    CommercialFreezerModel,
    CommercialPackagingLot,
    CommercialPackagingModel,
    CommercialPipetteLot,
    CommercialPipetteModel,
    CommercialPipetteTipLot,
    CommercialPipetteTipModel,
    CommercialPositioningDeviceLot,
    CommercialPositioningDeviceModel,
    CommercialProductLot,
    CommercialProduct,
    CommercialVolumeMeasuringDeviceLot,
    CommercialVolumeMeasuringDeviceModel,
    CommercialWeighingDeviceLot,
    CommercialWeighingDeviceModel,
    ContainerCompatibilityRule,
    ContainerModel,
    Container,
    Country,
    DigitalAssetModel,
    DigitalAsset,
    Document,
    EmailProvider,
    FreezeDryerModel,
    FreezeDryer,
    FreezerModel,
    Freezer,
    InstrumentState,
    LoginProvider,
    Material,
    NextProcedureTemplate,
    ObservationSubject,
    OrganismTaxon,
    Organism,
    Organization,
    PackagingModel,
    ParentProcedureTemplate,
    PermanenceCategory,
    PhoneModel,
    PhysicalAssetModel,
    PhysicalAsset,
    PipetteModel,
    PipetteTipModel,
    Pipette,
    PositioningDeviceModel,
    PositioningDevice,
    ProcedureAsset,
    ProcedureTemplateAssetModel,
    ProjectState,
    Project,
    Rank,
    ReagentModel,
    Role,
    Room,
    SampleState,
    SharedProcedureTemplateAssetModel,
    SpatialRefSy,
    Spectrum,
    SpectraCollection,
    Taxon,
    TeamMember,
    TeamProject,
    TeamState,
    Team,
    TemporaryUser,
    Unit,
    UserEmail,
    UserOrganization,
    User,
    VolumeMeasuringDeviceModel,
    VolumeMeasuringDevice,
    VolumetricContainerModel,
    VolumetricContainer,
    WeighingDeviceModel,
    WeighingDevice,
}
impl core::fmt::Display for TableName {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            TableName::AliquotingProcedureTemplate => {
                write!(f, stringify!(AliquotingProcedureTemplate))
            }
            TableName::BallMillProcedureTemplate => {
                write!(f, stringify!(BallMillProcedureTemplate))
            }
            TableName::CappingProcedureTemplate => {
                write!(f, stringify!(CappingProcedureTemplate))
            }
            TableName::CentrifugeProcedureTemplate => {
                write!(f, stringify!(CentrifugeProcedureTemplate))
            }
            TableName::DisposalProcedureTemplate => {
                write!(f, stringify!(DisposalProcedureTemplate))
            }
            TableName::FractioningProcedureTemplate => {
                write!(f, stringify!(FractioningProcedureTemplate))
            }
            TableName::FreezeDryingProcedureTemplate => {
                write!(f, stringify!(FreezeDryingProcedureTemplate))
            }
            TableName::FreezingProcedureTemplate => {
                write!(f, stringify!(FreezingProcedureTemplate))
            }
            TableName::GeolocationProcedureTemplate => {
                write!(f, stringify!(GeolocationProcedureTemplate))
            }
            TableName::PackagingProcedureTemplate => {
                write!(f, stringify!(PackagingProcedureTemplate))
            }
            TableName::PhotographProcedureTemplate => {
                write!(f, stringify!(PhotographProcedureTemplate))
            }
            TableName::PouringProcedureTemplate => {
                write!(f, stringify!(PouringProcedureTemplate))
            }
            TableName::ProcedureTemplate => write!(f, stringify!(ProcedureTemplate)),
            TableName::StorageProcedureTemplate => {
                write!(f, stringify!(StorageProcedureTemplate))
            }
            TableName::SupernatantProcedureTemplate => {
                write!(f, stringify!(SupernatantProcedureTemplate))
            }
            TableName::WeighingProcedureTemplate => {
                write!(f, stringify!(WeighingProcedureTemplate))
            }
            TableName::AliquotingProcedure => write!(f, stringify!(AliquotingProcedure)),
            TableName::BallMillProcedure => write!(f, stringify!(BallMillProcedure)),
            TableName::CappingProcedure => write!(f, stringify!(CappingProcedure)),
            TableName::CentrifugeProcedure => write!(f, stringify!(CentrifugeProcedure)),
            TableName::DisposalProcedure => write!(f, stringify!(DisposalProcedure)),
            TableName::FractioningProcedure => {
                write!(f, stringify!(FractioningProcedure))
            }
            TableName::FreezeDryingProcedure => {
                write!(f, stringify!(FreezeDryingProcedure))
            }
            TableName::FreezingProcedure => write!(f, stringify!(FreezingProcedure)),
            TableName::GeolocationProcedure => {
                write!(f, stringify!(GeolocationProcedure))
            }
            TableName::PackagingProcedure => write!(f, stringify!(PackagingProcedure)),
            TableName::PhotographProcedure => write!(f, stringify!(PhotographProcedure)),
            TableName::PouringProcedure => write!(f, stringify!(PouringProcedure)),
            TableName::Procedure => write!(f, stringify!(Procedure)),
            TableName::StorageProcedure => write!(f, stringify!(StorageProcedure)),
            TableName::SupernatantProcedure => {
                write!(f, stringify!(SupernatantProcedure))
            }
            TableName::WeighingProcedure => write!(f, stringify!(WeighingProcedure)),
            TableName::Address => write!(f, stringify!(Address)),
            TableName::AssetCompatibilityRule => {
                write!(f, stringify!(AssetCompatibilityRule))
            }
            TableName::AssetModelAncestor => write!(f, stringify!(AssetModelAncestor)),
            TableName::AssetModel => write!(f, stringify!(AssetModel)),
            TableName::Asset => write!(f, stringify!(Asset)),
            TableName::BallMillMachineModel => {
                write!(f, stringify!(BallMillMachineModel))
            }
            TableName::BallMillMachine => write!(f, stringify!(BallMillMachine)),
            TableName::BeadsModel => write!(f, stringify!(BeadsModel)),
            TableName::Brand => write!(f, stringify!(Brand)),
            TableName::CameraModel => write!(f, stringify!(CameraModel)),
            TableName::Camera => write!(f, stringify!(Camera)),
            TableName::CapsModel => write!(f, stringify!(CapsModel)),
            TableName::CentrifugeModel => write!(f, stringify!(CentrifugeModel)),
            TableName::Centrifuge => write!(f, stringify!(Centrifuge)),
            TableName::City => write!(f, stringify!(City)),
            TableName::Color => write!(f, stringify!(Color)),
            TableName::CommercialBallMillMachineLot => {
                write!(f, stringify!(CommercialBallMillMachineLot))
            }
            TableName::CommercialBallMillMachineModel => {
                write!(f, stringify!(CommercialBallMillMachineModel))
            }
            TableName::CommercialBeadsLot => write!(f, stringify!(CommercialBeadsLot)),
            TableName::CommercialBeadsModel => {
                write!(f, stringify!(CommercialBeadsModel))
            }
            TableName::CommercialCameraLot => write!(f, stringify!(CommercialCameraLot)),
            TableName::CommercialCameraModel => {
                write!(f, stringify!(CommercialCameraModel))
            }
            TableName::CommercialCentrifugeLot => {
                write!(f, stringify!(CommercialCentrifugeLot))
            }
            TableName::CommercialCentrifugeModel => {
                write!(f, stringify!(CommercialCentrifugeModel))
            }
            TableName::CommercialFreezeDryerLot => {
                write!(f, stringify!(CommercialFreezeDryerLot))
            }
            TableName::CommercialFreezeDryerModel => {
                write!(f, stringify!(CommercialFreezeDryerModel))
            }
            TableName::CommercialFreezerLot => {
                write!(f, stringify!(CommercialFreezerLot))
            }
            TableName::CommercialFreezerModel => {
                write!(f, stringify!(CommercialFreezerModel))
            }
            TableName::CommercialPackagingLot => {
                write!(f, stringify!(CommercialPackagingLot))
            }
            TableName::CommercialPackagingModel => {
                write!(f, stringify!(CommercialPackagingModel))
            }
            TableName::CommercialPipetteLot => {
                write!(f, stringify!(CommercialPipetteLot))
            }
            TableName::CommercialPipetteModel => {
                write!(f, stringify!(CommercialPipetteModel))
            }
            TableName::CommercialPipetteTipLot => {
                write!(f, stringify!(CommercialPipetteTipLot))
            }
            TableName::CommercialPipetteTipModel => {
                write!(f, stringify!(CommercialPipetteTipModel))
            }
            TableName::CommercialPositioningDeviceLot => {
                write!(f, stringify!(CommercialPositioningDeviceLot))
            }
            TableName::CommercialPositioningDeviceModel => {
                write!(f, stringify!(CommercialPositioningDeviceModel))
            }
            TableName::CommercialProductLot => {
                write!(f, stringify!(CommercialProductLot))
            }
            TableName::CommercialProduct => write!(f, stringify!(CommercialProduct)),
            TableName::CommercialVolumeMeasuringDeviceLot => {
                write!(f, stringify!(CommercialVolumeMeasuringDeviceLot))
            }
            TableName::CommercialVolumeMeasuringDeviceModel => {
                write!(f, stringify!(CommercialVolumeMeasuringDeviceModel))
            }
            TableName::CommercialWeighingDeviceLot => {
                write!(f, stringify!(CommercialWeighingDeviceLot))
            }
            TableName::CommercialWeighingDeviceModel => {
                write!(f, stringify!(CommercialWeighingDeviceModel))
            }
            TableName::ContainerCompatibilityRule => {
                write!(f, stringify!(ContainerCompatibilityRule))
            }
            TableName::ContainerModel => write!(f, stringify!(ContainerModel)),
            TableName::Container => write!(f, stringify!(Container)),
            TableName::Country => write!(f, stringify!(Country)),
            TableName::DigitalAssetModel => write!(f, stringify!(DigitalAssetModel)),
            TableName::DigitalAsset => write!(f, stringify!(DigitalAsset)),
            TableName::Document => write!(f, stringify!(Document)),
            TableName::EmailProvider => write!(f, stringify!(EmailProvider)),
            TableName::FreezeDryerModel => write!(f, stringify!(FreezeDryerModel)),
            TableName::FreezeDryer => write!(f, stringify!(FreezeDryer)),
            TableName::FreezerModel => write!(f, stringify!(FreezerModel)),
            TableName::Freezer => write!(f, stringify!(Freezer)),
            TableName::InstrumentState => write!(f, stringify!(InstrumentState)),
            TableName::LoginProvider => write!(f, stringify!(LoginProvider)),
            TableName::Material => write!(f, stringify!(Material)),
            TableName::NextProcedureTemplate => {
                write!(f, stringify!(NextProcedureTemplate))
            }
            TableName::ObservationSubject => write!(f, stringify!(ObservationSubject)),
            TableName::OrganismTaxon => write!(f, stringify!(OrganismTaxon)),
            TableName::Organism => write!(f, stringify!(Organism)),
            TableName::Organization => write!(f, stringify!(Organization)),
            TableName::PackagingModel => write!(f, stringify!(PackagingModel)),
            TableName::ParentProcedureTemplate => {
                write!(f, stringify!(ParentProcedureTemplate))
            }
            TableName::PermanenceCategory => write!(f, stringify!(PermanenceCategory)),
            TableName::PhoneModel => write!(f, stringify!(PhoneModel)),
            TableName::PhysicalAssetModel => write!(f, stringify!(PhysicalAssetModel)),
            TableName::PhysicalAsset => write!(f, stringify!(PhysicalAsset)),
            TableName::PipetteModel => write!(f, stringify!(PipetteModel)),
            TableName::PipetteTipModel => write!(f, stringify!(PipetteTipModel)),
            TableName::Pipette => write!(f, stringify!(Pipette)),
            TableName::PositioningDeviceModel => {
                write!(f, stringify!(PositioningDeviceModel))
            }
            TableName::PositioningDevice => write!(f, stringify!(PositioningDevice)),
            TableName::ProcedureAsset => write!(f, stringify!(ProcedureAsset)),
            TableName::ProcedureTemplateAssetModel => {
                write!(f, stringify!(ProcedureTemplateAssetModel))
            }
            TableName::ProjectState => write!(f, stringify!(ProjectState)),
            TableName::Project => write!(f, stringify!(Project)),
            TableName::Rank => write!(f, stringify!(Rank)),
            TableName::ReagentModel => write!(f, stringify!(ReagentModel)),
            TableName::Role => write!(f, stringify!(Role)),
            TableName::Room => write!(f, stringify!(Room)),
            TableName::SampleState => write!(f, stringify!(SampleState)),
            TableName::SharedProcedureTemplateAssetModel => {
                write!(f, stringify!(SharedProcedureTemplateAssetModel))
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
            TableName::Unit => write!(f, stringify!(Unit)),
            TableName::UserEmail => write!(f, stringify!(UserEmail)),
            TableName::UserOrganization => write!(f, stringify!(UserOrganization)),
            TableName::User => write!(f, stringify!(User)),
            TableName::VolumeMeasuringDeviceModel => {
                write!(f, stringify!(VolumeMeasuringDeviceModel))
            }
            TableName::VolumeMeasuringDevice => {
                write!(f, stringify!(VolumeMeasuringDevice))
            }
            TableName::VolumetricContainerModel => {
                write!(f, stringify!(VolumetricContainerModel))
            }
            TableName::VolumetricContainer => write!(f, stringify!(VolumetricContainer)),
            TableName::WeighingDeviceModel => write!(f, stringify!(WeighingDeviceModel)),
            TableName::WeighingDevice => write!(f, stringify!(WeighingDevice)),
        }
    }
}
