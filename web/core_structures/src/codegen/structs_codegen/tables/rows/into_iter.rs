impl From<super::Rows> for Vec<crate::codegen::tables::row::Row> {
    fn from(rows: super::Rows) -> Self {
        match rows {
            super::Rows::Address(rows) => {
                rows.into_iter().map(crate::codegen::tables::row::Row::Address).collect::<Vec<_>>()
            }
            super::Rows::AliquotingProcedureTemplate(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::AliquotingProcedureTemplate)
                    .collect::<Vec<_>>()
            }
            super::Rows::AliquotingProcedure(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::AliquotingProcedure)
                    .collect::<Vec<_>>()
            }
            super::Rows::AssetCompatibilityRule(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::AssetCompatibilityRule)
                    .collect::<Vec<_>>()
            }
            super::Rows::AssetModelAncestor(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::AssetModelAncestor)
                    .collect::<Vec<_>>()
            }
            super::Rows::AssetModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::AssetModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::Asset(rows) => {
                rows.into_iter().map(crate::codegen::tables::row::Row::Asset).collect::<Vec<_>>()
            }
            super::Rows::BallMillMachineModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::BallMillMachineModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::BallMillMachine(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::BallMillMachine)
                    .collect::<Vec<_>>()
            }
            super::Rows::BallMillProcedureTemplate(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::BallMillProcedureTemplate)
                    .collect::<Vec<_>>()
            }
            super::Rows::BallMillProcedure(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::BallMillProcedure)
                    .collect::<Vec<_>>()
            }
            super::Rows::BeadModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::BeadModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::Brand(rows) => {
                rows.into_iter().map(crate::codegen::tables::row::Row::Brand).collect::<Vec<_>>()
            }
            super::Rows::CameraModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::CameraModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::Camera(rows) => {
                rows.into_iter().map(crate::codegen::tables::row::Row::Camera).collect::<Vec<_>>()
            }
            super::Rows::CapModel(rows) => {
                rows.into_iter().map(crate::codegen::tables::row::Row::CapModel).collect::<Vec<_>>()
            }
            super::Rows::CappingProcedureTemplate(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::CappingProcedureTemplate)
                    .collect::<Vec<_>>()
            }
            super::Rows::CappingProcedure(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::CappingProcedure)
                    .collect::<Vec<_>>()
            }
            super::Rows::CentrifugeModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::CentrifugeModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::CentrifugeProcedureTemplate(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::CentrifugeProcedureTemplate)
                    .collect::<Vec<_>>()
            }
            super::Rows::CentrifugeProcedure(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::CentrifugeProcedure)
                    .collect::<Vec<_>>()
            }
            super::Rows::Centrifuge(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::Centrifuge)
                    .collect::<Vec<_>>()
            }
            super::Rows::City(rows) => {
                rows.into_iter().map(crate::codegen::tables::row::Row::City).collect::<Vec<_>>()
            }
            super::Rows::Color(rows) => {
                rows.into_iter().map(crate::codegen::tables::row::Row::Color).collect::<Vec<_>>()
            }
            super::Rows::CommercialBallMillMachineLot(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::CommercialBallMillMachineLot)
                    .collect::<Vec<_>>()
            }
            super::Rows::CommercialBallMillMachineModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::CommercialBallMillMachineModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::CommercialBeadLot(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::CommercialBeadLot)
                    .collect::<Vec<_>>()
            }
            super::Rows::CommercialBeadModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::CommercialBeadModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::CommercialCameraLot(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::CommercialCameraLot)
                    .collect::<Vec<_>>()
            }
            super::Rows::CommercialCameraModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::CommercialCameraModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::CommercialCapLot(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::CommercialCapLot)
                    .collect::<Vec<_>>()
            }
            super::Rows::CommercialCapModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::CommercialCapModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::CommercialCentrifugeLot(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::CommercialCentrifugeLot)
                    .collect::<Vec<_>>()
            }
            super::Rows::CommercialCentrifugeModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::CommercialCentrifugeModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::CommercialFreezeDryerLot(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::CommercialFreezeDryerLot)
                    .collect::<Vec<_>>()
            }
            super::Rows::CommercialFreezeDryerModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::CommercialFreezeDryerModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::CommercialFreezerLot(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::CommercialFreezerLot)
                    .collect::<Vec<_>>()
            }
            super::Rows::CommercialFreezerModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::CommercialFreezerModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::CommercialPackagingLot(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::CommercialPackagingLot)
                    .collect::<Vec<_>>()
            }
            super::Rows::CommercialPackagingModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::CommercialPackagingModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::CommercialPipetteLot(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::CommercialPipetteLot)
                    .collect::<Vec<_>>()
            }
            super::Rows::CommercialPipetteModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::CommercialPipetteModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::CommercialPipetteTipLot(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::CommercialPipetteTipLot)
                    .collect::<Vec<_>>()
            }
            super::Rows::CommercialPipetteTipModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::CommercialPipetteTipModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::CommercialPositioningDeviceLot(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::CommercialPositioningDeviceLot)
                    .collect::<Vec<_>>()
            }
            super::Rows::CommercialPositioningDeviceModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::CommercialPositioningDeviceModel)
                    .collect::<Vec<_>>()
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
            super::Rows::CommercialVolumeMeasuringDeviceLot(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::CommercialVolumeMeasuringDeviceLot)
                    .collect::<Vec<_>>()
            }
            super::Rows::CommercialVolumeMeasuringDeviceModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::CommercialVolumeMeasuringDeviceModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::CommercialWeighingDeviceLot(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::CommercialWeighingDeviceLot)
                    .collect::<Vec<_>>()
            }
            super::Rows::CommercialWeighingDeviceModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::CommercialWeighingDeviceModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::ContainerCompatibilityRule(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::ContainerCompatibilityRule)
                    .collect::<Vec<_>>()
            }
            super::Rows::ContainerModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::ContainerModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::Container(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::Container)
                    .collect::<Vec<_>>()
            }
            super::Rows::Country(rows) => {
                rows.into_iter().map(crate::codegen::tables::row::Row::Country).collect::<Vec<_>>()
            }
            super::Rows::DigitalAssetModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::DigitalAssetModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::DigitalAsset(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::DigitalAsset)
                    .collect::<Vec<_>>()
            }
            super::Rows::DisposalProcedureTemplate(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::DisposalProcedureTemplate)
                    .collect::<Vec<_>>()
            }
            super::Rows::DisposalProcedure(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::DisposalProcedure)
                    .collect::<Vec<_>>()
            }
            super::Rows::EmailProvider(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::EmailProvider)
                    .collect::<Vec<_>>()
            }
            super::Rows::FractioningProcedureTemplate(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::FractioningProcedureTemplate)
                    .collect::<Vec<_>>()
            }
            super::Rows::FractioningProcedure(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::FractioningProcedure)
                    .collect::<Vec<_>>()
            }
            super::Rows::FreezeDryerModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::FreezeDryerModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::FreezeDryer(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::FreezeDryer)
                    .collect::<Vec<_>>()
            }
            super::Rows::FreezeDryingProcedureTemplate(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::FreezeDryingProcedureTemplate)
                    .collect::<Vec<_>>()
            }
            super::Rows::FreezeDryingProcedure(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::FreezeDryingProcedure)
                    .collect::<Vec<_>>()
            }
            super::Rows::FreezerModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::FreezerModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::Freezer(rows) => {
                rows.into_iter().map(crate::codegen::tables::row::Row::Freezer).collect::<Vec<_>>()
            }
            super::Rows::FreezingProcedureTemplate(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::FreezingProcedureTemplate)
                    .collect::<Vec<_>>()
            }
            super::Rows::FreezingProcedure(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::FreezingProcedure)
                    .collect::<Vec<_>>()
            }
            super::Rows::GeolocationProcedureTemplate(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::GeolocationProcedureTemplate)
                    .collect::<Vec<_>>()
            }
            super::Rows::GeolocationProcedure(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::GeolocationProcedure)
                    .collect::<Vec<_>>()
            }
            super::Rows::InstrumentState(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::InstrumentState)
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
            super::Rows::NextProcedureTemplate(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::NextProcedureTemplate)
                    .collect::<Vec<_>>()
            }
            super::Rows::ObservationSubject(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::ObservationSubject)
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
            super::Rows::PackagingProcedureTemplate(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::PackagingProcedureTemplate)
                    .collect::<Vec<_>>()
            }
            super::Rows::PackagingProcedure(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::PackagingProcedure)
                    .collect::<Vec<_>>()
            }
            super::Rows::ParentProcedureTemplate(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::ParentProcedureTemplate)
                    .collect::<Vec<_>>()
            }
            super::Rows::PermanenceCategory(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::PermanenceCategory)
                    .collect::<Vec<_>>()
            }
            super::Rows::PhoneModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::PhoneModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::PhotographProcedureTemplate(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::PhotographProcedureTemplate)
                    .collect::<Vec<_>>()
            }
            super::Rows::PhotographProcedure(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::PhotographProcedure)
                    .collect::<Vec<_>>()
            }
            super::Rows::Photograph(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::Photograph)
                    .collect::<Vec<_>>()
            }
            super::Rows::PhysicalAssetModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::PhysicalAssetModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::PhysicalAsset(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::PhysicalAsset)
                    .collect::<Vec<_>>()
            }
            super::Rows::PipetteModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::PipetteModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::PipetteTipModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::PipetteTipModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::Pipette(rows) => {
                rows.into_iter().map(crate::codegen::tables::row::Row::Pipette).collect::<Vec<_>>()
            }
            super::Rows::PositioningDeviceModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::PositioningDeviceModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::PositioningDevice(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::PositioningDevice)
                    .collect::<Vec<_>>()
            }
            super::Rows::PouringProcedureTemplate(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::PouringProcedureTemplate)
                    .collect::<Vec<_>>()
            }
            super::Rows::PouringProcedure(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::PouringProcedure)
                    .collect::<Vec<_>>()
            }
            super::Rows::ProcedureAsset(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::ProcedureAsset)
                    .collect::<Vec<_>>()
            }
            super::Rows::ProcedureTemplateAssetModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::ProcedureTemplateAssetModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::ProcedureTemplate(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::ProcedureTemplate)
                    .collect::<Vec<_>>()
            }
            super::Rows::Procedure(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::Procedure)
                    .collect::<Vec<_>>()
            }
            super::Rows::ProjectState(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::ProjectState)
                    .collect::<Vec<_>>()
            }
            super::Rows::Project(rows) => {
                rows.into_iter().map(crate::codegen::tables::row::Row::Project).collect::<Vec<_>>()
            }
            super::Rows::Rank(rows) => {
                rows.into_iter().map(crate::codegen::tables::row::Row::Rank).collect::<Vec<_>>()
            }
            super::Rows::ReagentModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::ReagentModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::Role(rows) => {
                rows.into_iter().map(crate::codegen::tables::row::Row::Role).collect::<Vec<_>>()
            }
            super::Rows::Room(rows) => {
                rows.into_iter().map(crate::codegen::tables::row::Row::Room).collect::<Vec<_>>()
            }
            super::Rows::SampleModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::SampleModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::SampleSourceModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::SampleSourceModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::SampleSource(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::SampleSource)
                    .collect::<Vec<_>>()
            }
            super::Rows::SampleState(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::SampleState)
                    .collect::<Vec<_>>()
            }
            super::Rows::Sample(rows) => {
                rows.into_iter().map(crate::codegen::tables::row::Row::Sample).collect::<Vec<_>>()
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
            super::Rows::StorageProcedureTemplate(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::StorageProcedureTemplate)
                    .collect::<Vec<_>>()
            }
            super::Rows::StorageProcedure(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::StorageProcedure)
                    .collect::<Vec<_>>()
            }
            super::Rows::SupernatantProcedureTemplate(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::SupernatantProcedureTemplate)
                    .collect::<Vec<_>>()
            }
            super::Rows::SupernatantProcedure(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::SupernatantProcedure)
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
            super::Rows::TemporaryUser(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::TemporaryUser)
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
            super::Rows::VolumeMeasuringDeviceModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::VolumeMeasuringDeviceModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::VolumeMeasuringDevice(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::VolumeMeasuringDevice)
                    .collect::<Vec<_>>()
            }
            super::Rows::VolumetricContainerModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::VolumetricContainerModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::VolumetricContainer(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::VolumetricContainer)
                    .collect::<Vec<_>>()
            }
            super::Rows::WeighingDeviceModel(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::WeighingDeviceModel)
                    .collect::<Vec<_>>()
            }
            super::Rows::WeighingDevice(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::WeighingDevice)
                    .collect::<Vec<_>>()
            }
            super::Rows::WeighingProcedureTemplate(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::WeighingProcedureTemplate)
                    .collect::<Vec<_>>()
            }
            super::Rows::WeighingProcedure(rows) => {
                rows.into_iter()
                    .map(crate::codegen::tables::row::Row::WeighingProcedure)
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
