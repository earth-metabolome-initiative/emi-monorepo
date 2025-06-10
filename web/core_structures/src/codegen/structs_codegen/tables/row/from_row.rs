impl From<super::Row> for crate::codegen::tables::rows::Rows {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::Address(addresses) => crate::codegen::tables::rows::Rows::from(addresses),
            super::Row::AliquotingInstrumentModel(aliquoting_instrument_models) => {
                crate::codegen::tables::rows::Rows::from(aliquoting_instrument_models)
            }
            super::Row::AliquotingProcedureModel(aliquoting_procedure_models) => {
                crate::codegen::tables::rows::Rows::from(aliquoting_procedure_models)
            }
            super::Row::BallMillProcedureModel(ball_mill_procedure_models) => {
                crate::codegen::tables::rows::Rows::from(ball_mill_procedure_models)
            }
            super::Row::Brand(brands) => crate::codegen::tables::rows::Rows::from(brands),
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
            super::Row::FreezeDryingProcedureModel(freeze_drying_procedure_models) => {
                crate::codegen::tables::rows::Rows::from(freeze_drying_procedure_models)
            }
            super::Row::FreezingProcedureModel(freezing_procedure_models) => {
                crate::codegen::tables::rows::Rows::from(freezing_procedure_models)
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
            super::Row::MixCountableProcedureModel(mix_countable_procedure_models) => {
                crate::codegen::tables::rows::Rows::from(mix_countable_procedure_models)
            }
            super::Row::MixSolidProcedureModel(mix_solid_procedure_models) => {
                crate::codegen::tables::rows::Rows::from(mix_solid_procedure_models)
            }
            super::Row::NextProcedureModel(next_procedure_models) => {
                crate::codegen::tables::rows::Rows::from(next_procedure_models)
            }
            super::Row::ObservationSubject(observation_subjects) => {
                crate::codegen::tables::rows::Rows::from(observation_subjects)
            }
            super::Row::OrganismObservation(organism_observations) => {
                crate::codegen::tables::rows::Rows::from(organism_observations)
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
            super::Row::PackagingProcedureModel(packaging_procedure_models) => {
                crate::codegen::tables::rows::Rows::from(packaging_procedure_models)
            }
            super::Row::ParentProcedureModel(parent_procedure_models) => {
                crate::codegen::tables::rows::Rows::from(parent_procedure_models)
            }
            super::Row::PermanenceCategory(permanence_categories) => {
                crate::codegen::tables::rows::Rows::from(permanence_categories)
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
            super::Row::SamplingProcedureModel(sampling_procedure_models) => {
                crate::codegen::tables::rows::Rows::from(sampling_procedure_models)
            }
            super::Row::ShakingProcedureModel(shaking_procedure_models) => {
                crate::codegen::tables::rows::Rows::from(shaking_procedure_models)
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
