impl<C> web_common_traits::prelude::BoundedReadDispatch<C> for super::Rows
where
    crate::codegen::structs_codegen::tables::addresses::Address: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::aliquoting_procedures::AliquotingProcedure: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::asset_model_ancestors::AssetModelAncestor: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::asset_models::AssetModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::assets::Asset: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::ball_mill_machines::BallMillMachine: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::ball_mill_procedure_templates::BallMillProcedureTemplate: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::ball_mill_procedures::BallMillProcedure: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::bead_models::BeadModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::brands::Brand: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::camera_models::CameraModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::cameras::Camera: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::cap_models::CapModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::capping_procedure_templates::CappingProcedureTemplate: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::capping_procedures::CappingProcedure: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::centrifuge_models::CentrifugeModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::centrifuge_procedure_templates::CentrifugeProcedureTemplate: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::centrifuge_procedures::CentrifugeProcedure: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::centrifuges::Centrifuge: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::cities::City: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::cleaning_procedure_templates::CleaningProcedureTemplate: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::cleaning_procedures::CleaningProcedure: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::colors::Color: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_ball_mill_machine_lots::CommercialBallMillMachineLot: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_ball_mill_machine_models::CommercialBallMillMachineModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_bead_lots::CommercialBeadLot: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_bead_models::CommercialBeadModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_camera_lots::CommercialCameraLot: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_camera_models::CommercialCameraModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_cap_lots::CommercialCapLot: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_cap_models::CommercialCapModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_centrifuge_lots::CommercialCentrifugeLot: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_centrifuge_models::CommercialCentrifugeModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_freeze_dryer_lots::CommercialFreezeDryerLot: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_freeze_dryer_models::CommercialFreezeDryerModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_freezer_lots::CommercialFreezerLot: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_freezer_models::CommercialFreezerModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_packaging_lots::CommercialPackagingLot: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_packaging_models::CommercialPackagingModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_pipette_lots::CommercialPipetteLot: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_pipette_models::CommercialPipetteModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_pipette_tip_lots::CommercialPipetteTipLot: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_pipette_tip_models::CommercialPipetteTipModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_positioning_device_lots::CommercialPositioningDeviceLot: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_positioning_device_models::CommercialPositioningDeviceModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_volume_measuring_device_lots::CommercialVolumeMeasuringDeviceLot: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_volume_measuring_device_models::CommercialVolumeMeasuringDeviceModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_weighing_device_lots::CommercialWeighingDeviceLot: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::commercial_weighing_device_models::CommercialWeighingDeviceModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::container_compatibility_rules::ContainerCompatibilityRule: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::container_models::ContainerModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::containers::Container: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::countries::Country: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::digital_asset_models::DigitalAssetModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::digital_assets::DigitalAsset: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::disposal_procedure_templates::DisposalProcedureTemplate: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::disposal_procedures::DisposalProcedure: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::email_providers::EmailProvider: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::fractioning_procedure_templates::FractioningProcedureTemplate: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::fractioning_procedures::FractioningProcedure: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::freeze_dryer_models::FreezeDryerModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::freeze_dryers::FreezeDryer: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::freeze_drying_procedure_templates::FreezeDryingProcedureTemplate: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::freeze_drying_procedures::FreezeDryingProcedure: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::freezer_models::FreezerModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::freezers::Freezer: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::freezing_procedure_templates::FreezingProcedureTemplate: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::freezing_procedures::FreezingProcedure: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::geolocation_procedure_templates::GeolocationProcedureTemplate: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::geolocation_procedures::GeolocationProcedure: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::harvesting_procedure_templates::HarvestingProcedureTemplate: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::harvesting_procedures::HarvestingProcedure: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::instrument_states::InstrumentState: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::login_providers::LoginProvider: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::materials::Material: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::next_procedure_templates::NextProcedureTemplate: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::observation_subjects::ObservationSubject: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::organism_models::OrganismModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::organism_taxa::OrganismTaxon: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::organisms::Organism: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::organizations::Organization: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::packaging_models::PackagingModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::packaging_procedure_templates::PackagingProcedureTemplate: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::packaging_procedures::PackagingProcedure: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::permanence_categories::PermanenceCategory: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::personal_protective_equipment_models::PersonalProtectiveEquipmentModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::phone_models::PhoneModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::photograph_procedures::PhotographProcedure: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::photographs::Photograph: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::pipette_models::PipetteModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::pipettes::Pipette: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::positioning_device_models::PositioningDeviceModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::positioning_devices::PositioningDevice: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::pouring_procedure_templates::PouringProcedureTemplate: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::pouring_procedures::PouringProcedure: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::ppe_reminder_procedure_templates::PpeReminderProcedureTemplate: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::ppe_reminder_procedures::PpeReminderProcedure: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::procedures::Procedure: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::project_states::ProjectState: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::projects::Project: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::ranks::Rank: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::reagent_models::ReagentModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::roles::Role: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::rooms::Room: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::sample_models::SampleModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::sample_source_models::SampleSourceModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::sample_sources::SampleSource: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::sample_states::SampleState: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::samples::Sample: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::soil_models::SoilModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::soils::Soil: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::spatial_ref_sys::SpatialRefSy: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::spectra::Spectrum: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::storage_procedure_templates::StorageProcedureTemplate: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::storage_procedures::StorageProcedure: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::supernatant_procedure_templates::SupernatantProcedureTemplate: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::supernatant_procedures::SupernatantProcedure: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::tagging_procedure_templates::TaggingProcedureTemplate: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::tagging_procedures::TaggingProcedure: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::taxa::Taxon: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::team_members::TeamMember: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::team_projects::TeamProject: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::team_states::TeamState: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::teams::Team: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::temporary_user::TemporaryUser: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::units::Unit: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::user_emails::UserEmail: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::user_organizations::UserOrganization: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::users::User: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::volume_measuring_device_models::VolumeMeasuringDeviceModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::volume_measuring_devices::VolumeMeasuringDevice: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::weighing_device_models::WeighingDeviceModel: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::weighing_devices::WeighingDevice: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::weighing_procedure_templates::WeighingProcedureTemplate: web_common_traits::prelude::BoundedRead<
        C,
    >,
    crate::codegen::structs_codegen::tables::weighing_procedures::WeighingProcedure: web_common_traits::prelude::BoundedRead<
        C,
    >,
{
    type TableName = crate::codegen::tables::table_names::TableName;
    fn bounded_read(
        table_name: Self::TableName,
        offset: u16,
        limit: u16,
        conn: &mut C,
    ) -> Result<Self, diesel::result::Error> {
        use web_common_traits::database::BoundedRead;
        match table_name {
            crate::codegen::tables::table_names::TableName::Address => {
                crate::codegen::structs_codegen::tables::addresses::Address::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::AliquotingProcedureTemplate => {
                crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::AliquotingProcedure => {
                crate::codegen::structs_codegen::tables::aliquoting_procedures::AliquotingProcedure::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::AssetCompatibilityRule => {
                crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::AssetModelAncestor => {
                crate::codegen::structs_codegen::tables::asset_model_ancestors::AssetModelAncestor::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::AssetModel => {
                crate::codegen::structs_codegen::tables::asset_models::AssetModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Asset => {
                crate::codegen::structs_codegen::tables::assets::Asset::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::BallMillMachineModel => {
                crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::BallMillMachine => {
                crate::codegen::structs_codegen::tables::ball_mill_machines::BallMillMachine::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::BallMillProcedureTemplate => {
                crate::codegen::structs_codegen::tables::ball_mill_procedure_templates::BallMillProcedureTemplate::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::BallMillProcedure => {
                crate::codegen::structs_codegen::tables::ball_mill_procedures::BallMillProcedure::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::BeadModel => {
                crate::codegen::structs_codegen::tables::bead_models::BeadModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Brand => {
                crate::codegen::structs_codegen::tables::brands::Brand::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CameraModel => {
                crate::codegen::structs_codegen::tables::camera_models::CameraModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Camera => {
                crate::codegen::structs_codegen::tables::cameras::Camera::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CapModel => {
                crate::codegen::structs_codegen::tables::cap_models::CapModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CappingProcedureTemplate => {
                crate::codegen::structs_codegen::tables::capping_procedure_templates::CappingProcedureTemplate::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CappingProcedure => {
                crate::codegen::structs_codegen::tables::capping_procedures::CappingProcedure::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CentrifugeModel => {
                crate::codegen::structs_codegen::tables::centrifuge_models::CentrifugeModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CentrifugeProcedureTemplate => {
                crate::codegen::structs_codegen::tables::centrifuge_procedure_templates::CentrifugeProcedureTemplate::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CentrifugeProcedure => {
                crate::codegen::structs_codegen::tables::centrifuge_procedures::CentrifugeProcedure::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Centrifuge => {
                crate::codegen::structs_codegen::tables::centrifuges::Centrifuge::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::City => {
                crate::codegen::structs_codegen::tables::cities::City::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CleaningProcedureTemplate => {
                crate::codegen::structs_codegen::tables::cleaning_procedure_templates::CleaningProcedureTemplate::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CleaningProcedure => {
                crate::codegen::structs_codegen::tables::cleaning_procedures::CleaningProcedure::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Color => {
                crate::codegen::structs_codegen::tables::colors::Color::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CommercialBallMillMachineLot => {
                crate::codegen::structs_codegen::tables::commercial_ball_mill_machine_lots::CommercialBallMillMachineLot::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CommercialBallMillMachineModel => {
                crate::codegen::structs_codegen::tables::commercial_ball_mill_machine_models::CommercialBallMillMachineModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CommercialBeadLot => {
                crate::codegen::structs_codegen::tables::commercial_bead_lots::CommercialBeadLot::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CommercialBeadModel => {
                crate::codegen::structs_codegen::tables::commercial_bead_models::CommercialBeadModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CommercialCameraLot => {
                crate::codegen::structs_codegen::tables::commercial_camera_lots::CommercialCameraLot::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CommercialCameraModel => {
                crate::codegen::structs_codegen::tables::commercial_camera_models::CommercialCameraModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CommercialCapLot => {
                crate::codegen::structs_codegen::tables::commercial_cap_lots::CommercialCapLot::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CommercialCapModel => {
                crate::codegen::structs_codegen::tables::commercial_cap_models::CommercialCapModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CommercialCentrifugeLot => {
                crate::codegen::structs_codegen::tables::commercial_centrifuge_lots::CommercialCentrifugeLot::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CommercialCentrifugeModel => {
                crate::codegen::structs_codegen::tables::commercial_centrifuge_models::CommercialCentrifugeModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CommercialFreezeDryerLot => {
                crate::codegen::structs_codegen::tables::commercial_freeze_dryer_lots::CommercialFreezeDryerLot::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CommercialFreezeDryerModel => {
                crate::codegen::structs_codegen::tables::commercial_freeze_dryer_models::CommercialFreezeDryerModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CommercialFreezerLot => {
                crate::codegen::structs_codegen::tables::commercial_freezer_lots::CommercialFreezerLot::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CommercialFreezerModel => {
                crate::codegen::structs_codegen::tables::commercial_freezer_models::CommercialFreezerModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CommercialPackagingLot => {
                crate::codegen::structs_codegen::tables::commercial_packaging_lots::CommercialPackagingLot::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CommercialPackagingModel => {
                crate::codegen::structs_codegen::tables::commercial_packaging_models::CommercialPackagingModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CommercialPipetteLot => {
                crate::codegen::structs_codegen::tables::commercial_pipette_lots::CommercialPipetteLot::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CommercialPipetteModel => {
                crate::codegen::structs_codegen::tables::commercial_pipette_models::CommercialPipetteModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CommercialPipetteTipLot => {
                crate::codegen::structs_codegen::tables::commercial_pipette_tip_lots::CommercialPipetteTipLot::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CommercialPipetteTipModel => {
                crate::codegen::structs_codegen::tables::commercial_pipette_tip_models::CommercialPipetteTipModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CommercialPositioningDeviceLot => {
                crate::codegen::structs_codegen::tables::commercial_positioning_device_lots::CommercialPositioningDeviceLot::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CommercialPositioningDeviceModel => {
                crate::codegen::structs_codegen::tables::commercial_positioning_device_models::CommercialPositioningDeviceModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CommercialProductLot => {
                crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CommercialProduct => {
                crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CommercialVolumeMeasuringDeviceLot => {
                crate::codegen::structs_codegen::tables::commercial_volume_measuring_device_lots::CommercialVolumeMeasuringDeviceLot::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CommercialVolumeMeasuringDeviceModel => {
                crate::codegen::structs_codegen::tables::commercial_volume_measuring_device_models::CommercialVolumeMeasuringDeviceModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CommercialWeighingDeviceLot => {
                crate::codegen::structs_codegen::tables::commercial_weighing_device_lots::CommercialWeighingDeviceLot::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CommercialWeighingDeviceModel => {
                crate::codegen::structs_codegen::tables::commercial_weighing_device_models::CommercialWeighingDeviceModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::ContainerCompatibilityRule => {
                crate::codegen::structs_codegen::tables::container_compatibility_rules::ContainerCompatibilityRule::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::ContainerModel => {
                crate::codegen::structs_codegen::tables::container_models::ContainerModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Container => {
                crate::codegen::structs_codegen::tables::containers::Container::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Country => {
                crate::codegen::structs_codegen::tables::countries::Country::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::DigitalAssetModel => {
                crate::codegen::structs_codegen::tables::digital_asset_models::DigitalAssetModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::DigitalAsset => {
                crate::codegen::structs_codegen::tables::digital_assets::DigitalAsset::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::DisposalProcedureTemplate => {
                crate::codegen::structs_codegen::tables::disposal_procedure_templates::DisposalProcedureTemplate::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::DisposalProcedure => {
                crate::codegen::structs_codegen::tables::disposal_procedures::DisposalProcedure::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::EmailProvider => {
                crate::codegen::structs_codegen::tables::email_providers::EmailProvider::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::FractioningProcedureTemplate => {
                crate::codegen::structs_codegen::tables::fractioning_procedure_templates::FractioningProcedureTemplate::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::FractioningProcedure => {
                crate::codegen::structs_codegen::tables::fractioning_procedures::FractioningProcedure::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::FreezeDryerModel => {
                crate::codegen::structs_codegen::tables::freeze_dryer_models::FreezeDryerModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::FreezeDryer => {
                crate::codegen::structs_codegen::tables::freeze_dryers::FreezeDryer::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::FreezeDryingProcedureTemplate => {
                crate::codegen::structs_codegen::tables::freeze_drying_procedure_templates::FreezeDryingProcedureTemplate::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::FreezeDryingProcedure => {
                crate::codegen::structs_codegen::tables::freeze_drying_procedures::FreezeDryingProcedure::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::FreezerModel => {
                crate::codegen::structs_codegen::tables::freezer_models::FreezerModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Freezer => {
                crate::codegen::structs_codegen::tables::freezers::Freezer::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::FreezingProcedureTemplate => {
                crate::codegen::structs_codegen::tables::freezing_procedure_templates::FreezingProcedureTemplate::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::FreezingProcedure => {
                crate::codegen::structs_codegen::tables::freezing_procedures::FreezingProcedure::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::GeolocationProcedureTemplate => {
                crate::codegen::structs_codegen::tables::geolocation_procedure_templates::GeolocationProcedureTemplate::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::GeolocationProcedure => {
                crate::codegen::structs_codegen::tables::geolocation_procedures::GeolocationProcedure::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::HarvestingProcedureTemplate => {
                crate::codegen::structs_codegen::tables::harvesting_procedure_templates::HarvestingProcedureTemplate::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::HarvestingProcedure => {
                crate::codegen::structs_codegen::tables::harvesting_procedures::HarvestingProcedure::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::InstrumentState => {
                crate::codegen::structs_codegen::tables::instrument_states::InstrumentState::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::LoginProvider => {
                crate::codegen::structs_codegen::tables::login_providers::LoginProvider::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Material => {
                crate::codegen::structs_codegen::tables::materials::Material::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::NextProcedureTemplate => {
                crate::codegen::structs_codegen::tables::next_procedure_templates::NextProcedureTemplate::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::ObservationSubject => {
                crate::codegen::structs_codegen::tables::observation_subjects::ObservationSubject::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::OrganismModel => {
                crate::codegen::structs_codegen::tables::organism_models::OrganismModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::OrganismTaxon => {
                crate::codegen::structs_codegen::tables::organism_taxa::OrganismTaxon::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Organism => {
                crate::codegen::structs_codegen::tables::organisms::Organism::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Organization => {
                crate::codegen::structs_codegen::tables::organizations::Organization::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::PackagingModel => {
                crate::codegen::structs_codegen::tables::packaging_models::PackagingModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::PackagingProcedureTemplate => {
                crate::codegen::structs_codegen::tables::packaging_procedure_templates::PackagingProcedureTemplate::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::PackagingProcedure => {
                crate::codegen::structs_codegen::tables::packaging_procedures::PackagingProcedure::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::ParentProcedureTemplate => {
                crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::PermanenceCategory => {
                crate::codegen::structs_codegen::tables::permanence_categories::PermanenceCategory::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::PersonalProtectiveEquipmentModel => {
                crate::codegen::structs_codegen::tables::personal_protective_equipment_models::PersonalProtectiveEquipmentModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::PhoneModel => {
                crate::codegen::structs_codegen::tables::phone_models::PhoneModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::PhotographProcedureTemplate => {
                crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::PhotographProcedure => {
                crate::codegen::structs_codegen::tables::photograph_procedures::PhotographProcedure::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Photograph => {
                crate::codegen::structs_codegen::tables::photographs::Photograph::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::PhysicalAssetModel => {
                crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::PhysicalAsset => {
                crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::PipetteModel => {
                crate::codegen::structs_codegen::tables::pipette_models::PipetteModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::PipetteTipModel => {
                crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Pipette => {
                crate::codegen::structs_codegen::tables::pipettes::Pipette::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::PositioningDeviceModel => {
                crate::codegen::structs_codegen::tables::positioning_device_models::PositioningDeviceModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::PositioningDevice => {
                crate::codegen::structs_codegen::tables::positioning_devices::PositioningDevice::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::PouringProcedureTemplate => {
                crate::codegen::structs_codegen::tables::pouring_procedure_templates::PouringProcedureTemplate::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::PouringProcedure => {
                crate::codegen::structs_codegen::tables::pouring_procedures::PouringProcedure::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::PpeReminderProcedureTemplate => {
                crate::codegen::structs_codegen::tables::ppe_reminder_procedure_templates::PpeReminderProcedureTemplate::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::PpeReminderProcedure => {
                crate::codegen::structs_codegen::tables::ppe_reminder_procedures::PpeReminderProcedure::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::ProcedureAsset => {
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::ProcedureTemplateAssetModel => {
                crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::ProcedureTemplate => {
                crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Procedure => {
                crate::codegen::structs_codegen::tables::procedures::Procedure::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::ProjectState => {
                crate::codegen::structs_codegen::tables::project_states::ProjectState::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Project => {
                crate::codegen::structs_codegen::tables::projects::Project::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Rank => {
                crate::codegen::structs_codegen::tables::ranks::Rank::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::ReagentModel => {
                crate::codegen::structs_codegen::tables::reagent_models::ReagentModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Role => {
                crate::codegen::structs_codegen::tables::roles::Role::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Room => {
                crate::codegen::structs_codegen::tables::rooms::Room::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::SampleModel => {
                crate::codegen::structs_codegen::tables::sample_models::SampleModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::SampleSourceModel => {
                crate::codegen::structs_codegen::tables::sample_source_models::SampleSourceModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::SampleSource => {
                crate::codegen::structs_codegen::tables::sample_sources::SampleSource::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::SampleState => {
                crate::codegen::structs_codegen::tables::sample_states::SampleState::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Sample => {
                crate::codegen::structs_codegen::tables::samples::Sample::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::SoilModel => {
                crate::codegen::structs_codegen::tables::soil_models::SoilModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Soil => {
                crate::codegen::structs_codegen::tables::soils::Soil::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::SpatialRefSy => {
                crate::codegen::structs_codegen::tables::spatial_ref_sys::SpatialRefSy::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Spectrum => {
                crate::codegen::structs_codegen::tables::spectra::Spectrum::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::SpectraCollection => {
                crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::StorageProcedureTemplate => {
                crate::codegen::structs_codegen::tables::storage_procedure_templates::StorageProcedureTemplate::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::StorageProcedure => {
                crate::codegen::structs_codegen::tables::storage_procedures::StorageProcedure::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::SupernatantProcedureTemplate => {
                crate::codegen::structs_codegen::tables::supernatant_procedure_templates::SupernatantProcedureTemplate::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::SupernatantProcedure => {
                crate::codegen::structs_codegen::tables::supernatant_procedures::SupernatantProcedure::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::TaggingProcedureTemplate => {
                crate::codegen::structs_codegen::tables::tagging_procedure_templates::TaggingProcedureTemplate::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::TaggingProcedure => {
                crate::codegen::structs_codegen::tables::tagging_procedures::TaggingProcedure::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Taxon => {
                crate::codegen::structs_codegen::tables::taxa::Taxon::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::TeamMember => {
                crate::codegen::structs_codegen::tables::team_members::TeamMember::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::TeamProject => {
                crate::codegen::structs_codegen::tables::team_projects::TeamProject::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::TeamState => {
                crate::codegen::structs_codegen::tables::team_states::TeamState::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Team => {
                crate::codegen::structs_codegen::tables::teams::Team::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::TemporaryUser => {
                crate::codegen::structs_codegen::tables::temporary_user::TemporaryUser::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Unit => {
                crate::codegen::structs_codegen::tables::units::Unit::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::UserEmail => {
                crate::codegen::structs_codegen::tables::user_emails::UserEmail::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::UserOrganization => {
                crate::codegen::structs_codegen::tables::user_organizations::UserOrganization::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::User => {
                crate::codegen::structs_codegen::tables::users::User::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::VolumeMeasuringDeviceModel => {
                crate::codegen::structs_codegen::tables::volume_measuring_device_models::VolumeMeasuringDeviceModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::VolumeMeasuringDevice => {
                crate::codegen::structs_codegen::tables::volume_measuring_devices::VolumeMeasuringDevice::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::VolumetricContainerModel => {
                crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::VolumetricContainer => {
                crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::WeighingDeviceModel => {
                crate::codegen::structs_codegen::tables::weighing_device_models::WeighingDeviceModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::WeighingDevice => {
                crate::codegen::structs_codegen::tables::weighing_devices::WeighingDevice::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::WeighingProcedureTemplate => {
                crate::codegen::structs_codegen::tables::weighing_procedure_templates::WeighingProcedureTemplate::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::WeighingProcedure => {
                crate::codegen::structs_codegen::tables::weighing_procedures::WeighingProcedure::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
        }
    }
}
