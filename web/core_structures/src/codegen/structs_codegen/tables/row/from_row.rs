impl From<super::Row> for crate::codegen::tables::rows::Rows {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::Address(addresses) => crate::codegen::tables::rows::Rows::from(addresses),
            super::Row::AliquotingProcedureTemplate(aliquoting_procedure_templates) => {
                crate::codegen::tables::rows::Rows::from(aliquoting_procedure_templates)
            }
            super::Row::AliquotingProcedure(aliquoting_procedures) => {
                crate::codegen::tables::rows::Rows::from(aliquoting_procedures)
            }
            super::Row::AssetCompatibilityRule(asset_compatibility_rules) => {
                crate::codegen::tables::rows::Rows::from(asset_compatibility_rules)
            }
            super::Row::AssetModelAncestor(asset_model_ancestors) => {
                crate::codegen::tables::rows::Rows::from(asset_model_ancestors)
            }
            super::Row::AssetModel(asset_models) => {
                crate::codegen::tables::rows::Rows::from(asset_models)
            }
            super::Row::Asset(assets) => crate::codegen::tables::rows::Rows::from(assets),
            super::Row::BallMillMachineModel(ball_mill_machine_models) => {
                crate::codegen::tables::rows::Rows::from(ball_mill_machine_models)
            }
            super::Row::BallMillMachine(ball_mill_machines) => {
                crate::codegen::tables::rows::Rows::from(ball_mill_machines)
            }
            super::Row::BallMillProcedureTemplate(ball_mill_procedure_templates) => {
                crate::codegen::tables::rows::Rows::from(ball_mill_procedure_templates)
            }
            super::Row::BallMillProcedure(ball_mill_procedures) => {
                crate::codegen::tables::rows::Rows::from(ball_mill_procedures)
            }
            super::Row::BeadModel(bead_models) => {
                crate::codegen::tables::rows::Rows::from(bead_models)
            }
            super::Row::Brand(brands) => crate::codegen::tables::rows::Rows::from(brands),
            super::Row::CameraModel(camera_models) => {
                crate::codegen::tables::rows::Rows::from(camera_models)
            }
            super::Row::Camera(cameras) => crate::codegen::tables::rows::Rows::from(cameras),
            super::Row::CapModel(cap_models) => {
                crate::codegen::tables::rows::Rows::from(cap_models)
            }
            super::Row::CappingProcedureTemplate(capping_procedure_templates) => {
                crate::codegen::tables::rows::Rows::from(capping_procedure_templates)
            }
            super::Row::CappingProcedure(capping_procedures) => {
                crate::codegen::tables::rows::Rows::from(capping_procedures)
            }
            super::Row::CentrifugeModel(centrifuge_models) => {
                crate::codegen::tables::rows::Rows::from(centrifuge_models)
            }
            super::Row::CentrifugeProcedureTemplate(centrifuge_procedure_templates) => {
                crate::codegen::tables::rows::Rows::from(centrifuge_procedure_templates)
            }
            super::Row::CentrifugeProcedure(centrifuge_procedures) => {
                crate::codegen::tables::rows::Rows::from(centrifuge_procedures)
            }
            super::Row::Centrifuge(centrifuges) => {
                crate::codegen::tables::rows::Rows::from(centrifuges)
            }
            super::Row::City(cities) => crate::codegen::tables::rows::Rows::from(cities),
            super::Row::CleaningProcedureTemplate(cleaning_procedure_templates) => {
                crate::codegen::tables::rows::Rows::from(cleaning_procedure_templates)
            }
            super::Row::CleaningProcedure(cleaning_procedures) => {
                crate::codegen::tables::rows::Rows::from(cleaning_procedures)
            }
            super::Row::Color(colors) => crate::codegen::tables::rows::Rows::from(colors),
            super::Row::CommercialBallMillMachineLot(commercial_ball_mill_machine_lots) => {
                crate::codegen::tables::rows::Rows::from(commercial_ball_mill_machine_lots)
            }
            super::Row::CommercialBallMillMachineModel(commercial_ball_mill_machine_models) => {
                crate::codegen::tables::rows::Rows::from(commercial_ball_mill_machine_models)
            }
            super::Row::CommercialBeadLot(commercial_bead_lots) => {
                crate::codegen::tables::rows::Rows::from(commercial_bead_lots)
            }
            super::Row::CommercialBeadModel(commercial_bead_models) => {
                crate::codegen::tables::rows::Rows::from(commercial_bead_models)
            }
            super::Row::CommercialCameraLot(commercial_camera_lots) => {
                crate::codegen::tables::rows::Rows::from(commercial_camera_lots)
            }
            super::Row::CommercialCameraModel(commercial_camera_models) => {
                crate::codegen::tables::rows::Rows::from(commercial_camera_models)
            }
            super::Row::CommercialCapLot(commercial_cap_lots) => {
                crate::codegen::tables::rows::Rows::from(commercial_cap_lots)
            }
            super::Row::CommercialCapModel(commercial_cap_models) => {
                crate::codegen::tables::rows::Rows::from(commercial_cap_models)
            }
            super::Row::CommercialCentrifugeLot(commercial_centrifuge_lots) => {
                crate::codegen::tables::rows::Rows::from(commercial_centrifuge_lots)
            }
            super::Row::CommercialCentrifugeModel(commercial_centrifuge_models) => {
                crate::codegen::tables::rows::Rows::from(commercial_centrifuge_models)
            }
            super::Row::CommercialFreezeDryerLot(commercial_freeze_dryer_lots) => {
                crate::codegen::tables::rows::Rows::from(commercial_freeze_dryer_lots)
            }
            super::Row::CommercialFreezeDryerModel(commercial_freeze_dryer_models) => {
                crate::codegen::tables::rows::Rows::from(commercial_freeze_dryer_models)
            }
            super::Row::CommercialFreezerLot(commercial_freezer_lots) => {
                crate::codegen::tables::rows::Rows::from(commercial_freezer_lots)
            }
            super::Row::CommercialFreezerModel(commercial_freezer_models) => {
                crate::codegen::tables::rows::Rows::from(commercial_freezer_models)
            }
            super::Row::CommercialPackagingLot(commercial_packaging_lots) => {
                crate::codegen::tables::rows::Rows::from(commercial_packaging_lots)
            }
            super::Row::CommercialPackagingModel(commercial_packaging_models) => {
                crate::codegen::tables::rows::Rows::from(commercial_packaging_models)
            }
            super::Row::CommercialPipetteLot(commercial_pipette_lots) => {
                crate::codegen::tables::rows::Rows::from(commercial_pipette_lots)
            }
            super::Row::CommercialPipetteModel(commercial_pipette_models) => {
                crate::codegen::tables::rows::Rows::from(commercial_pipette_models)
            }
            super::Row::CommercialPipetteTipLot(commercial_pipette_tip_lots) => {
                crate::codegen::tables::rows::Rows::from(commercial_pipette_tip_lots)
            }
            super::Row::CommercialPipetteTipModel(commercial_pipette_tip_models) => {
                crate::codegen::tables::rows::Rows::from(commercial_pipette_tip_models)
            }
            super::Row::CommercialPositioningDeviceLot(commercial_positioning_device_lots) => {
                crate::codegen::tables::rows::Rows::from(commercial_positioning_device_lots)
            }
            super::Row::CommercialPositioningDeviceModel(commercial_positioning_device_models) => {
                crate::codegen::tables::rows::Rows::from(commercial_positioning_device_models)
            }
            super::Row::CommercialProductLot(commercial_product_lots) => {
                crate::codegen::tables::rows::Rows::from(commercial_product_lots)
            }
            super::Row::CommercialProduct(commercial_products) => {
                crate::codegen::tables::rows::Rows::from(commercial_products)
            }
            super::Row::CommercialVolumeMeasuringDeviceLot(
                commercial_volume_measuring_device_lots,
            ) => crate::codegen::tables::rows::Rows::from(commercial_volume_measuring_device_lots),
            super::Row::CommercialVolumeMeasuringDeviceModel(
                commercial_volume_measuring_device_models,
            ) => {
                crate::codegen::tables::rows::Rows::from(commercial_volume_measuring_device_models)
            }
            super::Row::CommercialWeighingDeviceLot(commercial_weighing_device_lots) => {
                crate::codegen::tables::rows::Rows::from(commercial_weighing_device_lots)
            }
            super::Row::CommercialWeighingDeviceModel(commercial_weighing_device_models) => {
                crate::codegen::tables::rows::Rows::from(commercial_weighing_device_models)
            }
            super::Row::ContainerCompatibilityRule(container_compatibility_rules) => {
                crate::codegen::tables::rows::Rows::from(container_compatibility_rules)
            }
            super::Row::ContainerModel(container_models) => {
                crate::codegen::tables::rows::Rows::from(container_models)
            }
            super::Row::Container(containers) => {
                crate::codegen::tables::rows::Rows::from(containers)
            }
            super::Row::Country(countries) => crate::codegen::tables::rows::Rows::from(countries),
            super::Row::DigitalAssetModel(digital_asset_models) => {
                crate::codegen::tables::rows::Rows::from(digital_asset_models)
            }
            super::Row::DigitalAsset(digital_assets) => {
                crate::codegen::tables::rows::Rows::from(digital_assets)
            }
            super::Row::DisposalProcedureTemplate(disposal_procedure_templates) => {
                crate::codegen::tables::rows::Rows::from(disposal_procedure_templates)
            }
            super::Row::DisposalProcedure(disposal_procedures) => {
                crate::codegen::tables::rows::Rows::from(disposal_procedures)
            }
            super::Row::EmailProvider(email_providers) => {
                crate::codegen::tables::rows::Rows::from(email_providers)
            }
            super::Row::FractioningProcedureTemplate(fractioning_procedure_templates) => {
                crate::codegen::tables::rows::Rows::from(fractioning_procedure_templates)
            }
            super::Row::FractioningProcedure(fractioning_procedures) => {
                crate::codegen::tables::rows::Rows::from(fractioning_procedures)
            }
            super::Row::FreezeDryerModel(freeze_dryer_models) => {
                crate::codegen::tables::rows::Rows::from(freeze_dryer_models)
            }
            super::Row::FreezeDryer(freeze_dryers) => {
                crate::codegen::tables::rows::Rows::from(freeze_dryers)
            }
            super::Row::FreezeDryingProcedureTemplate(freeze_drying_procedure_templates) => {
                crate::codegen::tables::rows::Rows::from(freeze_drying_procedure_templates)
            }
            super::Row::FreezeDryingProcedure(freeze_drying_procedures) => {
                crate::codegen::tables::rows::Rows::from(freeze_drying_procedures)
            }
            super::Row::FreezerModel(freezer_models) => {
                crate::codegen::tables::rows::Rows::from(freezer_models)
            }
            super::Row::Freezer(freezers) => crate::codegen::tables::rows::Rows::from(freezers),
            super::Row::FreezingProcedureTemplate(freezing_procedure_templates) => {
                crate::codegen::tables::rows::Rows::from(freezing_procedure_templates)
            }
            super::Row::FreezingProcedure(freezing_procedures) => {
                crate::codegen::tables::rows::Rows::from(freezing_procedures)
            }
            super::Row::GeolocationProcedureTemplate(geolocation_procedure_templates) => {
                crate::codegen::tables::rows::Rows::from(geolocation_procedure_templates)
            }
            super::Row::GeolocationProcedure(geolocation_procedures) => {
                crate::codegen::tables::rows::Rows::from(geolocation_procedures)
            }
            super::Row::HarvestingProcedureTemplate(harvesting_procedure_templates) => {
                crate::codegen::tables::rows::Rows::from(harvesting_procedure_templates)
            }
            super::Row::HarvestingProcedure(harvesting_procedures) => {
                crate::codegen::tables::rows::Rows::from(harvesting_procedures)
            }
            super::Row::InstrumentState(instrument_states) => {
                crate::codegen::tables::rows::Rows::from(instrument_states)
            }
            super::Row::LoginProvider(login_providers) => {
                crate::codegen::tables::rows::Rows::from(login_providers)
            }
            super::Row::Material(materials) => crate::codegen::tables::rows::Rows::from(materials),
            super::Row::NextProcedureTemplate(next_procedure_templates) => {
                crate::codegen::tables::rows::Rows::from(next_procedure_templates)
            }
            super::Row::ObservationSubject(observation_subjects) => {
                crate::codegen::tables::rows::Rows::from(observation_subjects)
            }
            super::Row::OrganismModel(organism_models) => {
                crate::codegen::tables::rows::Rows::from(organism_models)
            }
            super::Row::OrganismTaxon(organism_taxa) => {
                crate::codegen::tables::rows::Rows::from(organism_taxa)
            }
            super::Row::Organism(organisms) => crate::codegen::tables::rows::Rows::from(organisms),
            super::Row::Organization(organizations) => {
                crate::codegen::tables::rows::Rows::from(organizations)
            }
            super::Row::PackagingModel(packaging_models) => {
                crate::codegen::tables::rows::Rows::from(packaging_models)
            }
            super::Row::PackagingProcedureTemplate(packaging_procedure_templates) => {
                crate::codegen::tables::rows::Rows::from(packaging_procedure_templates)
            }
            super::Row::PackagingProcedure(packaging_procedures) => {
                crate::codegen::tables::rows::Rows::from(packaging_procedures)
            }
            super::Row::ParentProcedureTemplate(parent_procedure_templates) => {
                crate::codegen::tables::rows::Rows::from(parent_procedure_templates)
            }
            super::Row::PermanenceCategory(permanence_categories) => {
                crate::codegen::tables::rows::Rows::from(permanence_categories)
            }
            super::Row::PersonalProtectiveEquipmentModel(personal_protective_equipment_models) => {
                crate::codegen::tables::rows::Rows::from(personal_protective_equipment_models)
            }
            super::Row::PhoneModel(phone_models) => {
                crate::codegen::tables::rows::Rows::from(phone_models)
            }
            super::Row::PhotographProcedureTemplate(photograph_procedure_templates) => {
                crate::codegen::tables::rows::Rows::from(photograph_procedure_templates)
            }
            super::Row::PhotographProcedure(photograph_procedures) => {
                crate::codegen::tables::rows::Rows::from(photograph_procedures)
            }
            super::Row::Photograph(photographs) => {
                crate::codegen::tables::rows::Rows::from(photographs)
            }
            super::Row::PhysicalAssetModel(physical_asset_models) => {
                crate::codegen::tables::rows::Rows::from(physical_asset_models)
            }
            super::Row::PhysicalAsset(physical_assets) => {
                crate::codegen::tables::rows::Rows::from(physical_assets)
            }
            super::Row::PipetteModel(pipette_models) => {
                crate::codegen::tables::rows::Rows::from(pipette_models)
            }
            super::Row::PipetteTipModel(pipette_tip_models) => {
                crate::codegen::tables::rows::Rows::from(pipette_tip_models)
            }
            super::Row::Pipette(pipettes) => crate::codegen::tables::rows::Rows::from(pipettes),
            super::Row::PositioningDeviceModel(positioning_device_models) => {
                crate::codegen::tables::rows::Rows::from(positioning_device_models)
            }
            super::Row::PositioningDevice(positioning_devices) => {
                crate::codegen::tables::rows::Rows::from(positioning_devices)
            }
            super::Row::PouringProcedureTemplate(pouring_procedure_templates) => {
                crate::codegen::tables::rows::Rows::from(pouring_procedure_templates)
            }
            super::Row::PouringProcedure(pouring_procedures) => {
                crate::codegen::tables::rows::Rows::from(pouring_procedures)
            }
            super::Row::PpeReminderProcedureTemplate(ppe_reminder_procedure_templates) => {
                crate::codegen::tables::rows::Rows::from(ppe_reminder_procedure_templates)
            }
            super::Row::PpeReminderProcedure(ppe_reminder_procedures) => {
                crate::codegen::tables::rows::Rows::from(ppe_reminder_procedures)
            }
            super::Row::ProcedureAsset(procedure_assets) => {
                crate::codegen::tables::rows::Rows::from(procedure_assets)
            }
            super::Row::ProcedureTemplateAssetModel(procedure_template_asset_models) => {
                crate::codegen::tables::rows::Rows::from(procedure_template_asset_models)
            }
            super::Row::ProcedureTemplate(procedure_templates) => {
                crate::codegen::tables::rows::Rows::from(procedure_templates)
            }
            super::Row::Procedure(procedures) => {
                crate::codegen::tables::rows::Rows::from(procedures)
            }
            super::Row::ProjectState(project_states) => {
                crate::codegen::tables::rows::Rows::from(project_states)
            }
            super::Row::Project(projects) => crate::codegen::tables::rows::Rows::from(projects),
            super::Row::Rank(ranks) => crate::codegen::tables::rows::Rows::from(ranks),
            super::Row::ReagentModel(reagent_models) => {
                crate::codegen::tables::rows::Rows::from(reagent_models)
            }
            super::Row::Role(roles) => crate::codegen::tables::rows::Rows::from(roles),
            super::Row::Room(rooms) => crate::codegen::tables::rows::Rows::from(rooms),
            super::Row::SampleModel(sample_models) => {
                crate::codegen::tables::rows::Rows::from(sample_models)
            }
            super::Row::SampleSourceModel(sample_source_models) => {
                crate::codegen::tables::rows::Rows::from(sample_source_models)
            }
            super::Row::SampleSource(sample_sources) => {
                crate::codegen::tables::rows::Rows::from(sample_sources)
            }
            super::Row::SampleState(sample_states) => {
                crate::codegen::tables::rows::Rows::from(sample_states)
            }
            super::Row::Sample(samples) => crate::codegen::tables::rows::Rows::from(samples),
            super::Row::SoilModel(soil_models) => {
                crate::codegen::tables::rows::Rows::from(soil_models)
            }
            super::Row::Soil(soils) => crate::codegen::tables::rows::Rows::from(soils),
            super::Row::SpatialRefSy(spatial_ref_sys) => {
                crate::codegen::tables::rows::Rows::from(spatial_ref_sys)
            }
            super::Row::Spectrum(spectra) => crate::codegen::tables::rows::Rows::from(spectra),
            super::Row::SpectraCollection(spectra_collections) => {
                crate::codegen::tables::rows::Rows::from(spectra_collections)
            }
            super::Row::StorageProcedureTemplate(storage_procedure_templates) => {
                crate::codegen::tables::rows::Rows::from(storage_procedure_templates)
            }
            super::Row::StorageProcedure(storage_procedures) => {
                crate::codegen::tables::rows::Rows::from(storage_procedures)
            }
            super::Row::SupernatantProcedureTemplate(supernatant_procedure_templates) => {
                crate::codegen::tables::rows::Rows::from(supernatant_procedure_templates)
            }
            super::Row::SupernatantProcedure(supernatant_procedures) => {
                crate::codegen::tables::rows::Rows::from(supernatant_procedures)
            }
            super::Row::TaggingProcedureTemplate(tagging_procedure_templates) => {
                crate::codegen::tables::rows::Rows::from(tagging_procedure_templates)
            }
            super::Row::TaggingProcedure(tagging_procedures) => {
                crate::codegen::tables::rows::Rows::from(tagging_procedures)
            }
            super::Row::Taxon(taxa) => crate::codegen::tables::rows::Rows::from(taxa),
            super::Row::TeamMember(team_members) => {
                crate::codegen::tables::rows::Rows::from(team_members)
            }
            super::Row::TeamProject(team_projects) => {
                crate::codegen::tables::rows::Rows::from(team_projects)
            }
            super::Row::TeamState(team_states) => {
                crate::codegen::tables::rows::Rows::from(team_states)
            }
            super::Row::Team(teams) => crate::codegen::tables::rows::Rows::from(teams),
            super::Row::TemporaryUser(temporary_user) => {
                crate::codegen::tables::rows::Rows::from(temporary_user)
            }
            super::Row::Unit(units) => crate::codegen::tables::rows::Rows::from(units),
            super::Row::UserEmail(user_emails) => {
                crate::codegen::tables::rows::Rows::from(user_emails)
            }
            super::Row::UserOrganization(user_organizations) => {
                crate::codegen::tables::rows::Rows::from(user_organizations)
            }
            super::Row::User(users) => crate::codegen::tables::rows::Rows::from(users),
            super::Row::VolumeMeasuringDeviceModel(volume_measuring_device_models) => {
                crate::codegen::tables::rows::Rows::from(volume_measuring_device_models)
            }
            super::Row::VolumeMeasuringDevice(volume_measuring_devices) => {
                crate::codegen::tables::rows::Rows::from(volume_measuring_devices)
            }
            super::Row::VolumetricContainerModel(volumetric_container_models) => {
                crate::codegen::tables::rows::Rows::from(volumetric_container_models)
            }
            super::Row::VolumetricContainer(volumetric_containers) => {
                crate::codegen::tables::rows::Rows::from(volumetric_containers)
            }
            super::Row::WeighingDeviceModel(weighing_device_models) => {
                crate::codegen::tables::rows::Rows::from(weighing_device_models)
            }
            super::Row::WeighingDevice(weighing_devices) => {
                crate::codegen::tables::rows::Rows::from(weighing_devices)
            }
            super::Row::WeighingProcedureTemplate(weighing_procedure_templates) => {
                crate::codegen::tables::rows::Rows::from(weighing_procedure_templates)
            }
            super::Row::WeighingProcedure(weighing_procedures) => {
                crate::codegen::tables::rows::Rows::from(weighing_procedures)
            }
        }
    }
}
