impl web_common_traits::prelude::Tabular for super::Rows {
    type TableName = crate::codegen::tables::table_names::TableName;
    fn table_name(&self) -> Self::TableName {
        match self {
            super::Rows::Address(_) => crate::codegen::tables::table_names::TableName::Address,
            super::Rows::AliquotingProcedureTemplate(_) => {
                crate::codegen::tables::table_names::TableName::AliquotingProcedureTemplate
            }
            super::Rows::AliquotingProcedure(_) => {
                crate::codegen::tables::table_names::TableName::AliquotingProcedure
            }
            super::Rows::AssetCompatibilityRule(_) => {
                crate::codegen::tables::table_names::TableName::AssetCompatibilityRule
            }
            super::Rows::AssetModelAncestor(_) => {
                crate::codegen::tables::table_names::TableName::AssetModelAncestor
            }
            super::Rows::AssetModel(_) => {
                crate::codegen::tables::table_names::TableName::AssetModel
            }
            super::Rows::Asset(_) => crate::codegen::tables::table_names::TableName::Asset,
            super::Rows::BallMillMachineModel(_) => {
                crate::codegen::tables::table_names::TableName::BallMillMachineModel
            }
            super::Rows::BallMillMachine(_) => {
                crate::codegen::tables::table_names::TableName::BallMillMachine
            }
            super::Rows::BallMillProcedureTemplate(_) => {
                crate::codegen::tables::table_names::TableName::BallMillProcedureTemplate
            }
            super::Rows::BallMillProcedure(_) => {
                crate::codegen::tables::table_names::TableName::BallMillProcedure
            }
            super::Rows::BeadModel(_) => crate::codegen::tables::table_names::TableName::BeadModel,
            super::Rows::Brand(_) => crate::codegen::tables::table_names::TableName::Brand,
            super::Rows::CameraModel(_) => {
                crate::codegen::tables::table_names::TableName::CameraModel
            }
            super::Rows::Camera(_) => crate::codegen::tables::table_names::TableName::Camera,
            super::Rows::CapModel(_) => crate::codegen::tables::table_names::TableName::CapModel,
            super::Rows::CappingProcedureTemplate(_) => {
                crate::codegen::tables::table_names::TableName::CappingProcedureTemplate
            }
            super::Rows::CappingProcedure(_) => {
                crate::codegen::tables::table_names::TableName::CappingProcedure
            }
            super::Rows::CentrifugeModel(_) => {
                crate::codegen::tables::table_names::TableName::CentrifugeModel
            }
            super::Rows::CentrifugeProcedureTemplate(_) => {
                crate::codegen::tables::table_names::TableName::CentrifugeProcedureTemplate
            }
            super::Rows::CentrifugeProcedure(_) => {
                crate::codegen::tables::table_names::TableName::CentrifugeProcedure
            }
            super::Rows::Centrifuge(_) => {
                crate::codegen::tables::table_names::TableName::Centrifuge
            }
            super::Rows::City(_) => crate::codegen::tables::table_names::TableName::City,
            super::Rows::Color(_) => crate::codegen::tables::table_names::TableName::Color,
            super::Rows::CommercialBallMillMachineLot(_) => {
                crate::codegen::tables::table_names::TableName::CommercialBallMillMachineLot
            }
            super::Rows::CommercialBallMillMachineModel(_) => {
                crate::codegen::tables::table_names::TableName::CommercialBallMillMachineModel
            }
            super::Rows::CommercialBeadLot(_) => {
                crate::codegen::tables::table_names::TableName::CommercialBeadLot
            }
            super::Rows::CommercialBeadModel(_) => {
                crate::codegen::tables::table_names::TableName::CommercialBeadModel
            }
            super::Rows::CommercialCameraLot(_) => {
                crate::codegen::tables::table_names::TableName::CommercialCameraLot
            }
            super::Rows::CommercialCameraModel(_) => {
                crate::codegen::tables::table_names::TableName::CommercialCameraModel
            }
            super::Rows::CommercialCapLot(_) => {
                crate::codegen::tables::table_names::TableName::CommercialCapLot
            }
            super::Rows::CommercialCapModel(_) => {
                crate::codegen::tables::table_names::TableName::CommercialCapModel
            }
            super::Rows::CommercialCentrifugeLot(_) => {
                crate::codegen::tables::table_names::TableName::CommercialCentrifugeLot
            }
            super::Rows::CommercialCentrifugeModel(_) => {
                crate::codegen::tables::table_names::TableName::CommercialCentrifugeModel
            }
            super::Rows::CommercialFreezeDryerLot(_) => {
                crate::codegen::tables::table_names::TableName::CommercialFreezeDryerLot
            }
            super::Rows::CommercialFreezeDryerModel(_) => {
                crate::codegen::tables::table_names::TableName::CommercialFreezeDryerModel
            }
            super::Rows::CommercialFreezerLot(_) => {
                crate::codegen::tables::table_names::TableName::CommercialFreezerLot
            }
            super::Rows::CommercialFreezerModel(_) => {
                crate::codegen::tables::table_names::TableName::CommercialFreezerModel
            }
            super::Rows::CommercialPackagingLot(_) => {
                crate::codegen::tables::table_names::TableName::CommercialPackagingLot
            }
            super::Rows::CommercialPackagingModel(_) => {
                crate::codegen::tables::table_names::TableName::CommercialPackagingModel
            }
            super::Rows::CommercialPipetteLot(_) => {
                crate::codegen::tables::table_names::TableName::CommercialPipetteLot
            }
            super::Rows::CommercialPipetteModel(_) => {
                crate::codegen::tables::table_names::TableName::CommercialPipetteModel
            }
            super::Rows::CommercialPipetteTipLot(_) => {
                crate::codegen::tables::table_names::TableName::CommercialPipetteTipLot
            }
            super::Rows::CommercialPipetteTipModel(_) => {
                crate::codegen::tables::table_names::TableName::CommercialPipetteTipModel
            }
            super::Rows::CommercialPositioningDeviceLot(_) => {
                crate::codegen::tables::table_names::TableName::CommercialPositioningDeviceLot
            }
            super::Rows::CommercialPositioningDeviceModel(_) => {
                crate::codegen::tables::table_names::TableName::CommercialPositioningDeviceModel
            }
            super::Rows::CommercialProductLot(_) => {
                crate::codegen::tables::table_names::TableName::CommercialProductLot
            }
            super::Rows::CommercialProduct(_) => {
                crate::codegen::tables::table_names::TableName::CommercialProduct
            }
            super::Rows::CommercialVolumeMeasuringDeviceLot(_) => {
                crate::codegen::tables::table_names::TableName::CommercialVolumeMeasuringDeviceLot
            }
            super::Rows::CommercialVolumeMeasuringDeviceModel(_) => {
                crate::codegen::tables::table_names::TableName::CommercialVolumeMeasuringDeviceModel
            }
            super::Rows::CommercialWeighingDeviceLot(_) => {
                crate::codegen::tables::table_names::TableName::CommercialWeighingDeviceLot
            }
            super::Rows::CommercialWeighingDeviceModel(_) => {
                crate::codegen::tables::table_names::TableName::CommercialWeighingDeviceModel
            }
            super::Rows::ContainerCompatibilityRule(_) => {
                crate::codegen::tables::table_names::TableName::ContainerCompatibilityRule
            }
            super::Rows::ContainerModel(_) => {
                crate::codegen::tables::table_names::TableName::ContainerModel
            }
            super::Rows::Container(_) => crate::codegen::tables::table_names::TableName::Container,
            super::Rows::Country(_) => crate::codegen::tables::table_names::TableName::Country,
            super::Rows::DigitalAssetModel(_) => {
                crate::codegen::tables::table_names::TableName::DigitalAssetModel
            }
            super::Rows::DigitalAsset(_) => {
                crate::codegen::tables::table_names::TableName::DigitalAsset
            }
            super::Rows::DisposalProcedureTemplate(_) => {
                crate::codegen::tables::table_names::TableName::DisposalProcedureTemplate
            }
            super::Rows::DisposalProcedure(_) => {
                crate::codegen::tables::table_names::TableName::DisposalProcedure
            }
            super::Rows::EmailProvider(_) => {
                crate::codegen::tables::table_names::TableName::EmailProvider
            }
            super::Rows::FractioningProcedureTemplate(_) => {
                crate::codegen::tables::table_names::TableName::FractioningProcedureTemplate
            }
            super::Rows::FractioningProcedure(_) => {
                crate::codegen::tables::table_names::TableName::FractioningProcedure
            }
            super::Rows::FreezeDryerModel(_) => {
                crate::codegen::tables::table_names::TableName::FreezeDryerModel
            }
            super::Rows::FreezeDryer(_) => {
                crate::codegen::tables::table_names::TableName::FreezeDryer
            }
            super::Rows::FreezeDryingProcedureTemplate(_) => {
                crate::codegen::tables::table_names::TableName::FreezeDryingProcedureTemplate
            }
            super::Rows::FreezeDryingProcedure(_) => {
                crate::codegen::tables::table_names::TableName::FreezeDryingProcedure
            }
            super::Rows::FreezerModel(_) => {
                crate::codegen::tables::table_names::TableName::FreezerModel
            }
            super::Rows::Freezer(_) => crate::codegen::tables::table_names::TableName::Freezer,
            super::Rows::FreezingProcedureTemplate(_) => {
                crate::codegen::tables::table_names::TableName::FreezingProcedureTemplate
            }
            super::Rows::FreezingProcedure(_) => {
                crate::codegen::tables::table_names::TableName::FreezingProcedure
            }
            super::Rows::GeolocationProcedureTemplate(_) => {
                crate::codegen::tables::table_names::TableName::GeolocationProcedureTemplate
            }
            super::Rows::GeolocationProcedure(_) => {
                crate::codegen::tables::table_names::TableName::GeolocationProcedure
            }
            super::Rows::HarvestingProcedureTemplate(_) => {
                crate::codegen::tables::table_names::TableName::HarvestingProcedureTemplate
            }
            super::Rows::HarvestingProcedure(_) => {
                crate::codegen::tables::table_names::TableName::HarvestingProcedure
            }
            super::Rows::InstrumentState(_) => {
                crate::codegen::tables::table_names::TableName::InstrumentState
            }
            super::Rows::LoginProvider(_) => {
                crate::codegen::tables::table_names::TableName::LoginProvider
            }
            super::Rows::Material(_) => crate::codegen::tables::table_names::TableName::Material,
            super::Rows::NextProcedureTemplate(_) => {
                crate::codegen::tables::table_names::TableName::NextProcedureTemplate
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
            super::Rows::PackagingModel(_) => {
                crate::codegen::tables::table_names::TableName::PackagingModel
            }
            super::Rows::PackagingProcedureTemplate(_) => {
                crate::codegen::tables::table_names::TableName::PackagingProcedureTemplate
            }
            super::Rows::PackagingProcedure(_) => {
                crate::codegen::tables::table_names::TableName::PackagingProcedure
            }
            super::Rows::ParentProcedureTemplate(_) => {
                crate::codegen::tables::table_names::TableName::ParentProcedureTemplate
            }
            super::Rows::PermanenceCategory(_) => {
                crate::codegen::tables::table_names::TableName::PermanenceCategory
            }
            super::Rows::PhoneModel(_) => {
                crate::codegen::tables::table_names::TableName::PhoneModel
            }
            super::Rows::PhotographProcedureTemplate(_) => {
                crate::codegen::tables::table_names::TableName::PhotographProcedureTemplate
            }
            super::Rows::PhotographProcedure(_) => {
                crate::codegen::tables::table_names::TableName::PhotographProcedure
            }
            super::Rows::Photograph(_) => {
                crate::codegen::tables::table_names::TableName::Photograph
            }
            super::Rows::PhysicalAssetModel(_) => {
                crate::codegen::tables::table_names::TableName::PhysicalAssetModel
            }
            super::Rows::PhysicalAsset(_) => {
                crate::codegen::tables::table_names::TableName::PhysicalAsset
            }
            super::Rows::PipetteModel(_) => {
                crate::codegen::tables::table_names::TableName::PipetteModel
            }
            super::Rows::PipetteTipModel(_) => {
                crate::codegen::tables::table_names::TableName::PipetteTipModel
            }
            super::Rows::Pipette(_) => crate::codegen::tables::table_names::TableName::Pipette,
            super::Rows::PositioningDeviceModel(_) => {
                crate::codegen::tables::table_names::TableName::PositioningDeviceModel
            }
            super::Rows::PositioningDevice(_) => {
                crate::codegen::tables::table_names::TableName::PositioningDevice
            }
            super::Rows::PouringProcedureTemplate(_) => {
                crate::codegen::tables::table_names::TableName::PouringProcedureTemplate
            }
            super::Rows::PouringProcedure(_) => {
                crate::codegen::tables::table_names::TableName::PouringProcedure
            }
            super::Rows::ProcedureAsset(_) => {
                crate::codegen::tables::table_names::TableName::ProcedureAsset
            }
            super::Rows::ProcedureTemplateAssetModel(_) => {
                crate::codegen::tables::table_names::TableName::ProcedureTemplateAssetModel
            }
            super::Rows::ProcedureTemplate(_) => {
                crate::codegen::tables::table_names::TableName::ProcedureTemplate
            }
            super::Rows::Procedure(_) => crate::codegen::tables::table_names::TableName::Procedure,
            super::Rows::ProjectState(_) => {
                crate::codegen::tables::table_names::TableName::ProjectState
            }
            super::Rows::Project(_) => crate::codegen::tables::table_names::TableName::Project,
            super::Rows::Rank(_) => crate::codegen::tables::table_names::TableName::Rank,
            super::Rows::ReagentModel(_) => {
                crate::codegen::tables::table_names::TableName::ReagentModel
            }
            super::Rows::Role(_) => crate::codegen::tables::table_names::TableName::Role,
            super::Rows::Room(_) => crate::codegen::tables::table_names::TableName::Room,
            super::Rows::SampleModel(_) => {
                crate::codegen::tables::table_names::TableName::SampleModel
            }
            super::Rows::SampleSourceModel(_) => {
                crate::codegen::tables::table_names::TableName::SampleSourceModel
            }
            super::Rows::SampleSource(_) => {
                crate::codegen::tables::table_names::TableName::SampleSource
            }
            super::Rows::SampleState(_) => {
                crate::codegen::tables::table_names::TableName::SampleState
            }
            super::Rows::Sample(_) => crate::codegen::tables::table_names::TableName::Sample,
            super::Rows::SpatialRefSy(_) => {
                crate::codegen::tables::table_names::TableName::SpatialRefSy
            }
            super::Rows::Spectrum(_) => crate::codegen::tables::table_names::TableName::Spectrum,
            super::Rows::SpectraCollection(_) => {
                crate::codegen::tables::table_names::TableName::SpectraCollection
            }
            super::Rows::StorageProcedureTemplate(_) => {
                crate::codegen::tables::table_names::TableName::StorageProcedureTemplate
            }
            super::Rows::StorageProcedure(_) => {
                crate::codegen::tables::table_names::TableName::StorageProcedure
            }
            super::Rows::SupernatantProcedureTemplate(_) => {
                crate::codegen::tables::table_names::TableName::SupernatantProcedureTemplate
            }
            super::Rows::SupernatantProcedure(_) => {
                crate::codegen::tables::table_names::TableName::SupernatantProcedure
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
            super::Rows::Unit(_) => crate::codegen::tables::table_names::TableName::Unit,
            super::Rows::UserEmail(_) => crate::codegen::tables::table_names::TableName::UserEmail,
            super::Rows::UserOrganization(_) => {
                crate::codegen::tables::table_names::TableName::UserOrganization
            }
            super::Rows::User(_) => crate::codegen::tables::table_names::TableName::User,
            super::Rows::VolumeMeasuringDeviceModel(_) => {
                crate::codegen::tables::table_names::TableName::VolumeMeasuringDeviceModel
            }
            super::Rows::VolumeMeasuringDevice(_) => {
                crate::codegen::tables::table_names::TableName::VolumeMeasuringDevice
            }
            super::Rows::VolumetricContainerModel(_) => {
                crate::codegen::tables::table_names::TableName::VolumetricContainerModel
            }
            super::Rows::VolumetricContainer(_) => {
                crate::codegen::tables::table_names::TableName::VolumetricContainer
            }
            super::Rows::WeighingDeviceModel(_) => {
                crate::codegen::tables::table_names::TableName::WeighingDeviceModel
            }
            super::Rows::WeighingDevice(_) => {
                crate::codegen::tables::table_names::TableName::WeighingDevice
            }
            super::Rows::WeighingProcedureTemplate(_) => {
                crate::codegen::tables::table_names::TableName::WeighingProcedureTemplate
            }
            super::Rows::WeighingProcedure(_) => {
                crate::codegen::tables::table_names::TableName::WeighingProcedure
            }
        }
    }
}
