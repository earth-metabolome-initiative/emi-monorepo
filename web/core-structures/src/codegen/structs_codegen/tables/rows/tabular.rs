impl web_common_traits::prelude::Tabular for super::Rows {
    type TableName = crate::codegen::tables::table_names::TableName;
    fn table_name(&self) -> Self::TableName {
        match self {
            super::Rows::Address(_) => crate::codegen::tables::table_names::TableName::Address,
            super::Rows::AliquotingInstrumentModel(_) => {
                crate::codegen::tables::table_names::TableName::AliquotingInstrumentModel
            }
            super::Rows::AliquotingStepModel(_) => {
                crate::codegen::tables::table_names::TableName::AliquotingStepModel
            }
            super::Rows::AliquotingStep(_) => {
                crate::codegen::tables::table_names::TableName::AliquotingStep
            }
            super::Rows::BallMillStepModel(_) => {
                crate::codegen::tables::table_names::TableName::BallMillStepModel
            }
            super::Rows::BallMillStep(_) => {
                crate::codegen::tables::table_names::TableName::BallMillStep
            }
            super::Rows::Brand(_) => crate::codegen::tables::table_names::TableName::Brand,
            super::Rows::CentrifugeStepModel(_) => {
                crate::codegen::tables::table_names::TableName::CentrifugeStepModel
            }
            super::Rows::CentrifugeStep(_) => {
                crate::codegen::tables::table_names::TableName::CentrifugeStep
            }
            super::Rows::City(_) => crate::codegen::tables::table_names::TableName::City,
            super::Rows::Color(_) => crate::codegen::tables::table_names::TableName::Color,
            super::Rows::CommercialProductLot(_) => {
                crate::codegen::tables::table_names::TableName::CommercialProductLot
            }
            super::Rows::CommercialProduct(_) => {
                crate::codegen::tables::table_names::TableName::CommercialProduct
            }
            super::Rows::CommercialReagentModel(_) => {
                crate::codegen::tables::table_names::TableName::CommercialReagentModel
            }
            super::Rows::CommercialReagent(_) => {
                crate::codegen::tables::table_names::TableName::CommercialReagent
            }
            super::Rows::ContainerModel(_) => {
                crate::codegen::tables::table_names::TableName::ContainerModel
            }
            super::Rows::Country(_) => crate::codegen::tables::table_names::TableName::Country,
            super::Rows::DisposalStepModel(_) => {
                crate::codegen::tables::table_names::TableName::DisposalStepModel
            }
            super::Rows::DisposalStep(_) => {
                crate::codegen::tables::table_names::TableName::DisposalStep
            }
            super::Rows::DocumentFormat(_) => {
                crate::codegen::tables::table_names::TableName::DocumentFormat
            }
            super::Rows::EmailProvider(_) => {
                crate::codegen::tables::table_names::TableName::EmailProvider
            }
            super::Rows::FractioningStepModel(_) => {
                crate::codegen::tables::table_names::TableName::FractioningStepModel
            }
            super::Rows::FractioningStep(_) => {
                crate::codegen::tables::table_names::TableName::FractioningStep
            }
            super::Rows::FreezeDryingStepModel(_) => {
                crate::codegen::tables::table_names::TableName::FreezeDryingStepModel
            }
            super::Rows::InstrumentLocation(_) => {
                crate::codegen::tables::table_names::TableName::InstrumentLocation
            }
            super::Rows::InstrumentModelCategory(_) => {
                crate::codegen::tables::table_names::TableName::InstrumentModelCategory
            }
            super::Rows::InstrumentModel(_) => {
                crate::codegen::tables::table_names::TableName::InstrumentModel
            }
            super::Rows::InstrumentState(_) => {
                crate::codegen::tables::table_names::TableName::InstrumentState
            }
            super::Rows::Instrument(_) => {
                crate::codegen::tables::table_names::TableName::Instrument
            }
            super::Rows::LoginProvider(_) => {
                crate::codegen::tables::table_names::TableName::LoginProvider
            }
            super::Rows::Material(_) => crate::codegen::tables::table_names::TableName::Material,
            super::Rows::NameplateModel(_) => {
                crate::codegen::tables::table_names::TableName::NameplateModel
            }
            super::Rows::ObservationSubject(_) => {
                crate::codegen::tables::table_names::TableName::ObservationSubject
            }
            super::Rows::OrganismObservation(_) => {
                crate::codegen::tables::table_names::TableName::OrganismObservation
            }
            super::Rows::OrganismSamplingStepModel(_) => {
                crate::codegen::tables::table_names::TableName::OrganismSamplingStepModel
            }
            super::Rows::OrganismTaxon(_) => {
                crate::codegen::tables::table_names::TableName::OrganismTaxon
            }
            super::Rows::Organism(_) => crate::codegen::tables::table_names::TableName::Organism,
            super::Rows::Organization(_) => {
                crate::codegen::tables::table_names::TableName::Organization
            }
            super::Rows::PackagingModel(_) => {
                crate::codegen::tables::table_names::TableName::PackagingModel
            }
            super::Rows::PackagingStepModel(_) => {
                crate::codegen::tables::table_names::TableName::PackagingStepModel
            }
            super::Rows::PermanenceCategory(_) => {
                crate::codegen::tables::table_names::TableName::PermanenceCategory
            }
            super::Rows::Photograph(_) => {
                crate::codegen::tables::table_names::TableName::Photograph
            }
            super::Rows::ProcedureModelContainerCategory(_) => {
                crate::codegen::tables::table_names::TableName::ProcedureModelContainerCategory
            }
            super::Rows::ProcedureModelInstrumentCategory(_) => {
                crate::codegen::tables::table_names::TableName::ProcedureModelInstrumentCategory
            }
            super::Rows::ProcedureModelNameplateCategory(_) => {
                crate::codegen::tables::table_names::TableName::ProcedureModelNameplateCategory
            }
            super::Rows::ProcedureModelToolCategory(_) => {
                crate::codegen::tables::table_names::TableName::ProcedureModelToolCategory
            }
            super::Rows::ProcedureModel(_) => {
                crate::codegen::tables::table_names::TableName::ProcedureModel
            }
            super::Rows::ProcedureStepModel(_) => {
                crate::codegen::tables::table_names::TableName::ProcedureStepModel
            }
            super::Rows::Procedure(_) => crate::codegen::tables::table_names::TableName::Procedure,
            super::Rows::Processable(_) => {
                crate::codegen::tables::table_names::TableName::Processable
            }
            super::Rows::ProcessingStep(_) => {
                crate::codegen::tables::table_names::TableName::ProcessingStep
            }
            super::Rows::ProjectState(_) => {
                crate::codegen::tables::table_names::TableName::ProjectState
            }
            super::Rows::ProjectWorkflowModel(_) => {
                crate::codegen::tables::table_names::TableName::ProjectWorkflowModel
            }
            super::Rows::Project(_) => crate::codegen::tables::table_names::TableName::Project,
            super::Rows::Rank(_) => crate::codegen::tables::table_names::TableName::Rank,
            super::Rows::Reagent(_) => crate::codegen::tables::table_names::TableName::Reagent,
            super::Rows::Role(_) => crate::codegen::tables::table_names::TableName::Role,
            super::Rows::Room(_) => crate::codegen::tables::table_names::TableName::Room,
            super::Rows::SampleState(_) => {
                crate::codegen::tables::table_names::TableName::SampleState
            }
            super::Rows::SamplingStepModel(_) => {
                crate::codegen::tables::table_names::TableName::SamplingStepModel
            }
            super::Rows::SamplingStep(_) => {
                crate::codegen::tables::table_names::TableName::SamplingStep
            }
            super::Rows::ShakingStepModel(_) => {
                crate::codegen::tables::table_names::TableName::ShakingStepModel
            }
            super::Rows::ShakingStep(_) => {
                crate::codegen::tables::table_names::TableName::ShakingStep
            }
            super::Rows::SpatialRefSy(_) => {
                crate::codegen::tables::table_names::TableName::SpatialRefSy
            }
            super::Rows::Spectrum(_) => crate::codegen::tables::table_names::TableName::Spectrum,
            super::Rows::SpectraCollection(_) => {
                crate::codegen::tables::table_names::TableName::SpectraCollection
            }
            super::Rows::StepContainerModel(_) => {
                crate::codegen::tables::table_names::TableName::StepContainerModel
            }
            super::Rows::StepInstrument(_) => {
                crate::codegen::tables::table_names::TableName::StepInstrument
            }
            super::Rows::StepModelCategory(_) => {
                crate::codegen::tables::table_names::TableName::StepModelCategory
            }
            super::Rows::StepModelContainerCategory(_) => {
                crate::codegen::tables::table_names::TableName::StepModelContainerCategory
            }
            super::Rows::StepModelInstrumentCategory(_) => {
                crate::codegen::tables::table_names::TableName::StepModelInstrumentCategory
            }
            super::Rows::StepModelInstrumentModel(_) => {
                crate::codegen::tables::table_names::TableName::StepModelInstrumentModel
            }
            super::Rows::StepModelInstrument(_) => {
                crate::codegen::tables::table_names::TableName::StepModelInstrument
            }
            super::Rows::StepModelNameplateCategory(_) => {
                crate::codegen::tables::table_names::TableName::StepModelNameplateCategory
            }
            super::Rows::StepModelToolCategory(_) => {
                crate::codegen::tables::table_names::TableName::StepModelToolCategory
            }
            super::Rows::StepModel(_) => crate::codegen::tables::table_names::TableName::StepModel,
            super::Rows::StepNameplateModel(_) => {
                crate::codegen::tables::table_names::TableName::StepNameplateModel
            }
            super::Rows::StepStorageContainer(_) => {
                crate::codegen::tables::table_names::TableName::StepStorageContainer
            }
            super::Rows::StepToolModel(_) => {
                crate::codegen::tables::table_names::TableName::StepToolModel
            }
            super::Rows::Step(_) => crate::codegen::tables::table_names::TableName::Step,
            super::Rows::StorageContainer(_) => {
                crate::codegen::tables::table_names::TableName::StorageContainer
            }
            super::Rows::Taxon(_) => crate::codegen::tables::table_names::TableName::Taxon,
            super::Rows::TeamMember(_) => {
                crate::codegen::tables::table_names::TableName::TeamMember
            }
            super::Rows::TeamProject(_) => {
                crate::codegen::tables::table_names::TableName::TeamProject
            }
            super::Rows::TeamState(_) => crate::codegen::tables::table_names::TableName::TeamState,
            super::Rows::Team(_) => crate::codegen::tables::table_names::TableName::Team,
            super::Rows::ToolModel(_) => crate::codegen::tables::table_names::TableName::ToolModel,
            super::Rows::TrackableLocation(_) => {
                crate::codegen::tables::table_names::TableName::TrackableLocation
            }
            super::Rows::TrackableState(_) => {
                crate::codegen::tables::table_names::TableName::TrackableState
            }
            super::Rows::Trackable(_) => crate::codegen::tables::table_names::TableName::Trackable,
            super::Rows::Unit(_) => crate::codegen::tables::table_names::TableName::Unit,
            super::Rows::UserEmail(_) => crate::codegen::tables::table_names::TableName::UserEmail,
            super::Rows::UserOrganization(_) => {
                crate::codegen::tables::table_names::TableName::UserOrganization
            }
            super::Rows::User(_) => crate::codegen::tables::table_names::TableName::User,
            super::Rows::VolumetricProcessable(_) => {
                crate::codegen::tables::table_names::TableName::VolumetricProcessable
            }
            super::Rows::WeighingInstrumentModel(_) => {
                crate::codegen::tables::table_names::TableName::WeighingInstrumentModel
            }
            super::Rows::WeighingStepModel(_) => {
                crate::codegen::tables::table_names::TableName::WeighingStepModel
            }
            super::Rows::WeighingStep(_) => {
                crate::codegen::tables::table_names::TableName::WeighingStep
            }
        }
    }
}
