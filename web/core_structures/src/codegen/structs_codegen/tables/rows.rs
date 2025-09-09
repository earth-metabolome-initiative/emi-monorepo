mod addresses;
mod aliquoting_procedure_templates;
mod aliquoting_procedures;
mod asset_compatibility_rules;
mod asset_model_ancestors;
mod asset_models;
mod assets;
mod ball_mill_machine_models;
mod ball_mill_machines;
mod ball_mill_procedure_templates;
mod ball_mill_procedures;
mod bead_models;
mod bounded_read_dispatch;
mod brands;
mod camera_models;
mod cameras;
mod cap_models;
mod capping_procedure_templates;
mod capping_procedures;
mod centrifuge_models;
mod centrifuge_procedure_templates;
mod centrifuge_procedures;
mod centrifuges;
mod cities;
mod colors;
mod commercial_ball_mill_machine_lots;
mod commercial_ball_mill_machine_models;
mod commercial_bead_lots;
mod commercial_bead_models;
mod commercial_camera_lots;
mod commercial_camera_models;
mod commercial_cap_lots;
mod commercial_cap_models;
mod commercial_centrifuge_lots;
mod commercial_centrifuge_models;
mod commercial_freeze_dryer_lots;
mod commercial_freeze_dryer_models;
mod commercial_freezer_lots;
mod commercial_freezer_models;
mod commercial_packaging_lots;
mod commercial_packaging_models;
mod commercial_pipette_lots;
mod commercial_pipette_models;
mod commercial_pipette_tip_lots;
mod commercial_pipette_tip_models;
mod commercial_positioning_device_lots;
mod commercial_positioning_device_models;
mod commercial_product_lots;
mod commercial_products;
mod commercial_volume_measuring_device_lots;
mod commercial_volume_measuring_device_models;
mod commercial_weighing_device_lots;
mod commercial_weighing_device_models;
mod container_compatibility_rules;
mod container_models;
mod containers;
mod countries;
mod digital_asset_models;
mod digital_assets;
mod disposal_procedure_templates;
mod disposal_procedures;
mod email_providers;
mod fractioning_procedure_templates;
mod fractioning_procedures;
mod freeze_dryer_models;
mod freeze_dryers;
mod freeze_drying_procedure_templates;
mod freeze_drying_procedures;
mod freezer_models;
mod freezers;
mod freezing_procedure_templates;
mod freezing_procedures;
mod geolocation_procedure_templates;
mod geolocation_procedures;
mod instrument_states;
mod into_iter;
mod len;
mod login_providers;
mod materials;
mod next_procedure_templates;
mod observation_subjects;
mod organism_taxa;
mod organisms;
mod organizations;
mod packaging_models;
mod packaging_procedure_templates;
mod packaging_procedures;
mod parent_procedure_templates;
mod permanence_categories;
mod phone_models;
mod photograph_procedure_templates;
mod photograph_procedures;
mod photographs;
mod physical_asset_models;
mod physical_assets;
mod pipette_models;
mod pipette_tip_models;
mod pipettes;
mod positioning_device_models;
mod positioning_devices;
mod pouring_procedure_templates;
mod pouring_procedures;
mod procedure_assets;
mod procedure_template_asset_models;
mod procedure_templates;
mod procedures;
mod project_states;
mod projects;
mod ranks;
mod reagent_models;
mod roles;
mod rooms;
mod sample_models;
mod sample_source_models;
mod sample_sources;
mod sample_states;
mod samples;
mod spatial_ref_sys;
mod spectra;
mod spectra_collections;
mod storage_procedure_templates;
mod storage_procedures;
mod supernatant_procedure_templates;
mod supernatant_procedures;
mod tabular;
mod taxa;
mod team_members;
mod team_projects;
mod team_states;
mod teams;
mod temporary_user;
mod units;
mod user_emails;
mod user_organizations;
mod users;
mod volume_measuring_device_models;
mod volume_measuring_devices;
mod volumetric_container_models;
mod volumetric_containers;
mod weighing_device_models;
mod weighing_devices;
mod weighing_procedure_templates;
mod weighing_procedures;
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Rows {
    Address(Vec<crate::codegen::structs_codegen::tables::addresses::Address>),
    AliquotingProcedureTemplate(
        Vec<
            crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate,
        >,
    ),
    AliquotingProcedure(
        Vec<
            crate::codegen::structs_codegen::tables::aliquoting_procedures::AliquotingProcedure,
        >,
    ),
    AssetCompatibilityRule(
        Vec<
            crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule,
        >,
    ),
    AssetModelAncestor(
        Vec<
            crate::codegen::structs_codegen::tables::asset_model_ancestors::AssetModelAncestor,
        >,
    ),
    AssetModel(Vec<crate::codegen::structs_codegen::tables::asset_models::AssetModel>),
    Asset(Vec<crate::codegen::structs_codegen::tables::assets::Asset>),
    BallMillMachineModel(
        Vec<
            crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel,
        >,
    ),
    BallMillMachine(
        Vec<crate::codegen::structs_codegen::tables::ball_mill_machines::BallMillMachine>,
    ),
    BallMillProcedureTemplate(
        Vec<
            crate::codegen::structs_codegen::tables::ball_mill_procedure_templates::BallMillProcedureTemplate,
        >,
    ),
    BallMillProcedure(
        Vec<
            crate::codegen::structs_codegen::tables::ball_mill_procedures::BallMillProcedure,
        >,
    ),
    BeadModel(Vec<crate::codegen::structs_codegen::tables::bead_models::BeadModel>),
    Brand(Vec<crate::codegen::structs_codegen::tables::brands::Brand>),
    CameraModel(
        Vec<crate::codegen::structs_codegen::tables::camera_models::CameraModel>,
    ),
    Camera(Vec<crate::codegen::structs_codegen::tables::cameras::Camera>),
    CapModel(Vec<crate::codegen::structs_codegen::tables::cap_models::CapModel>),
    CappingProcedureTemplate(
        Vec<
            crate::codegen::structs_codegen::tables::capping_procedure_templates::CappingProcedureTemplate,
        >,
    ),
    CappingProcedure(
        Vec<
            crate::codegen::structs_codegen::tables::capping_procedures::CappingProcedure,
        >,
    ),
    CentrifugeModel(
        Vec<crate::codegen::structs_codegen::tables::centrifuge_models::CentrifugeModel>,
    ),
    CentrifugeProcedureTemplate(
        Vec<
            crate::codegen::structs_codegen::tables::centrifuge_procedure_templates::CentrifugeProcedureTemplate,
        >,
    ),
    CentrifugeProcedure(
        Vec<
            crate::codegen::structs_codegen::tables::centrifuge_procedures::CentrifugeProcedure,
        >,
    ),
    Centrifuge(Vec<crate::codegen::structs_codegen::tables::centrifuges::Centrifuge>),
    City(Vec<crate::codegen::structs_codegen::tables::cities::City>),
    Color(Vec<crate::codegen::structs_codegen::tables::colors::Color>),
    CommercialBallMillMachineLot(
        Vec<
            crate::codegen::structs_codegen::tables::commercial_ball_mill_machine_lots::CommercialBallMillMachineLot,
        >,
    ),
    CommercialBallMillMachineModel(
        Vec<
            crate::codegen::structs_codegen::tables::commercial_ball_mill_machine_models::CommercialBallMillMachineModel,
        >,
    ),
    CommercialBeadLot(
        Vec<
            crate::codegen::structs_codegen::tables::commercial_bead_lots::CommercialBeadLot,
        >,
    ),
    CommercialBeadModel(
        Vec<
            crate::codegen::structs_codegen::tables::commercial_bead_models::CommercialBeadModel,
        >,
    ),
    CommercialCameraLot(
        Vec<
            crate::codegen::structs_codegen::tables::commercial_camera_lots::CommercialCameraLot,
        >,
    ),
    CommercialCameraModel(
        Vec<
            crate::codegen::structs_codegen::tables::commercial_camera_models::CommercialCameraModel,
        >,
    ),
    CommercialCapLot(
        Vec<
            crate::codegen::structs_codegen::tables::commercial_cap_lots::CommercialCapLot,
        >,
    ),
    CommercialCapModel(
        Vec<
            crate::codegen::structs_codegen::tables::commercial_cap_models::CommercialCapModel,
        >,
    ),
    CommercialCentrifugeLot(
        Vec<
            crate::codegen::structs_codegen::tables::commercial_centrifuge_lots::CommercialCentrifugeLot,
        >,
    ),
    CommercialCentrifugeModel(
        Vec<
            crate::codegen::structs_codegen::tables::commercial_centrifuge_models::CommercialCentrifugeModel,
        >,
    ),
    CommercialFreezeDryerLot(
        Vec<
            crate::codegen::structs_codegen::tables::commercial_freeze_dryer_lots::CommercialFreezeDryerLot,
        >,
    ),
    CommercialFreezeDryerModel(
        Vec<
            crate::codegen::structs_codegen::tables::commercial_freeze_dryer_models::CommercialFreezeDryerModel,
        >,
    ),
    CommercialFreezerLot(
        Vec<
            crate::codegen::structs_codegen::tables::commercial_freezer_lots::CommercialFreezerLot,
        >,
    ),
    CommercialFreezerModel(
        Vec<
            crate::codegen::structs_codegen::tables::commercial_freezer_models::CommercialFreezerModel,
        >,
    ),
    CommercialPackagingLot(
        Vec<
            crate::codegen::structs_codegen::tables::commercial_packaging_lots::CommercialPackagingLot,
        >,
    ),
    CommercialPackagingModel(
        Vec<
            crate::codegen::structs_codegen::tables::commercial_packaging_models::CommercialPackagingModel,
        >,
    ),
    CommercialPipetteLot(
        Vec<
            crate::codegen::structs_codegen::tables::commercial_pipette_lots::CommercialPipetteLot,
        >,
    ),
    CommercialPipetteModel(
        Vec<
            crate::codegen::structs_codegen::tables::commercial_pipette_models::CommercialPipetteModel,
        >,
    ),
    CommercialPipetteTipLot(
        Vec<
            crate::codegen::structs_codegen::tables::commercial_pipette_tip_lots::CommercialPipetteTipLot,
        >,
    ),
    CommercialPipetteTipModel(
        Vec<
            crate::codegen::structs_codegen::tables::commercial_pipette_tip_models::CommercialPipetteTipModel,
        >,
    ),
    CommercialPositioningDeviceLot(
        Vec<
            crate::codegen::structs_codegen::tables::commercial_positioning_device_lots::CommercialPositioningDeviceLot,
        >,
    ),
    CommercialPositioningDeviceModel(
        Vec<
            crate::codegen::structs_codegen::tables::commercial_positioning_device_models::CommercialPositioningDeviceModel,
        >,
    ),
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
    CommercialVolumeMeasuringDeviceLot(
        Vec<
            crate::codegen::structs_codegen::tables::commercial_volume_measuring_device_lots::CommercialVolumeMeasuringDeviceLot,
        >,
    ),
    CommercialVolumeMeasuringDeviceModel(
        Vec<
            crate::codegen::structs_codegen::tables::commercial_volume_measuring_device_models::CommercialVolumeMeasuringDeviceModel,
        >,
    ),
    CommercialWeighingDeviceLot(
        Vec<
            crate::codegen::structs_codegen::tables::commercial_weighing_device_lots::CommercialWeighingDeviceLot,
        >,
    ),
    CommercialWeighingDeviceModel(
        Vec<
            crate::codegen::structs_codegen::tables::commercial_weighing_device_models::CommercialWeighingDeviceModel,
        >,
    ),
    ContainerCompatibilityRule(
        Vec<
            crate::codegen::structs_codegen::tables::container_compatibility_rules::ContainerCompatibilityRule,
        >,
    ),
    ContainerModel(
        Vec<crate::codegen::structs_codegen::tables::container_models::ContainerModel>,
    ),
    Container(Vec<crate::codegen::structs_codegen::tables::containers::Container>),
    Country(Vec<crate::codegen::structs_codegen::tables::countries::Country>),
    DigitalAssetModel(
        Vec<
            crate::codegen::structs_codegen::tables::digital_asset_models::DigitalAssetModel,
        >,
    ),
    DigitalAsset(
        Vec<crate::codegen::structs_codegen::tables::digital_assets::DigitalAsset>,
    ),
    DisposalProcedureTemplate(
        Vec<
            crate::codegen::structs_codegen::tables::disposal_procedure_templates::DisposalProcedureTemplate,
        >,
    ),
    DisposalProcedure(
        Vec<
            crate::codegen::structs_codegen::tables::disposal_procedures::DisposalProcedure,
        >,
    ),
    EmailProvider(
        Vec<crate::codegen::structs_codegen::tables::email_providers::EmailProvider>,
    ),
    FractioningProcedureTemplate(
        Vec<
            crate::codegen::structs_codegen::tables::fractioning_procedure_templates::FractioningProcedureTemplate,
        >,
    ),
    FractioningProcedure(
        Vec<
            crate::codegen::structs_codegen::tables::fractioning_procedures::FractioningProcedure,
        >,
    ),
    FreezeDryerModel(
        Vec<
            crate::codegen::structs_codegen::tables::freeze_dryer_models::FreezeDryerModel,
        >,
    ),
    FreezeDryer(
        Vec<crate::codegen::structs_codegen::tables::freeze_dryers::FreezeDryer>,
    ),
    FreezeDryingProcedureTemplate(
        Vec<
            crate::codegen::structs_codegen::tables::freeze_drying_procedure_templates::FreezeDryingProcedureTemplate,
        >,
    ),
    FreezeDryingProcedure(
        Vec<
            crate::codegen::structs_codegen::tables::freeze_drying_procedures::FreezeDryingProcedure,
        >,
    ),
    FreezerModel(
        Vec<crate::codegen::structs_codegen::tables::freezer_models::FreezerModel>,
    ),
    Freezer(Vec<crate::codegen::structs_codegen::tables::freezers::Freezer>),
    FreezingProcedureTemplate(
        Vec<
            crate::codegen::structs_codegen::tables::freezing_procedure_templates::FreezingProcedureTemplate,
        >,
    ),
    FreezingProcedure(
        Vec<
            crate::codegen::structs_codegen::tables::freezing_procedures::FreezingProcedure,
        >,
    ),
    GeolocationProcedureTemplate(
        Vec<
            crate::codegen::structs_codegen::tables::geolocation_procedure_templates::GeolocationProcedureTemplate,
        >,
    ),
    GeolocationProcedure(
        Vec<
            crate::codegen::structs_codegen::tables::geolocation_procedures::GeolocationProcedure,
        >,
    ),
    InstrumentState(
        Vec<crate::codegen::structs_codegen::tables::instrument_states::InstrumentState>,
    ),
    LoginProvider(
        Vec<crate::codegen::structs_codegen::tables::login_providers::LoginProvider>,
    ),
    Material(Vec<crate::codegen::structs_codegen::tables::materials::Material>),
    NextProcedureTemplate(
        Vec<
            crate::codegen::structs_codegen::tables::next_procedure_templates::NextProcedureTemplate,
        >,
    ),
    ObservationSubject(
        Vec<
            crate::codegen::structs_codegen::tables::observation_subjects::ObservationSubject,
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
    PackagingProcedureTemplate(
        Vec<
            crate::codegen::structs_codegen::tables::packaging_procedure_templates::PackagingProcedureTemplate,
        >,
    ),
    PackagingProcedure(
        Vec<
            crate::codegen::structs_codegen::tables::packaging_procedures::PackagingProcedure,
        >,
    ),
    ParentProcedureTemplate(
        Vec<
            crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate,
        >,
    ),
    PermanenceCategory(
        Vec<
            crate::codegen::structs_codegen::tables::permanence_categories::PermanenceCategory,
        >,
    ),
    PhoneModel(Vec<crate::codegen::structs_codegen::tables::phone_models::PhoneModel>),
    PhotographProcedureTemplate(
        Vec<
            crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate,
        >,
    ),
    PhotographProcedure(
        Vec<
            crate::codegen::structs_codegen::tables::photograph_procedures::PhotographProcedure,
        >,
    ),
    Photograph(Vec<crate::codegen::structs_codegen::tables::photographs::Photograph>),
    PhysicalAssetModel(
        Vec<
            crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel,
        >,
    ),
    PhysicalAsset(
        Vec<crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset>,
    ),
    PipetteModel(
        Vec<crate::codegen::structs_codegen::tables::pipette_models::PipetteModel>,
    ),
    PipetteTipModel(
        Vec<crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel>,
    ),
    Pipette(Vec<crate::codegen::structs_codegen::tables::pipettes::Pipette>),
    PositioningDeviceModel(
        Vec<
            crate::codegen::structs_codegen::tables::positioning_device_models::PositioningDeviceModel,
        >,
    ),
    PositioningDevice(
        Vec<
            crate::codegen::structs_codegen::tables::positioning_devices::PositioningDevice,
        >,
    ),
    PouringProcedureTemplate(
        Vec<
            crate::codegen::structs_codegen::tables::pouring_procedure_templates::PouringProcedureTemplate,
        >,
    ),
    PouringProcedure(
        Vec<
            crate::codegen::structs_codegen::tables::pouring_procedures::PouringProcedure,
        >,
    ),
    ProcedureAsset(
        Vec<crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset>,
    ),
    ProcedureTemplateAssetModel(
        Vec<
            crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
        >,
    ),
    ProcedureTemplate(
        Vec<
            crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate,
        >,
    ),
    Procedure(Vec<crate::codegen::structs_codegen::tables::procedures::Procedure>),
    ProjectState(
        Vec<crate::codegen::structs_codegen::tables::project_states::ProjectState>,
    ),
    Project(Vec<crate::codegen::structs_codegen::tables::projects::Project>),
    Rank(Vec<crate::codegen::structs_codegen::tables::ranks::Rank>),
    ReagentModel(
        Vec<crate::codegen::structs_codegen::tables::reagent_models::ReagentModel>,
    ),
    Role(Vec<crate::codegen::structs_codegen::tables::roles::Role>),
    Room(Vec<crate::codegen::structs_codegen::tables::rooms::Room>),
    SampleModel(
        Vec<crate::codegen::structs_codegen::tables::sample_models::SampleModel>,
    ),
    SampleSourceModel(
        Vec<
            crate::codegen::structs_codegen::tables::sample_source_models::SampleSourceModel,
        >,
    ),
    SampleSource(
        Vec<crate::codegen::structs_codegen::tables::sample_sources::SampleSource>,
    ),
    SampleState(
        Vec<crate::codegen::structs_codegen::tables::sample_states::SampleState>,
    ),
    Sample(Vec<crate::codegen::structs_codegen::tables::samples::Sample>),
    SpatialRefSy(
        Vec<crate::codegen::structs_codegen::tables::spatial_ref_sys::SpatialRefSy>,
    ),
    Spectrum(Vec<crate::codegen::structs_codegen::tables::spectra::Spectrum>),
    SpectraCollection(
        Vec<
            crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection,
        >,
    ),
    StorageProcedureTemplate(
        Vec<
            crate::codegen::structs_codegen::tables::storage_procedure_templates::StorageProcedureTemplate,
        >,
    ),
    StorageProcedure(
        Vec<
            crate::codegen::structs_codegen::tables::storage_procedures::StorageProcedure,
        >,
    ),
    SupernatantProcedureTemplate(
        Vec<
            crate::codegen::structs_codegen::tables::supernatant_procedure_templates::SupernatantProcedureTemplate,
        >,
    ),
    SupernatantProcedure(
        Vec<
            crate::codegen::structs_codegen::tables::supernatant_procedures::SupernatantProcedure,
        >,
    ),
    Taxon(Vec<crate::codegen::structs_codegen::tables::taxa::Taxon>),
    TeamMember(Vec<crate::codegen::structs_codegen::tables::team_members::TeamMember>),
    TeamProject(
        Vec<crate::codegen::structs_codegen::tables::team_projects::TeamProject>,
    ),
    TeamState(Vec<crate::codegen::structs_codegen::tables::team_states::TeamState>),
    Team(Vec<crate::codegen::structs_codegen::tables::teams::Team>),
    TemporaryUser(
        Vec<crate::codegen::structs_codegen::tables::temporary_user::TemporaryUser>,
    ),
    Unit(Vec<crate::codegen::structs_codegen::tables::units::Unit>),
    UserEmail(Vec<crate::codegen::structs_codegen::tables::user_emails::UserEmail>),
    UserOrganization(
        Vec<
            crate::codegen::structs_codegen::tables::user_organizations::UserOrganization,
        >,
    ),
    User(Vec<crate::codegen::structs_codegen::tables::users::User>),
    VolumeMeasuringDeviceModel(
        Vec<
            crate::codegen::structs_codegen::tables::volume_measuring_device_models::VolumeMeasuringDeviceModel,
        >,
    ),
    VolumeMeasuringDevice(
        Vec<
            crate::codegen::structs_codegen::tables::volume_measuring_devices::VolumeMeasuringDevice,
        >,
    ),
    VolumetricContainerModel(
        Vec<
            crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel,
        >,
    ),
    VolumetricContainer(
        Vec<
            crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer,
        >,
    ),
    WeighingDeviceModel(
        Vec<
            crate::codegen::structs_codegen::tables::weighing_device_models::WeighingDeviceModel,
        >,
    ),
    WeighingDevice(
        Vec<crate::codegen::structs_codegen::tables::weighing_devices::WeighingDevice>,
    ),
    WeighingProcedureTemplate(
        Vec<
            crate::codegen::structs_codegen::tables::weighing_procedure_templates::WeighingProcedureTemplate,
        >,
    ),
    WeighingProcedure(
        Vec<
            crate::codegen::structs_codegen::tables::weighing_procedures::WeighingProcedure,
        >,
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
            Rows::AliquotingProcedureTemplate(aliquoting_procedure_templates) => {
                aliquoting_procedure_templates
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::AliquotingProcedure(aliquoting_procedures) => {
                aliquoting_procedures
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::AssetCompatibilityRule(asset_compatibility_rules) => {
                asset_compatibility_rules
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::AssetModelAncestor(asset_model_ancestors) => {
                asset_model_ancestors
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::AssetModel(asset_models) => {
                asset_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::Asset(assets) => {
                assets
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::BallMillMachineModel(ball_mill_machine_models) => {
                ball_mill_machine_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::BallMillMachine(ball_mill_machines) => {
                ball_mill_machines
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::BallMillProcedureTemplate(ball_mill_procedure_templates) => {
                ball_mill_procedure_templates
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::BallMillProcedure(ball_mill_procedures) => {
                ball_mill_procedures
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::BeadModel(bead_models) => {
                bead_models
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
            Rows::CameraModel(camera_models) => {
                camera_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::Camera(cameras) => {
                cameras
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::CapModel(cap_models) => {
                cap_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::CappingProcedureTemplate(capping_procedure_templates) => {
                capping_procedure_templates
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::CappingProcedure(capping_procedures) => {
                capping_procedures
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::CentrifugeModel(centrifuge_models) => {
                centrifuge_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::CentrifugeProcedureTemplate(centrifuge_procedure_templates) => {
                centrifuge_procedure_templates
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::CentrifugeProcedure(centrifuge_procedures) => {
                centrifuge_procedures
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::Centrifuge(centrifuges) => {
                centrifuges
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
            Rows::CommercialBallMillMachineLot(commercial_ball_mill_machine_lots) => {
                commercial_ball_mill_machine_lots
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::CommercialBallMillMachineModel(commercial_ball_mill_machine_models) => {
                commercial_ball_mill_machine_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::CommercialBeadLot(commercial_bead_lots) => {
                commercial_bead_lots
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::CommercialBeadModel(commercial_bead_models) => {
                commercial_bead_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::CommercialCameraLot(commercial_camera_lots) => {
                commercial_camera_lots
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::CommercialCameraModel(commercial_camera_models) => {
                commercial_camera_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::CommercialCapLot(commercial_cap_lots) => {
                commercial_cap_lots
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::CommercialCapModel(commercial_cap_models) => {
                commercial_cap_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::CommercialCentrifugeLot(commercial_centrifuge_lots) => {
                commercial_centrifuge_lots
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::CommercialCentrifugeModel(commercial_centrifuge_models) => {
                commercial_centrifuge_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::CommercialFreezeDryerLot(commercial_freeze_dryer_lots) => {
                commercial_freeze_dryer_lots
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::CommercialFreezeDryerModel(commercial_freeze_dryer_models) => {
                commercial_freeze_dryer_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::CommercialFreezerLot(commercial_freezer_lots) => {
                commercial_freezer_lots
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::CommercialFreezerModel(commercial_freezer_models) => {
                commercial_freezer_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::CommercialPackagingLot(commercial_packaging_lots) => {
                commercial_packaging_lots
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::CommercialPackagingModel(commercial_packaging_models) => {
                commercial_packaging_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::CommercialPipetteLot(commercial_pipette_lots) => {
                commercial_pipette_lots
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::CommercialPipetteModel(commercial_pipette_models) => {
                commercial_pipette_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::CommercialPipetteTipLot(commercial_pipette_tip_lots) => {
                commercial_pipette_tip_lots
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::CommercialPipetteTipModel(commercial_pipette_tip_models) => {
                commercial_pipette_tip_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::CommercialPositioningDeviceLot(commercial_positioning_device_lots) => {
                commercial_positioning_device_lots
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::CommercialPositioningDeviceModel(commercial_positioning_device_models) => {
                commercial_positioning_device_models
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
            Rows::CommercialVolumeMeasuringDeviceLot(commercial_volume_measuring_device_lots) => {
                commercial_volume_measuring_device_lots
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::CommercialVolumeMeasuringDeviceModel(
                commercial_volume_measuring_device_models,
            ) => {
                commercial_volume_measuring_device_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::CommercialWeighingDeviceLot(commercial_weighing_device_lots) => {
                commercial_weighing_device_lots
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::CommercialWeighingDeviceModel(commercial_weighing_device_models) => {
                commercial_weighing_device_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::ContainerCompatibilityRule(container_compatibility_rules) => {
                container_compatibility_rules
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
            Rows::Container(containers) => {
                containers
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
            Rows::DigitalAssetModel(digital_asset_models) => {
                digital_asset_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::DigitalAsset(digital_assets) => {
                digital_assets
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::DisposalProcedureTemplate(disposal_procedure_templates) => {
                disposal_procedure_templates
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::DisposalProcedure(disposal_procedures) => {
                disposal_procedures
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
            Rows::FractioningProcedureTemplate(fractioning_procedure_templates) => {
                fractioning_procedure_templates
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::FractioningProcedure(fractioning_procedures) => {
                fractioning_procedures
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::FreezeDryerModel(freeze_dryer_models) => {
                freeze_dryer_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::FreezeDryer(freeze_dryers) => {
                freeze_dryers
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::FreezeDryingProcedureTemplate(freeze_drying_procedure_templates) => {
                freeze_drying_procedure_templates
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::FreezeDryingProcedure(freeze_drying_procedures) => {
                freeze_drying_procedures
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::FreezerModel(freezer_models) => {
                freezer_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::Freezer(freezers) => {
                freezers
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::FreezingProcedureTemplate(freezing_procedure_templates) => {
                freezing_procedure_templates
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::FreezingProcedure(freezing_procedures) => {
                freezing_procedures
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::GeolocationProcedureTemplate(geolocation_procedure_templates) => {
                geolocation_procedure_templates
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::GeolocationProcedure(geolocation_procedures) => {
                geolocation_procedures
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
            Rows::NextProcedureTemplate(next_procedure_templates) => {
                next_procedure_templates
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
            Rows::PackagingProcedureTemplate(packaging_procedure_templates) => {
                packaging_procedure_templates
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::PackagingProcedure(packaging_procedures) => {
                packaging_procedures
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::ParentProcedureTemplate(parent_procedure_templates) => {
                parent_procedure_templates
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
            Rows::PhoneModel(phone_models) => {
                phone_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::PhotographProcedureTemplate(photograph_procedure_templates) => {
                photograph_procedure_templates
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::PhotographProcedure(photograph_procedures) => {
                photograph_procedures
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
            Rows::PhysicalAssetModel(physical_asset_models) => {
                physical_asset_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::PhysicalAsset(physical_assets) => {
                physical_assets
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::PipetteModel(pipette_models) => {
                pipette_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::PipetteTipModel(pipette_tip_models) => {
                pipette_tip_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::Pipette(pipettes) => {
                pipettes
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::PositioningDeviceModel(positioning_device_models) => {
                positioning_device_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::PositioningDevice(positioning_devices) => {
                positioning_devices
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::PouringProcedureTemplate(pouring_procedure_templates) => {
                pouring_procedure_templates
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::PouringProcedure(pouring_procedures) => {
                pouring_procedures
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::ProcedureAsset(procedure_assets) => {
                procedure_assets
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::ProcedureTemplateAssetModel(procedure_template_asset_models) => {
                procedure_template_asset_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::ProcedureTemplate(procedure_templates) => {
                procedure_templates
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
            Rows::ProjectState(project_states) => {
                project_states
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
            Rows::ReagentModel(reagent_models) => {
                reagent_models
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
            Rows::SampleModel(sample_models) => {
                sample_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::SampleSourceModel(sample_source_models) => {
                sample_source_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::SampleSource(sample_sources) => {
                sample_sources
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
            Rows::Sample(samples) => {
                samples
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
            Rows::StorageProcedureTemplate(storage_procedure_templates) => {
                storage_procedure_templates
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::StorageProcedure(storage_procedures) => {
                storage_procedures
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::SupernatantProcedureTemplate(supernatant_procedure_templates) => {
                supernatant_procedure_templates
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::SupernatantProcedure(supernatant_procedures) => {
                supernatant_procedures
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
            Rows::TemporaryUser(temporary_user) => {
                temporary_user
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
            Rows::VolumeMeasuringDeviceModel(volume_measuring_device_models) => {
                volume_measuring_device_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::VolumeMeasuringDevice(volume_measuring_devices) => {
                volume_measuring_devices
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::VolumetricContainerModel(volumetric_container_models) => {
                volumetric_container_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::VolumetricContainer(volumetric_containers) => {
                volumetric_containers
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::WeighingDeviceModel(weighing_device_models) => {
                weighing_device_models
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::WeighingDevice(weighing_devices) => {
                weighing_devices
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::WeighingProcedureTemplate(weighing_procedure_templates) => {
                weighing_procedure_templates
                    .iter()
                    .filter_map(|entry| entry.upsert(conn).transpose())
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?
                    .into()
            }
            Rows::WeighingProcedure(weighing_procedures) => {
                weighing_procedures
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
            Rows::AliquotingProcedureTemplate(aliquoting_procedure_templates) => {
                aliquoting_procedure_templates.primary_keys()
            }
            Rows::AliquotingProcedure(aliquoting_procedures) => {
                aliquoting_procedures.primary_keys()
            }
            Rows::AssetCompatibilityRule(asset_compatibility_rules) => {
                asset_compatibility_rules.primary_keys()
            }
            Rows::AssetModelAncestor(asset_model_ancestors) => asset_model_ancestors.primary_keys(),
            Rows::AssetModel(asset_models) => asset_models.primary_keys(),
            Rows::Asset(assets) => assets.primary_keys(),
            Rows::BallMillMachineModel(ball_mill_machine_models) => {
                ball_mill_machine_models.primary_keys()
            }
            Rows::BallMillMachine(ball_mill_machines) => ball_mill_machines.primary_keys(),
            Rows::BallMillProcedureTemplate(ball_mill_procedure_templates) => {
                ball_mill_procedure_templates.primary_keys()
            }
            Rows::BallMillProcedure(ball_mill_procedures) => ball_mill_procedures.primary_keys(),
            Rows::BeadModel(bead_models) => bead_models.primary_keys(),
            Rows::Brand(brands) => brands.primary_keys(),
            Rows::CameraModel(camera_models) => camera_models.primary_keys(),
            Rows::Camera(cameras) => cameras.primary_keys(),
            Rows::CapModel(cap_models) => cap_models.primary_keys(),
            Rows::CappingProcedureTemplate(capping_procedure_templates) => {
                capping_procedure_templates.primary_keys()
            }
            Rows::CappingProcedure(capping_procedures) => capping_procedures.primary_keys(),
            Rows::CentrifugeModel(centrifuge_models) => centrifuge_models.primary_keys(),
            Rows::CentrifugeProcedureTemplate(centrifuge_procedure_templates) => {
                centrifuge_procedure_templates.primary_keys()
            }
            Rows::CentrifugeProcedure(centrifuge_procedures) => {
                centrifuge_procedures.primary_keys()
            }
            Rows::Centrifuge(centrifuges) => centrifuges.primary_keys(),
            Rows::City(cities) => cities.primary_keys(),
            Rows::Color(colors) => colors.primary_keys(),
            Rows::CommercialBallMillMachineLot(commercial_ball_mill_machine_lots) => {
                commercial_ball_mill_machine_lots.primary_keys()
            }
            Rows::CommercialBallMillMachineModel(commercial_ball_mill_machine_models) => {
                commercial_ball_mill_machine_models.primary_keys()
            }
            Rows::CommercialBeadLot(commercial_bead_lots) => commercial_bead_lots.primary_keys(),
            Rows::CommercialBeadModel(commercial_bead_models) => {
                commercial_bead_models.primary_keys()
            }
            Rows::CommercialCameraLot(commercial_camera_lots) => {
                commercial_camera_lots.primary_keys()
            }
            Rows::CommercialCameraModel(commercial_camera_models) => {
                commercial_camera_models.primary_keys()
            }
            Rows::CommercialCapLot(commercial_cap_lots) => commercial_cap_lots.primary_keys(),
            Rows::CommercialCapModel(commercial_cap_models) => commercial_cap_models.primary_keys(),
            Rows::CommercialCentrifugeLot(commercial_centrifuge_lots) => {
                commercial_centrifuge_lots.primary_keys()
            }
            Rows::CommercialCentrifugeModel(commercial_centrifuge_models) => {
                commercial_centrifuge_models.primary_keys()
            }
            Rows::CommercialFreezeDryerLot(commercial_freeze_dryer_lots) => {
                commercial_freeze_dryer_lots.primary_keys()
            }
            Rows::CommercialFreezeDryerModel(commercial_freeze_dryer_models) => {
                commercial_freeze_dryer_models.primary_keys()
            }
            Rows::CommercialFreezerLot(commercial_freezer_lots) => {
                commercial_freezer_lots.primary_keys()
            }
            Rows::CommercialFreezerModel(commercial_freezer_models) => {
                commercial_freezer_models.primary_keys()
            }
            Rows::CommercialPackagingLot(commercial_packaging_lots) => {
                commercial_packaging_lots.primary_keys()
            }
            Rows::CommercialPackagingModel(commercial_packaging_models) => {
                commercial_packaging_models.primary_keys()
            }
            Rows::CommercialPipetteLot(commercial_pipette_lots) => {
                commercial_pipette_lots.primary_keys()
            }
            Rows::CommercialPipetteModel(commercial_pipette_models) => {
                commercial_pipette_models.primary_keys()
            }
            Rows::CommercialPipetteTipLot(commercial_pipette_tip_lots) => {
                commercial_pipette_tip_lots.primary_keys()
            }
            Rows::CommercialPipetteTipModel(commercial_pipette_tip_models) => {
                commercial_pipette_tip_models.primary_keys()
            }
            Rows::CommercialPositioningDeviceLot(commercial_positioning_device_lots) => {
                commercial_positioning_device_lots.primary_keys()
            }
            Rows::CommercialPositioningDeviceModel(commercial_positioning_device_models) => {
                commercial_positioning_device_models.primary_keys()
            }
            Rows::CommercialProductLot(commercial_product_lots) => {
                commercial_product_lots.primary_keys()
            }
            Rows::CommercialProduct(commercial_products) => commercial_products.primary_keys(),
            Rows::CommercialVolumeMeasuringDeviceLot(commercial_volume_measuring_device_lots) => {
                commercial_volume_measuring_device_lots.primary_keys()
            }
            Rows::CommercialVolumeMeasuringDeviceModel(
                commercial_volume_measuring_device_models,
            ) => commercial_volume_measuring_device_models.primary_keys(),
            Rows::CommercialWeighingDeviceLot(commercial_weighing_device_lots) => {
                commercial_weighing_device_lots.primary_keys()
            }
            Rows::CommercialWeighingDeviceModel(commercial_weighing_device_models) => {
                commercial_weighing_device_models.primary_keys()
            }
            Rows::ContainerCompatibilityRule(container_compatibility_rules) => {
                container_compatibility_rules.primary_keys()
            }
            Rows::ContainerModel(container_models) => container_models.primary_keys(),
            Rows::Container(containers) => containers.primary_keys(),
            Rows::Country(countries) => countries.primary_keys(),
            Rows::DigitalAssetModel(digital_asset_models) => digital_asset_models.primary_keys(),
            Rows::DigitalAsset(digital_assets) => digital_assets.primary_keys(),
            Rows::DisposalProcedureTemplate(disposal_procedure_templates) => {
                disposal_procedure_templates.primary_keys()
            }
            Rows::DisposalProcedure(disposal_procedures) => disposal_procedures.primary_keys(),
            Rows::EmailProvider(email_providers) => email_providers.primary_keys(),
            Rows::FractioningProcedureTemplate(fractioning_procedure_templates) => {
                fractioning_procedure_templates.primary_keys()
            }
            Rows::FractioningProcedure(fractioning_procedures) => {
                fractioning_procedures.primary_keys()
            }
            Rows::FreezeDryerModel(freeze_dryer_models) => freeze_dryer_models.primary_keys(),
            Rows::FreezeDryer(freeze_dryers) => freeze_dryers.primary_keys(),
            Rows::FreezeDryingProcedureTemplate(freeze_drying_procedure_templates) => {
                freeze_drying_procedure_templates.primary_keys()
            }
            Rows::FreezeDryingProcedure(freeze_drying_procedures) => {
                freeze_drying_procedures.primary_keys()
            }
            Rows::FreezerModel(freezer_models) => freezer_models.primary_keys(),
            Rows::Freezer(freezers) => freezers.primary_keys(),
            Rows::FreezingProcedureTemplate(freezing_procedure_templates) => {
                freezing_procedure_templates.primary_keys()
            }
            Rows::FreezingProcedure(freezing_procedures) => freezing_procedures.primary_keys(),
            Rows::GeolocationProcedureTemplate(geolocation_procedure_templates) => {
                geolocation_procedure_templates.primary_keys()
            }
            Rows::GeolocationProcedure(geolocation_procedures) => {
                geolocation_procedures.primary_keys()
            }
            Rows::InstrumentState(instrument_states) => instrument_states.primary_keys(),
            Rows::LoginProvider(login_providers) => login_providers.primary_keys(),
            Rows::Material(materials) => materials.primary_keys(),
            Rows::NextProcedureTemplate(next_procedure_templates) => {
                next_procedure_templates.primary_keys()
            }
            Rows::ObservationSubject(observation_subjects) => observation_subjects.primary_keys(),
            Rows::OrganismTaxon(organism_taxa) => organism_taxa.primary_keys(),
            Rows::Organism(organisms) => organisms.primary_keys(),
            Rows::Organization(organizations) => organizations.primary_keys(),
            Rows::PackagingModel(packaging_models) => packaging_models.primary_keys(),
            Rows::PackagingProcedureTemplate(packaging_procedure_templates) => {
                packaging_procedure_templates.primary_keys()
            }
            Rows::PackagingProcedure(packaging_procedures) => packaging_procedures.primary_keys(),
            Rows::ParentProcedureTemplate(parent_procedure_templates) => {
                parent_procedure_templates.primary_keys()
            }
            Rows::PermanenceCategory(permanence_categories) => permanence_categories.primary_keys(),
            Rows::PhoneModel(phone_models) => phone_models.primary_keys(),
            Rows::PhotographProcedureTemplate(photograph_procedure_templates) => {
                photograph_procedure_templates.primary_keys()
            }
            Rows::PhotographProcedure(photograph_procedures) => {
                photograph_procedures.primary_keys()
            }
            Rows::Photograph(photographs) => photographs.primary_keys(),
            Rows::PhysicalAssetModel(physical_asset_models) => physical_asset_models.primary_keys(),
            Rows::PhysicalAsset(physical_assets) => physical_assets.primary_keys(),
            Rows::PipetteModel(pipette_models) => pipette_models.primary_keys(),
            Rows::PipetteTipModel(pipette_tip_models) => pipette_tip_models.primary_keys(),
            Rows::Pipette(pipettes) => pipettes.primary_keys(),
            Rows::PositioningDeviceModel(positioning_device_models) => {
                positioning_device_models.primary_keys()
            }
            Rows::PositioningDevice(positioning_devices) => positioning_devices.primary_keys(),
            Rows::PouringProcedureTemplate(pouring_procedure_templates) => {
                pouring_procedure_templates.primary_keys()
            }
            Rows::PouringProcedure(pouring_procedures) => pouring_procedures.primary_keys(),
            Rows::ProcedureAsset(procedure_assets) => procedure_assets.primary_keys(),
            Rows::ProcedureTemplateAssetModel(procedure_template_asset_models) => {
                procedure_template_asset_models.primary_keys()
            }
            Rows::ProcedureTemplate(procedure_templates) => procedure_templates.primary_keys(),
            Rows::Procedure(procedures) => procedures.primary_keys(),
            Rows::ProjectState(project_states) => project_states.primary_keys(),
            Rows::Project(projects) => projects.primary_keys(),
            Rows::Rank(ranks) => ranks.primary_keys(),
            Rows::ReagentModel(reagent_models) => reagent_models.primary_keys(),
            Rows::Role(roles) => roles.primary_keys(),
            Rows::Room(rooms) => rooms.primary_keys(),
            Rows::SampleModel(sample_models) => sample_models.primary_keys(),
            Rows::SampleSourceModel(sample_source_models) => sample_source_models.primary_keys(),
            Rows::SampleSource(sample_sources) => sample_sources.primary_keys(),
            Rows::SampleState(sample_states) => sample_states.primary_keys(),
            Rows::Sample(samples) => samples.primary_keys(),
            Rows::SpatialRefSy(spatial_ref_sys) => spatial_ref_sys.primary_keys(),
            Rows::Spectrum(spectra) => spectra.primary_keys(),
            Rows::SpectraCollection(spectra_collections) => spectra_collections.primary_keys(),
            Rows::StorageProcedureTemplate(storage_procedure_templates) => {
                storage_procedure_templates.primary_keys()
            }
            Rows::StorageProcedure(storage_procedures) => storage_procedures.primary_keys(),
            Rows::SupernatantProcedureTemplate(supernatant_procedure_templates) => {
                supernatant_procedure_templates.primary_keys()
            }
            Rows::SupernatantProcedure(supernatant_procedures) => {
                supernatant_procedures.primary_keys()
            }
            Rows::Taxon(taxa) => taxa.primary_keys(),
            Rows::TeamMember(team_members) => team_members.primary_keys(),
            Rows::TeamProject(team_projects) => team_projects.primary_keys(),
            Rows::TeamState(team_states) => team_states.primary_keys(),
            Rows::Team(teams) => teams.primary_keys(),
            Rows::TemporaryUser(temporary_user) => temporary_user.primary_keys(),
            Rows::Unit(units) => units.primary_keys(),
            Rows::UserEmail(user_emails) => user_emails.primary_keys(),
            Rows::UserOrganization(user_organizations) => user_organizations.primary_keys(),
            Rows::User(users) => users.primary_keys(),
            Rows::VolumeMeasuringDeviceModel(volume_measuring_device_models) => {
                volume_measuring_device_models.primary_keys()
            }
            Rows::VolumeMeasuringDevice(volume_measuring_devices) => {
                volume_measuring_devices.primary_keys()
            }
            Rows::VolumetricContainerModel(volumetric_container_models) => {
                volumetric_container_models.primary_keys()
            }
            Rows::VolumetricContainer(volumetric_containers) => {
                volumetric_containers.primary_keys()
            }
            Rows::WeighingDeviceModel(weighing_device_models) => {
                weighing_device_models.primary_keys()
            }
            Rows::WeighingDevice(weighing_devices) => weighing_devices.primary_keys(),
            Rows::WeighingProcedureTemplate(weighing_procedure_templates) => {
                weighing_procedure_templates.primary_keys()
            }
            Rows::WeighingProcedure(weighing_procedures) => weighing_procedures.primary_keys(),
        }
    }
}
