impl web_common_traits::prelude::Rows for super::Rows {
    type PrimaryKey = crate::codegen::tables::table_primary_keys::TablePrimaryKey;
    fn primary_keys(&self) -> Vec<Self::PrimaryKey> {
        match self {
            super::Rows::Address(addresses) => addresses.primary_keys(),
            super::Rows::AliquotingInstrumentModel(aliquoting_instrument_models) => {
                aliquoting_instrument_models.primary_keys()
            }
            super::Rows::AliquotingStepModel(aliquoting_step_models) => {
                aliquoting_step_models.primary_keys()
            }
            super::Rows::AliquotingStep(aliquoting_steps) => aliquoting_steps.primary_keys(),
            super::Rows::BrandState(brand_states) => brand_states.primary_keys(),
            super::Rows::Brand(brands) => brands.primary_keys(),
            super::Rows::City(cities) => cities.primary_keys(),
            super::Rows::Color(colors) => colors.primary_keys(),
            super::Rows::CommercialProduct(commercial_products) => {
                commercial_products.primary_keys()
            }
            super::Rows::CommercialReagentModel(commercial_reagent_models) => {
                commercial_reagent_models.primary_keys()
            }
            super::Rows::ContainerCategory(container_categories) => {
                container_categories.primary_keys()
            }
            super::Rows::ContainerModel(container_models) => container_models.primary_keys(),
            super::Rows::Country(countries) => countries.primary_keys(),
            super::Rows::DocumentFormat(document_formats) => document_formats.primary_keys(),
            super::Rows::EmailProvider(email_providers) => email_providers.primary_keys(),
            super::Rows::FractioningStepModel(fractioning_step_models) => {
                fractioning_step_models.primary_keys()
            }
            super::Rows::FractioningStep(fractioning_steps) => fractioning_steps.primary_keys(),
            super::Rows::FreezeDryingStepModel(freeze_drying_step_models) => {
                freeze_drying_step_models.primary_keys()
            }
            super::Rows::GrindingStepModel(grinding_step_models) => {
                grinding_step_models.primary_keys()
            }
            super::Rows::InstrumentCategory(instrument_categories) => {
                instrument_categories.primary_keys()
            }
            super::Rows::InstrumentLocation(instrument_locations) => {
                instrument_locations.primary_keys()
            }
            super::Rows::InstrumentModelCategory(instrument_model_categories) => {
                instrument_model_categories.primary_keys()
            }
            super::Rows::InstrumentModel(instrument_models) => instrument_models.primary_keys(),
            super::Rows::InstrumentState(instrument_states) => instrument_states.primary_keys(),
            super::Rows::Instrument(instruments) => instruments.primary_keys(),
            super::Rows::LoginProvider(login_providers) => login_providers.primary_keys(),
            super::Rows::Material(materials) => materials.primary_keys(),
            super::Rows::NameplateCategory(nameplate_categories) => {
                nameplate_categories.primary_keys()
            }
            super::Rows::NameplateModel(nameplate_models) => nameplate_models.primary_keys(),
            super::Rows::ObservationSubject(observation_subjects) => {
                observation_subjects.primary_keys()
            }
            super::Rows::OrganismObservation(organism_observations) => {
                organism_observations.primary_keys()
            }
            super::Rows::OrganismSamplingStepModel(organism_sampling_step_models) => {
                organism_sampling_step_models.primary_keys()
            }
            super::Rows::OrganismTaxon(organism_taxa) => organism_taxa.primary_keys(),
            super::Rows::Organism(organisms) => organisms.primary_keys(),
            super::Rows::Organization(organizations) => organizations.primary_keys(),
            super::Rows::PackagingModel(packaging_models) => packaging_models.primary_keys(),
            super::Rows::PackagingStepModel(packaging_step_models) => {
                packaging_step_models.primary_keys()
            }
            super::Rows::PermanenceCategory(permanence_categories) => {
                permanence_categories.primary_keys()
            }
            super::Rows::Photograph(photographs) => photographs.primary_keys(),
            super::Rows::ProcedureModelContainerCategory(procedure_model_container_categories) => {
                procedure_model_container_categories.primary_keys()
            }
            super::Rows::ProcedureModelInstrumentCategory(
                procedure_model_instrument_categories,
            ) => procedure_model_instrument_categories.primary_keys(),
            super::Rows::ProcedureModelNameplateCategory(procedure_model_nameplate_categories) => {
                procedure_model_nameplate_categories.primary_keys()
            }
            super::Rows::ProcedureModelToolCategory(procedure_model_tool_categories) => {
                procedure_model_tool_categories.primary_keys()
            }
            super::Rows::ProcedureModel(procedure_models) => procedure_models.primary_keys(),
            super::Rows::ProcedureStepModel(procedure_step_models) => {
                procedure_step_models.primary_keys()
            }
            super::Rows::Procedure(procedures) => procedures.primary_keys(),
            super::Rows::Processable(processables) => processables.primary_keys(),
            super::Rows::ProcessingStep(processing_steps) => processing_steps.primary_keys(),
            super::Rows::ProjectState(project_states) => project_states.primary_keys(),
            super::Rows::ProjectWorkflowModel(project_workflow_models) => {
                project_workflow_models.primary_keys()
            }
            super::Rows::Project(projects) => projects.primary_keys(),
            super::Rows::Rank(ranks) => ranks.primary_keys(),
            super::Rows::Role(roles) => roles.primary_keys(),
            super::Rows::Room(rooms) => rooms.primary_keys(),
            super::Rows::SampleState(sample_states) => sample_states.primary_keys(),
            super::Rows::SamplingStepModel(sampling_step_models) => {
                sampling_step_models.primary_keys()
            }
            super::Rows::SamplingStep(sampling_steps) => sampling_steps.primary_keys(),
            super::Rows::SpatialRefSy(spatial_ref_sys) => spatial_ref_sys.primary_keys(),
            super::Rows::Spectrum(spectra) => spectra.primary_keys(),
            super::Rows::SpectraCollection(spectra_collections) => {
                spectra_collections.primary_keys()
            }
            super::Rows::StepContainerModel(step_container_models) => {
                step_container_models.primary_keys()
            }
            super::Rows::StepInstrument(step_instruments) => step_instruments.primary_keys(),
            super::Rows::StepModelCategory(step_model_categories) => {
                step_model_categories.primary_keys()
            }
            super::Rows::StepModelContainerCategory(step_model_container_categories) => {
                step_model_container_categories.primary_keys()
            }
            super::Rows::StepModelInstrumentCategory(step_model_instrument_categories) => {
                step_model_instrument_categories.primary_keys()
            }
            super::Rows::StepModelInstrumentModel(step_model_instrument_models) => {
                step_model_instrument_models.primary_keys()
            }
            super::Rows::StepModelInstrument(step_model_instruments) => {
                step_model_instruments.primary_keys()
            }
            super::Rows::StepModelNameplateCategory(step_model_nameplate_categories) => {
                step_model_nameplate_categories.primary_keys()
            }
            super::Rows::StepModelToolCategory(step_model_tool_categories) => {
                step_model_tool_categories.primary_keys()
            }
            super::Rows::StepModel(step_models) => step_models.primary_keys(),
            super::Rows::StepNameplateModel(step_nameplate_models) => {
                step_nameplate_models.primary_keys()
            }
            super::Rows::StepStorageContainer(step_storage_containers) => {
                step_storage_containers.primary_keys()
            }
            super::Rows::StepToolModel(step_tool_models) => step_tool_models.primary_keys(),
            super::Rows::Step(steps) => steps.primary_keys(),
            super::Rows::StorageContainer(storage_containers) => storage_containers.primary_keys(),
            super::Rows::Taxon(taxa) => taxa.primary_keys(),
            super::Rows::TeamMember(team_members) => team_members.primary_keys(),
            super::Rows::TeamProject(team_projects) => team_projects.primary_keys(),
            super::Rows::TeamState(team_states) => team_states.primary_keys(),
            super::Rows::Team(teams) => teams.primary_keys(),
            super::Rows::ToolCategory(tool_categories) => tool_categories.primary_keys(),
            super::Rows::ToolModel(tool_models) => tool_models.primary_keys(),
            super::Rows::TrackableLocation(trackable_locations) => {
                trackable_locations.primary_keys()
            }
            super::Rows::TrackableState(trackable_states) => trackable_states.primary_keys(),
            super::Rows::Trackable(trackables) => trackables.primary_keys(),
            super::Rows::Unit(units) => units.primary_keys(),
            super::Rows::UserEmail(user_emails) => user_emails.primary_keys(),
            super::Rows::UserOrganization(user_organizations) => user_organizations.primary_keys(),
            super::Rows::User(users) => users.primary_keys(),
            super::Rows::WeighingInstrumentModel(weighing_instrument_models) => {
                weighing_instrument_models.primary_keys()
            }
            super::Rows::WeighingStepModel(weighing_step_models) => {
                weighing_step_models.primary_keys()
            }
            super::Rows::WeighingStep(weighing_steps) => weighing_steps.primary_keys(),
        }
    }
}
