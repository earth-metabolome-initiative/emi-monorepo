impl super::Rows {
    pub fn len(&self) -> usize {
        match self {
            super::Rows::Address(rows) => rows.len(),
            super::Rows::AliquotingInstrumentModel(rows) => rows.len(),
            super::Rows::AliquotingStepModel(rows) => rows.len(),
            super::Rows::AliquotingStep(rows) => rows.len(),
            super::Rows::BallMillStepModel(rows) => rows.len(),
            super::Rows::BallMillStep(rows) => rows.len(),
            super::Rows::Brand(rows) => rows.len(),
            super::Rows::CentrifugeStepModel(rows) => rows.len(),
            super::Rows::CentrifugeStep(rows) => rows.len(),
            super::Rows::ChemicalEntity(rows) => rows.len(),
            super::Rows::City(rows) => rows.len(),
            super::Rows::Color(rows) => rows.len(),
            super::Rows::CommercialProductLot(rows) => rows.len(),
            super::Rows::CommercialProduct(rows) => rows.len(),
            super::Rows::CommercialReagent(rows) => rows.len(),
            super::Rows::ContainerModel(rows) => rows.len(),
            super::Rows::Country(rows) => rows.len(),
            super::Rows::DisposalStepModel(rows) => rows.len(),
            super::Rows::DisposalStep(rows) => rows.len(),
            super::Rows::DocumentFormat(rows) => rows.len(),
            super::Rows::Document(rows) => rows.len(),
            super::Rows::EmailProvider(rows) => rows.len(),
            super::Rows::FractioningStepModel(rows) => rows.len(),
            super::Rows::FractioningStep(rows) => rows.len(),
            super::Rows::FreezeDryingStepModel(rows) => rows.len(),
            super::Rows::InstrumentLocation(rows) => rows.len(),
            super::Rows::InstrumentModelCategory(rows) => rows.len(),
            super::Rows::InstrumentModel(rows) => rows.len(),
            super::Rows::InstrumentState(rows) => rows.len(),
            super::Rows::Instrument(rows) => rows.len(),
            super::Rows::LoginProvider(rows) => rows.len(),
            super::Rows::Material(rows) => rows.len(),
            super::Rows::NameplateModel(rows) => rows.len(),
            super::Rows::ObservationSubject(rows) => rows.len(),
            super::Rows::OrganismObservation(rows) => rows.len(),
            super::Rows::OrganismSamplingStepModel(rows) => rows.len(),
            super::Rows::OrganismTaxon(rows) => rows.len(),
            super::Rows::Organism(rows) => rows.len(),
            super::Rows::Organization(rows) => rows.len(),
            super::Rows::PackagingModel(rows) => rows.len(),
            super::Rows::PackagingStepModel(rows) => rows.len(),
            super::Rows::ParentProcedureModel(rows) => rows.len(),
            super::Rows::PermanenceCategory(rows) => rows.len(),
            super::Rows::ProcedureModelContainerCategory(rows) => rows.len(),
            super::Rows::ProcedureModelInstrumentCategory(rows) => rows.len(),
            super::Rows::ProcedureModelNameplateCategory(rows) => rows.len(),
            super::Rows::ProcedureModelToolCategory(rows) => rows.len(),
            super::Rows::ProcedureModel(rows) => rows.len(),
            super::Rows::Procedure(rows) => rows.len(),
            super::Rows::Processable(rows) => rows.len(),
            super::Rows::ProcessingStep(rows) => rows.len(),
            super::Rows::ProjectState(rows) => rows.len(),
            super::Rows::ProjectWorkflowModel(rows) => rows.len(),
            super::Rows::Project(rows) => rows.len(),
            super::Rows::Rank(rows) => rows.len(),
            super::Rows::Reagent(rows) => rows.len(),
            super::Rows::Role(rows) => rows.len(),
            super::Rows::Room(rows) => rows.len(),
            super::Rows::SampleState(rows) => rows.len(),
            super::Rows::SamplingStepModel(rows) => rows.len(),
            super::Rows::SamplingStep(rows) => rows.len(),
            super::Rows::ShakingStepModel(rows) => rows.len(),
            super::Rows::ShakingStep(rows) => rows.len(),
            super::Rows::SpatialRefSy(rows) => rows.len(),
            super::Rows::Spectrum(rows) => rows.len(),
            super::Rows::SpectraCollection(rows) => rows.len(),
            super::Rows::StepContainerModel(rows) => rows.len(),
            super::Rows::StepInstrument(rows) => rows.len(),
            super::Rows::StepModelContainerCategory(rows) => rows.len(),
            super::Rows::StepModelInstrumentCategory(rows) => rows.len(),
            super::Rows::StepModelInstrumentModel(rows) => rows.len(),
            super::Rows::StepModelInstrument(rows) => rows.len(),
            super::Rows::StepModelNameplateCategory(rows) => rows.len(),
            super::Rows::StepModelToolCategory(rows) => rows.len(),
            super::Rows::StepModelTrackableCategory(rows) => rows.len(),
            super::Rows::StepModel(rows) => rows.len(),
            super::Rows::StepNameplateModel(rows) => rows.len(),
            super::Rows::StepStorageContainer(rows) => rows.len(),
            super::Rows::StepToolModel(rows) => rows.len(),
            super::Rows::Step(rows) => rows.len(),
            super::Rows::StorageContainer(rows) => rows.len(),
            super::Rows::Taxon(rows) => rows.len(),
            super::Rows::TeamMember(rows) => rows.len(),
            super::Rows::TeamProject(rows) => rows.len(),
            super::Rows::TeamState(rows) => rows.len(),
            super::Rows::Team(rows) => rows.len(),
            super::Rows::TemporaryUser(rows) => rows.len(),
            super::Rows::ToolModel(rows) => rows.len(),
            super::Rows::TrackableCategory(rows) => rows.len(),
            super::Rows::TrackableLocation(rows) => rows.len(),
            super::Rows::TrackableState(rows) => rows.len(),
            super::Rows::Trackable(rows) => rows.len(),
            super::Rows::Unit(rows) => rows.len(),
            super::Rows::UserEmail(rows) => rows.len(),
            super::Rows::UserOrganization(rows) => rows.len(),
            super::Rows::User(rows) => rows.len(),
            super::Rows::VolumetricProcessable(rows) => rows.len(),
            super::Rows::WeighingInstrumentModel(rows) => rows.len(),
            super::Rows::WeighingStepModel(rows) => rows.len(),
            super::Rows::WeighingStep(rows) => rows.len(),
        }
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
