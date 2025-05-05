impl From<super::Rows> for Vec<crate::codegen::tables::row::Row> {
    fn from(rows: super::Rows) -> Self {
        match rows {
            super::Rows::Address(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::Address(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::AliquotingInstrumentModel(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::AliquotingInstrumentModel(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::AliquotingStepModel(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::AliquotingStepModel(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::AliquotingStep(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::AliquotingStep(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::BrandState(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::BrandState(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::Brand(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::Brand(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::City(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::City(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::Color(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::Color(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::CommercialProduct(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::CommercialProduct(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::CommercialReagentModel(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::CommercialReagentModel(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::ContainerCategory(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::ContainerCategory(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::ContainerModel(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::ContainerModel(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::Country(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::Country(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::DocumentFormat(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::DocumentFormat(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::EmailProvider(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::EmailProvider(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::FractioningStepModel(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::FractioningStepModel(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::FractioningStep(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::FractioningStep(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::FreezeDryingStepModel(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::FreezeDryingStepModel(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::GrindingStepModel(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::GrindingStepModel(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::InstrumentCategory(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::InstrumentCategory(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::InstrumentLocation(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::InstrumentLocation(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::InstrumentModelCategory(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::InstrumentModelCategory(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::InstrumentModel(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::InstrumentModel(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::InstrumentState(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::InstrumentState(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::Instrument(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::Instrument(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::LoginProvider(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::LoginProvider(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::Material(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::Material(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::NameplateCategory(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::NameplateCategory(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::NameplateModel(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::NameplateModel(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::ObservationSubject(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::ObservationSubject(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::OrganismObservation(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::OrganismObservation(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::OrganismSamplingStepModel(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::OrganismSamplingStepModel(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::OrganismTaxon(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::OrganismTaxon(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::Organism(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::Organism(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::Organization(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::Organization(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::PackagingModel(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::PackagingModel(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::PackagingStepModel(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::PackagingStepModel(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::PermanenceCategory(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::PermanenceCategory(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::Photograph(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::Photograph(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::ProcedureModelContainerCategory(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| {
                        crate::codegen::tables::row::Row::ProcedureModelContainerCategory(row)
                    })
                    .collect::<Vec<_>>()
            }
            super::Rows::ProcedureModelInstrumentCategory(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| {
                        crate::codegen::tables::row::Row::ProcedureModelInstrumentCategory(row)
                    })
                    .collect::<Vec<_>>()
            }
            super::Rows::ProcedureModelNameplateCategory(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| {
                        crate::codegen::tables::row::Row::ProcedureModelNameplateCategory(row)
                    })
                    .collect::<Vec<_>>()
            }
            super::Rows::ProcedureModelToolCategory(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::ProcedureModelToolCategory(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::ProcedureModel(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::ProcedureModel(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::ProcedureStepModel(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::ProcedureStepModel(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::Procedure(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::Procedure(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::Processable(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::Processable(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::ProcessingStep(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::ProcessingStep(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::ProjectState(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::ProjectState(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::ProjectWorkflowModel(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::ProjectWorkflowModel(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::Project(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::Project(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::Rank(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::Rank(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::Role(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::Role(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::Room(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::Room(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::SampleState(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::SampleState(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::SamplingStepModel(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::SamplingStepModel(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::SamplingStep(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::SamplingStep(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::SpatialRefSy(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::SpatialRefSy(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::Spectrum(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::Spectrum(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::SpectraCollection(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::SpectraCollection(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::StepContainerModel(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::StepContainerModel(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::StepInstrument(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::StepInstrument(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::StepModelCategory(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::StepModelCategory(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::StepModelContainerCategory(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::StepModelContainerCategory(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::StepModelInstrumentCategory(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::StepModelInstrumentCategory(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::StepModelInstrumentModel(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::StepModelInstrumentModel(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::StepModelInstrument(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::StepModelInstrument(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::StepModelNameplateCategory(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::StepModelNameplateCategory(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::StepModelToolCategory(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::StepModelToolCategory(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::StepModel(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::StepModel(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::StepNameplateModel(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::StepNameplateModel(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::StepStorageContainer(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::StepStorageContainer(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::StepToolModel(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::StepToolModel(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::Step(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::Step(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::StorageContainer(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::StorageContainer(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::Taxon(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::Taxon(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::TeamMember(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::TeamMember(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::TeamProject(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::TeamProject(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::TeamState(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::TeamState(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::Team(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::Team(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::ToolCategory(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::ToolCategory(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::ToolModel(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::ToolModel(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::TrackableLocation(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::TrackableLocation(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::TrackableState(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::TrackableState(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::Trackable(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::Trackable(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::Unit(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::Unit(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::UserEmail(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::UserEmail(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::UserOrganization(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::UserOrganization(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::User(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::User(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::WeighingInstrumentModel(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::WeighingInstrumentModel(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::WeighingStepModel(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::WeighingStepModel(row))
                    .collect::<Vec<_>>()
            }
            super::Rows::WeighingStep(rows) => {
                rows.iter()
                    .cloned()
                    .map(|row| crate::codegen::tables::row::Row::WeighingStep(row))
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
