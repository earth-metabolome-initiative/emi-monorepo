impl web_common_traits::prelude::Tabular for super::Row {
    type TableName = crate::codegen::tables::table_names::TableName;
    fn table_name(&self) -> Self::TableName {
        match self {
            super::Row::Address(_) => crate::codegen::tables::table_names::TableName::Address,
            super::Row::AliquotingProcedureTemplate(_) => {
                crate::codegen::tables::table_names::TableName::AliquotingProcedureTemplate
            }
            super::Row::AliquotingProcedure(_) => {
                crate::codegen::tables::table_names::TableName::AliquotingProcedure
            }
            super::Row::AssetCompatibilityRule(_) => {
                crate::codegen::tables::table_names::TableName::AssetCompatibilityRule
            }
            super::Row::AssetModelAncestor(_) => {
                crate::codegen::tables::table_names::TableName::AssetModelAncestor
            }
            super::Row::AssetModel(_) => crate::codegen::tables::table_names::TableName::AssetModel,
            super::Row::Asset(_) => crate::codegen::tables::table_names::TableName::Asset,
            super::Row::BallMillMachineModel(_) => {
                crate::codegen::tables::table_names::TableName::BallMillMachineModel
            }
            super::Row::BallMillMachine(_) => {
                crate::codegen::tables::table_names::TableName::BallMillMachine
            }
            super::Row::BallMillProcedureTemplate(_) => {
                crate::codegen::tables::table_names::TableName::BallMillProcedureTemplate
            }
            super::Row::BallMillProcedure(_) => {
                crate::codegen::tables::table_names::TableName::BallMillProcedure
            }
            super::Row::BeadModel(_) => crate::codegen::tables::table_names::TableName::BeadModel,
            super::Row::Brand(_) => crate::codegen::tables::table_names::TableName::Brand,
            super::Row::CameraModel(_) => {
                crate::codegen::tables::table_names::TableName::CameraModel
            }
            super::Row::Camera(_) => crate::codegen::tables::table_names::TableName::Camera,
            super::Row::CapModel(_) => crate::codegen::tables::table_names::TableName::CapModel,
            super::Row::CappingProcedureTemplate(_) => {
                crate::codegen::tables::table_names::TableName::CappingProcedureTemplate
            }
            super::Row::CappingProcedure(_) => {
                crate::codegen::tables::table_names::TableName::CappingProcedure
            }
            super::Row::CentrifugeModel(_) => {
                crate::codegen::tables::table_names::TableName::CentrifugeModel
            }
            super::Row::CentrifugeProcedureTemplate(_) => {
                crate::codegen::tables::table_names::TableName::CentrifugeProcedureTemplate
            }
            super::Row::CentrifugeProcedure(_) => {
                crate::codegen::tables::table_names::TableName::CentrifugeProcedure
            }
            super::Row::Centrifuge(_) => crate::codegen::tables::table_names::TableName::Centrifuge,
            super::Row::City(_) => crate::codegen::tables::table_names::TableName::City,
            super::Row::CleaningProcedureTemplate(_) => {
                crate::codegen::tables::table_names::TableName::CleaningProcedureTemplate
            }
            super::Row::CleaningProcedure(_) => {
                crate::codegen::tables::table_names::TableName::CleaningProcedure
            }
            super::Row::Color(_) => crate::codegen::tables::table_names::TableName::Color,
            super::Row::CommercialBallMillMachineLot(_) => {
                crate::codegen::tables::table_names::TableName::CommercialBallMillMachineLot
            }
            super::Row::CommercialBallMillMachineModel(_) => {
                crate::codegen::tables::table_names::TableName::CommercialBallMillMachineModel
            }
            super::Row::CommercialBeadLot(_) => {
                crate::codegen::tables::table_names::TableName::CommercialBeadLot
            }
            super::Row::CommercialBeadModel(_) => {
                crate::codegen::tables::table_names::TableName::CommercialBeadModel
            }
            super::Row::CommercialCameraLot(_) => {
                crate::codegen::tables::table_names::TableName::CommercialCameraLot
            }
            super::Row::CommercialCameraModel(_) => {
                crate::codegen::tables::table_names::TableName::CommercialCameraModel
            }
            super::Row::CommercialCapLot(_) => {
                crate::codegen::tables::table_names::TableName::CommercialCapLot
            }
            super::Row::CommercialCapModel(_) => {
                crate::codegen::tables::table_names::TableName::CommercialCapModel
            }
            super::Row::CommercialCentrifugeLot(_) => {
                crate::codegen::tables::table_names::TableName::CommercialCentrifugeLot
            }
            super::Row::CommercialCentrifugeModel(_) => {
                crate::codegen::tables::table_names::TableName::CommercialCentrifugeModel
            }
            super::Row::CommercialFreezeDryerLot(_) => {
                crate::codegen::tables::table_names::TableName::CommercialFreezeDryerLot
            }
            super::Row::CommercialFreezeDryerModel(_) => {
                crate::codegen::tables::table_names::TableName::CommercialFreezeDryerModel
            }
            super::Row::CommercialFreezerLot(_) => {
                crate::codegen::tables::table_names::TableName::CommercialFreezerLot
            }
            super::Row::CommercialFreezerModel(_) => {
                crate::codegen::tables::table_names::TableName::CommercialFreezerModel
            }
            super::Row::CommercialPackagingLot(_) => {
                crate::codegen::tables::table_names::TableName::CommercialPackagingLot
            }
            super::Row::CommercialPackagingModel(_) => {
                crate::codegen::tables::table_names::TableName::CommercialPackagingModel
            }
            super::Row::CommercialPipetteLot(_) => {
                crate::codegen::tables::table_names::TableName::CommercialPipetteLot
            }
            super::Row::CommercialPipetteModel(_) => {
                crate::codegen::tables::table_names::TableName::CommercialPipetteModel
            }
            super::Row::CommercialPipetteTipLot(_) => {
                crate::codegen::tables::table_names::TableName::CommercialPipetteTipLot
            }
            super::Row::CommercialPipetteTipModel(_) => {
                crate::codegen::tables::table_names::TableName::CommercialPipetteTipModel
            }
            super::Row::CommercialPositioningDeviceLot(_) => {
                crate::codegen::tables::table_names::TableName::CommercialPositioningDeviceLot
            }
            super::Row::CommercialPositioningDeviceModel(_) => {
                crate::codegen::tables::table_names::TableName::CommercialPositioningDeviceModel
            }
            super::Row::CommercialProductLot(_) => {
                crate::codegen::tables::table_names::TableName::CommercialProductLot
            }
            super::Row::CommercialProduct(_) => {
                crate::codegen::tables::table_names::TableName::CommercialProduct
            }
            super::Row::CommercialVolumeMeasuringDeviceLot(_) => {
                crate::codegen::tables::table_names::TableName::CommercialVolumeMeasuringDeviceLot
            }
            super::Row::CommercialVolumeMeasuringDeviceModel(_) => {
                crate::codegen::tables::table_names::TableName::CommercialVolumeMeasuringDeviceModel
            }
            super::Row::CommercialWeighingDeviceLot(_) => {
                crate::codegen::tables::table_names::TableName::CommercialWeighingDeviceLot
            }
            super::Row::CommercialWeighingDeviceModel(_) => {
                crate::codegen::tables::table_names::TableName::CommercialWeighingDeviceModel
            }
            super::Row::ContainerCompatibilityRule(_) => {
                crate::codegen::tables::table_names::TableName::ContainerCompatibilityRule
            }
            super::Row::ContainerModel(_) => {
                crate::codegen::tables::table_names::TableName::ContainerModel
            }
            super::Row::Container(_) => crate::codegen::tables::table_names::TableName::Container,
            super::Row::Country(_) => crate::codegen::tables::table_names::TableName::Country,
            super::Row::DigitalAssetModel(_) => {
                crate::codegen::tables::table_names::TableName::DigitalAssetModel
            }
            super::Row::DigitalAsset(_) => {
                crate::codegen::tables::table_names::TableName::DigitalAsset
            }
            super::Row::DisposalProcedureTemplate(_) => {
                crate::codegen::tables::table_names::TableName::DisposalProcedureTemplate
            }
            super::Row::DisposalProcedure(_) => {
                crate::codegen::tables::table_names::TableName::DisposalProcedure
            }
            super::Row::EmailProvider(_) => {
                crate::codegen::tables::table_names::TableName::EmailProvider
            }
            super::Row::FractioningProcedureTemplate(_) => {
                crate::codegen::tables::table_names::TableName::FractioningProcedureTemplate
            }
            super::Row::FractioningProcedure(_) => {
                crate::codegen::tables::table_names::TableName::FractioningProcedure
            }
            super::Row::FreezeDryerModel(_) => {
                crate::codegen::tables::table_names::TableName::FreezeDryerModel
            }
            super::Row::FreezeDryer(_) => {
                crate::codegen::tables::table_names::TableName::FreezeDryer
            }
            super::Row::FreezeDryingProcedureTemplate(_) => {
                crate::codegen::tables::table_names::TableName::FreezeDryingProcedureTemplate
            }
            super::Row::FreezeDryingProcedure(_) => {
                crate::codegen::tables::table_names::TableName::FreezeDryingProcedure
            }
            super::Row::FreezerModel(_) => {
                crate::codegen::tables::table_names::TableName::FreezerModel
            }
            super::Row::Freezer(_) => crate::codegen::tables::table_names::TableName::Freezer,
            super::Row::FreezingProcedureTemplate(_) => {
                crate::codegen::tables::table_names::TableName::FreezingProcedureTemplate
            }
            super::Row::FreezingProcedure(_) => {
                crate::codegen::tables::table_names::TableName::FreezingProcedure
            }
            super::Row::GeolocationProcedureTemplate(_) => {
                crate::codegen::tables::table_names::TableName::GeolocationProcedureTemplate
            }
            super::Row::GeolocationProcedure(_) => {
                crate::codegen::tables::table_names::TableName::GeolocationProcedure
            }
            super::Row::HarvestingProcedureTemplate(_) => {
                crate::codegen::tables::table_names::TableName::HarvestingProcedureTemplate
            }
            super::Row::HarvestingProcedure(_) => {
                crate::codegen::tables::table_names::TableName::HarvestingProcedure
            }
            super::Row::InstrumentState(_) => {
                crate::codegen::tables::table_names::TableName::InstrumentState
            }
            super::Row::LoginProvider(_) => {
                crate::codegen::tables::table_names::TableName::LoginProvider
            }
            super::Row::Material(_) => crate::codegen::tables::table_names::TableName::Material,
            super::Row::NextProcedureTemplate(_) => {
                crate::codegen::tables::table_names::TableName::NextProcedureTemplate
            }
            super::Row::ObservationSubject(_) => {
                crate::codegen::tables::table_names::TableName::ObservationSubject
            }
            super::Row::OrganismModel(_) => {
                crate::codegen::tables::table_names::TableName::OrganismModel
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
            super::Row::PackagingProcedureTemplate(_) => {
                crate::codegen::tables::table_names::TableName::PackagingProcedureTemplate
            }
            super::Row::PackagingProcedure(_) => {
                crate::codegen::tables::table_names::TableName::PackagingProcedure
            }
            super::Row::ParentProcedureTemplate(_) => {
                crate::codegen::tables::table_names::TableName::ParentProcedureTemplate
            }
            super::Row::PermanenceCategory(_) => {
                crate::codegen::tables::table_names::TableName::PermanenceCategory
            }
            super::Row::PersonalProtectiveEquipmentModel(_) => {
                crate::codegen::tables::table_names::TableName::PersonalProtectiveEquipmentModel
            }
            super::Row::PhoneModel(_) => crate::codegen::tables::table_names::TableName::PhoneModel,
            super::Row::PhotographProcedureTemplate(_) => {
                crate::codegen::tables::table_names::TableName::PhotographProcedureTemplate
            }
            super::Row::PhotographProcedure(_) => {
                crate::codegen::tables::table_names::TableName::PhotographProcedure
            }
            super::Row::Photograph(_) => crate::codegen::tables::table_names::TableName::Photograph,
            super::Row::PhysicalAssetModel(_) => {
                crate::codegen::tables::table_names::TableName::PhysicalAssetModel
            }
            super::Row::PhysicalAsset(_) => {
                crate::codegen::tables::table_names::TableName::PhysicalAsset
            }
            super::Row::PipetteModel(_) => {
                crate::codegen::tables::table_names::TableName::PipetteModel
            }
            super::Row::PipetteTipModel(_) => {
                crate::codegen::tables::table_names::TableName::PipetteTipModel
            }
            super::Row::Pipette(_) => crate::codegen::tables::table_names::TableName::Pipette,
            super::Row::PositioningDeviceModel(_) => {
                crate::codegen::tables::table_names::TableName::PositioningDeviceModel
            }
            super::Row::PositioningDevice(_) => {
                crate::codegen::tables::table_names::TableName::PositioningDevice
            }
            super::Row::PouringProcedureTemplate(_) => {
                crate::codegen::tables::table_names::TableName::PouringProcedureTemplate
            }
            super::Row::PouringProcedure(_) => {
                crate::codegen::tables::table_names::TableName::PouringProcedure
            }
            super::Row::PpeReminderProcedureTemplate(_) => {
                crate::codegen::tables::table_names::TableName::PpeReminderProcedureTemplate
            }
            super::Row::PpeReminderProcedure(_) => {
                crate::codegen::tables::table_names::TableName::PpeReminderProcedure
            }
            super::Row::ProcedureAsset(_) => {
                crate::codegen::tables::table_names::TableName::ProcedureAsset
            }
            super::Row::ProcedureTemplateAssetModel(_) => {
                crate::codegen::tables::table_names::TableName::ProcedureTemplateAssetModel
            }
            super::Row::ProcedureTemplate(_) => {
                crate::codegen::tables::table_names::TableName::ProcedureTemplate
            }
            super::Row::Procedure(_) => crate::codegen::tables::table_names::TableName::Procedure,
            super::Row::ProjectState(_) => {
                crate::codegen::tables::table_names::TableName::ProjectState
            }
            super::Row::Project(_) => crate::codegen::tables::table_names::TableName::Project,
            super::Row::Rank(_) => crate::codegen::tables::table_names::TableName::Rank,
            super::Row::ReagentModel(_) => {
                crate::codegen::tables::table_names::TableName::ReagentModel
            }
            super::Row::Role(_) => crate::codegen::tables::table_names::TableName::Role,
            super::Row::Room(_) => crate::codegen::tables::table_names::TableName::Room,
            super::Row::SampleModel(_) => {
                crate::codegen::tables::table_names::TableName::SampleModel
            }
            super::Row::SampleSourceModel(_) => {
                crate::codegen::tables::table_names::TableName::SampleSourceModel
            }
            super::Row::SampleSource(_) => {
                crate::codegen::tables::table_names::TableName::SampleSource
            }
            super::Row::SampleState(_) => {
                crate::codegen::tables::table_names::TableName::SampleState
            }
            super::Row::Sample(_) => crate::codegen::tables::table_names::TableName::Sample,
            super::Row::SoilModel(_) => crate::codegen::tables::table_names::TableName::SoilModel,
            super::Row::Soil(_) => crate::codegen::tables::table_names::TableName::Soil,
            super::Row::SpatialRefSy(_) => {
                crate::codegen::tables::table_names::TableName::SpatialRefSy
            }
            super::Row::Spectrum(_) => crate::codegen::tables::table_names::TableName::Spectrum,
            super::Row::SpectraCollection(_) => {
                crate::codegen::tables::table_names::TableName::SpectraCollection
            }
            super::Row::StorageProcedureTemplate(_) => {
                crate::codegen::tables::table_names::TableName::StorageProcedureTemplate
            }
            super::Row::StorageProcedure(_) => {
                crate::codegen::tables::table_names::TableName::StorageProcedure
            }
            super::Row::SupernatantProcedureTemplate(_) => {
                crate::codegen::tables::table_names::TableName::SupernatantProcedureTemplate
            }
            super::Row::SupernatantProcedure(_) => {
                crate::codegen::tables::table_names::TableName::SupernatantProcedure
            }
            super::Row::TaggingProcedureTemplate(_) => {
                crate::codegen::tables::table_names::TableName::TaggingProcedureTemplate
            }
            super::Row::TaggingProcedure(_) => {
                crate::codegen::tables::table_names::TableName::TaggingProcedure
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
            super::Row::Unit(_) => crate::codegen::tables::table_names::TableName::Unit,
            super::Row::UserEmail(_) => crate::codegen::tables::table_names::TableName::UserEmail,
            super::Row::UserOrganization(_) => {
                crate::codegen::tables::table_names::TableName::UserOrganization
            }
            super::Row::User(_) => crate::codegen::tables::table_names::TableName::User,
            super::Row::VolumeMeasuringDeviceModel(_) => {
                crate::codegen::tables::table_names::TableName::VolumeMeasuringDeviceModel
            }
            super::Row::VolumeMeasuringDevice(_) => {
                crate::codegen::tables::table_names::TableName::VolumeMeasuringDevice
            }
            super::Row::VolumetricContainerModel(_) => {
                crate::codegen::tables::table_names::TableName::VolumetricContainerModel
            }
            super::Row::VolumetricContainer(_) => {
                crate::codegen::tables::table_names::TableName::VolumetricContainer
            }
            super::Row::WeighingDeviceModel(_) => {
                crate::codegen::tables::table_names::TableName::WeighingDeviceModel
            }
            super::Row::WeighingDevice(_) => {
                crate::codegen::tables::table_names::TableName::WeighingDevice
            }
            super::Row::WeighingProcedureTemplate(_) => {
                crate::codegen::tables::table_names::TableName::WeighingProcedureTemplate
            }
            super::Row::WeighingProcedure(_) => {
                crate::codegen::tables::table_names::TableName::WeighingProcedure
            }
        }
    }
}
