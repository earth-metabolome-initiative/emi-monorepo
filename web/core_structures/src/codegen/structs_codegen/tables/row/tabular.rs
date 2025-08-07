impl web_common_traits::prelude::Tabular for super::Row {
    type TableName = crate::codegen::tables::table_names::TableName;
    fn table_name(&self) -> Self::TableName {
        match self {
            super::Row::Address(_) => crate::codegen::tables::table_names::TableName::Address,
            super::Row::AliquotingProcedureModel(_) => {
                crate::codegen::tables::table_names::TableName::AliquotingProcedureModel
            }
            super::Row::AliquotingProcedure(_) => {
                crate::codegen::tables::table_names::TableName::AliquotingProcedure
            }
            super::Row::BallMillMachineModel(_) => {
                crate::codegen::tables::table_names::TableName::BallMillMachineModel
            }
            super::Row::BallMillProcedureModel(_) => {
                crate::codegen::tables::table_names::TableName::BallMillProcedureModel
            }
            super::Row::BinaryQuestionProcedureModel(_) => {
                crate::codegen::tables::table_names::TableName::BinaryQuestionProcedureModel
            }
            super::Row::Brand(_) => crate::codegen::tables::table_names::TableName::Brand,
            super::Row::CameraModel(_) => {
                crate::codegen::tables::table_names::TableName::CameraModel
            }
            super::Row::CappingProcedureModel(_) => {
                crate::codegen::tables::table_names::TableName::CappingProcedureModel
            }
            super::Row::CentrifugeModel(_) => {
                crate::codegen::tables::table_names::TableName::CentrifugeModel
            }
            super::Row::CentrifugeProcedureModel(_) => {
                crate::codegen::tables::table_names::TableName::CentrifugeProcedureModel
            }
            super::Row::City(_) => crate::codegen::tables::table_names::TableName::City,
            super::Row::Color(_) => crate::codegen::tables::table_names::TableName::Color,
            super::Row::CommercialProductLot(_) => {
                crate::codegen::tables::table_names::TableName::CommercialProductLot
            }
            super::Row::CommercialProduct(_) => {
                crate::codegen::tables::table_names::TableName::CommercialProduct
            }
            super::Row::CommercialReagent(_) => {
                crate::codegen::tables::table_names::TableName::CommercialReagent
            }
            super::Row::CompatibilityRule(_) => {
                crate::codegen::tables::table_names::TableName::CompatibilityRule
            }
            super::Row::ContainerModel(_) => {
                crate::codegen::tables::table_names::TableName::ContainerModel
            }
            super::Row::Container(_) => crate::codegen::tables::table_names::TableName::Container,
            super::Row::Country(_) => crate::codegen::tables::table_names::TableName::Country,
            super::Row::DisposalProcedureModel(_) => {
                crate::codegen::tables::table_names::TableName::DisposalProcedureModel
            }
            super::Row::Document(_) => crate::codegen::tables::table_names::TableName::Document,
            super::Row::EmailProvider(_) => {
                crate::codegen::tables::table_names::TableName::EmailProvider
            }
            super::Row::FractioningProcedureModel(_) => {
                crate::codegen::tables::table_names::TableName::FractioningProcedureModel
            }
            super::Row::FreezeDrierModel(_) => {
                crate::codegen::tables::table_names::TableName::FreezeDrierModel
            }
            super::Row::FreezeDryingProcedureModel(_) => {
                crate::codegen::tables::table_names::TableName::FreezeDryingProcedureModel
            }
            super::Row::FreezerModel(_) => {
                crate::codegen::tables::table_names::TableName::FreezerModel
            }
            super::Row::FreezingProcedureModel(_) => {
                crate::codegen::tables::table_names::TableName::FreezingProcedureModel
            }
            super::Row::GeolocationProcedureModel(_) => {
                crate::codegen::tables::table_names::TableName::GeolocationProcedureModel
            }
            super::Row::InstrumentModel(_) => {
                crate::codegen::tables::table_names::TableName::InstrumentModel
            }
            super::Row::InstrumentState(_) => {
                crate::codegen::tables::table_names::TableName::InstrumentState
            }
            super::Row::LoginProvider(_) => {
                crate::codegen::tables::table_names::TableName::LoginProvider
            }
            super::Row::Material(_) => crate::codegen::tables::table_names::TableName::Material,
            super::Row::MixingProcedureModel(_) => {
                crate::codegen::tables::table_names::TableName::MixingProcedureModel
            }
            super::Row::NextProcedureModel(_) => {
                crate::codegen::tables::table_names::TableName::NextProcedureModel
            }
            super::Row::ObservationSubject(_) => {
                crate::codegen::tables::table_names::TableName::ObservationSubject
            }
            super::Row::OrganismTaxon(_) => {
                crate::codegen::tables::table_names::TableName::OrganismTaxon
            }
            super::Row::Organism(_) => crate::codegen::tables::table_names::TableName::Organism,
            super::Row::Organization(_) => {
                crate::codegen::tables::table_names::TableName::Organization
            }
            super::Row::PackagingProcedureModel(_) => {
                crate::codegen::tables::table_names::TableName::PackagingProcedureModel
            }
            super::Row::ParentProcedureModel(_) => {
                crate::codegen::tables::table_names::TableName::ParentProcedureModel
            }
            super::Row::PermanenceCategory(_) => {
                crate::codegen::tables::table_names::TableName::PermanenceCategory
            }
            super::Row::PhoneModel(_) => crate::codegen::tables::table_names::TableName::PhoneModel,
            super::Row::PhotographProcedureModel(_) => {
                crate::codegen::tables::table_names::TableName::PhotographProcedureModel
            }
            super::Row::PipetteModel(_) => {
                crate::codegen::tables::table_names::TableName::PipetteModel
            }
            super::Row::PipetteTipModel(_) => {
                crate::codegen::tables::table_names::TableName::PipetteTipModel
            }
            super::Row::PlacingProcedureModel(_) => {
                crate::codegen::tables::table_names::TableName::PlacingProcedureModel
            }
            super::Row::PositioningDeviceModel(_) => {
                crate::codegen::tables::table_names::TableName::PositioningDeviceModel
            }
            super::Row::PouringProcedureModel(_) => {
                crate::codegen::tables::table_names::TableName::PouringProcedureModel
            }
            super::Row::ProcedureModelTrackable(_) => {
                crate::codegen::tables::table_names::TableName::ProcedureModelTrackable
            }
            super::Row::ProcedureModel(_) => {
                crate::codegen::tables::table_names::TableName::ProcedureModel
            }
            super::Row::ProcedureTrackable(_) => {
                crate::codegen::tables::table_names::TableName::ProcedureTrackable
            }
            super::Row::Procedure(_) => crate::codegen::tables::table_names::TableName::Procedure,
            super::Row::Processable(_) => {
                crate::codegen::tables::table_names::TableName::Processable
            }
            super::Row::ProjectState(_) => {
                crate::codegen::tables::table_names::TableName::ProjectState
            }
            super::Row::Project(_) => crate::codegen::tables::table_names::TableName::Project,
            super::Row::Rank(_) => crate::codegen::tables::table_names::TableName::Rank,
            super::Row::Reagent(_) => crate::codegen::tables::table_names::TableName::Reagent,
            super::Row::Role(_) => crate::codegen::tables::table_names::TableName::Role,
            super::Row::Room(_) => crate::codegen::tables::table_names::TableName::Room,
            super::Row::SampleState(_) => {
                crate::codegen::tables::table_names::TableName::SampleState
            }
            super::Row::SharedProcedureModelTrackable(_) => {
                crate::codegen::tables::table_names::TableName::SharedProcedureModelTrackable
            }
            super::Row::SpatialRefSy(_) => {
                crate::codegen::tables::table_names::TableName::SpatialRefSy
            }
            super::Row::Spectrum(_) => crate::codegen::tables::table_names::TableName::Spectrum,
            super::Row::SpectraCollection(_) => {
                crate::codegen::tables::table_names::TableName::SpectraCollection
            }
            super::Row::StorageProcedureModel(_) => {
                crate::codegen::tables::table_names::TableName::StorageProcedureModel
            }
            super::Row::SupernatantProcedureModel(_) => {
                crate::codegen::tables::table_names::TableName::SupernatantProcedureModel
            }
            super::Row::SupernatantProcedure(_) => {
                crate::codegen::tables::table_names::TableName::SupernatantProcedure
            }
            super::Row::Taxon(_) => crate::codegen::tables::table_names::TableName::Taxon,
            super::Row::TeamMember(_) => crate::codegen::tables::table_names::TableName::TeamMember,
            super::Row::TeamProject(_) => {
                crate::codegen::tables::table_names::TableName::TeamProject
            }
            super::Row::TeamState(_) => crate::codegen::tables::table_names::TableName::TeamState,
            super::Row::Team(_) => crate::codegen::tables::table_names::TableName::Team,
            super::Row::TemporaryUser(_) => {
                crate::codegen::tables::table_names::TableName::TemporaryUser
            }
            super::Row::TrackableAncestor(_) => {
                crate::codegen::tables::table_names::TableName::TrackableAncestor
            }
            super::Row::TrackableLocation(_) => {
                crate::codegen::tables::table_names::TableName::TrackableLocation
            }
            super::Row::Trackable(_) => crate::codegen::tables::table_names::TableName::Trackable,
            super::Row::Unit(_) => crate::codegen::tables::table_names::TableName::Unit,
            super::Row::UserEmail(_) => crate::codegen::tables::table_names::TableName::UserEmail,
            super::Row::UserOrganization(_) => {
                crate::codegen::tables::table_names::TableName::UserOrganization
            }
            super::Row::User(_) => crate::codegen::tables::table_names::TableName::User,
            super::Row::VolumetricContainerModel(_) => {
                crate::codegen::tables::table_names::TableName::VolumetricContainerModel
            }
            super::Row::VolumetricProcessable(_) => {
                crate::codegen::tables::table_names::TableName::VolumetricProcessable
            }
            super::Row::WeighingInstrumentModel(_) => {
                crate::codegen::tables::table_names::TableName::WeighingInstrumentModel
            }
            super::Row::WeighingProcedureModel(_) => {
                crate::codegen::tables::table_names::TableName::WeighingProcedureModel
            }
            super::Row::WeighingProcedure(_) => {
                crate::codegen::tables::table_names::TableName::WeighingProcedure
            }
        }
    }
}
