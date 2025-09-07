impl<C> web_common_traits::prelude::ReadDispatch<C> for super::Row
where
    crate::Address: web_common_traits::database::Read<C>,
    crate::AliquotingProcedureTemplate: web_common_traits::database::Read<C>,
    crate::AliquotingProcedure: web_common_traits::database::Read<C>,
    crate::AssetCompatibilityRule: web_common_traits::database::Read<C>,
    crate::AssetModelAncestor: web_common_traits::database::Read<C>,
    crate::AssetModel: web_common_traits::database::Read<C>,
    crate::Asset: web_common_traits::database::Read<C>,
    crate::BallMillMachineModel: web_common_traits::database::Read<C>,
    crate::BallMillMachine: web_common_traits::database::Read<C>,
    crate::BallMillProcedureTemplate: web_common_traits::database::Read<C>,
    crate::BallMillProcedure: web_common_traits::database::Read<C>,
    crate::BeadModel: web_common_traits::database::Read<C>,
    crate::Brand: web_common_traits::database::Read<C>,
    crate::CameraModel: web_common_traits::database::Read<C>,
    crate::Camera: web_common_traits::database::Read<C>,
    crate::CapModel: web_common_traits::database::Read<C>,
    crate::CappingProcedureTemplate: web_common_traits::database::Read<C>,
    crate::CappingProcedure: web_common_traits::database::Read<C>,
    crate::CentrifugeModel: web_common_traits::database::Read<C>,
    crate::CentrifugeProcedureTemplate: web_common_traits::database::Read<C>,
    crate::CentrifugeProcedure: web_common_traits::database::Read<C>,
    crate::Centrifuge: web_common_traits::database::Read<C>,
    crate::City: web_common_traits::database::Read<C>,
    crate::Color: web_common_traits::database::Read<C>,
    crate::CommercialBallMillMachineLot: web_common_traits::database::Read<C>,
    crate::CommercialBallMillMachineModel: web_common_traits::database::Read<C>,
    crate::CommercialBeadLot: web_common_traits::database::Read<C>,
    crate::CommercialBeadModel: web_common_traits::database::Read<C>,
    crate::CommercialCameraLot: web_common_traits::database::Read<C>,
    crate::CommercialCameraModel: web_common_traits::database::Read<C>,
    crate::CommercialCapLot: web_common_traits::database::Read<C>,
    crate::CommercialCapModel: web_common_traits::database::Read<C>,
    crate::CommercialCentrifugeLot: web_common_traits::database::Read<C>,
    crate::CommercialCentrifugeModel: web_common_traits::database::Read<C>,
    crate::CommercialFreezeDryerLot: web_common_traits::database::Read<C>,
    crate::CommercialFreezeDryerModel: web_common_traits::database::Read<C>,
    crate::CommercialFreezerLot: web_common_traits::database::Read<C>,
    crate::CommercialFreezerModel: web_common_traits::database::Read<C>,
    crate::CommercialPackagingLot: web_common_traits::database::Read<C>,
    crate::CommercialPackagingModel: web_common_traits::database::Read<C>,
    crate::CommercialPipetteLot: web_common_traits::database::Read<C>,
    crate::CommercialPipetteModel: web_common_traits::database::Read<C>,
    crate::CommercialPipetteTipLot: web_common_traits::database::Read<C>,
    crate::CommercialPipetteTipModel: web_common_traits::database::Read<C>,
    crate::CommercialPositioningDeviceLot: web_common_traits::database::Read<C>,
    crate::CommercialPositioningDeviceModel: web_common_traits::database::Read<C>,
    crate::CommercialProductLot: web_common_traits::database::Read<C>,
    crate::CommercialProduct: web_common_traits::database::Read<C>,
    crate::CommercialVolumeMeasuringDeviceLot: web_common_traits::database::Read<C>,
    crate::CommercialVolumeMeasuringDeviceModel: web_common_traits::database::Read<C>,
    crate::CommercialWeighingDeviceLot: web_common_traits::database::Read<C>,
    crate::CommercialWeighingDeviceModel: web_common_traits::database::Read<C>,
    crate::ContainerCompatibilityRule: web_common_traits::database::Read<C>,
    crate::ContainerModel: web_common_traits::database::Read<C>,
    crate::Container: web_common_traits::database::Read<C>,
    crate::Country: web_common_traits::database::Read<C>,
    crate::DigitalAssetModel: web_common_traits::database::Read<C>,
    crate::DigitalAsset: web_common_traits::database::Read<C>,
    crate::DisposalProcedureTemplate: web_common_traits::database::Read<C>,
    crate::DisposalProcedure: web_common_traits::database::Read<C>,
    crate::Document: web_common_traits::database::Read<C>,
    crate::EmailProvider: web_common_traits::database::Read<C>,
    crate::FractioningProcedureTemplate: web_common_traits::database::Read<C>,
    crate::FractioningProcedure: web_common_traits::database::Read<C>,
    crate::FreezeDryerModel: web_common_traits::database::Read<C>,
    crate::FreezeDryer: web_common_traits::database::Read<C>,
    crate::FreezeDryingProcedureTemplate: web_common_traits::database::Read<C>,
    crate::FreezeDryingProcedure: web_common_traits::database::Read<C>,
    crate::FreezerModel: web_common_traits::database::Read<C>,
    crate::Freezer: web_common_traits::database::Read<C>,
    crate::FreezingProcedureTemplate: web_common_traits::database::Read<C>,
    crate::FreezingProcedure: web_common_traits::database::Read<C>,
    crate::GeolocationProcedureTemplate: web_common_traits::database::Read<C>,
    crate::GeolocationProcedure: web_common_traits::database::Read<C>,
    crate::InstrumentState: web_common_traits::database::Read<C>,
    crate::LoginProvider: web_common_traits::database::Read<C>,
    crate::Material: web_common_traits::database::Read<C>,
    crate::NextProcedureTemplate: web_common_traits::database::Read<C>,
    crate::ObservationSubject: web_common_traits::database::Read<C>,
    crate::OrganismTaxon: web_common_traits::database::Read<C>,
    crate::Organism: web_common_traits::database::Read<C>,
    crate::Organization: web_common_traits::database::Read<C>,
    crate::PackagingModel: web_common_traits::database::Read<C>,
    crate::PackagingProcedureTemplate: web_common_traits::database::Read<C>,
    crate::PackagingProcedure: web_common_traits::database::Read<C>,
    crate::ParentProcedureTemplate: web_common_traits::database::Read<C>,
    crate::PermanenceCategory: web_common_traits::database::Read<C>,
    crate::PhoneModel: web_common_traits::database::Read<C>,
    crate::PhotographProcedureTemplate: web_common_traits::database::Read<C>,
    crate::PhotographProcedure: web_common_traits::database::Read<C>,
    crate::PhysicalAssetModel: web_common_traits::database::Read<C>,
    crate::PhysicalAsset: web_common_traits::database::Read<C>,
    crate::PipetteModel: web_common_traits::database::Read<C>,
    crate::PipetteTipModel: web_common_traits::database::Read<C>,
    crate::Pipette: web_common_traits::database::Read<C>,
    crate::PositioningDeviceModel: web_common_traits::database::Read<C>,
    crate::PositioningDevice: web_common_traits::database::Read<C>,
    crate::PouringProcedureTemplate: web_common_traits::database::Read<C>,
    crate::PouringProcedure: web_common_traits::database::Read<C>,
    crate::ProcedureAsset: web_common_traits::database::Read<C>,
    crate::ProcedureTemplateAssetModel: web_common_traits::database::Read<C>,
    crate::ProcedureTemplate: web_common_traits::database::Read<C>,
    crate::Procedure: web_common_traits::database::Read<C>,
    crate::ProjectState: web_common_traits::database::Read<C>,
    crate::Project: web_common_traits::database::Read<C>,
    crate::Rank: web_common_traits::database::Read<C>,
    crate::ReagentModel: web_common_traits::database::Read<C>,
    crate::Role: web_common_traits::database::Read<C>,
    crate::Room: web_common_traits::database::Read<C>,
    crate::SampleState: web_common_traits::database::Read<C>,
    crate::SpatialRefSy: web_common_traits::database::Read<C>,
    crate::Spectrum: web_common_traits::database::Read<C>,
    crate::SpectraCollection: web_common_traits::database::Read<C>,
    crate::StorageProcedureTemplate: web_common_traits::database::Read<C>,
    crate::StorageProcedure: web_common_traits::database::Read<C>,
    crate::SupernatantProcedureTemplate: web_common_traits::database::Read<C>,
    crate::SupernatantProcedure: web_common_traits::database::Read<C>,
    crate::Taxon: web_common_traits::database::Read<C>,
    crate::TeamMember: web_common_traits::database::Read<C>,
    crate::TeamProject: web_common_traits::database::Read<C>,
    crate::TeamState: web_common_traits::database::Read<C>,
    crate::Team: web_common_traits::database::Read<C>,
    crate::TemporaryUser: web_common_traits::database::Read<C>,
    crate::Unit: web_common_traits::database::Read<C>,
    crate::UserEmail: web_common_traits::database::Read<C>,
    crate::UserOrganization: web_common_traits::database::Read<C>,
    crate::User: web_common_traits::database::Read<C>,
    crate::VolumeMeasuringDeviceModel: web_common_traits::database::Read<C>,
    crate::VolumeMeasuringDevice: web_common_traits::database::Read<C>,
    crate::VolumetricContainerModel: web_common_traits::database::Read<C>,
    crate::VolumetricContainer: web_common_traits::database::Read<C>,
    crate::WeighingDeviceModel: web_common_traits::database::Read<C>,
    crate::WeighingDevice: web_common_traits::database::Read<C>,
    crate::WeighingProcedureTemplate: web_common_traits::database::Read<C>,
    crate::WeighingProcedure: web_common_traits::database::Read<C>,
{
    type PrimaryKey = crate::codegen::tables::table_primary_keys::TablePrimaryKey;
    fn read(primary_key: Self::PrimaryKey, conn: &mut C) -> Result<Self, diesel::result::Error> {
        use web_common_traits::database::Read;
        Ok(
            match primary_key {
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Address(
                    primary_key,
                ) => crate::Address::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::AliquotingProcedureTemplate(
                    primary_key,
                ) => crate::AliquotingProcedureTemplate::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::AliquotingProcedure(
                    primary_key,
                ) => crate::AliquotingProcedure::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::AssetCompatibilityRule(
                    primary_key,
                ) => crate::AssetCompatibilityRule::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::AssetModelAncestor(
                    primary_key,
                ) => crate::AssetModelAncestor::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::AssetModel(
                    primary_key,
                ) => crate::AssetModel::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Asset(
                    primary_key,
                ) => crate::Asset::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::BallMillMachineModel(
                    primary_key,
                ) => crate::BallMillMachineModel::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::BallMillMachine(
                    primary_key,
                ) => crate::BallMillMachine::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::BallMillProcedureTemplate(
                    primary_key,
                ) => crate::BallMillProcedureTemplate::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::BallMillProcedure(
                    primary_key,
                ) => crate::BallMillProcedure::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::BeadModel(
                    primary_key,
                ) => crate::BeadModel::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Brand(
                    primary_key,
                ) => crate::Brand::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CameraModel(
                    primary_key,
                ) => crate::CameraModel::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Camera(
                    primary_key,
                ) => crate::Camera::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CapModel(
                    primary_key,
                ) => crate::CapModel::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CappingProcedureTemplate(
                    primary_key,
                ) => crate::CappingProcedureTemplate::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CappingProcedure(
                    primary_key,
                ) => crate::CappingProcedure::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CentrifugeModel(
                    primary_key,
                ) => crate::CentrifugeModel::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CentrifugeProcedureTemplate(
                    primary_key,
                ) => crate::CentrifugeProcedureTemplate::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CentrifugeProcedure(
                    primary_key,
                ) => crate::CentrifugeProcedure::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Centrifuge(
                    primary_key,
                ) => crate::Centrifuge::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::City(
                    primary_key,
                ) => crate::City::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Color(
                    primary_key,
                ) => crate::Color::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialBallMillMachineLot(
                    primary_key,
                ) => crate::CommercialBallMillMachineLot::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialBallMillMachineModel(
                    primary_key,
                ) => {
                    crate::CommercialBallMillMachineModel::read(primary_key, conn)?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialBeadLot(
                    primary_key,
                ) => crate::CommercialBeadLot::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialBeadModel(
                    primary_key,
                ) => crate::CommercialBeadModel::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialCameraLot(
                    primary_key,
                ) => crate::CommercialCameraLot::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialCameraModel(
                    primary_key,
                ) => crate::CommercialCameraModel::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialCapLot(
                    primary_key,
                ) => crate::CommercialCapLot::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialCapModel(
                    primary_key,
                ) => crate::CommercialCapModel::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialCentrifugeLot(
                    primary_key,
                ) => crate::CommercialCentrifugeLot::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialCentrifugeModel(
                    primary_key,
                ) => crate::CommercialCentrifugeModel::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialFreezeDryerLot(
                    primary_key,
                ) => crate::CommercialFreezeDryerLot::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialFreezeDryerModel(
                    primary_key,
                ) => crate::CommercialFreezeDryerModel::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialFreezerLot(
                    primary_key,
                ) => crate::CommercialFreezerLot::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialFreezerModel(
                    primary_key,
                ) => crate::CommercialFreezerModel::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialPackagingLot(
                    primary_key,
                ) => crate::CommercialPackagingLot::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialPackagingModel(
                    primary_key,
                ) => crate::CommercialPackagingModel::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialPipetteLot(
                    primary_key,
                ) => crate::CommercialPipetteLot::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialPipetteModel(
                    primary_key,
                ) => crate::CommercialPipetteModel::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialPipetteTipLot(
                    primary_key,
                ) => crate::CommercialPipetteTipLot::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialPipetteTipModel(
                    primary_key,
                ) => crate::CommercialPipetteTipModel::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialPositioningDeviceLot(
                    primary_key,
                ) => {
                    crate::CommercialPositioningDeviceLot::read(primary_key, conn)?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialPositioningDeviceModel(
                    primary_key,
                ) => {
                    crate::CommercialPositioningDeviceModel::read(primary_key, conn)?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialProductLot(
                    primary_key,
                ) => crate::CommercialProductLot::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialProduct(
                    primary_key,
                ) => crate::CommercialProduct::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialVolumeMeasuringDeviceLot(
                    primary_key,
                ) => {
                    crate::CommercialVolumeMeasuringDeviceLot::read(primary_key, conn)?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialVolumeMeasuringDeviceModel(
                    primary_key,
                ) => {
                    crate::CommercialVolumeMeasuringDeviceModel::read(primary_key, conn)?
                        .into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialWeighingDeviceLot(
                    primary_key,
                ) => crate::CommercialWeighingDeviceLot::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::CommercialWeighingDeviceModel(
                    primary_key,
                ) => {
                    crate::CommercialWeighingDeviceModel::read(primary_key, conn)?.into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::ContainerCompatibilityRule(
                    primary_key,
                ) => crate::ContainerCompatibilityRule::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::ContainerModel(
                    primary_key,
                ) => crate::ContainerModel::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Container(
                    primary_key,
                ) => crate::Container::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Country(
                    primary_key,
                ) => crate::Country::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::DigitalAssetModel(
                    primary_key,
                ) => crate::DigitalAssetModel::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::DigitalAsset(
                    primary_key,
                ) => crate::DigitalAsset::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::DisposalProcedureTemplate(
                    primary_key,
                ) => crate::DisposalProcedureTemplate::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::DisposalProcedure(
                    primary_key,
                ) => crate::DisposalProcedure::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Document(
                    primary_key,
                ) => crate::Document::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::EmailProvider(
                    primary_key,
                ) => crate::EmailProvider::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::FractioningProcedureTemplate(
                    primary_key,
                ) => crate::FractioningProcedureTemplate::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::FractioningProcedure(
                    primary_key,
                ) => crate::FractioningProcedure::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::FreezeDryerModel(
                    primary_key,
                ) => crate::FreezeDryerModel::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::FreezeDryer(
                    primary_key,
                ) => crate::FreezeDryer::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::FreezeDryingProcedureTemplate(
                    primary_key,
                ) => {
                    crate::FreezeDryingProcedureTemplate::read(primary_key, conn)?.into()
                }
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::FreezeDryingProcedure(
                    primary_key,
                ) => crate::FreezeDryingProcedure::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::FreezerModel(
                    primary_key,
                ) => crate::FreezerModel::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Freezer(
                    primary_key,
                ) => crate::Freezer::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::FreezingProcedureTemplate(
                    primary_key,
                ) => crate::FreezingProcedureTemplate::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::FreezingProcedure(
                    primary_key,
                ) => crate::FreezingProcedure::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::GeolocationProcedureTemplate(
                    primary_key,
                ) => crate::GeolocationProcedureTemplate::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::GeolocationProcedure(
                    primary_key,
                ) => crate::GeolocationProcedure::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::InstrumentState(
                    primary_key,
                ) => crate::InstrumentState::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::LoginProvider(
                    primary_key,
                ) => crate::LoginProvider::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Material(
                    primary_key,
                ) => crate::Material::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::NextProcedureTemplate(
                    primary_key,
                ) => crate::NextProcedureTemplate::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::ObservationSubject(
                    primary_key,
                ) => crate::ObservationSubject::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::OrganismTaxon(
                    primary_key,
                ) => crate::OrganismTaxon::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Organism(
                    primary_key,
                ) => crate::Organism::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Organization(
                    primary_key,
                ) => crate::Organization::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::PackagingModel(
                    primary_key,
                ) => crate::PackagingModel::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::PackagingProcedureTemplate(
                    primary_key,
                ) => crate::PackagingProcedureTemplate::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::PackagingProcedure(
                    primary_key,
                ) => crate::PackagingProcedure::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::ParentProcedureTemplate(
                    primary_key,
                ) => crate::ParentProcedureTemplate::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::PermanenceCategory(
                    primary_key,
                ) => crate::PermanenceCategory::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::PhoneModel(
                    primary_key,
                ) => crate::PhoneModel::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::PhotographProcedureTemplate(
                    primary_key,
                ) => crate::PhotographProcedureTemplate::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::PhotographProcedure(
                    primary_key,
                ) => crate::PhotographProcedure::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::PhysicalAssetModel(
                    primary_key,
                ) => crate::PhysicalAssetModel::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::PhysicalAsset(
                    primary_key,
                ) => crate::PhysicalAsset::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::PipetteModel(
                    primary_key,
                ) => crate::PipetteModel::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::PipetteTipModel(
                    primary_key,
                ) => crate::PipetteTipModel::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Pipette(
                    primary_key,
                ) => crate::Pipette::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::PositioningDeviceModel(
                    primary_key,
                ) => crate::PositioningDeviceModel::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::PositioningDevice(
                    primary_key,
                ) => crate::PositioningDevice::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::PouringProcedureTemplate(
                    primary_key,
                ) => crate::PouringProcedureTemplate::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::PouringProcedure(
                    primary_key,
                ) => crate::PouringProcedure::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureAsset(
                    primary_key,
                ) => crate::ProcedureAsset::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplateAssetModel(
                    primary_key,
                ) => crate::ProcedureTemplateAssetModel::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplate(
                    primary_key,
                ) => crate::ProcedureTemplate::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Procedure(
                    primary_key,
                ) => crate::Procedure::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProjectState(
                    primary_key,
                ) => crate::ProjectState::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Project(
                    primary_key,
                ) => crate::Project::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Rank(
                    primary_key,
                ) => crate::Rank::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::ReagentModel(
                    primary_key,
                ) => crate::ReagentModel::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Role(
                    primary_key,
                ) => crate::Role::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Room(
                    primary_key,
                ) => crate::Room::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::SampleState(
                    primary_key,
                ) => crate::SampleState::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::SpatialRefSy(
                    primary_key,
                ) => crate::SpatialRefSy::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Spectrum(
                    primary_key,
                ) => crate::Spectrum::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::SpectraCollection(
                    primary_key,
                ) => crate::SpectraCollection::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::StorageProcedureTemplate(
                    primary_key,
                ) => crate::StorageProcedureTemplate::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::StorageProcedure(
                    primary_key,
                ) => crate::StorageProcedure::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::SupernatantProcedureTemplate(
                    primary_key,
                ) => crate::SupernatantProcedureTemplate::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::SupernatantProcedure(
                    primary_key,
                ) => crate::SupernatantProcedure::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Taxon(
                    primary_key,
                ) => crate::Taxon::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::TeamMember(
                    primary_key,
                ) => crate::TeamMember::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::TeamProject(
                    primary_key,
                ) => crate::TeamProject::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::TeamState(
                    primary_key,
                ) => crate::TeamState::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Team(
                    primary_key,
                ) => crate::Team::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::TemporaryUser(
                    primary_key,
                ) => crate::TemporaryUser::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Unit(
                    primary_key,
                ) => crate::Unit::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::UserEmail(
                    primary_key,
                ) => crate::UserEmail::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::UserOrganization(
                    primary_key,
                ) => crate::UserOrganization::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::User(
                    primary_key,
                ) => crate::User::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::VolumeMeasuringDeviceModel(
                    primary_key,
                ) => crate::VolumeMeasuringDeviceModel::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::VolumeMeasuringDevice(
                    primary_key,
                ) => crate::VolumeMeasuringDevice::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::VolumetricContainerModel(
                    primary_key,
                ) => crate::VolumetricContainerModel::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::VolumetricContainer(
                    primary_key,
                ) => crate::VolumetricContainer::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::WeighingDeviceModel(
                    primary_key,
                ) => crate::WeighingDeviceModel::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::WeighingDevice(
                    primary_key,
                ) => crate::WeighingDevice::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::WeighingProcedureTemplate(
                    primary_key,
                ) => crate::WeighingProcedureTemplate::read(primary_key, conn)?.into(),
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::WeighingProcedure(
                    primary_key,
                ) => crate::WeighingProcedure::read(primary_key, conn)?.into(),
            },
        )
    }
}
