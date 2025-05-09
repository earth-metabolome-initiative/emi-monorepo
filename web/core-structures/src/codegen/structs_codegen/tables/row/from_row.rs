impl From<super::Row> for crate::codegen::tables::rows::Rows {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::Address(addresses) => crate::codegen::tables::rows::Rows::from(addresses),
            super::Row::AliquotingInstrumentModel(aliquoting_instrument_models) => {
                crate::codegen::tables::rows::Rows::from(aliquoting_instrument_models)
            }
            super::Row::AliquotingStepModel(aliquoting_step_models) => {
                crate::codegen::tables::rows::Rows::from(aliquoting_step_models)
            }
            super::Row::AliquotingStep(aliquoting_steps) => {
                crate::codegen::tables::rows::Rows::from(aliquoting_steps)
            }
            super::Row::BallMillStepModel(ball_mill_step_models) => {
                crate::codegen::tables::rows::Rows::from(ball_mill_step_models)
            }
            super::Row::BallMillStep(ball_mill_steps) => {
                crate::codegen::tables::rows::Rows::from(ball_mill_steps)
            }
            super::Row::Brand(brands) => crate::codegen::tables::rows::Rows::from(brands),
            super::Row::CentrifugeStepModel(centrifuge_step_models) => {
                crate::codegen::tables::rows::Rows::from(centrifuge_step_models)
            }
            super::Row::CentrifugeStep(centrifuge_steps) => {
                crate::codegen::tables::rows::Rows::from(centrifuge_steps)
            }
            super::Row::City(cities) => crate::codegen::tables::rows::Rows::from(cities),
            super::Row::Color(colors) => crate::codegen::tables::rows::Rows::from(colors),
            super::Row::CommercialProductLot(commercial_product_lots) => {
                crate::codegen::tables::rows::Rows::from(commercial_product_lots)
            }
            super::Row::CommercialProduct(commercial_products) => {
                crate::codegen::tables::rows::Rows::from(commercial_products)
            }
            super::Row::CommercialReagentModel(commercial_reagent_models) => {
                crate::codegen::tables::rows::Rows::from(commercial_reagent_models)
            }
            super::Row::CommercialReagent(commercial_reagents) => {
                crate::codegen::tables::rows::Rows::from(commercial_reagents)
            }
            super::Row::ContainerModel(container_models) => {
                crate::codegen::tables::rows::Rows::from(container_models)
            }
            super::Row::Country(countries) => crate::codegen::tables::rows::Rows::from(countries),
            super::Row::DisposalStepModel(disposal_step_models) => {
                crate::codegen::tables::rows::Rows::from(disposal_step_models)
            }
            super::Row::DisposalStep(disposal_steps) => {
                crate::codegen::tables::rows::Rows::from(disposal_steps)
            }
            super::Row::DocumentFormat(document_formats) => {
                crate::codegen::tables::rows::Rows::from(document_formats)
            }
            super::Row::EmailProvider(email_providers) => {
                crate::codegen::tables::rows::Rows::from(email_providers)
            }
            super::Row::FractioningStepModel(fractioning_step_models) => {
                crate::codegen::tables::rows::Rows::from(fractioning_step_models)
            }
            super::Row::FractioningStep(fractioning_steps) => {
                crate::codegen::tables::rows::Rows::from(fractioning_steps)
            }
            super::Row::FreezeDryingStepModel(freeze_drying_step_models) => {
                crate::codegen::tables::rows::Rows::from(freeze_drying_step_models)
            }
            super::Row::InstrumentLocation(instrument_locations) => {
                crate::codegen::tables::rows::Rows::from(instrument_locations)
            }
            super::Row::InstrumentModelCategory(instrument_model_categories) => {
                crate::codegen::tables::rows::Rows::from(instrument_model_categories)
            }
            super::Row::InstrumentModel(instrument_models) => {
                crate::codegen::tables::rows::Rows::from(instrument_models)
            }
            super::Row::InstrumentState(instrument_states) => {
                crate::codegen::tables::rows::Rows::from(instrument_states)
            }
            super::Row::Instrument(instruments) => {
                crate::codegen::tables::rows::Rows::from(instruments)
            }
            super::Row::LoginProvider(login_providers) => {
                crate::codegen::tables::rows::Rows::from(login_providers)
            }
            super::Row::Material(materials) => crate::codegen::tables::rows::Rows::from(materials),
            super::Row::NameplateModel(nameplate_models) => {
                crate::codegen::tables::rows::Rows::from(nameplate_models)
            }
            super::Row::ObservationSubject(observation_subjects) => {
                crate::codegen::tables::rows::Rows::from(observation_subjects)
            }
            super::Row::OrganismObservation(organism_observations) => {
                crate::codegen::tables::rows::Rows::from(organism_observations)
            }
            super::Row::OrganismSamplingStepModel(organism_sampling_step_models) => {
                crate::codegen::tables::rows::Rows::from(organism_sampling_step_models)
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
            super::Row::PackagingStepModel(packaging_step_models) => {
                crate::codegen::tables::rows::Rows::from(packaging_step_models)
            }
            super::Row::PermanenceCategory(permanence_categories) => {
                crate::codegen::tables::rows::Rows::from(permanence_categories)
            }
            super::Row::Photograph(photographs) => {
                crate::codegen::tables::rows::Rows::from(photographs)
            }
            super::Row::ProcedureModelContainerCategory(procedure_model_container_categories) => {
                crate::codegen::tables::rows::Rows::from(procedure_model_container_categories)
            }
            super::Row::ProcedureModelInstrumentCategory(procedure_model_instrument_categories) => {
                crate::codegen::tables::rows::Rows::from(procedure_model_instrument_categories)
            }
            super::Row::ProcedureModelNameplateCategory(procedure_model_nameplate_categories) => {
                crate::codegen::tables::rows::Rows::from(procedure_model_nameplate_categories)
            }
            super::Row::ProcedureModelToolCategory(procedure_model_tool_categories) => {
                crate::codegen::tables::rows::Rows::from(procedure_model_tool_categories)
            }
            super::Row::ProcedureModel(procedure_models) => {
                crate::codegen::tables::rows::Rows::from(procedure_models)
            }
            super::Row::ProcedureStepModel(procedure_step_models) => {
                crate::codegen::tables::rows::Rows::from(procedure_step_models)
            }
            super::Row::Procedure(procedures) => {
                crate::codegen::tables::rows::Rows::from(procedures)
            }
            super::Row::Processable(processables) => {
                crate::codegen::tables::rows::Rows::from(processables)
            }
            super::Row::ProcessingStep(processing_steps) => {
                crate::codegen::tables::rows::Rows::from(processing_steps)
            }
            super::Row::ProjectState(project_states) => {
                crate::codegen::tables::rows::Rows::from(project_states)
            }
            super::Row::ProjectWorkflowModel(project_workflow_models) => {
                crate::codegen::tables::rows::Rows::from(project_workflow_models)
            }
            super::Row::Project(projects) => crate::codegen::tables::rows::Rows::from(projects),
            super::Row::Rank(ranks) => crate::codegen::tables::rows::Rows::from(ranks),
            super::Row::Reagent(reagents) => crate::codegen::tables::rows::Rows::from(reagents),
            super::Row::Role(roles) => crate::codegen::tables::rows::Rows::from(roles),
            super::Row::Room(rooms) => crate::codegen::tables::rows::Rows::from(rooms),
            super::Row::SampleState(sample_states) => {
                crate::codegen::tables::rows::Rows::from(sample_states)
            }
            super::Row::SamplingStepModel(sampling_step_models) => {
                crate::codegen::tables::rows::Rows::from(sampling_step_models)
            }
            super::Row::SamplingStep(sampling_steps) => {
                crate::codegen::tables::rows::Rows::from(sampling_steps)
            }
            super::Row::ShakingStepModel(shaking_step_models) => {
                crate::codegen::tables::rows::Rows::from(shaking_step_models)
            }
            super::Row::ShakingStep(shaking_steps) => {
                crate::codegen::tables::rows::Rows::from(shaking_steps)
            }
            super::Row::SpatialRefSy(spatial_ref_sys) => {
                crate::codegen::tables::rows::Rows::from(spatial_ref_sys)
            }
            super::Row::Spectrum(spectra) => crate::codegen::tables::rows::Rows::from(spectra),
            super::Row::SpectraCollection(spectra_collections) => {
                crate::codegen::tables::rows::Rows::from(spectra_collections)
            }
            super::Row::StepContainerModel(step_container_models) => {
                crate::codegen::tables::rows::Rows::from(step_container_models)
            }
            super::Row::StepInstrument(step_instruments) => {
                crate::codegen::tables::rows::Rows::from(step_instruments)
            }
            super::Row::StepModelCategory(step_model_categories) => {
                crate::codegen::tables::rows::Rows::from(step_model_categories)
            }
            super::Row::StepModelContainerCategory(step_model_container_categories) => {
                crate::codegen::tables::rows::Rows::from(step_model_container_categories)
            }
            super::Row::StepModelInstrumentCategory(step_model_instrument_categories) => {
                crate::codegen::tables::rows::Rows::from(step_model_instrument_categories)
            }
            super::Row::StepModelInstrumentModel(step_model_instrument_models) => {
                crate::codegen::tables::rows::Rows::from(step_model_instrument_models)
            }
            super::Row::StepModelInstrument(step_model_instruments) => {
                crate::codegen::tables::rows::Rows::from(step_model_instruments)
            }
            super::Row::StepModelNameplateCategory(step_model_nameplate_categories) => {
                crate::codegen::tables::rows::Rows::from(step_model_nameplate_categories)
            }
            super::Row::StepModelToolCategory(step_model_tool_categories) => {
                crate::codegen::tables::rows::Rows::from(step_model_tool_categories)
            }
            super::Row::StepModel(step_models) => {
                crate::codegen::tables::rows::Rows::from(step_models)
            }
            super::Row::StepNameplateModel(step_nameplate_models) => {
                crate::codegen::tables::rows::Rows::from(step_nameplate_models)
            }
            super::Row::StepStorageContainer(step_storage_containers) => {
                crate::codegen::tables::rows::Rows::from(step_storage_containers)
            }
            super::Row::StepToolModel(step_tool_models) => {
                crate::codegen::tables::rows::Rows::from(step_tool_models)
            }
            super::Row::Step(steps) => crate::codegen::tables::rows::Rows::from(steps),
            super::Row::StorageContainer(storage_containers) => {
                crate::codegen::tables::rows::Rows::from(storage_containers)
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
            super::Row::ToolModel(tool_models) => {
                crate::codegen::tables::rows::Rows::from(tool_models)
            }
            super::Row::TrackableLocation(trackable_locations) => {
                crate::codegen::tables::rows::Rows::from(trackable_locations)
            }
            super::Row::TrackableState(trackable_states) => {
                crate::codegen::tables::rows::Rows::from(trackable_states)
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
            super::Row::VolumetricProcessable(volumetric_processables) => {
                crate::codegen::tables::rows::Rows::from(volumetric_processables)
            }
            super::Row::WeighingInstrumentModel(weighing_instrument_models) => {
                crate::codegen::tables::rows::Rows::from(weighing_instrument_models)
            }
            super::Row::WeighingStepModel(weighing_step_models) => {
                crate::codegen::tables::rows::Rows::from(weighing_step_models)
            }
            super::Row::WeighingStep(weighing_steps) => {
                crate::codegen::tables::rows::Rows::from(weighing_steps)
            }
        }
    }
}
