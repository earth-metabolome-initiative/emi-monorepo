impl web_common_traits::prelude::Tabular for super::Row {
    type TableName = crate::codegen::tables::table_names::TableName;
    fn table_name(&self) -> Self::TableName {
        match self {
            super::Row::Address(_) => crate::codegen::tables::table_names::TableName::Address,
            super::Row::AliquotingInstrumentModel(_) => {
                crate::codegen::tables::table_names::TableName::AliquotingInstrumentModel
            }
            super::Row::AliquotingStepModel(_) => {
                crate::codegen::tables::table_names::TableName::AliquotingStepModel
            }
            super::Row::AliquotingStep(_) => {
                crate::codegen::tables::table_names::TableName::AliquotingStep
            }
            super::Row::BallMillStepModel(_) => {
                crate::codegen::tables::table_names::TableName::BallMillStepModel
            }
            super::Row::BallMillStep(_) => {
                crate::codegen::tables::table_names::TableName::BallMillStep
            }
            super::Row::Brand(_) => crate::codegen::tables::table_names::TableName::Brand,
            super::Row::CentrifugeStepModel(_) => {
                crate::codegen::tables::table_names::TableName::CentrifugeStepModel
            }
            super::Row::CentrifugeStep(_) => {
                crate::codegen::tables::table_names::TableName::CentrifugeStep
            }
            super::Row::City(_) => crate::codegen::tables::table_names::TableName::City,
            super::Row::Color(_) => crate::codegen::tables::table_names::TableName::Color,
            super::Row::CommercialProductLot(_) => {
                crate::codegen::tables::table_names::TableName::CommercialProductLot
            }
            super::Row::CommercialProduct(_) => {
                crate::codegen::tables::table_names::TableName::CommercialProduct
            }
            super::Row::CommercialReagentModel(_) => {
                crate::codegen::tables::table_names::TableName::CommercialReagentModel
            }
            super::Row::CommercialReagent(_) => {
                crate::codegen::tables::table_names::TableName::CommercialReagent
            }
            super::Row::ContainerModel(_) => {
                crate::codegen::tables::table_names::TableName::ContainerModel
            }
            super::Row::Country(_) => crate::codegen::tables::table_names::TableName::Country,
            super::Row::DisposalStepModel(_) => {
                crate::codegen::tables::table_names::TableName::DisposalStepModel
            }
            super::Row::DisposalStep(_) => {
                crate::codegen::tables::table_names::TableName::DisposalStep
            }
            super::Row::DocumentFormat(_) => {
                crate::codegen::tables::table_names::TableName::DocumentFormat
            }
            super::Row::EmailProvider(_) => {
                crate::codegen::tables::table_names::TableName::EmailProvider
            }
            super::Row::FractioningStepModel(_) => {
                crate::codegen::tables::table_names::TableName::FractioningStepModel
            }
            super::Row::FractioningStep(_) => {
                crate::codegen::tables::table_names::TableName::FractioningStep
            }
            super::Row::FreezeDryingStepModel(_) => {
                crate::codegen::tables::table_names::TableName::FreezeDryingStepModel
            }
            super::Row::InstrumentLocation(_) => {
                crate::codegen::tables::table_names::TableName::InstrumentLocation
            }
            super::Row::InstrumentModelCategory(_) => {
                crate::codegen::tables::table_names::TableName::InstrumentModelCategory
            }
            super::Row::InstrumentModel(_) => {
                crate::codegen::tables::table_names::TableName::InstrumentModel
            }
            super::Row::InstrumentState(_) => {
                crate::codegen::tables::table_names::TableName::InstrumentState
            }
            super::Row::Instrument(_) => crate::codegen::tables::table_names::TableName::Instrument,
            super::Row::LoginProvider(_) => {
                crate::codegen::tables::table_names::TableName::LoginProvider
            }
            super::Row::Material(_) => crate::codegen::tables::table_names::TableName::Material,
            super::Row::NameplateModel(_) => {
                crate::codegen::tables::table_names::TableName::NameplateModel
            }
            super::Row::ObservationSubject(_) => {
                crate::codegen::tables::table_names::TableName::ObservationSubject
            }
            super::Row::OrganismObservation(_) => {
                crate::codegen::tables::table_names::TableName::OrganismObservation
            }
            super::Row::OrganismSamplingStepModel(_) => {
                crate::codegen::tables::table_names::TableName::OrganismSamplingStepModel
            }
            super::Row::OrganismTaxon(_) => {
                crate::codegen::tables::table_names::TableName::OrganismTaxon
            }
            super::Row::Organism(_) => crate::codegen::tables::table_names::TableName::Organism,
            super::Row::Organization(_) => {
                crate::codegen::tables::table_names::TableName::Organization
            }
            super::Row::PackagingModel(_) => {
                crate::codegen::tables::table_names::TableName::PackagingModel
            }
            super::Row::PackagingStepModel(_) => {
                crate::codegen::tables::table_names::TableName::PackagingStepModel
            }
            super::Row::PermanenceCategory(_) => {
                crate::codegen::tables::table_names::TableName::PermanenceCategory
            }
            super::Row::Photograph(_) => crate::codegen::tables::table_names::TableName::Photograph,
            super::Row::ProcedureModelContainerCategory(_) => {
                crate::codegen::tables::table_names::TableName::ProcedureModelContainerCategory
            }
            super::Row::ProcedureModelInstrumentCategory(_) => {
                crate::codegen::tables::table_names::TableName::ProcedureModelInstrumentCategory
            }
            super::Row::ProcedureModelNameplateCategory(_) => {
                crate::codegen::tables::table_names::TableName::ProcedureModelNameplateCategory
            }
            super::Row::ProcedureModelToolCategory(_) => {
                crate::codegen::tables::table_names::TableName::ProcedureModelToolCategory
            }
            super::Row::ProcedureModel(_) => {
                crate::codegen::tables::table_names::TableName::ProcedureModel
            }
            super::Row::ProcedureStepModel(_) => {
                crate::codegen::tables::table_names::TableName::ProcedureStepModel
            }
            super::Row::Procedure(_) => crate::codegen::tables::table_names::TableName::Procedure,
            super::Row::Processable(_) => {
                crate::codegen::tables::table_names::TableName::Processable
            }
            super::Row::ProcessingStep(_) => {
                crate::codegen::tables::table_names::TableName::ProcessingStep
            }
            super::Row::ProjectState(_) => {
                crate::codegen::tables::table_names::TableName::ProjectState
            }
            super::Row::ProjectWorkflowModel(_) => {
                crate::codegen::tables::table_names::TableName::ProjectWorkflowModel
            }
            super::Row::Project(_) => crate::codegen::tables::table_names::TableName::Project,
            super::Row::Rank(_) => crate::codegen::tables::table_names::TableName::Rank,
            super::Row::Reagent(_) => crate::codegen::tables::table_names::TableName::Reagent,
            super::Row::Role(_) => crate::codegen::tables::table_names::TableName::Role,
            super::Row::Room(_) => crate::codegen::tables::table_names::TableName::Room,
            super::Row::SampleState(_) => {
                crate::codegen::tables::table_names::TableName::SampleState
            }
            super::Row::SamplingStepModel(_) => {
                crate::codegen::tables::table_names::TableName::SamplingStepModel
            }
            super::Row::SamplingStep(_) => {
                crate::codegen::tables::table_names::TableName::SamplingStep
            }
            super::Row::ShakingStepModel(_) => {
                crate::codegen::tables::table_names::TableName::ShakingStepModel
            }
            super::Row::ShakingStep(_) => {
                crate::codegen::tables::table_names::TableName::ShakingStep
            }
            super::Row::SpatialRefSy(_) => {
                crate::codegen::tables::table_names::TableName::SpatialRefSy
            }
            super::Row::Spectrum(_) => crate::codegen::tables::table_names::TableName::Spectrum,
            super::Row::SpectraCollection(_) => {
                crate::codegen::tables::table_names::TableName::SpectraCollection
            }
            super::Row::StepContainerModel(_) => {
                crate::codegen::tables::table_names::TableName::StepContainerModel
            }
            super::Row::StepInstrument(_) => {
                crate::codegen::tables::table_names::TableName::StepInstrument
            }
            super::Row::StepModelCategory(_) => {
                crate::codegen::tables::table_names::TableName::StepModelCategory
            }
            super::Row::StepModelContainerCategory(_) => {
                crate::codegen::tables::table_names::TableName::StepModelContainerCategory
            }
            super::Row::StepModelInstrumentCategory(_) => {
                crate::codegen::tables::table_names::TableName::StepModelInstrumentCategory
            }
            super::Row::StepModelInstrumentModel(_) => {
                crate::codegen::tables::table_names::TableName::StepModelInstrumentModel
            }
            super::Row::StepModelInstrument(_) => {
                crate::codegen::tables::table_names::TableName::StepModelInstrument
            }
            super::Row::StepModelNameplateCategory(_) => {
                crate::codegen::tables::table_names::TableName::StepModelNameplateCategory
            }
            super::Row::StepModelToolCategory(_) => {
                crate::codegen::tables::table_names::TableName::StepModelToolCategory
            }
            super::Row::StepModel(_) => crate::codegen::tables::table_names::TableName::StepModel,
            super::Row::StepNameplateModel(_) => {
                crate::codegen::tables::table_names::TableName::StepNameplateModel
            }
            super::Row::StepStorageContainer(_) => {
                crate::codegen::tables::table_names::TableName::StepStorageContainer
            }
            super::Row::StepToolModel(_) => {
                crate::codegen::tables::table_names::TableName::StepToolModel
            }
            super::Row::Step(_) => crate::codegen::tables::table_names::TableName::Step,
            super::Row::StorageContainer(_) => {
                crate::codegen::tables::table_names::TableName::StorageContainer
            }
            super::Row::Taxon(_) => crate::codegen::tables::table_names::TableName::Taxon,
            super::Row::TeamMember(_) => crate::codegen::tables::table_names::TableName::TeamMember,
            super::Row::TeamProject(_) => {
                crate::codegen::tables::table_names::TableName::TeamProject
            }
            super::Row::TeamState(_) => crate::codegen::tables::table_names::TableName::TeamState,
            super::Row::Team(_) => crate::codegen::tables::table_names::TableName::Team,
            super::Row::ToolModel(_) => crate::codegen::tables::table_names::TableName::ToolModel,
            super::Row::TrackableLocation(_) => {
                crate::codegen::tables::table_names::TableName::TrackableLocation
            }
            super::Row::TrackableState(_) => {
                crate::codegen::tables::table_names::TableName::TrackableState
            }
            super::Row::Trackable(_) => crate::codegen::tables::table_names::TableName::Trackable,
            super::Row::Unit(_) => crate::codegen::tables::table_names::TableName::Unit,
            super::Row::UserEmail(_) => crate::codegen::tables::table_names::TableName::UserEmail,
            super::Row::UserOrganization(_) => {
                crate::codegen::tables::table_names::TableName::UserOrganization
            }
            super::Row::User(_) => crate::codegen::tables::table_names::TableName::User,
            super::Row::VolumetricProcessable(_) => {
                crate::codegen::tables::table_names::TableName::VolumetricProcessable
            }
            super::Row::WeighingInstrumentModel(_) => {
                crate::codegen::tables::table_names::TableName::WeighingInstrumentModel
            }
            super::Row::WeighingStepModel(_) => {
                crate::codegen::tables::table_names::TableName::WeighingStepModel
            }
            super::Row::WeighingStep(_) => {
                crate::codegen::tables::table_names::TableName::WeighingStep
            }
        }
    }
}
