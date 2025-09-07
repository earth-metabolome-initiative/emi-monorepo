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
mod documents;
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
mod from_row;
mod geolocation_procedure_templates;
mod geolocation_procedures;
mod instrument_states;
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
mod read_dispatch;
mod reagent_models;
mod roles;
mod rooms;
mod sample_states;
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
pub enum Row {
    Address(crate::Address),
    AliquotingProcedureTemplate(crate::AliquotingProcedureTemplate),
    AliquotingProcedure(crate::AliquotingProcedure),
    AssetCompatibilityRule(crate::AssetCompatibilityRule),
    AssetModelAncestor(crate::AssetModelAncestor),
    AssetModel(crate::AssetModel),
    Asset(crate::Asset),
    BallMillMachineModel(crate::BallMillMachineModel),
    BallMillMachine(crate::BallMillMachine),
    BallMillProcedureTemplate(crate::BallMillProcedureTemplate),
    BallMillProcedure(crate::BallMillProcedure),
    BeadModel(crate::BeadModel),
    Brand(crate::Brand),
    CameraModel(crate::CameraModel),
    Camera(crate::Camera),
    CapModel(crate::CapModel),
    CappingProcedureTemplate(crate::CappingProcedureTemplate),
    CappingProcedure(crate::CappingProcedure),
    CentrifugeModel(crate::CentrifugeModel),
    CentrifugeProcedureTemplate(crate::CentrifugeProcedureTemplate),
    CentrifugeProcedure(crate::CentrifugeProcedure),
    Centrifuge(crate::Centrifuge),
    City(crate::City),
    Color(crate::Color),
    CommercialBallMillMachineLot(crate::CommercialBallMillMachineLot),
    CommercialBallMillMachineModel(crate::CommercialBallMillMachineModel),
    CommercialBeadLot(crate::CommercialBeadLot),
    CommercialBeadModel(crate::CommercialBeadModel),
    CommercialCameraLot(crate::CommercialCameraLot),
    CommercialCameraModel(crate::CommercialCameraModel),
    CommercialCapLot(crate::CommercialCapLot),
    CommercialCapModel(crate::CommercialCapModel),
    CommercialCentrifugeLot(crate::CommercialCentrifugeLot),
    CommercialCentrifugeModel(crate::CommercialCentrifugeModel),
    CommercialFreezeDryerLot(crate::CommercialFreezeDryerLot),
    CommercialFreezeDryerModel(crate::CommercialFreezeDryerModel),
    CommercialFreezerLot(crate::CommercialFreezerLot),
    CommercialFreezerModel(crate::CommercialFreezerModel),
    CommercialPackagingLot(crate::CommercialPackagingLot),
    CommercialPackagingModel(crate::CommercialPackagingModel),
    CommercialPipetteLot(crate::CommercialPipetteLot),
    CommercialPipetteModel(crate::CommercialPipetteModel),
    CommercialPipetteTipLot(crate::CommercialPipetteTipLot),
    CommercialPipetteTipModel(crate::CommercialPipetteTipModel),
    CommercialPositioningDeviceLot(crate::CommercialPositioningDeviceLot),
    CommercialPositioningDeviceModel(crate::CommercialPositioningDeviceModel),
    CommercialProductLot(crate::CommercialProductLot),
    CommercialProduct(crate::CommercialProduct),
    CommercialVolumeMeasuringDeviceLot(crate::CommercialVolumeMeasuringDeviceLot),
    CommercialVolumeMeasuringDeviceModel(crate::CommercialVolumeMeasuringDeviceModel),
    CommercialWeighingDeviceLot(crate::CommercialWeighingDeviceLot),
    CommercialWeighingDeviceModel(crate::CommercialWeighingDeviceModel),
    ContainerCompatibilityRule(crate::ContainerCompatibilityRule),
    ContainerModel(crate::ContainerModel),
    Container(crate::Container),
    Country(crate::Country),
    DigitalAssetModel(crate::DigitalAssetModel),
    DigitalAsset(crate::DigitalAsset),
    DisposalProcedureTemplate(crate::DisposalProcedureTemplate),
    DisposalProcedure(crate::DisposalProcedure),
    Document(crate::Document),
    EmailProvider(crate::EmailProvider),
    FractioningProcedureTemplate(crate::FractioningProcedureTemplate),
    FractioningProcedure(crate::FractioningProcedure),
    FreezeDryerModel(crate::FreezeDryerModel),
    FreezeDryer(crate::FreezeDryer),
    FreezeDryingProcedureTemplate(crate::FreezeDryingProcedureTemplate),
    FreezeDryingProcedure(crate::FreezeDryingProcedure),
    FreezerModel(crate::FreezerModel),
    Freezer(crate::Freezer),
    FreezingProcedureTemplate(crate::FreezingProcedureTemplate),
    FreezingProcedure(crate::FreezingProcedure),
    GeolocationProcedureTemplate(crate::GeolocationProcedureTemplate),
    GeolocationProcedure(crate::GeolocationProcedure),
    InstrumentState(crate::InstrumentState),
    LoginProvider(crate::LoginProvider),
    Material(crate::Material),
    NextProcedureTemplate(crate::NextProcedureTemplate),
    ObservationSubject(crate::ObservationSubject),
    OrganismTaxon(crate::OrganismTaxon),
    Organism(crate::Organism),
    Organization(crate::Organization),
    PackagingModel(crate::PackagingModel),
    PackagingProcedureTemplate(crate::PackagingProcedureTemplate),
    PackagingProcedure(crate::PackagingProcedure),
    ParentProcedureTemplate(crate::ParentProcedureTemplate),
    PermanenceCategory(crate::PermanenceCategory),
    PhoneModel(crate::PhoneModel),
    PhotographProcedureTemplate(crate::PhotographProcedureTemplate),
    PhotographProcedure(crate::PhotographProcedure),
    PhysicalAssetModel(crate::PhysicalAssetModel),
    PhysicalAsset(crate::PhysicalAsset),
    PipetteModel(crate::PipetteModel),
    PipetteTipModel(crate::PipetteTipModel),
    Pipette(crate::Pipette),
    PositioningDeviceModel(crate::PositioningDeviceModel),
    PositioningDevice(crate::PositioningDevice),
    PouringProcedureTemplate(crate::PouringProcedureTemplate),
    PouringProcedure(crate::PouringProcedure),
    ProcedureAsset(crate::ProcedureAsset),
    ProcedureTemplateAssetModel(crate::ProcedureTemplateAssetModel),
    ProcedureTemplate(crate::ProcedureTemplate),
    Procedure(crate::Procedure),
    ProjectState(crate::ProjectState),
    Project(crate::Project),
    Rank(crate::Rank),
    ReagentModel(crate::ReagentModel),
    Role(crate::Role),
    Room(crate::Room),
    SampleState(crate::SampleState),
    SpatialRefSy(crate::SpatialRefSy),
    Spectrum(crate::Spectrum),
    SpectraCollection(crate::SpectraCollection),
    StorageProcedureTemplate(crate::StorageProcedureTemplate),
    StorageProcedure(crate::StorageProcedure),
    SupernatantProcedureTemplate(crate::SupernatantProcedureTemplate),
    SupernatantProcedure(crate::SupernatantProcedure),
    Taxon(crate::Taxon),
    TeamMember(crate::TeamMember),
    TeamProject(crate::TeamProject),
    TeamState(crate::TeamState),
    Team(crate::Team),
    TemporaryUser(crate::TemporaryUser),
    Unit(crate::Unit),
    UserEmail(crate::UserEmail),
    UserOrganization(crate::UserOrganization),
    User(crate::User),
    VolumeMeasuringDeviceModel(crate::VolumeMeasuringDeviceModel),
    VolumeMeasuringDevice(crate::VolumeMeasuringDevice),
    VolumetricContainerModel(crate::VolumetricContainerModel),
    VolumetricContainer(crate::VolumetricContainer),
    WeighingDeviceModel(crate::WeighingDeviceModel),
    WeighingDevice(crate::WeighingDevice),
    WeighingProcedureTemplate(crate::WeighingProcedureTemplate),
    WeighingProcedure(crate::WeighingProcedure),
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
            Row::AliquotingProcedureTemplate(aliquoting_procedure_templates) => {
                aliquoting_procedure_templates.upsert(conn)?.map(Row::from)
            }
            Row::AliquotingProcedure(aliquoting_procedures) => {
                aliquoting_procedures.upsert(conn)?.map(Row::from)
            }
            Row::AssetCompatibilityRule(asset_compatibility_rules) => {
                asset_compatibility_rules.upsert(conn)?.map(Row::from)
            }
            Row::AssetModelAncestor(asset_model_ancestors) => {
                asset_model_ancestors.upsert(conn)?.map(Row::from)
            }
            Row::AssetModel(asset_models) => asset_models.upsert(conn)?.map(Row::from),
            Row::Asset(assets) => assets.upsert(conn)?.map(Row::from),
            Row::BallMillMachineModel(ball_mill_machine_models) => {
                ball_mill_machine_models.upsert(conn)?.map(Row::from)
            }
            Row::BallMillMachine(ball_mill_machines) => {
                ball_mill_machines.upsert(conn)?.map(Row::from)
            }
            Row::BallMillProcedureTemplate(ball_mill_procedure_templates) => {
                ball_mill_procedure_templates.upsert(conn)?.map(Row::from)
            }
            Row::BallMillProcedure(ball_mill_procedures) => {
                ball_mill_procedures.upsert(conn)?.map(Row::from)
            }
            Row::BeadModel(bead_models) => bead_models.upsert(conn)?.map(Row::from),
            Row::Brand(brands) => brands.upsert(conn)?.map(Row::from),
            Row::CameraModel(camera_models) => camera_models.upsert(conn)?.map(Row::from),
            Row::Camera(cameras) => cameras.upsert(conn)?.map(Row::from),
            Row::CapModel(cap_models) => cap_models.upsert(conn)?.map(Row::from),
            Row::CappingProcedureTemplate(capping_procedure_templates) => {
                capping_procedure_templates.upsert(conn)?.map(Row::from)
            }
            Row::CappingProcedure(capping_procedures) => {
                capping_procedures.upsert(conn)?.map(Row::from)
            }
            Row::CentrifugeModel(centrifuge_models) => {
                centrifuge_models.upsert(conn)?.map(Row::from)
            }
            Row::CentrifugeProcedureTemplate(centrifuge_procedure_templates) => {
                centrifuge_procedure_templates.upsert(conn)?.map(Row::from)
            }
            Row::CentrifugeProcedure(centrifuge_procedures) => {
                centrifuge_procedures.upsert(conn)?.map(Row::from)
            }
            Row::Centrifuge(centrifuges) => centrifuges.upsert(conn)?.map(Row::from),
            Row::City(cities) => cities.upsert(conn)?.map(Row::from),
            Row::Color(colors) => colors.upsert(conn)?.map(Row::from),
            Row::CommercialBallMillMachineLot(commercial_ball_mill_machine_lots) => {
                commercial_ball_mill_machine_lots.upsert(conn)?.map(Row::from)
            }
            Row::CommercialBallMillMachineModel(commercial_ball_mill_machine_models) => {
                commercial_ball_mill_machine_models.upsert(conn)?.map(Row::from)
            }
            Row::CommercialBeadLot(commercial_bead_lots) => {
                commercial_bead_lots.upsert(conn)?.map(Row::from)
            }
            Row::CommercialBeadModel(commercial_bead_models) => {
                commercial_bead_models.upsert(conn)?.map(Row::from)
            }
            Row::CommercialCameraLot(commercial_camera_lots) => {
                commercial_camera_lots.upsert(conn)?.map(Row::from)
            }
            Row::CommercialCameraModel(commercial_camera_models) => {
                commercial_camera_models.upsert(conn)?.map(Row::from)
            }
            Row::CommercialCapLot(commercial_cap_lots) => {
                commercial_cap_lots.upsert(conn)?.map(Row::from)
            }
            Row::CommercialCapModel(commercial_cap_models) => {
                commercial_cap_models.upsert(conn)?.map(Row::from)
            }
            Row::CommercialCentrifugeLot(commercial_centrifuge_lots) => {
                commercial_centrifuge_lots.upsert(conn)?.map(Row::from)
            }
            Row::CommercialCentrifugeModel(commercial_centrifuge_models) => {
                commercial_centrifuge_models.upsert(conn)?.map(Row::from)
            }
            Row::CommercialFreezeDryerLot(commercial_freeze_dryer_lots) => {
                commercial_freeze_dryer_lots.upsert(conn)?.map(Row::from)
            }
            Row::CommercialFreezeDryerModel(commercial_freeze_dryer_models) => {
                commercial_freeze_dryer_models.upsert(conn)?.map(Row::from)
            }
            Row::CommercialFreezerLot(commercial_freezer_lots) => {
                commercial_freezer_lots.upsert(conn)?.map(Row::from)
            }
            Row::CommercialFreezerModel(commercial_freezer_models) => {
                commercial_freezer_models.upsert(conn)?.map(Row::from)
            }
            Row::CommercialPackagingLot(commercial_packaging_lots) => {
                commercial_packaging_lots.upsert(conn)?.map(Row::from)
            }
            Row::CommercialPackagingModel(commercial_packaging_models) => {
                commercial_packaging_models.upsert(conn)?.map(Row::from)
            }
            Row::CommercialPipetteLot(commercial_pipette_lots) => {
                commercial_pipette_lots.upsert(conn)?.map(Row::from)
            }
            Row::CommercialPipetteModel(commercial_pipette_models) => {
                commercial_pipette_models.upsert(conn)?.map(Row::from)
            }
            Row::CommercialPipetteTipLot(commercial_pipette_tip_lots) => {
                commercial_pipette_tip_lots.upsert(conn)?.map(Row::from)
            }
            Row::CommercialPipetteTipModel(commercial_pipette_tip_models) => {
                commercial_pipette_tip_models.upsert(conn)?.map(Row::from)
            }
            Row::CommercialPositioningDeviceLot(commercial_positioning_device_lots) => {
                commercial_positioning_device_lots.upsert(conn)?.map(Row::from)
            }
            Row::CommercialPositioningDeviceModel(commercial_positioning_device_models) => {
                commercial_positioning_device_models.upsert(conn)?.map(Row::from)
            }
            Row::CommercialProductLot(commercial_product_lots) => {
                commercial_product_lots.upsert(conn)?.map(Row::from)
            }
            Row::CommercialProduct(commercial_products) => {
                commercial_products.upsert(conn)?.map(Row::from)
            }
            Row::CommercialVolumeMeasuringDeviceLot(commercial_volume_measuring_device_lots) => {
                commercial_volume_measuring_device_lots.upsert(conn)?.map(Row::from)
            }
            Row::CommercialVolumeMeasuringDeviceModel(
                commercial_volume_measuring_device_models,
            ) => commercial_volume_measuring_device_models.upsert(conn)?.map(Row::from),
            Row::CommercialWeighingDeviceLot(commercial_weighing_device_lots) => {
                commercial_weighing_device_lots.upsert(conn)?.map(Row::from)
            }
            Row::CommercialWeighingDeviceModel(commercial_weighing_device_models) => {
                commercial_weighing_device_models.upsert(conn)?.map(Row::from)
            }
            Row::ContainerCompatibilityRule(container_compatibility_rules) => {
                container_compatibility_rules.upsert(conn)?.map(Row::from)
            }
            Row::ContainerModel(container_models) => container_models.upsert(conn)?.map(Row::from),
            Row::Container(containers) => containers.upsert(conn)?.map(Row::from),
            Row::Country(countries) => countries.upsert(conn)?.map(Row::from),
            Row::DigitalAssetModel(digital_asset_models) => {
                digital_asset_models.upsert(conn)?.map(Row::from)
            }
            Row::DigitalAsset(digital_assets) => digital_assets.upsert(conn)?.map(Row::from),
            Row::DisposalProcedureTemplate(disposal_procedure_templates) => {
                disposal_procedure_templates.upsert(conn)?.map(Row::from)
            }
            Row::DisposalProcedure(disposal_procedures) => {
                disposal_procedures.upsert(conn)?.map(Row::from)
            }
            Row::Document(documents) => documents.upsert(conn)?.map(Row::from),
            Row::EmailProvider(email_providers) => email_providers.upsert(conn)?.map(Row::from),
            Row::FractioningProcedureTemplate(fractioning_procedure_templates) => {
                fractioning_procedure_templates.upsert(conn)?.map(Row::from)
            }
            Row::FractioningProcedure(fractioning_procedures) => {
                fractioning_procedures.upsert(conn)?.map(Row::from)
            }
            Row::FreezeDryerModel(freeze_dryer_models) => {
                freeze_dryer_models.upsert(conn)?.map(Row::from)
            }
            Row::FreezeDryer(freeze_dryers) => freeze_dryers.upsert(conn)?.map(Row::from),
            Row::FreezeDryingProcedureTemplate(freeze_drying_procedure_templates) => {
                freeze_drying_procedure_templates.upsert(conn)?.map(Row::from)
            }
            Row::FreezeDryingProcedure(freeze_drying_procedures) => {
                freeze_drying_procedures.upsert(conn)?.map(Row::from)
            }
            Row::FreezerModel(freezer_models) => freezer_models.upsert(conn)?.map(Row::from),
            Row::Freezer(freezers) => freezers.upsert(conn)?.map(Row::from),
            Row::FreezingProcedureTemplate(freezing_procedure_templates) => {
                freezing_procedure_templates.upsert(conn)?.map(Row::from)
            }
            Row::FreezingProcedure(freezing_procedures) => {
                freezing_procedures.upsert(conn)?.map(Row::from)
            }
            Row::GeolocationProcedureTemplate(geolocation_procedure_templates) => {
                geolocation_procedure_templates.upsert(conn)?.map(Row::from)
            }
            Row::GeolocationProcedure(geolocation_procedures) => {
                geolocation_procedures.upsert(conn)?.map(Row::from)
            }
            Row::InstrumentState(instrument_states) => {
                instrument_states.upsert(conn)?.map(Row::from)
            }
            Row::LoginProvider(login_providers) => login_providers.upsert(conn)?.map(Row::from),
            Row::Material(materials) => materials.upsert(conn)?.map(Row::from),
            Row::NextProcedureTemplate(next_procedure_templates) => {
                next_procedure_templates.upsert(conn)?.map(Row::from)
            }
            Row::ObservationSubject(observation_subjects) => {
                observation_subjects.upsert(conn)?.map(Row::from)
            }
            Row::OrganismTaxon(organism_taxa) => organism_taxa.upsert(conn)?.map(Row::from),
            Row::Organism(organisms) => organisms.upsert(conn)?.map(Row::from),
            Row::Organization(organizations) => organizations.upsert(conn)?.map(Row::from),
            Row::PackagingModel(packaging_models) => packaging_models.upsert(conn)?.map(Row::from),
            Row::PackagingProcedureTemplate(packaging_procedure_templates) => {
                packaging_procedure_templates.upsert(conn)?.map(Row::from)
            }
            Row::PackagingProcedure(packaging_procedures) => {
                packaging_procedures.upsert(conn)?.map(Row::from)
            }
            Row::ParentProcedureTemplate(parent_procedure_templates) => {
                parent_procedure_templates.upsert(conn)?.map(Row::from)
            }
            Row::PermanenceCategory(permanence_categories) => {
                permanence_categories.upsert(conn)?.map(Row::from)
            }
            Row::PhoneModel(phone_models) => phone_models.upsert(conn)?.map(Row::from),
            Row::PhotographProcedureTemplate(photograph_procedure_templates) => {
                photograph_procedure_templates.upsert(conn)?.map(Row::from)
            }
            Row::PhotographProcedure(photograph_procedures) => {
                photograph_procedures.upsert(conn)?.map(Row::from)
            }
            Row::PhysicalAssetModel(physical_asset_models) => {
                physical_asset_models.upsert(conn)?.map(Row::from)
            }
            Row::PhysicalAsset(physical_assets) => physical_assets.upsert(conn)?.map(Row::from),
            Row::PipetteModel(pipette_models) => pipette_models.upsert(conn)?.map(Row::from),
            Row::PipetteTipModel(pipette_tip_models) => {
                pipette_tip_models.upsert(conn)?.map(Row::from)
            }
            Row::Pipette(pipettes) => pipettes.upsert(conn)?.map(Row::from),
            Row::PositioningDeviceModel(positioning_device_models) => {
                positioning_device_models.upsert(conn)?.map(Row::from)
            }
            Row::PositioningDevice(positioning_devices) => {
                positioning_devices.upsert(conn)?.map(Row::from)
            }
            Row::PouringProcedureTemplate(pouring_procedure_templates) => {
                pouring_procedure_templates.upsert(conn)?.map(Row::from)
            }
            Row::PouringProcedure(pouring_procedures) => {
                pouring_procedures.upsert(conn)?.map(Row::from)
            }
            Row::ProcedureAsset(procedure_assets) => procedure_assets.upsert(conn)?.map(Row::from),
            Row::ProcedureTemplateAssetModel(procedure_template_asset_models) => {
                procedure_template_asset_models.upsert(conn)?.map(Row::from)
            }
            Row::ProcedureTemplate(procedure_templates) => {
                procedure_templates.upsert(conn)?.map(Row::from)
            }
            Row::Procedure(procedures) => procedures.upsert(conn)?.map(Row::from),
            Row::ProjectState(project_states) => project_states.upsert(conn)?.map(Row::from),
            Row::Project(projects) => projects.upsert(conn)?.map(Row::from),
            Row::Rank(ranks) => ranks.upsert(conn)?.map(Row::from),
            Row::ReagentModel(reagent_models) => reagent_models.upsert(conn)?.map(Row::from),
            Row::Role(roles) => roles.upsert(conn)?.map(Row::from),
            Row::Room(rooms) => rooms.upsert(conn)?.map(Row::from),
            Row::SampleState(sample_states) => sample_states.upsert(conn)?.map(Row::from),
            Row::SpatialRefSy(spatial_ref_sys) => spatial_ref_sys.upsert(conn)?.map(Row::from),
            Row::Spectrum(spectra) => spectra.upsert(conn)?.map(Row::from),
            Row::SpectraCollection(spectra_collections) => {
                spectra_collections.upsert(conn)?.map(Row::from)
            }
            Row::StorageProcedureTemplate(storage_procedure_templates) => {
                storage_procedure_templates.upsert(conn)?.map(Row::from)
            }
            Row::StorageProcedure(storage_procedures) => {
                storage_procedures.upsert(conn)?.map(Row::from)
            }
            Row::SupernatantProcedureTemplate(supernatant_procedure_templates) => {
                supernatant_procedure_templates.upsert(conn)?.map(Row::from)
            }
            Row::SupernatantProcedure(supernatant_procedures) => {
                supernatant_procedures.upsert(conn)?.map(Row::from)
            }
            Row::Taxon(taxa) => taxa.upsert(conn)?.map(Row::from),
            Row::TeamMember(team_members) => team_members.upsert(conn)?.map(Row::from),
            Row::TeamProject(team_projects) => team_projects.upsert(conn)?.map(Row::from),
            Row::TeamState(team_states) => team_states.upsert(conn)?.map(Row::from),
            Row::Team(teams) => teams.upsert(conn)?.map(Row::from),
            Row::TemporaryUser(temporary_user) => temporary_user.upsert(conn)?.map(Row::from),
            Row::Unit(units) => units.upsert(conn)?.map(Row::from),
            Row::UserEmail(user_emails) => user_emails.upsert(conn)?.map(Row::from),
            Row::UserOrganization(user_organizations) => {
                user_organizations.upsert(conn)?.map(Row::from)
            }
            Row::User(users) => users.upsert(conn)?.map(Row::from),
            Row::VolumeMeasuringDeviceModel(volume_measuring_device_models) => {
                volume_measuring_device_models.upsert(conn)?.map(Row::from)
            }
            Row::VolumeMeasuringDevice(volume_measuring_devices) => {
                volume_measuring_devices.upsert(conn)?.map(Row::from)
            }
            Row::VolumetricContainerModel(volumetric_container_models) => {
                volumetric_container_models.upsert(conn)?.map(Row::from)
            }
            Row::VolumetricContainer(volumetric_containers) => {
                volumetric_containers.upsert(conn)?.map(Row::from)
            }
            Row::WeighingDeviceModel(weighing_device_models) => {
                weighing_device_models.upsert(conn)?.map(Row::from)
            }
            Row::WeighingDevice(weighing_devices) => weighing_devices.upsert(conn)?.map(Row::from),
            Row::WeighingProcedureTemplate(weighing_procedure_templates) => {
                weighing_procedure_templates.upsert(conn)?.map(Row::from)
            }
            Row::WeighingProcedure(weighing_procedures) => {
                weighing_procedures.upsert(conn)?.map(Row::from)
            }
        })
    }
}
impl web_common_traits::prelude::Row for Row {
    type PrimaryKey = crate::codegen::tables::table_primary_keys::TablePrimaryKey;
    fn primary_key(&self) -> Self::PrimaryKey {
        match self {
            Row::Address(addresses) => addresses.primary_key(),
            Row::AliquotingProcedureTemplate(aliquoting_procedure_templates) => {
                aliquoting_procedure_templates.primary_key()
            }
            Row::AliquotingProcedure(aliquoting_procedures) => aliquoting_procedures.primary_key(),
            Row::AssetCompatibilityRule(asset_compatibility_rules) => {
                asset_compatibility_rules.primary_key()
            }
            Row::AssetModelAncestor(asset_model_ancestors) => asset_model_ancestors.primary_key(),
            Row::AssetModel(asset_models) => asset_models.primary_key(),
            Row::Asset(assets) => assets.primary_key(),
            Row::BallMillMachineModel(ball_mill_machine_models) => {
                ball_mill_machine_models.primary_key()
            }
            Row::BallMillMachine(ball_mill_machines) => ball_mill_machines.primary_key(),
            Row::BallMillProcedureTemplate(ball_mill_procedure_templates) => {
                ball_mill_procedure_templates.primary_key()
            }
            Row::BallMillProcedure(ball_mill_procedures) => ball_mill_procedures.primary_key(),
            Row::BeadModel(bead_models) => bead_models.primary_key(),
            Row::Brand(brands) => brands.primary_key(),
            Row::CameraModel(camera_models) => camera_models.primary_key(),
            Row::Camera(cameras) => cameras.primary_key(),
            Row::CapModel(cap_models) => cap_models.primary_key(),
            Row::CappingProcedureTemplate(capping_procedure_templates) => {
                capping_procedure_templates.primary_key()
            }
            Row::CappingProcedure(capping_procedures) => capping_procedures.primary_key(),
            Row::CentrifugeModel(centrifuge_models) => centrifuge_models.primary_key(),
            Row::CentrifugeProcedureTemplate(centrifuge_procedure_templates) => {
                centrifuge_procedure_templates.primary_key()
            }
            Row::CentrifugeProcedure(centrifuge_procedures) => centrifuge_procedures.primary_key(),
            Row::Centrifuge(centrifuges) => centrifuges.primary_key(),
            Row::City(cities) => cities.primary_key(),
            Row::Color(colors) => colors.primary_key(),
            Row::CommercialBallMillMachineLot(commercial_ball_mill_machine_lots) => {
                commercial_ball_mill_machine_lots.primary_key()
            }
            Row::CommercialBallMillMachineModel(commercial_ball_mill_machine_models) => {
                commercial_ball_mill_machine_models.primary_key()
            }
            Row::CommercialBeadLot(commercial_bead_lots) => commercial_bead_lots.primary_key(),
            Row::CommercialBeadModel(commercial_bead_models) => {
                commercial_bead_models.primary_key()
            }
            Row::CommercialCameraLot(commercial_camera_lots) => {
                commercial_camera_lots.primary_key()
            }
            Row::CommercialCameraModel(commercial_camera_models) => {
                commercial_camera_models.primary_key()
            }
            Row::CommercialCapLot(commercial_cap_lots) => commercial_cap_lots.primary_key(),
            Row::CommercialCapModel(commercial_cap_models) => commercial_cap_models.primary_key(),
            Row::CommercialCentrifugeLot(commercial_centrifuge_lots) => {
                commercial_centrifuge_lots.primary_key()
            }
            Row::CommercialCentrifugeModel(commercial_centrifuge_models) => {
                commercial_centrifuge_models.primary_key()
            }
            Row::CommercialFreezeDryerLot(commercial_freeze_dryer_lots) => {
                commercial_freeze_dryer_lots.primary_key()
            }
            Row::CommercialFreezeDryerModel(commercial_freeze_dryer_models) => {
                commercial_freeze_dryer_models.primary_key()
            }
            Row::CommercialFreezerLot(commercial_freezer_lots) => {
                commercial_freezer_lots.primary_key()
            }
            Row::CommercialFreezerModel(commercial_freezer_models) => {
                commercial_freezer_models.primary_key()
            }
            Row::CommercialPackagingLot(commercial_packaging_lots) => {
                commercial_packaging_lots.primary_key()
            }
            Row::CommercialPackagingModel(commercial_packaging_models) => {
                commercial_packaging_models.primary_key()
            }
            Row::CommercialPipetteLot(commercial_pipette_lots) => {
                commercial_pipette_lots.primary_key()
            }
            Row::CommercialPipetteModel(commercial_pipette_models) => {
                commercial_pipette_models.primary_key()
            }
            Row::CommercialPipetteTipLot(commercial_pipette_tip_lots) => {
                commercial_pipette_tip_lots.primary_key()
            }
            Row::CommercialPipetteTipModel(commercial_pipette_tip_models) => {
                commercial_pipette_tip_models.primary_key()
            }
            Row::CommercialPositioningDeviceLot(commercial_positioning_device_lots) => {
                commercial_positioning_device_lots.primary_key()
            }
            Row::CommercialPositioningDeviceModel(commercial_positioning_device_models) => {
                commercial_positioning_device_models.primary_key()
            }
            Row::CommercialProductLot(commercial_product_lots) => {
                commercial_product_lots.primary_key()
            }
            Row::CommercialProduct(commercial_products) => commercial_products.primary_key(),
            Row::CommercialVolumeMeasuringDeviceLot(commercial_volume_measuring_device_lots) => {
                commercial_volume_measuring_device_lots.primary_key()
            }
            Row::CommercialVolumeMeasuringDeviceModel(
                commercial_volume_measuring_device_models,
            ) => commercial_volume_measuring_device_models.primary_key(),
            Row::CommercialWeighingDeviceLot(commercial_weighing_device_lots) => {
                commercial_weighing_device_lots.primary_key()
            }
            Row::CommercialWeighingDeviceModel(commercial_weighing_device_models) => {
                commercial_weighing_device_models.primary_key()
            }
            Row::ContainerCompatibilityRule(container_compatibility_rules) => {
                container_compatibility_rules.primary_key()
            }
            Row::ContainerModel(container_models) => container_models.primary_key(),
            Row::Container(containers) => containers.primary_key(),
            Row::Country(countries) => countries.primary_key(),
            Row::DigitalAssetModel(digital_asset_models) => digital_asset_models.primary_key(),
            Row::DigitalAsset(digital_assets) => digital_assets.primary_key(),
            Row::DisposalProcedureTemplate(disposal_procedure_templates) => {
                disposal_procedure_templates.primary_key()
            }
            Row::DisposalProcedure(disposal_procedures) => disposal_procedures.primary_key(),
            Row::Document(documents) => documents.primary_key(),
            Row::EmailProvider(email_providers) => email_providers.primary_key(),
            Row::FractioningProcedureTemplate(fractioning_procedure_templates) => {
                fractioning_procedure_templates.primary_key()
            }
            Row::FractioningProcedure(fractioning_procedures) => {
                fractioning_procedures.primary_key()
            }
            Row::FreezeDryerModel(freeze_dryer_models) => freeze_dryer_models.primary_key(),
            Row::FreezeDryer(freeze_dryers) => freeze_dryers.primary_key(),
            Row::FreezeDryingProcedureTemplate(freeze_drying_procedure_templates) => {
                freeze_drying_procedure_templates.primary_key()
            }
            Row::FreezeDryingProcedure(freeze_drying_procedures) => {
                freeze_drying_procedures.primary_key()
            }
            Row::FreezerModel(freezer_models) => freezer_models.primary_key(),
            Row::Freezer(freezers) => freezers.primary_key(),
            Row::FreezingProcedureTemplate(freezing_procedure_templates) => {
                freezing_procedure_templates.primary_key()
            }
            Row::FreezingProcedure(freezing_procedures) => freezing_procedures.primary_key(),
            Row::GeolocationProcedureTemplate(geolocation_procedure_templates) => {
                geolocation_procedure_templates.primary_key()
            }
            Row::GeolocationProcedure(geolocation_procedures) => {
                geolocation_procedures.primary_key()
            }
            Row::InstrumentState(instrument_states) => instrument_states.primary_key(),
            Row::LoginProvider(login_providers) => login_providers.primary_key(),
            Row::Material(materials) => materials.primary_key(),
            Row::NextProcedureTemplate(next_procedure_templates) => {
                next_procedure_templates.primary_key()
            }
            Row::ObservationSubject(observation_subjects) => observation_subjects.primary_key(),
            Row::OrganismTaxon(organism_taxa) => organism_taxa.primary_key(),
            Row::Organism(organisms) => organisms.primary_key(),
            Row::Organization(organizations) => organizations.primary_key(),
            Row::PackagingModel(packaging_models) => packaging_models.primary_key(),
            Row::PackagingProcedureTemplate(packaging_procedure_templates) => {
                packaging_procedure_templates.primary_key()
            }
            Row::PackagingProcedure(packaging_procedures) => packaging_procedures.primary_key(),
            Row::ParentProcedureTemplate(parent_procedure_templates) => {
                parent_procedure_templates.primary_key()
            }
            Row::PermanenceCategory(permanence_categories) => permanence_categories.primary_key(),
            Row::PhoneModel(phone_models) => phone_models.primary_key(),
            Row::PhotographProcedureTemplate(photograph_procedure_templates) => {
                photograph_procedure_templates.primary_key()
            }
            Row::PhotographProcedure(photograph_procedures) => photograph_procedures.primary_key(),
            Row::PhysicalAssetModel(physical_asset_models) => physical_asset_models.primary_key(),
            Row::PhysicalAsset(physical_assets) => physical_assets.primary_key(),
            Row::PipetteModel(pipette_models) => pipette_models.primary_key(),
            Row::PipetteTipModel(pipette_tip_models) => pipette_tip_models.primary_key(),
            Row::Pipette(pipettes) => pipettes.primary_key(),
            Row::PositioningDeviceModel(positioning_device_models) => {
                positioning_device_models.primary_key()
            }
            Row::PositioningDevice(positioning_devices) => positioning_devices.primary_key(),
            Row::PouringProcedureTemplate(pouring_procedure_templates) => {
                pouring_procedure_templates.primary_key()
            }
            Row::PouringProcedure(pouring_procedures) => pouring_procedures.primary_key(),
            Row::ProcedureAsset(procedure_assets) => procedure_assets.primary_key(),
            Row::ProcedureTemplateAssetModel(procedure_template_asset_models) => {
                procedure_template_asset_models.primary_key()
            }
            Row::ProcedureTemplate(procedure_templates) => procedure_templates.primary_key(),
            Row::Procedure(procedures) => procedures.primary_key(),
            Row::ProjectState(project_states) => project_states.primary_key(),
            Row::Project(projects) => projects.primary_key(),
            Row::Rank(ranks) => ranks.primary_key(),
            Row::ReagentModel(reagent_models) => reagent_models.primary_key(),
            Row::Role(roles) => roles.primary_key(),
            Row::Room(rooms) => rooms.primary_key(),
            Row::SampleState(sample_states) => sample_states.primary_key(),
            Row::SpatialRefSy(spatial_ref_sys) => spatial_ref_sys.primary_key(),
            Row::Spectrum(spectra) => spectra.primary_key(),
            Row::SpectraCollection(spectra_collections) => spectra_collections.primary_key(),
            Row::StorageProcedureTemplate(storage_procedure_templates) => {
                storage_procedure_templates.primary_key()
            }
            Row::StorageProcedure(storage_procedures) => storage_procedures.primary_key(),
            Row::SupernatantProcedureTemplate(supernatant_procedure_templates) => {
                supernatant_procedure_templates.primary_key()
            }
            Row::SupernatantProcedure(supernatant_procedures) => {
                supernatant_procedures.primary_key()
            }
            Row::Taxon(taxa) => taxa.primary_key(),
            Row::TeamMember(team_members) => team_members.primary_key(),
            Row::TeamProject(team_projects) => team_projects.primary_key(),
            Row::TeamState(team_states) => team_states.primary_key(),
            Row::Team(teams) => teams.primary_key(),
            Row::TemporaryUser(temporary_user) => temporary_user.primary_key(),
            Row::Unit(units) => units.primary_key(),
            Row::UserEmail(user_emails) => user_emails.primary_key(),
            Row::UserOrganization(user_organizations) => user_organizations.primary_key(),
            Row::User(users) => users.primary_key(),
            Row::VolumeMeasuringDeviceModel(volume_measuring_device_models) => {
                volume_measuring_device_models.primary_key()
            }
            Row::VolumeMeasuringDevice(volume_measuring_devices) => {
                volume_measuring_devices.primary_key()
            }
            Row::VolumetricContainerModel(volumetric_container_models) => {
                volumetric_container_models.primary_key()
            }
            Row::VolumetricContainer(volumetric_containers) => volumetric_containers.primary_key(),
            Row::WeighingDeviceModel(weighing_device_models) => {
                weighing_device_models.primary_key()
            }
            Row::WeighingDevice(weighing_devices) => weighing_devices.primary_key(),
            Row::WeighingProcedureTemplate(weighing_procedure_templates) => {
                weighing_procedure_templates.primary_key()
            }
            Row::WeighingProcedure(weighing_procedures) => weighing_procedures.primary_key(),
        }
    }
}
