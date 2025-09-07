impl<C> web_common_traits::prelude::BoundedReadDispatch<C> for super::Rows
where
    crate::Address: web_common_traits::prelude::BoundedRead<C>,
    crate::AliquotingProcedureTemplate: web_common_traits::prelude::BoundedRead<C>,
    crate::AliquotingProcedure: web_common_traits::prelude::BoundedRead<C>,
    crate::AssetCompatibilityRule: web_common_traits::prelude::BoundedRead<C>,
    crate::AssetModelAncestor: web_common_traits::prelude::BoundedRead<C>,
    crate::AssetModel: web_common_traits::prelude::BoundedRead<C>,
    crate::Asset: web_common_traits::prelude::BoundedRead<C>,
    crate::BallMillMachineModel: web_common_traits::prelude::BoundedRead<C>,
    crate::BallMillMachine: web_common_traits::prelude::BoundedRead<C>,
    crate::BallMillProcedureTemplate: web_common_traits::prelude::BoundedRead<C>,
    crate::BallMillProcedure: web_common_traits::prelude::BoundedRead<C>,
    crate::BeadModel: web_common_traits::prelude::BoundedRead<C>,
    crate::Brand: web_common_traits::prelude::BoundedRead<C>,
    crate::CameraModel: web_common_traits::prelude::BoundedRead<C>,
    crate::Camera: web_common_traits::prelude::BoundedRead<C>,
    crate::CapModel: web_common_traits::prelude::BoundedRead<C>,
    crate::CappingProcedureTemplate: web_common_traits::prelude::BoundedRead<C>,
    crate::CappingProcedure: web_common_traits::prelude::BoundedRead<C>,
    crate::CentrifugeModel: web_common_traits::prelude::BoundedRead<C>,
    crate::CentrifugeProcedureTemplate: web_common_traits::prelude::BoundedRead<C>,
    crate::CentrifugeProcedure: web_common_traits::prelude::BoundedRead<C>,
    crate::Centrifuge: web_common_traits::prelude::BoundedRead<C>,
    crate::City: web_common_traits::prelude::BoundedRead<C>,
    crate::Color: web_common_traits::prelude::BoundedRead<C>,
    crate::CommercialBallMillMachineLot: web_common_traits::prelude::BoundedRead<C>,
    crate::CommercialBallMillMachineModel: web_common_traits::prelude::BoundedRead<C>,
    crate::CommercialBeadLot: web_common_traits::prelude::BoundedRead<C>,
    crate::CommercialBeadModel: web_common_traits::prelude::BoundedRead<C>,
    crate::CommercialCameraLot: web_common_traits::prelude::BoundedRead<C>,
    crate::CommercialCameraModel: web_common_traits::prelude::BoundedRead<C>,
    crate::CommercialCapLot: web_common_traits::prelude::BoundedRead<C>,
    crate::CommercialCapModel: web_common_traits::prelude::BoundedRead<C>,
    crate::CommercialCentrifugeLot: web_common_traits::prelude::BoundedRead<C>,
    crate::CommercialCentrifugeModel: web_common_traits::prelude::BoundedRead<C>,
    crate::CommercialFreezeDryerLot: web_common_traits::prelude::BoundedRead<C>,
    crate::CommercialFreezeDryerModel: web_common_traits::prelude::BoundedRead<C>,
    crate::CommercialFreezerLot: web_common_traits::prelude::BoundedRead<C>,
    crate::CommercialFreezerModel: web_common_traits::prelude::BoundedRead<C>,
    crate::CommercialPackagingLot: web_common_traits::prelude::BoundedRead<C>,
    crate::CommercialPackagingModel: web_common_traits::prelude::BoundedRead<C>,
    crate::CommercialPipetteLot: web_common_traits::prelude::BoundedRead<C>,
    crate::CommercialPipetteModel: web_common_traits::prelude::BoundedRead<C>,
    crate::CommercialPipetteTipLot: web_common_traits::prelude::BoundedRead<C>,
    crate::CommercialPipetteTipModel: web_common_traits::prelude::BoundedRead<C>,
    crate::CommercialPositioningDeviceLot: web_common_traits::prelude::BoundedRead<C>,
    crate::CommercialPositioningDeviceModel: web_common_traits::prelude::BoundedRead<C>,
    crate::CommercialProductLot: web_common_traits::prelude::BoundedRead<C>,
    crate::CommercialProduct: web_common_traits::prelude::BoundedRead<C>,
    crate::CommercialVolumeMeasuringDeviceLot: web_common_traits::prelude::BoundedRead<C>,
    crate::CommercialVolumeMeasuringDeviceModel: web_common_traits::prelude::BoundedRead<C>,
    crate::CommercialWeighingDeviceLot: web_common_traits::prelude::BoundedRead<C>,
    crate::CommercialWeighingDeviceModel: web_common_traits::prelude::BoundedRead<C>,
    crate::ContainerCompatibilityRule: web_common_traits::prelude::BoundedRead<C>,
    crate::ContainerModel: web_common_traits::prelude::BoundedRead<C>,
    crate::Container: web_common_traits::prelude::BoundedRead<C>,
    crate::Country: web_common_traits::prelude::BoundedRead<C>,
    crate::DigitalAssetModel: web_common_traits::prelude::BoundedRead<C>,
    crate::DigitalAsset: web_common_traits::prelude::BoundedRead<C>,
    crate::DisposalProcedureTemplate: web_common_traits::prelude::BoundedRead<C>,
    crate::DisposalProcedure: web_common_traits::prelude::BoundedRead<C>,
    crate::Document: web_common_traits::prelude::BoundedRead<C>,
    crate::EmailProvider: web_common_traits::prelude::BoundedRead<C>,
    crate::FractioningProcedureTemplate: web_common_traits::prelude::BoundedRead<C>,
    crate::FractioningProcedure: web_common_traits::prelude::BoundedRead<C>,
    crate::FreezeDryerModel: web_common_traits::prelude::BoundedRead<C>,
    crate::FreezeDryer: web_common_traits::prelude::BoundedRead<C>,
    crate::FreezeDryingProcedureTemplate: web_common_traits::prelude::BoundedRead<C>,
    crate::FreezeDryingProcedure: web_common_traits::prelude::BoundedRead<C>,
    crate::FreezerModel: web_common_traits::prelude::BoundedRead<C>,
    crate::Freezer: web_common_traits::prelude::BoundedRead<C>,
    crate::FreezingProcedureTemplate: web_common_traits::prelude::BoundedRead<C>,
    crate::FreezingProcedure: web_common_traits::prelude::BoundedRead<C>,
    crate::GeolocationProcedureTemplate: web_common_traits::prelude::BoundedRead<C>,
    crate::GeolocationProcedure: web_common_traits::prelude::BoundedRead<C>,
    crate::InstrumentState: web_common_traits::prelude::BoundedRead<C>,
    crate::LoginProvider: web_common_traits::prelude::BoundedRead<C>,
    crate::Material: web_common_traits::prelude::BoundedRead<C>,
    crate::NextProcedureTemplate: web_common_traits::prelude::BoundedRead<C>,
    crate::ObservationSubject: web_common_traits::prelude::BoundedRead<C>,
    crate::OrganismTaxon: web_common_traits::prelude::BoundedRead<C>,
    crate::Organism: web_common_traits::prelude::BoundedRead<C>,
    crate::Organization: web_common_traits::prelude::BoundedRead<C>,
    crate::PackagingModel: web_common_traits::prelude::BoundedRead<C>,
    crate::PackagingProcedureTemplate: web_common_traits::prelude::BoundedRead<C>,
    crate::PackagingProcedure: web_common_traits::prelude::BoundedRead<C>,
    crate::ParentProcedureTemplate: web_common_traits::prelude::BoundedRead<C>,
    crate::PermanenceCategory: web_common_traits::prelude::BoundedRead<C>,
    crate::PhoneModel: web_common_traits::prelude::BoundedRead<C>,
    crate::PhotographProcedureTemplate: web_common_traits::prelude::BoundedRead<C>,
    crate::PhotographProcedure: web_common_traits::prelude::BoundedRead<C>,
    crate::PhysicalAssetModel: web_common_traits::prelude::BoundedRead<C>,
    crate::PhysicalAsset: web_common_traits::prelude::BoundedRead<C>,
    crate::PipetteModel: web_common_traits::prelude::BoundedRead<C>,
    crate::PipetteTipModel: web_common_traits::prelude::BoundedRead<C>,
    crate::Pipette: web_common_traits::prelude::BoundedRead<C>,
    crate::PositioningDeviceModel: web_common_traits::prelude::BoundedRead<C>,
    crate::PositioningDevice: web_common_traits::prelude::BoundedRead<C>,
    crate::PouringProcedureTemplate: web_common_traits::prelude::BoundedRead<C>,
    crate::PouringProcedure: web_common_traits::prelude::BoundedRead<C>,
    crate::ProcedureAsset: web_common_traits::prelude::BoundedRead<C>,
    crate::ProcedureTemplateAssetModel: web_common_traits::prelude::BoundedRead<C>,
    crate::ProcedureTemplate: web_common_traits::prelude::BoundedRead<C>,
    crate::Procedure: web_common_traits::prelude::BoundedRead<C>,
    crate::ProjectState: web_common_traits::prelude::BoundedRead<C>,
    crate::Project: web_common_traits::prelude::BoundedRead<C>,
    crate::Rank: web_common_traits::prelude::BoundedRead<C>,
    crate::ReagentModel: web_common_traits::prelude::BoundedRead<C>,
    crate::Role: web_common_traits::prelude::BoundedRead<C>,
    crate::Room: web_common_traits::prelude::BoundedRead<C>,
    crate::SampleState: web_common_traits::prelude::BoundedRead<C>,
    crate::SpatialRefSy: web_common_traits::prelude::BoundedRead<C>,
    crate::Spectrum: web_common_traits::prelude::BoundedRead<C>,
    crate::SpectraCollection: web_common_traits::prelude::BoundedRead<C>,
    crate::StorageProcedureTemplate: web_common_traits::prelude::BoundedRead<C>,
    crate::StorageProcedure: web_common_traits::prelude::BoundedRead<C>,
    crate::SupernatantProcedureTemplate: web_common_traits::prelude::BoundedRead<C>,
    crate::SupernatantProcedure: web_common_traits::prelude::BoundedRead<C>,
    crate::Taxon: web_common_traits::prelude::BoundedRead<C>,
    crate::TeamMember: web_common_traits::prelude::BoundedRead<C>,
    crate::TeamProject: web_common_traits::prelude::BoundedRead<C>,
    crate::TeamState: web_common_traits::prelude::BoundedRead<C>,
    crate::Team: web_common_traits::prelude::BoundedRead<C>,
    crate::TemporaryUser: web_common_traits::prelude::BoundedRead<C>,
    crate::Unit: web_common_traits::prelude::BoundedRead<C>,
    crate::UserEmail: web_common_traits::prelude::BoundedRead<C>,
    crate::UserOrganization: web_common_traits::prelude::BoundedRead<C>,
    crate::User: web_common_traits::prelude::BoundedRead<C>,
    crate::VolumeMeasuringDeviceModel: web_common_traits::prelude::BoundedRead<C>,
    crate::VolumeMeasuringDevice: web_common_traits::prelude::BoundedRead<C>,
    crate::VolumetricContainerModel: web_common_traits::prelude::BoundedRead<C>,
    crate::VolumetricContainer: web_common_traits::prelude::BoundedRead<C>,
    crate::WeighingDeviceModel: web_common_traits::prelude::BoundedRead<C>,
    crate::WeighingDevice: web_common_traits::prelude::BoundedRead<C>,
    crate::WeighingProcedureTemplate: web_common_traits::prelude::BoundedRead<C>,
    crate::WeighingProcedure: web_common_traits::prelude::BoundedRead<C>,
{
    type TableName = crate::codegen::tables::table_names::TableName;
    fn bounded_read(
        table_name: Self::TableName,
        offset: u16,
        limit: u16,
        conn: &mut C,
    ) -> Result<Self, diesel::result::Error> {
        use web_common_traits::database::BoundedRead;
        match table_name {
            crate::codegen::tables::table_names::TableName::Address => {
                crate::Address::bounded_read(offset, limit, conn).map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::AliquotingProcedureTemplate => {
                crate::AliquotingProcedureTemplate::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::AliquotingProcedure => {
                crate::AliquotingProcedure::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::AssetCompatibilityRule => {
                crate::AssetCompatibilityRule::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::AssetModelAncestor => {
                crate::AssetModelAncestor::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::AssetModel => {
                crate::AssetModel::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Asset => {
                crate::Asset::bounded_read(offset, limit, conn).map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::BallMillMachineModel => {
                crate::BallMillMachineModel::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::BallMillMachine => {
                crate::BallMillMachine::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::BallMillProcedureTemplate => {
                crate::BallMillProcedureTemplate::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::BallMillProcedure => {
                crate::BallMillProcedure::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::BeadModel => {
                crate::BeadModel::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Brand => {
                crate::Brand::bounded_read(offset, limit, conn).map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CameraModel => {
                crate::CameraModel::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Camera => {
                crate::Camera::bounded_read(offset, limit, conn).map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CapModel => {
                crate::CapModel::bounded_read(offset, limit, conn).map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CappingProcedureTemplate => {
                crate::CappingProcedureTemplate::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CappingProcedure => {
                crate::CappingProcedure::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CentrifugeModel => {
                crate::CentrifugeModel::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CentrifugeProcedureTemplate => {
                crate::CentrifugeProcedureTemplate::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CentrifugeProcedure => {
                crate::CentrifugeProcedure::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Centrifuge => {
                crate::Centrifuge::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::City => {
                crate::City::bounded_read(offset, limit, conn).map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Color => {
                crate::Color::bounded_read(offset, limit, conn).map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CommercialBallMillMachineLot => {
                crate::CommercialBallMillMachineLot::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CommercialBallMillMachineModel => {
                crate::CommercialBallMillMachineModel::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CommercialBeadLot => {
                crate::CommercialBeadLot::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CommercialBeadModel => {
                crate::CommercialBeadModel::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CommercialCameraLot => {
                crate::CommercialCameraLot::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CommercialCameraModel => {
                crate::CommercialCameraModel::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CommercialCapLot => {
                crate::CommercialCapLot::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CommercialCapModel => {
                crate::CommercialCapModel::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CommercialCentrifugeLot => {
                crate::CommercialCentrifugeLot::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CommercialCentrifugeModel => {
                crate::CommercialCentrifugeModel::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CommercialFreezeDryerLot => {
                crate::CommercialFreezeDryerLot::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CommercialFreezeDryerModel => {
                crate::CommercialFreezeDryerModel::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CommercialFreezerLot => {
                crate::CommercialFreezerLot::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CommercialFreezerModel => {
                crate::CommercialFreezerModel::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CommercialPackagingLot => {
                crate::CommercialPackagingLot::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CommercialPackagingModel => {
                crate::CommercialPackagingModel::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CommercialPipetteLot => {
                crate::CommercialPipetteLot::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CommercialPipetteModel => {
                crate::CommercialPipetteModel::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CommercialPipetteTipLot => {
                crate::CommercialPipetteTipLot::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CommercialPipetteTipModel => {
                crate::CommercialPipetteTipModel::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CommercialPositioningDeviceLot => {
                crate::CommercialPositioningDeviceLot::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CommercialPositioningDeviceModel => {
                crate::CommercialPositioningDeviceModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CommercialProductLot => {
                crate::CommercialProductLot::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CommercialProduct => {
                crate::CommercialProduct::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CommercialVolumeMeasuringDeviceLot => {
                crate::CommercialVolumeMeasuringDeviceLot::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CommercialVolumeMeasuringDeviceModel => {
                crate::CommercialVolumeMeasuringDeviceModel::bounded_read(
                        offset,
                        limit,
                        conn,
                    )
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CommercialWeighingDeviceLot => {
                crate::CommercialWeighingDeviceLot::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::CommercialWeighingDeviceModel => {
                crate::CommercialWeighingDeviceModel::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::ContainerCompatibilityRule => {
                crate::ContainerCompatibilityRule::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::ContainerModel => {
                crate::ContainerModel::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Container => {
                crate::Container::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Country => {
                crate::Country::bounded_read(offset, limit, conn).map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::DigitalAssetModel => {
                crate::DigitalAssetModel::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::DigitalAsset => {
                crate::DigitalAsset::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::DisposalProcedureTemplate => {
                crate::DisposalProcedureTemplate::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::DisposalProcedure => {
                crate::DisposalProcedure::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Document => {
                crate::Document::bounded_read(offset, limit, conn).map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::EmailProvider => {
                crate::EmailProvider::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::FractioningProcedureTemplate => {
                crate::FractioningProcedureTemplate::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::FractioningProcedure => {
                crate::FractioningProcedure::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::FreezeDryerModel => {
                crate::FreezeDryerModel::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::FreezeDryer => {
                crate::FreezeDryer::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::FreezeDryingProcedureTemplate => {
                crate::FreezeDryingProcedureTemplate::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::FreezeDryingProcedure => {
                crate::FreezeDryingProcedure::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::FreezerModel => {
                crate::FreezerModel::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Freezer => {
                crate::Freezer::bounded_read(offset, limit, conn).map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::FreezingProcedureTemplate => {
                crate::FreezingProcedureTemplate::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::FreezingProcedure => {
                crate::FreezingProcedure::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::GeolocationProcedureTemplate => {
                crate::GeolocationProcedureTemplate::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::GeolocationProcedure => {
                crate::GeolocationProcedure::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::InstrumentState => {
                crate::InstrumentState::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::LoginProvider => {
                crate::LoginProvider::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Material => {
                crate::Material::bounded_read(offset, limit, conn).map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::NextProcedureTemplate => {
                crate::NextProcedureTemplate::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::ObservationSubject => {
                crate::ObservationSubject::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::OrganismTaxon => {
                crate::OrganismTaxon::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Organism => {
                crate::Organism::bounded_read(offset, limit, conn).map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Organization => {
                crate::Organization::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::PackagingModel => {
                crate::PackagingModel::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::PackagingProcedureTemplate => {
                crate::PackagingProcedureTemplate::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::PackagingProcedure => {
                crate::PackagingProcedure::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::ParentProcedureTemplate => {
                crate::ParentProcedureTemplate::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::PermanenceCategory => {
                crate::PermanenceCategory::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::PhoneModel => {
                crate::PhoneModel::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::PhotographProcedureTemplate => {
                crate::PhotographProcedureTemplate::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::PhotographProcedure => {
                crate::PhotographProcedure::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::PhysicalAssetModel => {
                crate::PhysicalAssetModel::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::PhysicalAsset => {
                crate::PhysicalAsset::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::PipetteModel => {
                crate::PipetteModel::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::PipetteTipModel => {
                crate::PipetteTipModel::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Pipette => {
                crate::Pipette::bounded_read(offset, limit, conn).map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::PositioningDeviceModel => {
                crate::PositioningDeviceModel::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::PositioningDevice => {
                crate::PositioningDevice::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::PouringProcedureTemplate => {
                crate::PouringProcedureTemplate::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::PouringProcedure => {
                crate::PouringProcedure::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::ProcedureAsset => {
                crate::ProcedureAsset::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::ProcedureTemplateAssetModel => {
                crate::ProcedureTemplateAssetModel::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::ProcedureTemplate => {
                crate::ProcedureTemplate::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Procedure => {
                crate::Procedure::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::ProjectState => {
                crate::ProjectState::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Project => {
                crate::Project::bounded_read(offset, limit, conn).map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Rank => {
                crate::Rank::bounded_read(offset, limit, conn).map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::ReagentModel => {
                crate::ReagentModel::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Role => {
                crate::Role::bounded_read(offset, limit, conn).map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Room => {
                crate::Room::bounded_read(offset, limit, conn).map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::SampleState => {
                crate::SampleState::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::SpatialRefSy => {
                crate::SpatialRefSy::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Spectrum => {
                crate::Spectrum::bounded_read(offset, limit, conn).map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::SpectraCollection => {
                crate::SpectraCollection::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::StorageProcedureTemplate => {
                crate::StorageProcedureTemplate::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::StorageProcedure => {
                crate::StorageProcedure::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::SupernatantProcedureTemplate => {
                crate::SupernatantProcedureTemplate::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::SupernatantProcedure => {
                crate::SupernatantProcedure::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Taxon => {
                crate::Taxon::bounded_read(offset, limit, conn).map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::TeamMember => {
                crate::TeamMember::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::TeamProject => {
                crate::TeamProject::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::TeamState => {
                crate::TeamState::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Team => {
                crate::Team::bounded_read(offset, limit, conn).map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::TemporaryUser => {
                crate::TemporaryUser::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::Unit => {
                crate::Unit::bounded_read(offset, limit, conn).map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::UserEmail => {
                crate::UserEmail::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::UserOrganization => {
                crate::UserOrganization::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::User => {
                crate::User::bounded_read(offset, limit, conn).map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::VolumeMeasuringDeviceModel => {
                crate::VolumeMeasuringDeviceModel::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::VolumeMeasuringDevice => {
                crate::VolumeMeasuringDevice::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::VolumetricContainerModel => {
                crate::VolumetricContainerModel::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::VolumetricContainer => {
                crate::VolumetricContainer::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::WeighingDeviceModel => {
                crate::WeighingDeviceModel::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::WeighingDevice => {
                crate::WeighingDevice::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::WeighingProcedureTemplate => {
                crate::WeighingProcedureTemplate::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
            crate::codegen::tables::table_names::TableName::WeighingProcedure => {
                crate::WeighingProcedure::bounded_read(offset, limit, conn)
                    .map(super::Rows::from)
            }
        }
    }
}
