impl From<super::Rows> for Vec<crate::codegen::tables::row::Row> {
    fn from(rows: super::Rows) -> Self {
        match rows {
            super::Rows::Address(rows) => {
                rows.into_iter().map(crate::codegen::tables::row::Row::Address).collect::<Vec<_>>()
            }
            super::Rows::AliquotingInstrumentModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::AliquotingInstrumentModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::AliquotingStepModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::AliquotingStepModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::AliquotingStep(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::AliquotingStep)
                    .collect::<Vec<_>>()
            }
            super::Rows::BallMillStepModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::BallMillStepModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::BallMillStep(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::BallMillStep)
                    .collect::<Vec<_>>()
            }
            super::Rows::Brand(rows) => {
                rows.into_iter().map(crate::codegen::tables::row::Row::Brand).collect::<Vec<_>>()
            }
            super::Rows::CentrifugeStepModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::CentrifugeStepModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::CentrifugeStep(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::CentrifugeStep)
                    .collect::<Vec<_>>()
            }
            super::Rows::ChemicalEntity(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::ChemicalEntity)
                    .collect::<Vec<_>>()
            }
            super::Rows::City(rows) => {
                rows.into_iter().map(crate::codegen::tables::row::Row::City).collect::<Vec<_>>()
            }
            super::Rows::Color(rows) => {
                rows.into_iter().map(crate::codegen::tables::row::Row::Color).collect::<Vec<_>>()
            }
            super::Rows::CommercialProductLot(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::CommercialProductLot)
                    .collect::<Vec<_>>()
            }
            super::Rows::CommercialProduct(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::CommercialProduct)
                    .collect::<Vec<_>>()
            }
            super::Rows::CommercialReagentModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::CommercialReagentModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::CommercialReagent(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::CommercialReagent)
                    .collect::<Vec<_>>()
            }
            super::Rows::ContainerModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::ContainerModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::Country(rows) => {
                rows.into_iter().map(crate::codegen::tables::row::Row::Country).collect::<Vec<_>>()
            }
            super::Rows::DisposalStepModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::DisposalStepModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::DisposalStep(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::DisposalStep)
                    .collect::<Vec<_>>()
            }
            super::Rows::DocumentFormat(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::DocumentFormat)
                    .collect::<Vec<_>>()
            }
            super::Rows::EmailProvider(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::EmailProvider)
                    .collect::<Vec<_>>()
            }
            super::Rows::FractioningStepModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::FractioningStepModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::FractioningStep(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::FractioningStep)
                    .collect::<Vec<_>>()
            }
            super::Rows::FreezeDryingStepModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::FreezeDryingStepModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::InstrumentLocation(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::InstrumentLocation)
                    .collect::<Vec<_>>()
            }
            super::Rows::InstrumentModelCategory(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::InstrumentModelCategory)
                    .collect::<Vec<_>>()
            }
            super::Rows::InstrumentModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::InstrumentModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::InstrumentState(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::InstrumentState)
                    .collect::<Vec<_>>()
            }
            super::Rows::Instrument(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::Instrument)
                    .collect::<Vec<_>>()
            }
            super::Rows::LoginProvider(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::LoginProvider)
                    .collect::<Vec<_>>()
            }
            super::Rows::Material(rows) => {
                rows.into_iter().map(crate::codegen::tables::row::Row::Material).collect::<Vec<_>>()
            }
            super::Rows::NameplateModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::NameplateModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::ObservationSubject(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::ObservationSubject)
                    .collect::<Vec<_>>()
            }
            super::Rows::OrganismObservation(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::OrganismObservation)
                    .collect::<Vec<_>>()
            }
            super::Rows::OrganismSamplingStepModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::OrganismSamplingStepModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::OrganismTaxon(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::OrganismTaxon)
                    .collect::<Vec<_>>()
            }
            super::Rows::Organism(rows) => {
                rows.into_iter().map(crate::codegen::tables::row::Row::Organism).collect::<Vec<_>>()
            }
            super::Rows::Organization(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::Organization)
                    .collect::<Vec<_>>()
            }
            super::Rows::PackagingModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::PackagingModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::PackagingStepModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::PackagingStepModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::PermanenceCategory(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::PermanenceCategory)
                    .collect::<Vec<_>>()
            }
            super::Rows::Photograph(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::Photograph)
                    .collect::<Vec<_>>()
            }
            super::Rows::ProcedureModelContainerCategory(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::ProcedureModelContainerCategory)
                    .collect::<Vec<_>>()
            }
            super::Rows::ProcedureModelInstrumentCategory(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::ProcedureModelInstrumentCategory)
                    .collect::<Vec<_>>()
            }
            super::Rows::ProcedureModelNameplateCategory(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::ProcedureModelNameplateCategory)
                    .collect::<Vec<_>>()
            }
            super::Rows::ProcedureModelReagent(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::ProcedureModelReagent)
                    .collect::<Vec<_>>()
            }
            super::Rows::ProcedureModelToolCategory(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::ProcedureModelToolCategory)
                    .collect::<Vec<_>>()
            }
            super::Rows::ProcedureModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::ProcedureModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::ProcedureStepModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::ProcedureStepModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::Procedure(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::Procedure)
                    .collect::<Vec<_>>()
            }
            super::Rows::Processable(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::Processable)
                    .collect::<Vec<_>>()
            }
            super::Rows::ProcessingStep(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::ProcessingStep)
                    .collect::<Vec<_>>()
            }
            super::Rows::ProjectState(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::ProjectState)
                    .collect::<Vec<_>>()
            }
            super::Rows::ProjectWorkflowModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::ProjectWorkflowModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::Project(rows) => {
                rows.into_iter().map(crate::codegen::tables::row::Row::Project).collect::<Vec<_>>()
            }
            super::Rows::Rank(rows) => {
                rows.into_iter().map(crate::codegen::tables::row::Row::Rank).collect::<Vec<_>>()
            }
            super::Rows::Reagent(rows) => {
                rows.into_iter().map(crate::codegen::tables::row::Row::Reagent).collect::<Vec<_>>()
            }
            super::Rows::Role(rows) => {
                rows.into_iter().map(crate::codegen::tables::row::Row::Role).collect::<Vec<_>>()
            }
            super::Rows::Room(rows) => {
                rows.into_iter().map(crate::codegen::tables::row::Row::Room).collect::<Vec<_>>()
            }
            super::Rows::SampleState(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::SampleState)
                    .collect::<Vec<_>>()
            }
            super::Rows::SamplingStepModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::SamplingStepModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::SamplingStep(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::SamplingStep)
                    .collect::<Vec<_>>()
            }
            super::Rows::ShakingStepModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::ShakingStepModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::ShakingStep(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::ShakingStep)
                    .collect::<Vec<_>>()
            }
            super::Rows::SpatialRefSy(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::SpatialRefSy)
                    .collect::<Vec<_>>()
            }
            super::Rows::Spectrum(rows) => {
                rows.into_iter().map(crate::codegen::tables::row::Row::Spectrum).collect::<Vec<_>>()
            }
            super::Rows::SpectraCollection(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::SpectraCollection)
                    .collect::<Vec<_>>()
            }
            super::Rows::StepContainerModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::StepContainerModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::StepInstrument(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::StepInstrument)
                    .collect::<Vec<_>>()
            }
            super::Rows::StepModelCategory(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::StepModelCategory)
                    .collect::<Vec<_>>()
            }
            super::Rows::StepModelContainerCategory(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::StepModelContainerCategory)
                    .collect::<Vec<_>>()
            }
            super::Rows::StepModelInstrumentCategory(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::StepModelInstrumentCategory)
                    .collect::<Vec<_>>()
            }
            super::Rows::StepModelInstrumentModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::StepModelInstrumentModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::StepModelInstrument(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::StepModelInstrument)
                    .collect::<Vec<_>>()
            }
            super::Rows::StepModelNameplateCategory(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::StepModelNameplateCategory)
                    .collect::<Vec<_>>()
            }
            super::Rows::StepModelToolCategory(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::StepModelToolCategory)
                    .collect::<Vec<_>>()
            }
            super::Rows::StepModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::StepModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::StepNameplateModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::StepNameplateModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::StepStorageContainer(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::StepStorageContainer)
                    .collect::<Vec<_>>()
            }
            super::Rows::StepToolModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::StepToolModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::Step(rows) => {
                rows.into_iter().map(crate::codegen::tables::row::Row::Step).collect::<Vec<_>>()
            }
            super::Rows::StorageContainer(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::StorageContainer)
                    .collect::<Vec<_>>()
            }
            super::Rows::Taxon(rows) => {
                rows.into_iter().map(crate::codegen::tables::row::Row::Taxon).collect::<Vec<_>>()
            }
            super::Rows::TeamMember(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::TeamMember)
                    .collect::<Vec<_>>()
            }
            super::Rows::TeamProject(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::TeamProject)
                    .collect::<Vec<_>>()
            }
            super::Rows::TeamState(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::TeamState)
                    .collect::<Vec<_>>()
            }
            super::Rows::Team(rows) => {
                rows.into_iter().map(crate::codegen::tables::row::Row::Team).collect::<Vec<_>>()
            }
            super::Rows::TemporaryUserEmail(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::TemporaryUserEmail)
                    .collect::<Vec<_>>()
            }
            super::Rows::ToolModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::ToolModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::TrackableLocation(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::TrackableLocation)
                    .collect::<Vec<_>>()
            }
            super::Rows::TrackableState(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::TrackableState)
                    .collect::<Vec<_>>()
            }
            super::Rows::Trackable(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::Trackable)
                    .collect::<Vec<_>>()
            }
            super::Rows::Unit(rows) => {
                rows.into_iter().map(crate::codegen::tables::row::Row::Unit).collect::<Vec<_>>()
            }
            super::Rows::UserEmail(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::UserEmail)
                    .collect::<Vec<_>>()
            }
            super::Rows::UserOrganization(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::UserOrganization)
                    .collect::<Vec<_>>()
            }
            super::Rows::User(rows) => {
                rows.into_iter().map(crate::codegen::tables::row::Row::User).collect::<Vec<_>>()
            }
            super::Rows::VolumetricProcessable(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::VolumetricProcessable)
                    .collect::<Vec<_>>()
            }
            super::Rows::WeighingInstrumentModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::WeighingInstrumentModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::WeighingStepModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::WeighingStepModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::WeighingStep(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::WeighingStep)
                    .collect::<Vec<_>>()
            }
        }
    }
}
impl IntoIterator for super::Rows {
    type Item = crate::codegen::tables::row::Row;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        let row_vec: Vec<crate::codegen::tables::row::Row> = self.into();
        row_vec.into_iter()
    }
}
