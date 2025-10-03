#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TableName {
    Address,
    AliquotingProcedureTemplate,
    AliquotingProcedure,
    AssetCompatibilityRule,
    AssetModelAncestor,
    AssetModel,
    Asset,
    BallMillMachineModel,
    BallMillMachine,
    BallMillProcedureTemplate,
    BallMillProcedure,
    BeadModel,
    Brand,
    CameraModel,
    Camera,
    CapModel,
    CappingProcedureTemplate,
    CappingProcedure,
    CentrifugeModel,
    CentrifugeProcedureTemplate,
    CentrifugeProcedure,
    Centrifuge,
    City,
    Color,
    CommercialBallMillMachineLot,
    CommercialBallMillMachineModel,
    CommercialBeadLot,
    CommercialBeadModel,
    CommercialCameraLot,
    CommercialCameraModel,
    CommercialCapLot,
    CommercialCapModel,
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
    DisposalProcedureTemplate,
    DisposalProcedure,
    EmailProvider,
    FractioningProcedureTemplate,
    FractioningProcedure,
    FreezeDryerModel,
    FreezeDryer,
    FreezeDryingProcedureTemplate,
    FreezeDryingProcedure,
    FreezerModel,
    Freezer,
    FreezingProcedureTemplate,
    FreezingProcedure,
    GeolocationProcedureTemplate,
    GeolocationProcedure,
    HarvestingProcedureTemplate,
    HarvestingProcedure,
    InstrumentState,
    LoginProvider,
    Material,
    NextProcedureTemplate,
    ObservationSubject,
    OrganismModel,
    OrganismTaxon,
    Organism,
    Organization,
    PackagingModel,
    PackagingProcedureTemplate,
    PackagingProcedure,
    ParentProcedureTemplate,
    PermanenceCategory,
    PhoneModel,
    PhotographProcedureTemplate,
    PhotographProcedure,
    Photograph,
    PhysicalAssetModel,
    PhysicalAsset,
    PipetteModel,
    PipetteTipModel,
    Pipette,
    PositioningDeviceModel,
    PositioningDevice,
    PouringProcedureTemplate,
    PouringProcedure,
    ProcedureAsset,
    ProcedureTemplateAssetModel,
    ProcedureTemplate,
    Procedure,
    ProjectState,
    Project,
    Rank,
    ReagentModel,
    Role,
    Room,
    SampleModel,
    SampleSourceModel,
    SampleSource,
    SampleState,
    Sample,
    SoilModel,
    Soil,
    SpatialRefSy,
    Spectrum,
    SpectraCollection,
    StorageProcedureTemplate,
    StorageProcedure,
    SupernatantProcedureTemplate,
    SupernatantProcedure,
    TaggingProcedureTemplate,
    TaggingProcedure,
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
    WeighingProcedureTemplate,
    WeighingProcedure,
}
impl web_common_traits::database::TableEnum for TableName {}
impl core::str::FromStr for TableName {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "addresses" => Ok(TableName::Address),
            "aliquoting_procedure_templates" => Ok(TableName::AliquotingProcedureTemplate),
            "aliquoting_procedures" => Ok(TableName::AliquotingProcedure),
            "asset_compatibility_rules" => Ok(TableName::AssetCompatibilityRule),
            "asset_model_ancestors" => Ok(TableName::AssetModelAncestor),
            "asset_models" => Ok(TableName::AssetModel),
            "assets" => Ok(TableName::Asset),
            "ball_mill_machine_models" => Ok(TableName::BallMillMachineModel),
            "ball_mill_machines" => Ok(TableName::BallMillMachine),
            "ball_mill_procedure_templates" => Ok(TableName::BallMillProcedureTemplate),
            "ball_mill_procedures" => Ok(TableName::BallMillProcedure),
            "bead_models" => Ok(TableName::BeadModel),
            "brands" => Ok(TableName::Brand),
            "camera_models" => Ok(TableName::CameraModel),
            "cameras" => Ok(TableName::Camera),
            "cap_models" => Ok(TableName::CapModel),
            "capping_procedure_templates" => Ok(TableName::CappingProcedureTemplate),
            "capping_procedures" => Ok(TableName::CappingProcedure),
            "centrifuge_models" => Ok(TableName::CentrifugeModel),
            "centrifuge_procedure_templates" => Ok(TableName::CentrifugeProcedureTemplate),
            "centrifuge_procedures" => Ok(TableName::CentrifugeProcedure),
            "centrifuges" => Ok(TableName::Centrifuge),
            "cities" => Ok(TableName::City),
            "colors" => Ok(TableName::Color),
            "commercial_ball_mill_machine_lots" => Ok(TableName::CommercialBallMillMachineLot),
            "commercial_ball_mill_machine_models" => Ok(TableName::CommercialBallMillMachineModel),
            "commercial_bead_lots" => Ok(TableName::CommercialBeadLot),
            "commercial_bead_models" => Ok(TableName::CommercialBeadModel),
            "commercial_camera_lots" => Ok(TableName::CommercialCameraLot),
            "commercial_camera_models" => Ok(TableName::CommercialCameraModel),
            "commercial_cap_lots" => Ok(TableName::CommercialCapLot),
            "commercial_cap_models" => Ok(TableName::CommercialCapModel),
            "commercial_centrifuge_lots" => Ok(TableName::CommercialCentrifugeLot),
            "commercial_centrifuge_models" => Ok(TableName::CommercialCentrifugeModel),
            "commercial_freeze_dryer_lots" => Ok(TableName::CommercialFreezeDryerLot),
            "commercial_freeze_dryer_models" => Ok(TableName::CommercialFreezeDryerModel),
            "commercial_freezer_lots" => Ok(TableName::CommercialFreezerLot),
            "commercial_freezer_models" => Ok(TableName::CommercialFreezerModel),
            "commercial_packaging_lots" => Ok(TableName::CommercialPackagingLot),
            "commercial_packaging_models" => Ok(TableName::CommercialPackagingModel),
            "commercial_pipette_lots" => Ok(TableName::CommercialPipetteLot),
            "commercial_pipette_models" => Ok(TableName::CommercialPipetteModel),
            "commercial_pipette_tip_lots" => Ok(TableName::CommercialPipetteTipLot),
            "commercial_pipette_tip_models" => Ok(TableName::CommercialPipetteTipModel),
            "commercial_positioning_device_lots" => Ok(TableName::CommercialPositioningDeviceLot),
            "commercial_positioning_device_models" => {
                Ok(TableName::CommercialPositioningDeviceModel)
            }
            "commercial_product_lots" => Ok(TableName::CommercialProductLot),
            "commercial_products" => Ok(TableName::CommercialProduct),
            "commercial_volume_measuring_device_lots" => {
                Ok(TableName::CommercialVolumeMeasuringDeviceLot)
            }
            "commercial_volume_measuring_device_models" => {
                Ok(TableName::CommercialVolumeMeasuringDeviceModel)
            }
            "commercial_weighing_device_lots" => Ok(TableName::CommercialWeighingDeviceLot),
            "commercial_weighing_device_models" => Ok(TableName::CommercialWeighingDeviceModel),
            "container_compatibility_rules" => Ok(TableName::ContainerCompatibilityRule),
            "container_models" => Ok(TableName::ContainerModel),
            "containers" => Ok(TableName::Container),
            "countries" => Ok(TableName::Country),
            "digital_asset_models" => Ok(TableName::DigitalAssetModel),
            "digital_assets" => Ok(TableName::DigitalAsset),
            "disposal_procedure_templates" => Ok(TableName::DisposalProcedureTemplate),
            "disposal_procedures" => Ok(TableName::DisposalProcedure),
            "email_providers" => Ok(TableName::EmailProvider),
            "fractioning_procedure_templates" => Ok(TableName::FractioningProcedureTemplate),
            "fractioning_procedures" => Ok(TableName::FractioningProcedure),
            "freeze_dryer_models" => Ok(TableName::FreezeDryerModel),
            "freeze_dryers" => Ok(TableName::FreezeDryer),
            "freeze_drying_procedure_templates" => Ok(TableName::FreezeDryingProcedureTemplate),
            "freeze_drying_procedures" => Ok(TableName::FreezeDryingProcedure),
            "freezer_models" => Ok(TableName::FreezerModel),
            "freezers" => Ok(TableName::Freezer),
            "freezing_procedure_templates" => Ok(TableName::FreezingProcedureTemplate),
            "freezing_procedures" => Ok(TableName::FreezingProcedure),
            "geolocation_procedure_templates" => Ok(TableName::GeolocationProcedureTemplate),
            "geolocation_procedures" => Ok(TableName::GeolocationProcedure),
            "harvesting_procedure_templates" => Ok(TableName::HarvestingProcedureTemplate),
            "harvesting_procedures" => Ok(TableName::HarvestingProcedure),
            "instrument_states" => Ok(TableName::InstrumentState),
            "login_providers" => Ok(TableName::LoginProvider),
            "materials" => Ok(TableName::Material),
            "next_procedure_templates" => Ok(TableName::NextProcedureTemplate),
            "observation_subjects" => Ok(TableName::ObservationSubject),
            "organism_models" => Ok(TableName::OrganismModel),
            "organism_taxa" => Ok(TableName::OrganismTaxon),
            "organisms" => Ok(TableName::Organism),
            "organizations" => Ok(TableName::Organization),
            "packaging_models" => Ok(TableName::PackagingModel),
            "packaging_procedure_templates" => Ok(TableName::PackagingProcedureTemplate),
            "packaging_procedures" => Ok(TableName::PackagingProcedure),
            "parent_procedure_templates" => Ok(TableName::ParentProcedureTemplate),
            "permanence_categories" => Ok(TableName::PermanenceCategory),
            "phone_models" => Ok(TableName::PhoneModel),
            "photograph_procedure_templates" => Ok(TableName::PhotographProcedureTemplate),
            "photograph_procedures" => Ok(TableName::PhotographProcedure),
            "photographs" => Ok(TableName::Photograph),
            "physical_asset_models" => Ok(TableName::PhysicalAssetModel),
            "physical_assets" => Ok(TableName::PhysicalAsset),
            "pipette_models" => Ok(TableName::PipetteModel),
            "pipette_tip_models" => Ok(TableName::PipetteTipModel),
            "pipettes" => Ok(TableName::Pipette),
            "positioning_device_models" => Ok(TableName::PositioningDeviceModel),
            "positioning_devices" => Ok(TableName::PositioningDevice),
            "pouring_procedure_templates" => Ok(TableName::PouringProcedureTemplate),
            "pouring_procedures" => Ok(TableName::PouringProcedure),
            "procedure_assets" => Ok(TableName::ProcedureAsset),
            "procedure_template_asset_models" => Ok(TableName::ProcedureTemplateAssetModel),
            "procedure_templates" => Ok(TableName::ProcedureTemplate),
            "procedures" => Ok(TableName::Procedure),
            "project_states" => Ok(TableName::ProjectState),
            "projects" => Ok(TableName::Project),
            "ranks" => Ok(TableName::Rank),
            "reagent_models" => Ok(TableName::ReagentModel),
            "roles" => Ok(TableName::Role),
            "rooms" => Ok(TableName::Room),
            "sample_models" => Ok(TableName::SampleModel),
            "sample_source_models" => Ok(TableName::SampleSourceModel),
            "sample_sources" => Ok(TableName::SampleSource),
            "sample_states" => Ok(TableName::SampleState),
            "samples" => Ok(TableName::Sample),
            "soil_models" => Ok(TableName::SoilModel),
            "soils" => Ok(TableName::Soil),
            "spatial_ref_sys" => Ok(TableName::SpatialRefSy),
            "spectra" => Ok(TableName::Spectrum),
            "spectra_collections" => Ok(TableName::SpectraCollection),
            "storage_procedure_templates" => Ok(TableName::StorageProcedureTemplate),
            "storage_procedures" => Ok(TableName::StorageProcedure),
            "supernatant_procedure_templates" => Ok(TableName::SupernatantProcedureTemplate),
            "supernatant_procedures" => Ok(TableName::SupernatantProcedure),
            "tagging_procedure_templates" => Ok(TableName::TaggingProcedureTemplate),
            "tagging_procedures" => Ok(TableName::TaggingProcedure),
            "taxa" => Ok(TableName::Taxon),
            "team_members" => Ok(TableName::TeamMember),
            "team_projects" => Ok(TableName::TeamProject),
            "team_states" => Ok(TableName::TeamState),
            "teams" => Ok(TableName::Team),
            "temporary_user" => Ok(TableName::TemporaryUser),
            "units" => Ok(TableName::Unit),
            "user_emails" => Ok(TableName::UserEmail),
            "user_organizations" => Ok(TableName::UserOrganization),
            "users" => Ok(TableName::User),
            "volume_measuring_device_models" => Ok(TableName::VolumeMeasuringDeviceModel),
            "volume_measuring_devices" => Ok(TableName::VolumeMeasuringDevice),
            "volumetric_container_models" => Ok(TableName::VolumetricContainerModel),
            "volumetric_containers" => Ok(TableName::VolumetricContainer),
            "weighing_device_models" => Ok(TableName::WeighingDeviceModel),
            "weighing_devices" => Ok(TableName::WeighingDevice),
            "weighing_procedure_templates" => Ok(TableName::WeighingProcedureTemplate),
            "weighing_procedures" => Ok(TableName::WeighingProcedure),
            _ => Err(format!("Unknown table name: {}", s)),
        }
    }
}
impl core::fmt::Display for TableName {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            TableName::Address => write!(f, "addresses"),
            TableName::AliquotingProcedureTemplate => {
                write!(f, "aliquoting_procedure_templates")
            }
            TableName::AliquotingProcedure => write!(f, "aliquoting_procedures"),
            TableName::AssetCompatibilityRule => write!(f, "asset_compatibility_rules"),
            TableName::AssetModelAncestor => write!(f, "asset_model_ancestors"),
            TableName::AssetModel => write!(f, "asset_models"),
            TableName::Asset => write!(f, "assets"),
            TableName::BallMillMachineModel => write!(f, "ball_mill_machine_models"),
            TableName::BallMillMachine => write!(f, "ball_mill_machines"),
            TableName::BallMillProcedureTemplate => {
                write!(f, "ball_mill_procedure_templates")
            }
            TableName::BallMillProcedure => write!(f, "ball_mill_procedures"),
            TableName::BeadModel => write!(f, "bead_models"),
            TableName::Brand => write!(f, "brands"),
            TableName::CameraModel => write!(f, "camera_models"),
            TableName::Camera => write!(f, "cameras"),
            TableName::CapModel => write!(f, "cap_models"),
            TableName::CappingProcedureTemplate => {
                write!(f, "capping_procedure_templates")
            }
            TableName::CappingProcedure => write!(f, "capping_procedures"),
            TableName::CentrifugeModel => write!(f, "centrifuge_models"),
            TableName::CentrifugeProcedureTemplate => {
                write!(f, "centrifuge_procedure_templates")
            }
            TableName::CentrifugeProcedure => write!(f, "centrifuge_procedures"),
            TableName::Centrifuge => write!(f, "centrifuges"),
            TableName::City => write!(f, "cities"),
            TableName::Color => write!(f, "colors"),
            TableName::CommercialBallMillMachineLot => {
                write!(f, "commercial_ball_mill_machine_lots")
            }
            TableName::CommercialBallMillMachineModel => {
                write!(f, "commercial_ball_mill_machine_models")
            }
            TableName::CommercialBeadLot => write!(f, "commercial_bead_lots"),
            TableName::CommercialBeadModel => write!(f, "commercial_bead_models"),
            TableName::CommercialCameraLot => write!(f, "commercial_camera_lots"),
            TableName::CommercialCameraModel => write!(f, "commercial_camera_models"),
            TableName::CommercialCapLot => write!(f, "commercial_cap_lots"),
            TableName::CommercialCapModel => write!(f, "commercial_cap_models"),
            TableName::CommercialCentrifugeLot => write!(f, "commercial_centrifuge_lots"),
            TableName::CommercialCentrifugeModel => {
                write!(f, "commercial_centrifuge_models")
            }
            TableName::CommercialFreezeDryerLot => {
                write!(f, "commercial_freeze_dryer_lots")
            }
            TableName::CommercialFreezeDryerModel => {
                write!(f, "commercial_freeze_dryer_models")
            }
            TableName::CommercialFreezerLot => write!(f, "commercial_freezer_lots"),
            TableName::CommercialFreezerModel => write!(f, "commercial_freezer_models"),
            TableName::CommercialPackagingLot => write!(f, "commercial_packaging_lots"),
            TableName::CommercialPackagingModel => {
                write!(f, "commercial_packaging_models")
            }
            TableName::CommercialPipetteLot => write!(f, "commercial_pipette_lots"),
            TableName::CommercialPipetteModel => write!(f, "commercial_pipette_models"),
            TableName::CommercialPipetteTipLot => {
                write!(f, "commercial_pipette_tip_lots")
            }
            TableName::CommercialPipetteTipModel => {
                write!(f, "commercial_pipette_tip_models")
            }
            TableName::CommercialPositioningDeviceLot => {
                write!(f, "commercial_positioning_device_lots")
            }
            TableName::CommercialPositioningDeviceModel => {
                write!(f, "commercial_positioning_device_models")
            }
            TableName::CommercialProductLot => write!(f, "commercial_product_lots"),
            TableName::CommercialProduct => write!(f, "commercial_products"),
            TableName::CommercialVolumeMeasuringDeviceLot => {
                write!(f, "commercial_volume_measuring_device_lots")
            }
            TableName::CommercialVolumeMeasuringDeviceModel => {
                write!(f, "commercial_volume_measuring_device_models")
            }
            TableName::CommercialWeighingDeviceLot => {
                write!(f, "commercial_weighing_device_lots")
            }
            TableName::CommercialWeighingDeviceModel => {
                write!(f, "commercial_weighing_device_models")
            }
            TableName::ContainerCompatibilityRule => {
                write!(f, "container_compatibility_rules")
            }
            TableName::ContainerModel => write!(f, "container_models"),
            TableName::Container => write!(f, "containers"),
            TableName::Country => write!(f, "countries"),
            TableName::DigitalAssetModel => write!(f, "digital_asset_models"),
            TableName::DigitalAsset => write!(f, "digital_assets"),
            TableName::DisposalProcedureTemplate => {
                write!(f, "disposal_procedure_templates")
            }
            TableName::DisposalProcedure => write!(f, "disposal_procedures"),
            TableName::EmailProvider => write!(f, "email_providers"),
            TableName::FractioningProcedureTemplate => {
                write!(f, "fractioning_procedure_templates")
            }
            TableName::FractioningProcedure => write!(f, "fractioning_procedures"),
            TableName::FreezeDryerModel => write!(f, "freeze_dryer_models"),
            TableName::FreezeDryer => write!(f, "freeze_dryers"),
            TableName::FreezeDryingProcedureTemplate => {
                write!(f, "freeze_drying_procedure_templates")
            }
            TableName::FreezeDryingProcedure => write!(f, "freeze_drying_procedures"),
            TableName::FreezerModel => write!(f, "freezer_models"),
            TableName::Freezer => write!(f, "freezers"),
            TableName::FreezingProcedureTemplate => {
                write!(f, "freezing_procedure_templates")
            }
            TableName::FreezingProcedure => write!(f, "freezing_procedures"),
            TableName::GeolocationProcedureTemplate => {
                write!(f, "geolocation_procedure_templates")
            }
            TableName::GeolocationProcedure => write!(f, "geolocation_procedures"),
            TableName::HarvestingProcedureTemplate => {
                write!(f, "harvesting_procedure_templates")
            }
            TableName::HarvestingProcedure => write!(f, "harvesting_procedures"),
            TableName::InstrumentState => write!(f, "instrument_states"),
            TableName::LoginProvider => write!(f, "login_providers"),
            TableName::Material => write!(f, "materials"),
            TableName::NextProcedureTemplate => write!(f, "next_procedure_templates"),
            TableName::ObservationSubject => write!(f, "observation_subjects"),
            TableName::OrganismModel => write!(f, "organism_models"),
            TableName::OrganismTaxon => write!(f, "organism_taxa"),
            TableName::Organism => write!(f, "organisms"),
            TableName::Organization => write!(f, "organizations"),
            TableName::PackagingModel => write!(f, "packaging_models"),
            TableName::PackagingProcedureTemplate => {
                write!(f, "packaging_procedure_templates")
            }
            TableName::PackagingProcedure => write!(f, "packaging_procedures"),
            TableName::ParentProcedureTemplate => write!(f, "parent_procedure_templates"),
            TableName::PermanenceCategory => write!(f, "permanence_categories"),
            TableName::PhoneModel => write!(f, "phone_models"),
            TableName::PhotographProcedureTemplate => {
                write!(f, "photograph_procedure_templates")
            }
            TableName::PhotographProcedure => write!(f, "photograph_procedures"),
            TableName::Photograph => write!(f, "photographs"),
            TableName::PhysicalAssetModel => write!(f, "physical_asset_models"),
            TableName::PhysicalAsset => write!(f, "physical_assets"),
            TableName::PipetteModel => write!(f, "pipette_models"),
            TableName::PipetteTipModel => write!(f, "pipette_tip_models"),
            TableName::Pipette => write!(f, "pipettes"),
            TableName::PositioningDeviceModel => write!(f, "positioning_device_models"),
            TableName::PositioningDevice => write!(f, "positioning_devices"),
            TableName::PouringProcedureTemplate => {
                write!(f, "pouring_procedure_templates")
            }
            TableName::PouringProcedure => write!(f, "pouring_procedures"),
            TableName::ProcedureAsset => write!(f, "procedure_assets"),
            TableName::ProcedureTemplateAssetModel => {
                write!(f, "procedure_template_asset_models")
            }
            TableName::ProcedureTemplate => write!(f, "procedure_templates"),
            TableName::Procedure => write!(f, "procedures"),
            TableName::ProjectState => write!(f, "project_states"),
            TableName::Project => write!(f, "projects"),
            TableName::Rank => write!(f, "ranks"),
            TableName::ReagentModel => write!(f, "reagent_models"),
            TableName::Role => write!(f, "roles"),
            TableName::Room => write!(f, "rooms"),
            TableName::SampleModel => write!(f, "sample_models"),
            TableName::SampleSourceModel => write!(f, "sample_source_models"),
            TableName::SampleSource => write!(f, "sample_sources"),
            TableName::SampleState => write!(f, "sample_states"),
            TableName::Sample => write!(f, "samples"),
            TableName::SoilModel => write!(f, "soil_models"),
            TableName::Soil => write!(f, "soils"),
            TableName::SpatialRefSy => write!(f, "spatial_ref_sys"),
            TableName::Spectrum => write!(f, "spectra"),
            TableName::SpectraCollection => write!(f, "spectra_collections"),
            TableName::StorageProcedureTemplate => {
                write!(f, "storage_procedure_templates")
            }
            TableName::StorageProcedure => write!(f, "storage_procedures"),
            TableName::SupernatantProcedureTemplate => {
                write!(f, "supernatant_procedure_templates")
            }
            TableName::SupernatantProcedure => write!(f, "supernatant_procedures"),
            TableName::TaggingProcedureTemplate => {
                write!(f, "tagging_procedure_templates")
            }
            TableName::TaggingProcedure => write!(f, "tagging_procedures"),
            TableName::Taxon => write!(f, "taxa"),
            TableName::TeamMember => write!(f, "team_members"),
            TableName::TeamProject => write!(f, "team_projects"),
            TableName::TeamState => write!(f, "team_states"),
            TableName::Team => write!(f, "teams"),
            TableName::TemporaryUser => write!(f, "temporary_user"),
            TableName::Unit => write!(f, "units"),
            TableName::UserEmail => write!(f, "user_emails"),
            TableName::UserOrganization => write!(f, "user_organizations"),
            TableName::User => write!(f, "users"),
            TableName::VolumeMeasuringDeviceModel => {
                write!(f, "volume_measuring_device_models")
            }
            TableName::VolumeMeasuringDevice => write!(f, "volume_measuring_devices"),
            TableName::VolumetricContainerModel => {
                write!(f, "volumetric_container_models")
            }
            TableName::VolumetricContainer => write!(f, "volumetric_containers"),
            TableName::WeighingDeviceModel => write!(f, "weighing_device_models"),
            TableName::WeighingDevice => write!(f, "weighing_devices"),
            TableName::WeighingProcedureTemplate => {
                write!(f, "weighing_procedure_templates")
            }
            TableName::WeighingProcedure => write!(f, "weighing_procedures"),
        }
    }
}
