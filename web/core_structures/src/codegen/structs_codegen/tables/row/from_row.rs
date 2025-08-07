impl From<super::Row> for crate::codegen::tables::rows::Rows {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::Address(addresses) => crate::codegen::tables::rows::Rows::from(addresses),
            super::Row::AliquotingProcedureModel(aliquoting_procedure_models) => {
                crate::codegen::tables::rows::Rows::from(aliquoting_procedure_models)
            }
            super::Row::AliquotingProcedure(aliquoting_procedures) => {
                crate::codegen::tables::rows::Rows::from(aliquoting_procedures)
            }
            super::Row::BallMillMachineModel(ball_mill_machine_models) => {
                crate::codegen::tables::rows::Rows::from(ball_mill_machine_models)
            }
            super::Row::BallMillProcedureModel(ball_mill_procedure_models) => {
                crate::codegen::tables::rows::Rows::from(ball_mill_procedure_models)
            }
            super::Row::BinaryQuestionProcedureModel(binary_question_procedure_models) => {
                crate::codegen::tables::rows::Rows::from(binary_question_procedure_models)
            }
            super::Row::Brand(brands) => crate::codegen::tables::rows::Rows::from(brands),
            super::Row::CameraModel(camera_models) => {
                crate::codegen::tables::rows::Rows::from(camera_models)
            }
            super::Row::CappingProcedureModel(capping_procedure_models) => {
                crate::codegen::tables::rows::Rows::from(capping_procedure_models)
            }
            super::Row::CentrifugeModel(centrifuge_models) => {
                crate::codegen::tables::rows::Rows::from(centrifuge_models)
            }
            super::Row::CentrifugeProcedureModel(centrifuge_procedure_models) => {
                crate::codegen::tables::rows::Rows::from(centrifuge_procedure_models)
            }
            super::Row::City(cities) => crate::codegen::tables::rows::Rows::from(cities),
            super::Row::Color(colors) => crate::codegen::tables::rows::Rows::from(colors),
            super::Row::CommercialProductLot(commercial_product_lots) => {
                crate::codegen::tables::rows::Rows::from(commercial_product_lots)
            }
            super::Row::CommercialProduct(commercial_products) => {
                crate::codegen::tables::rows::Rows::from(commercial_products)
            }
            super::Row::CommercialReagent(commercial_reagents) => {
                crate::codegen::tables::rows::Rows::from(commercial_reagents)
            }
            super::Row::CompatibilityRule(compatibility_rules) => {
                crate::codegen::tables::rows::Rows::from(compatibility_rules)
            }
            super::Row::ContainerModel(container_models) => {
                crate::codegen::tables::rows::Rows::from(container_models)
            }
            super::Row::Container(containers) => {
                crate::codegen::tables::rows::Rows::from(containers)
            }
            super::Row::Country(countries) => crate::codegen::tables::rows::Rows::from(countries),
            super::Row::DisposalProcedureModel(disposal_procedure_models) => {
                crate::codegen::tables::rows::Rows::from(disposal_procedure_models)
            }
            super::Row::Document(documents) => crate::codegen::tables::rows::Rows::from(documents),
            super::Row::EmailProvider(email_providers) => {
                crate::codegen::tables::rows::Rows::from(email_providers)
            }
            super::Row::FractioningProcedureModel(fractioning_procedure_models) => {
                crate::codegen::tables::rows::Rows::from(fractioning_procedure_models)
            }
            super::Row::FreezeDrierModel(freeze_drier_models) => {
                crate::codegen::tables::rows::Rows::from(freeze_drier_models)
            }
            super::Row::FreezeDryingProcedureModel(freeze_drying_procedure_models) => {
                crate::codegen::tables::rows::Rows::from(freeze_drying_procedure_models)
            }
            super::Row::FreezerModel(freezer_models) => {
                crate::codegen::tables::rows::Rows::from(freezer_models)
            }
            super::Row::FreezingProcedureModel(freezing_procedure_models) => {
                crate::codegen::tables::rows::Rows::from(freezing_procedure_models)
            }
            super::Row::GeolocationProcedureModel(geolocation_procedure_models) => {
                crate::codegen::tables::rows::Rows::from(geolocation_procedure_models)
            }
            super::Row::InstrumentModel(instrument_models) => {
                crate::codegen::tables::rows::Rows::from(instrument_models)
            }
            super::Row::InstrumentState(instrument_states) => {
                crate::codegen::tables::rows::Rows::from(instrument_states)
            }
            super::Row::LoginProvider(login_providers) => {
                crate::codegen::tables::rows::Rows::from(login_providers)
            }
            super::Row::Material(materials) => crate::codegen::tables::rows::Rows::from(materials),
            super::Row::MixingProcedureModel(mixing_procedure_models) => {
                crate::codegen::tables::rows::Rows::from(mixing_procedure_models)
            }
            super::Row::NextProcedureModel(next_procedure_models) => {
                crate::codegen::tables::rows::Rows::from(next_procedure_models)
            }
            super::Row::ObservationSubject(observation_subjects) => {
                crate::codegen::tables::rows::Rows::from(observation_subjects)
            }
            super::Row::OrganismTaxon(organism_taxa) => {
                crate::codegen::tables::rows::Rows::from(organism_taxa)
            }
            super::Row::Organism(organisms) => crate::codegen::tables::rows::Rows::from(organisms),
            super::Row::Organization(organizations) => {
                crate::codegen::tables::rows::Rows::from(organizations)
            }
            super::Row::PackagingProcedureModel(packaging_procedure_models) => {
                crate::codegen::tables::rows::Rows::from(packaging_procedure_models)
            }
            super::Row::ParentProcedureModel(parent_procedure_models) => {
                crate::codegen::tables::rows::Rows::from(parent_procedure_models)
            }
            super::Row::PermanenceCategory(permanence_categories) => {
                crate::codegen::tables::rows::Rows::from(permanence_categories)
            }
            super::Row::PhoneModel(phone_models) => {
                crate::codegen::tables::rows::Rows::from(phone_models)
            }
            super::Row::PhotographProcedureModel(photograph_procedure_models) => {
                crate::codegen::tables::rows::Rows::from(photograph_procedure_models)
            }
            super::Row::PipetteModel(pipette_models) => {
                crate::codegen::tables::rows::Rows::from(pipette_models)
            }
            super::Row::PipetteTipModel(pipette_tip_models) => {
                crate::codegen::tables::rows::Rows::from(pipette_tip_models)
            }
            super::Row::PlacingProcedureModel(placing_procedure_models) => {
                crate::codegen::tables::rows::Rows::from(placing_procedure_models)
            }
            super::Row::PositioningDeviceModel(positioning_device_models) => {
                crate::codegen::tables::rows::Rows::from(positioning_device_models)
            }
            super::Row::PouringProcedureModel(pouring_procedure_models) => {
                crate::codegen::tables::rows::Rows::from(pouring_procedure_models)
            }
            super::Row::ProcedureModelTrackable(procedure_model_trackables) => {
                crate::codegen::tables::rows::Rows::from(procedure_model_trackables)
            }
            super::Row::ProcedureModel(procedure_models) => {
                crate::codegen::tables::rows::Rows::from(procedure_models)
            }
            super::Row::ProcedureTrackable(procedure_trackables) => {
                crate::codegen::tables::rows::Rows::from(procedure_trackables)
            }
            super::Row::Procedure(procedures) => {
                crate::codegen::tables::rows::Rows::from(procedures)
            }
            super::Row::Processable(processables) => {
                crate::codegen::tables::rows::Rows::from(processables)
            }
            super::Row::ProjectState(project_states) => {
                crate::codegen::tables::rows::Rows::from(project_states)
            }
            super::Row::Project(projects) => crate::codegen::tables::rows::Rows::from(projects),
            super::Row::Rank(ranks) => crate::codegen::tables::rows::Rows::from(ranks),
            super::Row::Reagent(reagents) => crate::codegen::tables::rows::Rows::from(reagents),
            super::Row::Role(roles) => crate::codegen::tables::rows::Rows::from(roles),
            super::Row::Room(rooms) => crate::codegen::tables::rows::Rows::from(rooms),
            super::Row::SampleState(sample_states) => {
                crate::codegen::tables::rows::Rows::from(sample_states)
            }
            super::Row::SharedProcedureModelTrackable(shared_procedure_model_trackables) => {
                crate::codegen::tables::rows::Rows::from(shared_procedure_model_trackables)
            }
            super::Row::SpatialRefSy(spatial_ref_sys) => {
                crate::codegen::tables::rows::Rows::from(spatial_ref_sys)
            }
            super::Row::Spectrum(spectra) => crate::codegen::tables::rows::Rows::from(spectra),
            super::Row::SpectraCollection(spectra_collections) => {
                crate::codegen::tables::rows::Rows::from(spectra_collections)
            }
            super::Row::StorageProcedureModel(storage_procedure_models) => {
                crate::codegen::tables::rows::Rows::from(storage_procedure_models)
            }
            super::Row::SupernatantProcedureModel(supernatant_procedure_models) => {
                crate::codegen::tables::rows::Rows::from(supernatant_procedure_models)
            }
            super::Row::SupernatantProcedure(supernatant_procedures) => {
                crate::codegen::tables::rows::Rows::from(supernatant_procedures)
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
            super::Row::TrackableAncestor(trackable_ancestors) => {
                crate::codegen::tables::rows::Rows::from(trackable_ancestors)
            }
            super::Row::TrackableLocation(trackable_locations) => {
                crate::codegen::tables::rows::Rows::from(trackable_locations)
            }
            super::Row::Trackable(trackables) => {
                crate::codegen::tables::rows::Rows::from(trackables)
            }
            super::Row::Unit(units) => crate::codegen::tables::rows::Rows::from(units),
            super::Row::UserEmail(user_emails) => {
                crate::codegen::tables::rows::Rows::from(user_emails)
            }
            super::Row::UserOrganization(user_organizations) => {
                crate::codegen::tables::rows::Rows::from(user_organizations)
            }
            super::Row::User(users) => crate::codegen::tables::rows::Rows::from(users),
            super::Row::VolumetricContainerModel(volumetric_container_models) => {
                crate::codegen::tables::rows::Rows::from(volumetric_container_models)
            }
            super::Row::VolumetricProcessable(volumetric_processables) => {
                crate::codegen::tables::rows::Rows::from(volumetric_processables)
            }
            super::Row::WeighingInstrumentModel(weighing_instrument_models) => {
                crate::codegen::tables::rows::Rows::from(weighing_instrument_models)
            }
            super::Row::WeighingProcedureModel(weighing_procedure_models) => {
                crate::codegen::tables::rows::Rows::from(weighing_procedure_models)
            }
            super::Row::WeighingProcedure(weighing_procedures) => {
                crate::codegen::tables::rows::Rows::from(weighing_procedures)
            }
        }
    }
}
