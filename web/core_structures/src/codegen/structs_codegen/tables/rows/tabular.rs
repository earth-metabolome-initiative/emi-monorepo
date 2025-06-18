impl web_common_traits::prelude::Tabular for super::Rows {
    type TableName = crate::codegen::tables::table_names::TableName;
    fn table_name(&self) -> Self::TableName {
        match self {
            super::Rows::Address(_) => crate::codegen::tables::table_names::TableName::Address,
            super::Rows::AliquotingProcedureModel(_) => {
                crate::codegen::tables::table_names::TableName::AliquotingProcedureModel
            }
            super::Rows::BallMillContainerModel(_) => {
                crate::codegen::tables::table_names::TableName::BallMillContainerModel
            }
            super::Rows::BallMillMachineModel(_) => {
                crate::codegen::tables::table_names::TableName::BallMillMachineModel
            }
            super::Rows::BallMillProcedureModel(_) => {
                crate::codegen::tables::table_names::TableName::BallMillProcedureModel
            }
            super::Rows::BinaryQuestionProcedureModel(_) => {
                crate::codegen::tables::table_names::TableName::BinaryQuestionProcedureModel
            }
            super::Rows::Brand(_) => crate::codegen::tables::table_names::TableName::Brand,
            super::Rows::CameraModel(_) => {
                crate::codegen::tables::table_names::TableName::CameraModel
            }
            super::Rows::CappingProcedureModel(_) => {
                crate::codegen::tables::table_names::TableName::CappingProcedureModel
            }
            super::Rows::CappingRule(_) => {
                crate::codegen::tables::table_names::TableName::CappingRule
            }
            super::Rows::CentrifugableContainerModel(_) => {
                crate::codegen::tables::table_names::TableName::CentrifugableContainerModel
            }
            super::Rows::CentrifugeModel(_) => {
                crate::codegen::tables::table_names::TableName::CentrifugeModel
            }
            super::Rows::CentrifugeProcedureModel(_) => {
                crate::codegen::tables::table_names::TableName::CentrifugeProcedureModel
            }
            super::Rows::City(_) => crate::codegen::tables::table_names::TableName::City,
            super::Rows::Color(_) => crate::codegen::tables::table_names::TableName::Color,
            super::Rows::CommercialProductLot(_) => {
                crate::codegen::tables::table_names::TableName::CommercialProductLot
            }
            super::Rows::CommercialProduct(_) => {
                crate::codegen::tables::table_names::TableName::CommercialProduct
            }
            super::Rows::CommercialReagent(_) => {
                crate::codegen::tables::table_names::TableName::CommercialReagent
            }
            super::Rows::ContainerModel(_) => {
                crate::codegen::tables::table_names::TableName::ContainerModel
            }
            super::Rows::Container(_) => crate::codegen::tables::table_names::TableName::Container,
            super::Rows::Country(_) => crate::codegen::tables::table_names::TableName::Country,
            super::Rows::DisposalProcedureModel(_) => {
                crate::codegen::tables::table_names::TableName::DisposalProcedureModel
            }
            super::Rows::Document(_) => crate::codegen::tables::table_names::TableName::Document,
            super::Rows::EmailProvider(_) => {
                crate::codegen::tables::table_names::TableName::EmailProvider
            }
            super::Rows::FractioningProcedureModel(_) => {
                crate::codegen::tables::table_names::TableName::FractioningProcedureModel
            }
            super::Rows::FreezeDrierModel(_) => {
                crate::codegen::tables::table_names::TableName::FreezeDrierModel
            }
            super::Rows::FreezeDryingProcedureModel(_) => {
                crate::codegen::tables::table_names::TableName::FreezeDryingProcedureModel
            }
            super::Rows::FreezerModel(_) => {
                crate::codegen::tables::table_names::TableName::FreezerModel
            }
            super::Rows::FreezingProcedureModel(_) => {
                crate::codegen::tables::table_names::TableName::FreezingProcedureModel
            }
            super::Rows::GeolocationProcedureModel(_) => {
                crate::codegen::tables::table_names::TableName::GeolocationProcedureModel
            }
            super::Rows::InstrumentModel(_) => {
                crate::codegen::tables::table_names::TableName::InstrumentModel
            }
            super::Rows::InstrumentState(_) => {
                crate::codegen::tables::table_names::TableName::InstrumentState
            }
            super::Rows::LoginProvider(_) => {
                crate::codegen::tables::table_names::TableName::LoginProvider
            }
            super::Rows::Material(_) => crate::codegen::tables::table_names::TableName::Material,
            super::Rows::MixCountableProcedureModel(_) => {
                crate::codegen::tables::table_names::TableName::MixCountableProcedureModel
            }
            super::Rows::MixSolidProcedureModel(_) => {
                crate::codegen::tables::table_names::TableName::MixSolidProcedureModel
            }
            super::Rows::MountTipProcedureModel(_) => {
                crate::codegen::tables::table_names::TableName::MountTipProcedureModel
            }
            super::Rows::NextProcedureModel(_) => {
                crate::codegen::tables::table_names::TableName::NextProcedureModel
            }
            super::Rows::ObservationSubject(_) => {
                crate::codegen::tables::table_names::TableName::ObservationSubject
            }
            super::Rows::OrganismTaxon(_) => {
                crate::codegen::tables::table_names::TableName::OrganismTaxon
            }
            super::Rows::Organism(_) => crate::codegen::tables::table_names::TableName::Organism,
            super::Rows::Organization(_) => {
                crate::codegen::tables::table_names::TableName::Organization
            }
            super::Rows::PackagingProcedureModel(_) => {
                crate::codegen::tables::table_names::TableName::PackagingProcedureModel
            }
            super::Rows::ParentProcedureModel(_) => {
                crate::codegen::tables::table_names::TableName::ParentProcedureModel
            }
            super::Rows::PermanenceCategory(_) => {
                crate::codegen::tables::table_names::TableName::PermanenceCategory
            }
            super::Rows::PhotographProcedureModel(_) => {
                crate::codegen::tables::table_names::TableName::PhotographProcedureModel
            }
            super::Rows::PositioningDeviceModel(_) => {
                crate::codegen::tables::table_names::TableName::PositioningDeviceModel
            }
            super::Rows::PouringProcedureModel(_) => {
                crate::codegen::tables::table_names::TableName::PouringProcedureModel
            }
            super::Rows::ProcedureModelTrackable(_) => {
                crate::codegen::tables::table_names::TableName::ProcedureModelTrackable
            }
            super::Rows::ProcedureModel(_) => {
                crate::codegen::tables::table_names::TableName::ProcedureModel
            }
            super::Rows::ProcedureTrackable(_) => {
                crate::codegen::tables::table_names::TableName::ProcedureTrackable
            }
            super::Rows::Procedure(_) => crate::codegen::tables::table_names::TableName::Procedure,
            super::Rows::Processable(_) => {
                crate::codegen::tables::table_names::TableName::Processable
            }
            super::Rows::ProjectState(_) => {
                crate::codegen::tables::table_names::TableName::ProjectState
            }
            super::Rows::Project(_) => crate::codegen::tables::table_names::TableName::Project,
            super::Rows::Rank(_) => crate::codegen::tables::table_names::TableName::Rank,
            super::Rows::Reagent(_) => crate::codegen::tables::table_names::TableName::Reagent,
            super::Rows::Role(_) => crate::codegen::tables::table_names::TableName::Role,
            super::Rows::Room(_) => crate::codegen::tables::table_names::TableName::Room,
            super::Rows::SampleState(_) => {
                crate::codegen::tables::table_names::TableName::SampleState
            }
            super::Rows::SharedProcedureModelTrackable(_) => {
                crate::codegen::tables::table_names::TableName::SharedProcedureModelTrackable
            }
            super::Rows::SpatialRefSy(_) => {
                crate::codegen::tables::table_names::TableName::SpatialRefSy
            }
            super::Rows::Spectrum(_) => crate::codegen::tables::table_names::TableName::Spectrum,
            super::Rows::SpectraCollection(_) => {
                crate::codegen::tables::table_names::TableName::SpectraCollection
            }
            super::Rows::StorageProcedureModel(_) => {
                crate::codegen::tables::table_names::TableName::StorageProcedureModel
            }
            super::Rows::StorageRule(_) => {
                crate::codegen::tables::table_names::TableName::StorageRule
            }
            super::Rows::SupernatantProcedureModel(_) => {
                crate::codegen::tables::table_names::TableName::SupernatantProcedureModel
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
            super::Rows::TemporaryUser(_) => {
                crate::codegen::tables::table_names::TableName::TemporaryUser
            }
            super::Rows::TrackableLocation(_) => {
                crate::codegen::tables::table_names::TableName::TrackableLocation
            }
            super::Rows::Trackable(_) => crate::codegen::tables::table_names::TableName::Trackable,
            super::Rows::Unit(_) => crate::codegen::tables::table_names::TableName::Unit,
            super::Rows::UserEmail(_) => crate::codegen::tables::table_names::TableName::UserEmail,
            super::Rows::UserOrganization(_) => {
                crate::codegen::tables::table_names::TableName::UserOrganization
            }
            super::Rows::User(_) => crate::codegen::tables::table_names::TableName::User,
            super::Rows::VolumetricContainerModel(_) => {
                crate::codegen::tables::table_names::TableName::VolumetricContainerModel
            }
            super::Rows::VolumetricProcessable(_) => {
                crate::codegen::tables::table_names::TableName::VolumetricProcessable
            }
            super::Rows::WeighingInstrumentModel(_) => {
                crate::codegen::tables::table_names::TableName::WeighingInstrumentModel
            }
            super::Rows::WeighingProcedureModel(_) => {
                crate::codegen::tables::table_names::TableName::WeighingProcedureModel
            }
            super::Rows::WeighingProcedure(_) => {
                crate::codegen::tables::table_names::TableName::WeighingProcedure
            }
        }
    }
}
