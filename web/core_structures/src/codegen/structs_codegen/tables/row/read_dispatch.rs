impl<C> web_common_traits::prelude::ReadDispatch<C> for super::Row
where
    crate::codegen::structs_codegen::tables::addresses::Address: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::aliquoting_procedures::AliquotingProcedure: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::asset_model_ancestors::AssetModelAncestor: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::asset_models::AssetModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::assets::Asset: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::ball_mill_machines::BallMillMachine: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::ball_mill_procedure_templates::BallMillProcedureTemplate: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::ball_mill_procedures::BallMillProcedure: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::bead_models::BeadModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::brands::Brand: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::camera_models::CameraModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::cameras::Camera: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::cap_models::CapModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::capping_procedure_templates::CappingProcedureTemplate: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::capping_procedures::CappingProcedure: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::centrifuge_models::CentrifugeModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::centrifuge_procedure_templates::CentrifugeProcedureTemplate: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::centrifuge_procedures::CentrifugeProcedure: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::centrifuges::Centrifuge: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::cities::City: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::colors::Color: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_ball_mill_machine_lots::CommercialBallMillMachineLot: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_ball_mill_machine_models::CommercialBallMillMachineModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_bead_lots::CommercialBeadLot: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_bead_models::CommercialBeadModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_camera_lots::CommercialCameraLot: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_camera_models::CommercialCameraModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_cap_lots::CommercialCapLot: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_cap_models::CommercialCapModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_centrifuge_lots::CommercialCentrifugeLot: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_centrifuge_models::CommercialCentrifugeModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_freeze_dryer_lots::CommercialFreezeDryerLot: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_freeze_dryer_models::CommercialFreezeDryerModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_freezer_lots::CommercialFreezerLot: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_freezer_models::CommercialFreezerModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_packaging_lots::CommercialPackagingLot: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_packaging_models::CommercialPackagingModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_pipette_lots::CommercialPipetteLot: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_pipette_models::CommercialPipetteModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_pipette_tip_lots::CommercialPipetteTipLot: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_pipette_tip_models::CommercialPipetteTipModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_positioning_device_lots::CommercialPositioningDeviceLot: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_positioning_device_models::CommercialPositioningDeviceModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_volume_measuring_device_lots::CommercialVolumeMeasuringDeviceLot: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_volume_measuring_device_models::CommercialVolumeMeasuringDeviceModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_weighing_device_lots::CommercialWeighingDeviceLot: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_weighing_device_models::CommercialWeighingDeviceModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::container_compatibility_rules::ContainerCompatibilityRule: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::container_models::ContainerModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::containers::Container: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::countries::Country: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::digital_asset_models::DigitalAssetModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::digital_assets::DigitalAsset: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::disposal_procedure_templates::DisposalProcedureTemplate: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::disposal_procedures::DisposalProcedure: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::email_providers::EmailProvider: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::fractioning_procedure_templates::FractioningProcedureTemplate: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::fractioning_procedures::FractioningProcedure: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::freeze_dryer_models::FreezeDryerModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::freeze_dryers::FreezeDryer: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::freeze_drying_procedure_templates::FreezeDryingProcedureTemplate: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::freeze_drying_procedures::FreezeDryingProcedure: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::freezer_models::FreezerModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::freezers::Freezer: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::freezing_procedure_templates::FreezingProcedureTemplate: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::freezing_procedures::FreezingProcedure: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::geolocation_procedure_templates::GeolocationProcedureTemplate: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::geolocation_procedures::GeolocationProcedure: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::instrument_states::InstrumentState: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::login_providers::LoginProvider: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::materials::Material: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::next_procedure_templates::NextProcedureTemplate: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::observation_subjects::ObservationSubject: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::organism_taxa::OrganismTaxon: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::organisms::Organism: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::organizations::Organization: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::packaging_models::PackagingModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::packaging_procedure_templates::PackagingProcedureTemplate: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::packaging_procedures::PackagingProcedure: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::permanence_categories::PermanenceCategory: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::phone_models::PhoneModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::photograph_procedures::PhotographProcedure: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::photographs::Photograph: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::pipette_models::PipetteModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::pipettes::Pipette: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::positioning_device_models::PositioningDeviceModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::positioning_devices::PositioningDevice: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::pouring_procedure_templates::PouringProcedureTemplate: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::pouring_procedures::PouringProcedure: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::procedures::Procedure: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::project_states::ProjectState: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::projects::Project: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::ranks::Rank: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::reagent_models::ReagentModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::roles::Role: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::rooms::Room: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::sample_models::SampleModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::sample_source_models::SampleSourceModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::sample_sources::SampleSource: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::sample_states::SampleState: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::samples::Sample: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::spatial_ref_sys::SpatialRefSy: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::spectra::Spectrum: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::storage_procedure_templates::StorageProcedureTemplate: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::storage_procedures::StorageProcedure: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::supernatant_procedure_templates::SupernatantProcedureTemplate: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::supernatant_procedures::SupernatantProcedure: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::taxa::Taxon: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::team_members::TeamMember: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::team_projects::TeamProject: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::team_states::TeamState: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::teams::Team: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::temporary_user::TemporaryUser: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::units::Unit: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::user_emails::UserEmail: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::user_organizations::UserOrganization: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::users::User: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::volume_measuring_device_models::VolumeMeasuringDeviceModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::volume_measuring_devices::VolumeMeasuringDevice: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::weighing_device_models::WeighingDeviceModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::weighing_devices::WeighingDevice: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::weighing_procedure_templates::WeighingProcedureTemplate: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::weighing_procedures::WeighingProcedure: web_common_traits::database::Read<
        C,
    >,
{
    type PrimaryKey = crate::codegen::tables::table_primary_keys::TablePrimaryKey;
    fn read(
        primary_key: Self::PrimaryKey,
        conn: &mut C,
    ) -> Result<Self, diesel::result::Error> {
        use web_common_traits::database::Read;
        Ok(
            match primary_key {
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Address(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::addresses::Address::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::AliquotingProcedureTemplate(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::AliquotingProcedure(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::aliquoting_procedures::AliquotingProcedure::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::AssetCompatibilityRule(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::AssetModelAncestor(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::asset_model_ancestors::AssetModelAncestor::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::AssetModel(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::asset_models::AssetModel::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Asset(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::assets::Asset::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::BallMillMachineModel(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::BallMillMachine(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::ball_mill_machines::BallMillMachine::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::BallMillProcedureTemplate(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::ball_mill_procedure_templates::BallMillProcedureTemplate::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::BallMillProcedure(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::ball_mill_procedures::BallMillProcedure::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::BeadModel(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::bead_models::BeadModel::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Brand(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::brands::Brand::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CameraModel(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::camera_models::CameraModel::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Camera(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::cameras::Camera::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CapModel(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::cap_models::CapModel::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CappingProcedureTemplate(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::capping_procedure_templates::CappingProcedureTemplate::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CappingProcedure(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::capping_procedures::CappingProcedure::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CentrifugeModel(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::centrifuge_models::CentrifugeModel::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CentrifugeProcedureTemplate(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::centrifuge_procedure_templates::CentrifugeProcedureTemplate::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CentrifugeProcedure(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::centrifuge_procedures::CentrifugeProcedure::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Centrifuge(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::centrifuges::Centrifuge::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::City(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::cities::City::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Color(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::colors::Color::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialBallMillMachineLot(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::commercial_ball_mill_machine_lots::CommercialBallMillMachineLot::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialBallMillMachineModel(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::commercial_ball_mill_machine_models::CommercialBallMillMachineModel::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialBeadLot(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::commercial_bead_lots::CommercialBeadLot::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialBeadModel(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::commercial_bead_models::CommercialBeadModel::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialCameraLot(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::commercial_camera_lots::CommercialCameraLot::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialCameraModel(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::commercial_camera_models::CommercialCameraModel::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialCapLot(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::commercial_cap_lots::CommercialCapLot::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialCapModel(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::commercial_cap_models::CommercialCapModel::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialCentrifugeLot(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::commercial_centrifuge_lots::CommercialCentrifugeLot::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialCentrifugeModel(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::commercial_centrifuge_models::CommercialCentrifugeModel::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialFreezeDryerLot(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::commercial_freeze_dryer_lots::CommercialFreezeDryerLot::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialFreezeDryerModel(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::commercial_freeze_dryer_models::CommercialFreezeDryerModel::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialFreezerLot(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::commercial_freezer_lots::CommercialFreezerLot::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialFreezerModel(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::commercial_freezer_models::CommercialFreezerModel::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialPackagingLot(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::commercial_packaging_lots::CommercialPackagingLot::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialPackagingModel(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::commercial_packaging_models::CommercialPackagingModel::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialPipetteLot(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::commercial_pipette_lots::CommercialPipetteLot::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialPipetteModel(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::commercial_pipette_models::CommercialPipetteModel::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialPipetteTipLot(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::commercial_pipette_tip_lots::CommercialPipetteTipLot::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialPipetteTipModel(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::commercial_pipette_tip_models::CommercialPipetteTipModel::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialPositioningDeviceLot(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::commercial_positioning_device_lots::CommercialPositioningDeviceLot::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialPositioningDeviceModel(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::commercial_positioning_device_models::CommercialPositioningDeviceModel::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialProductLot(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialProduct(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialVolumeMeasuringDeviceLot(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::commercial_volume_measuring_device_lots::CommercialVolumeMeasuringDeviceLot::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialVolumeMeasuringDeviceModel(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::commercial_volume_measuring_device_models::CommercialVolumeMeasuringDeviceModel::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialWeighingDeviceLot(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::commercial_weighing_device_lots::CommercialWeighingDeviceLot::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialWeighingDeviceModel(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::commercial_weighing_device_models::CommercialWeighingDeviceModel::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::ContainerCompatibilityRule(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::container_compatibility_rules::ContainerCompatibilityRule::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::ContainerModel(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::container_models::ContainerModel::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Container(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::containers::Container::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Country(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::countries::Country::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::DigitalAssetModel(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::digital_asset_models::DigitalAssetModel::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::DigitalAsset(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::digital_assets::DigitalAsset::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::DisposalProcedureTemplate(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::disposal_procedure_templates::DisposalProcedureTemplate::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::DisposalProcedure(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::disposal_procedures::DisposalProcedure::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::EmailProvider(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::email_providers::EmailProvider::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::FractioningProcedureTemplate(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::fractioning_procedure_templates::FractioningProcedureTemplate::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::FractioningProcedure(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::fractioning_procedures::FractioningProcedure::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::FreezeDryerModel(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::freeze_dryer_models::FreezeDryerModel::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::FreezeDryer(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::freeze_dryers::FreezeDryer::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::FreezeDryingProcedureTemplate(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::freeze_drying_procedure_templates::FreezeDryingProcedureTemplate::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::FreezeDryingProcedure(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::freeze_drying_procedures::FreezeDryingProcedure::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::FreezerModel(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::freezer_models::FreezerModel::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Freezer(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::freezers::Freezer::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::FreezingProcedureTemplate(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::freezing_procedure_templates::FreezingProcedureTemplate::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::FreezingProcedure(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::freezing_procedures::FreezingProcedure::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::GeolocationProcedureTemplate(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::geolocation_procedure_templates::GeolocationProcedureTemplate::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::GeolocationProcedure(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::geolocation_procedures::GeolocationProcedure::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::InstrumentState(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::instrument_states::InstrumentState::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::LoginProvider(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::login_providers::LoginProvider::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Material(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::materials::Material::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::NextProcedureTemplate(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::next_procedure_templates::NextProcedureTemplate::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::ObservationSubject(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::observation_subjects::ObservationSubject::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::OrganismTaxon(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::organism_taxa::OrganismTaxon::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Organism(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::organisms::Organism::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Organization(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::organizations::Organization::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::PackagingModel(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::packaging_models::PackagingModel::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::PackagingProcedureTemplate(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::packaging_procedure_templates::PackagingProcedureTemplate::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::PackagingProcedure(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::packaging_procedures::PackagingProcedure::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::ParentProcedureTemplate(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::PermanenceCategory(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::permanence_categories::PermanenceCategory::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::PhoneModel(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::phone_models::PhoneModel::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::PhotographProcedureTemplate(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::PhotographProcedure(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::photograph_procedures::PhotographProcedure::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Photograph(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::photographs::Photograph::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::PhysicalAssetModel(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::PhysicalAsset(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::PipetteModel(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::pipette_models::PipetteModel::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::PipetteTipModel(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Pipette(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::pipettes::Pipette::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::PositioningDeviceModel(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::positioning_device_models::PositioningDeviceModel::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::PositioningDevice(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::positioning_devices::PositioningDevice::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::PouringProcedureTemplate(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::pouring_procedure_templates::PouringProcedureTemplate::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::PouringProcedure(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::pouring_procedures::PouringProcedure::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureAsset(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplateAssetModel(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplate(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Procedure(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::procedures::Procedure::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProjectState(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::project_states::ProjectState::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Project(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::projects::Project::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Rank(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::ranks::Rank::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::ReagentModel(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::reagent_models::ReagentModel::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Role(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::roles::Role::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Room(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::rooms::Room::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::SampleModel(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::sample_models::SampleModel::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::SampleSourceModel(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::sample_source_models::SampleSourceModel::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::SampleSource(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::sample_sources::SampleSource::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::SampleState(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::sample_states::SampleState::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Sample(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::samples::Sample::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::SpatialRefSy(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::spatial_ref_sys::SpatialRefSy::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Spectrum(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::spectra::Spectrum::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::SpectraCollection(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::StorageProcedureTemplate(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::storage_procedure_templates::StorageProcedureTemplate::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::StorageProcedure(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::storage_procedures::StorageProcedure::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::SupernatantProcedureTemplate(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::supernatant_procedure_templates::SupernatantProcedureTemplate::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::SupernatantProcedure(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::supernatant_procedures::SupernatantProcedure::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Taxon(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::taxa::Taxon::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::TeamMember(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::team_members::TeamMember::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::TeamProject(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::team_projects::TeamProject::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::TeamState(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::team_states::TeamState::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Team(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::teams::Team::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::TemporaryUser(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::temporary_user::TemporaryUser::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Unit(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::units::Unit::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::UserEmail(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::user_emails::UserEmail::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::UserOrganization(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::user_organizations::UserOrganization::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::User(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::users::User::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::VolumeMeasuringDeviceModel(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::volume_measuring_device_models::VolumeMeasuringDeviceModel::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::VolumeMeasuringDevice(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::volume_measuring_devices::VolumeMeasuringDevice::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::VolumetricContainerModel(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::VolumetricContainer(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::WeighingDeviceModel(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::weighing_device_models::WeighingDeviceModel::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::WeighingDevice(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::weighing_devices::WeighingDevice::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::WeighingProcedureTemplate(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::weighing_procedure_templates::WeighingProcedureTemplate::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::WeighingProcedure(
                    primary_key,
                ) => {
                    crate::codegen::structs_codegen::tables::weighing_procedures::WeighingProcedure::read(
                            primary_key,
                            conn,
                        )?
                        .into()
                }
            },
        )
    }
}
