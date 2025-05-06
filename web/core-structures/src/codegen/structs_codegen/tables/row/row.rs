impl web_common_traits::prelude::Row for super::Row {
    type PrimaryKey = crate::codegen::tables::table_primary_keys::TablePrimaryKey;
    fn primary_key(&self) -> Self::PrimaryKey {
        match self {
            super::Row::Address(addresses) => addresses.primary_key(),
            super::Row::AliquotingInstrumentModel(aliquoting_instrument_models) => {
                aliquoting_instrument_models.primary_key()
            }
            super::Row::AliquotingStepModel(aliquoting_step_models) => {
                aliquoting_step_models.primary_key()
            }
            super::Row::AliquotingStep(aliquoting_steps) => aliquoting_steps.primary_key(),
            super::Row::BallMillStepModel(ball_mill_step_models) => {
                ball_mill_step_models.primary_key()
            }
            super::Row::BallMillStep(ball_mill_steps) => ball_mill_steps.primary_key(),
            super::Row::Brand(brands) => brands.primary_key(),
            super::Row::CentrifugeStepModel(centrifuge_step_models) => {
                centrifuge_step_models.primary_key()
            }
            super::Row::CentrifugeStep(centrifuge_steps) => centrifuge_steps.primary_key(),
            super::Row::City(cities) => cities.primary_key(),
            super::Row::Color(colors) => colors.primary_key(),
            super::Row::CommercialProductLot(commercial_product_lots) => {
                commercial_product_lots.primary_key()
            }
            super::Row::CommercialProduct(commercial_products) => commercial_products.primary_key(),
            super::Row::CommercialReagent(commercial_reagents) => commercial_reagents.primary_key(),
            super::Row::ContainerModel(container_models) => container_models.primary_key(),
            super::Row::Country(countries) => countries.primary_key(),
            super::Row::DisposalStepModel(disposal_step_models) => {
                disposal_step_models.primary_key()
            }
            super::Row::DisposalStep(disposal_steps) => disposal_steps.primary_key(),
            super::Row::DocumentFormat(document_formats) => document_formats.primary_key(),
            super::Row::EmailProvider(email_providers) => email_providers.primary_key(),
            super::Row::FractioningStepModel(fractioning_step_models) => {
                fractioning_step_models.primary_key()
            }
            super::Row::FractioningStep(fractioning_steps) => fractioning_steps.primary_key(),
            super::Row::FreezeDryingStepModel(freeze_drying_step_models) => {
                freeze_drying_step_models.primary_key()
            }
            super::Row::InstrumentLocation(instrument_locations) => {
                instrument_locations.primary_key()
            }
            super::Row::InstrumentModelCategory(instrument_model_categories) => {
                instrument_model_categories.primary_key()
            }
            super::Row::InstrumentModel(instrument_models) => instrument_models.primary_key(),
            super::Row::InstrumentState(instrument_states) => instrument_states.primary_key(),
            super::Row::Instrument(instruments) => instruments.primary_key(),
            super::Row::LoginProvider(login_providers) => login_providers.primary_key(),
            super::Row::Material(materials) => materials.primary_key(),
            super::Row::NameplateModel(nameplate_models) => nameplate_models.primary_key(),
            super::Row::ObservationSubject(observation_subjects) => {
                observation_subjects.primary_key()
            }
            super::Row::OrganismObservation(organism_observations) => {
                organism_observations.primary_key()
            }
            super::Row::OrganismSamplingStepModel(organism_sampling_step_models) => {
                organism_sampling_step_models.primary_key()
            }
            super::Row::OrganismTaxon(organism_taxa) => organism_taxa.primary_key(),
            super::Row::Organism(organisms) => organisms.primary_key(),
            super::Row::Organization(organizations) => organizations.primary_key(),
            super::Row::PackagingModel(packaging_models) => packaging_models.primary_key(),
            super::Row::PackagingStepModel(packaging_step_models) => {
                packaging_step_models.primary_key()
            }
            super::Row::PermanenceCategory(permanence_categories) => {
                permanence_categories.primary_key()
            }
            super::Row::Photograph(photographs) => photographs.primary_key(),
            super::Row::ProcedureModelContainerCategory(procedure_model_container_categories) => {
                procedure_model_container_categories.primary_key()
            }
            super::Row::ProcedureModelInstrumentCategory(procedure_model_instrument_categories) => {
                procedure_model_instrument_categories.primary_key()
            }
            super::Row::ProcedureModelNameplateCategory(procedure_model_nameplate_categories) => {
                procedure_model_nameplate_categories.primary_key()
            }
            super::Row::ProcedureModelToolCategory(procedure_model_tool_categories) => {
                procedure_model_tool_categories.primary_key()
            }
            super::Row::ProcedureModel(procedure_models) => procedure_models.primary_key(),
            super::Row::ProcedureStepModel(procedure_step_models) => {
                procedure_step_models.primary_key()
            }
            super::Row::Procedure(procedures) => procedures.primary_key(),
            super::Row::Processable(processables) => processables.primary_key(),
            super::Row::ProcessingStep(processing_steps) => processing_steps.primary_key(),
            super::Row::ProjectState(project_states) => project_states.primary_key(),
            super::Row::ProjectWorkflowModel(project_workflow_models) => {
                project_workflow_models.primary_key()
            }
            super::Row::Project(projects) => projects.primary_key(),
            super::Row::Rank(ranks) => ranks.primary_key(),
            super::Row::Role(roles) => roles.primary_key(),
            super::Row::Room(rooms) => rooms.primary_key(),
            super::Row::SampleState(sample_states) => sample_states.primary_key(),
            super::Row::SamplingStepModel(sampling_step_models) => {
                sampling_step_models.primary_key()
            }
            super::Row::SamplingStep(sampling_steps) => sampling_steps.primary_key(),
            super::Row::ShakingStepModel(shaking_step_models) => shaking_step_models.primary_key(),
            super::Row::ShakingStep(shaking_steps) => shaking_steps.primary_key(),
            super::Row::SpatialRefSy(spatial_ref_sys) => spatial_ref_sys.primary_key(),
            super::Row::Spectrum(spectra) => spectra.primary_key(),
            super::Row::SpectraCollection(spectra_collections) => spectra_collections.primary_key(),
            super::Row::StepContainerModel(step_container_models) => {
                step_container_models.primary_key()
            }
            super::Row::StepInstrument(step_instruments) => step_instruments.primary_key(),
            super::Row::StepModelCategory(step_model_categories) => {
                step_model_categories.primary_key()
            }
            super::Row::StepModelContainerCategory(step_model_container_categories) => {
                step_model_container_categories.primary_key()
            }
            super::Row::StepModelInstrumentCategory(step_model_instrument_categories) => {
                step_model_instrument_categories.primary_key()
            }
            super::Row::StepModelInstrumentModel(step_model_instrument_models) => {
                step_model_instrument_models.primary_key()
            }
            super::Row::StepModelInstrument(step_model_instruments) => {
                step_model_instruments.primary_key()
            }
            super::Row::StepModelNameplateCategory(step_model_nameplate_categories) => {
                step_model_nameplate_categories.primary_key()
            }
            super::Row::StepModelToolCategory(step_model_tool_categories) => {
                step_model_tool_categories.primary_key()
            }
            super::Row::StepModel(step_models) => step_models.primary_key(),
            super::Row::StepNameplateModel(step_nameplate_models) => {
                step_nameplate_models.primary_key()
            }
            super::Row::StepStorageContainer(step_storage_containers) => {
                step_storage_containers.primary_key()
            }
            super::Row::StepToolModel(step_tool_models) => step_tool_models.primary_key(),
            super::Row::Step(steps) => steps.primary_key(),
            super::Row::StorageContainer(storage_containers) => storage_containers.primary_key(),
            super::Row::Taxon(taxa) => taxa.primary_key(),
            super::Row::TeamMember(team_members) => team_members.primary_key(),
            super::Row::TeamProject(team_projects) => team_projects.primary_key(),
            super::Row::TeamState(team_states) => team_states.primary_key(),
            super::Row::Team(teams) => teams.primary_key(),
            super::Row::ToolModel(tool_models) => tool_models.primary_key(),
            super::Row::TrackableLocation(trackable_locations) => trackable_locations.primary_key(),
            super::Row::TrackableState(trackable_states) => trackable_states.primary_key(),
            super::Row::Trackable(trackables) => trackables.primary_key(),
            super::Row::Unit(units) => units.primary_key(),
            super::Row::UserEmail(user_emails) => user_emails.primary_key(),
            super::Row::UserOrganization(user_organizations) => user_organizations.primary_key(),
            super::Row::User(users) => users.primary_key(),
            super::Row::VolumetricProcessable(volumetric_processables) => {
                volumetric_processables.primary_key()
            }
            super::Row::WeighingInstrumentModel(weighing_instrument_models) => {
                weighing_instrument_models.primary_key()
            }
            super::Row::WeighingStepModel(weighing_step_models) => {
                weighing_step_models.primary_key()
            }
            super::Row::WeighingStep(weighing_steps) => weighing_steps.primary_key(),
        }
    }
}
