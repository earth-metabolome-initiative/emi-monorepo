#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableCommercialVolumeMeasuringDeviceLotExtensionAttributes {
    CommercialProductLot(
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductLotAttributes,
    ),
    VolumeMeasuringDeviceModel(
        crate::codegen::structs_codegen::tables::insertables::InsertableVolumeMeasuringDeviceModelAttributes,
    ),
}
impl core::fmt::Display for InsertableCommercialVolumeMeasuringDeviceLotExtensionAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::CommercialProductLot(e) => write!(f, "{e}"),
            Self::VolumeMeasuringDeviceModel(e) => write!(f, "{e}"),
        }
    }
}
impl From<
    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductLotAttributes,
> for InsertableCommercialVolumeMeasuringDeviceLotExtensionAttributes {
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductLotAttributes,
    ) -> Self {
        Self::CommercialProductLot(attribute)
    }
}
impl From<
    crate::codegen::structs_codegen::tables::insertables::InsertableVolumeMeasuringDeviceModelAttributes,
> for InsertableCommercialVolumeMeasuringDeviceLotExtensionAttributes {
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::InsertableVolumeMeasuringDeviceModelAttributes,
    ) -> Self {
        Self::VolumeMeasuringDeviceModel(attribute)
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableCommercialVolumeMeasuringDeviceLotAttributes {
    Extension(InsertableCommercialVolumeMeasuringDeviceLotExtensionAttributes),
    Id,
    ProductModelId,
}
impl core::str::FromStr for InsertableCommercialVolumeMeasuringDeviceLotAttributes {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ProductModelId" => Ok(Self::ProductModelId),
            "product_model_id" => Ok(Self::ProductModelId),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl core::fmt::Display for InsertableCommercialVolumeMeasuringDeviceLotAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::Id => write!(f, "id"),
            Self::ProductModelId => write!(f, "product_model_id"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::commercial_volume_measuring_device_lots::commercial_volume_measuring_device_lots
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableCommercialVolumeMeasuringDeviceLot {
    pub(crate) id: i32,
    pub(crate) product_model_id: i32,
}
impl InsertableCommercialVolumeMeasuringDeviceLot {
    pub fn commercial_volume_measuring_device_lots_id_fkey<
        C: diesel::connection::LoadConnection,
    >(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot::table(),
                self.id,
            ),
            conn,
        )
    }
    pub fn commercial_volume_measuring_device_lots_id_fkey1<
        C: diesel::connection::LoadConnection,
    >(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::volume_measuring_device_models::VolumeMeasuringDeviceModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::volume_measuring_device_models::VolumeMeasuringDeviceModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::volume_measuring_device_models::VolumeMeasuringDeviceModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::volume_measuring_device_models::VolumeMeasuringDeviceModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::volume_measuring_device_models::VolumeMeasuringDeviceModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::volume_measuring_device_models::VolumeMeasuringDeviceModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::volume_measuring_device_models::VolumeMeasuringDeviceModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::volume_measuring_device_models::VolumeMeasuringDeviceModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::volume_measuring_device_models::VolumeMeasuringDeviceModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::volume_measuring_device_models::VolumeMeasuringDeviceModel::table(),
                self.id,
            ),
            conn,
        )
    }
    pub fn product_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::commercial_volume_measuring_device_models::CommercialVolumeMeasuringDeviceModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::commercial_volume_measuring_device_models::CommercialVolumeMeasuringDeviceModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::commercial_volume_measuring_device_models::CommercialVolumeMeasuringDeviceModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::commercial_volume_measuring_device_models::CommercialVolumeMeasuringDeviceModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::commercial_volume_measuring_device_models::CommercialVolumeMeasuringDeviceModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::commercial_volume_measuring_device_models::CommercialVolumeMeasuringDeviceModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::commercial_volume_measuring_device_models::CommercialVolumeMeasuringDeviceModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::commercial_volume_measuring_device_models::CommercialVolumeMeasuringDeviceModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::commercial_volume_measuring_device_models::CommercialVolumeMeasuringDeviceModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::commercial_volume_measuring_device_models::CommercialVolumeMeasuringDeviceModel::table(),
                self.product_model_id,
            ),
            conn,
        )
    }
}
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableCommercialVolumeMeasuringDeviceLotBuilder<
    CommercialProductLot
        = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductLotBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetModelBuilder<
                crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelBuilder,
            >,
        >,
    VolumeMeasuringDeviceModel
        = crate::codegen::structs_codegen::tables::insertables::InsertableVolumeMeasuringDeviceModelBuilder<
            Option<i32>,
        >,
> {
    pub(crate) product_model_id: Option<i32>,
    pub(crate) commercial_volume_measuring_device_lots_id_fkey: CommercialProductLot,
    pub(crate) commercial_volume_measuring_device_lots_id_fkey1: VolumeMeasuringDeviceModel,
}
/// Trait defining setters for attributes of an instance of
/// `CommercialVolumeMeasuringDeviceLot` or descendant tables.
pub trait CommercialVolumeMeasuringDeviceLotBuildable:
    crate::codegen::structs_codegen::tables::insertables::CommercialProductLotBuildable
    + crate::codegen::structs_codegen::tables::insertables::VolumeMeasuringDeviceModelBuildable
{
    /// Sets the value of the
    /// `public.commercial_volume_measuring_device_lots.product_model_id`
    /// column.
    ///
    /// # Arguments
    /// * `product_model_id`: The value to set for the
    ///   `public.commercial_volume_measuring_device_lots.product_model_id`
    ///   column.
    ///
    /// # Implementation details
    /// This method accepts a reference to a generic value which can be
    /// converted to the required type for the column. This allows passing
    /// values of different types, as long as they can be converted to the
    /// required type using the `TryFrom` trait. The method, additionally,
    /// employs same-as and inferred same-as rules to ensure that the
    /// schema-defined ancestral tables and associated table values associated
    /// to the current column (if any) are also set appropriately.
    ///
    /// # Errors
    /// * If the provided value cannot be converted to the required type `i32`.
    /// * If the provided value does not pass schema-defined validation.
    fn product_model(
        self,
        product_model_id: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
}
impl CommercialVolumeMeasuringDeviceLotBuildable for Option<i32> {
    fn product_model(
        self,
        _product_model_id: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        Ok(self)
    }
}
impl<
    CommercialProductLot: crate::codegen::structs_codegen::tables::insertables::CommercialProductLotBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductLotAttributes,
        >,
    VolumeMeasuringDeviceModel: crate::codegen::structs_codegen::tables::insertables::VolumeMeasuringDeviceModelBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableVolumeMeasuringDeviceModelAttributes,
        >,
> CommercialVolumeMeasuringDeviceLotBuildable
for InsertableCommercialVolumeMeasuringDeviceLotBuilder<
    CommercialProductLot,
    VolumeMeasuringDeviceModel,
> {
    ///Sets the value of the `public.commercial_volume_measuring_device_lots.product_model_id` column.
    fn product_model(
        mut self,
        product_model_id: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        let product_model_id = product_model_id
            .try_into()
            .map_err(|err| {
                validation_errors::SingleFieldError::from(err)
                    .rename_field(
                        InsertableCommercialVolumeMeasuringDeviceLotAttributes::ProductModelId,
                    )
            })?;
        self.product_model_id = Some(product_model_id);
        Ok(self)
    }
}
impl<
    CommercialProductLot: crate::codegen::structs_codegen::tables::insertables::CommercialProductLotBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductLotAttributes,
        >,
    VolumeMeasuringDeviceModel: crate::codegen::structs_codegen::tables::insertables::VolumeMeasuringDeviceModelBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableVolumeMeasuringDeviceModelAttributes,
        >,
> crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable
for InsertableCommercialVolumeMeasuringDeviceLotBuilder<
    CommercialProductLot,
    VolumeMeasuringDeviceModel,
> {
    type Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialVolumeMeasuringDeviceLotAttributes;
    #[inline]
    ///Sets the value of the `public.asset_models.name` column.
    fn name<'N, N>(
        mut self,
        name: &'N N,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        &'N N: TryInto<Option<String>>,
        validation_errors::SingleFieldError: From<
            <&'N N as TryInto<Option<String>>>::Error,
        >,
    {
        self.commercial_volume_measuring_device_lots_id_fkey = <CommercialProductLot as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::name(
                self.commercial_volume_measuring_device_lots_id_fkey,
                name,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| Self::Attributes::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    ///Sets the value of the `public.asset_models.description` column.
    fn description<'D, D>(
        mut self,
        description: &'D D,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        &'D D: TryInto<Option<String>>,
        validation_errors::SingleFieldError: From<
            <&'D D as TryInto<Option<String>>>::Error,
        >,
    {
        self.commercial_volume_measuring_device_lots_id_fkey = <CommercialProductLot as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::description(
                self.commercial_volume_measuring_device_lots_id_fkey,
                description,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| Self::Attributes::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    ///Sets the value of the `public.asset_models.parent_model_id` column.
    ///
    ///# Implementation notes
    ///This method also set the values of other columns, due to
    ///same-as relationships or inferred values.
    ///
    ///## Mermaid illustration
    ///
    ///```mermaid
    ///flowchart LR
    ///classDef column-of-interest stroke: #f0746c,fill: #f49f9a
    ///classDef directly-involved-column stroke: #6c74f0,fill: #9a9ff4
    ///subgraph v2 ["`asset_models`"]
    ///    v0@{shape: rounded, label: "parent_model_id"}
    ///class v0 column-of-interest
    ///end
    ///subgraph v3 ["`commercial_volume_measuring_device_lots`"]
    ///    v1@{shape: rounded, label: "product_model_id"}
    ///class v1 directly-involved-column
    ///end
    ///v1 --->|"`ancestral same as`"| v0
    ///v3 -.->|"`descendant of`"| v2
    ///```
    fn parent_model(
        self,
        parent_model_id: Option<i32>,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        <Self as CommercialVolumeMeasuringDeviceLotBuildable>::product_model(
            self,
            parent_model_id
                .ok_or(
                    common_traits::prelude::BuilderError::IncompleteBuild(
                        Self::Attributes::ProductModelId,
                    ),
                )?,
        )
    }
    #[inline]
    ///Sets the value of the `public.asset_models.created_by` column.
    fn created_by(
        mut self,
        created_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.commercial_volume_measuring_device_lots_id_fkey = <CommercialProductLot as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::created_by(
                self.commercial_volume_measuring_device_lots_id_fkey,
                created_by,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| Self::Attributes::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    ///Sets the value of the `public.asset_models.created_at` column.
    fn created_at<'CA, CA>(
        mut self,
        created_at: &'CA CA,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        &'CA CA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::SingleFieldError: From<
            <&'CA CA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error,
        >,
    {
        self.commercial_volume_measuring_device_lots_id_fkey = <CommercialProductLot as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::created_at(
                self.commercial_volume_measuring_device_lots_id_fkey,
                created_at,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| Self::Attributes::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    ///Sets the value of the `public.asset_models.updated_by` column.
    fn updated_by(
        mut self,
        updated_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.commercial_volume_measuring_device_lots_id_fkey = <CommercialProductLot as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::updated_by(
                self.commercial_volume_measuring_device_lots_id_fkey,
                updated_by,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| Self::Attributes::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    ///Sets the value of the `public.asset_models.updated_at` column.
    fn updated_at<'UA, UA>(
        mut self,
        updated_at: &'UA UA,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        &'UA UA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::SingleFieldError: From<
            <&'UA UA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error,
        >,
    {
        self.commercial_volume_measuring_device_lots_id_fkey = <CommercialProductLot as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::updated_at(
                self.commercial_volume_measuring_device_lots_id_fkey,
                updated_at,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| Self::Attributes::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
}
impl<
    CommercialProductLot: crate::codegen::structs_codegen::tables::insertables::CommercialProductLotBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductLotAttributes,
        >,
    VolumeMeasuringDeviceModel: crate::codegen::structs_codegen::tables::insertables::VolumeMeasuringDeviceModelBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableVolumeMeasuringDeviceModelAttributes,
        >,
> crate::codegen::structs_codegen::tables::insertables::CommercialProductLotBuildable
for InsertableCommercialVolumeMeasuringDeviceLotBuilder<
    CommercialProductLot,
    VolumeMeasuringDeviceModel,
> {
    #[inline]
    ///Sets the value of the `public.commercial_product_lots.lot` column.
    fn lot<'L, L>(
        mut self,
        lot: &'L L,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        &'L L: TryInto<String>,
        validation_errors::SingleFieldError: From<<&'L L as TryInto<String>>::Error>,
    {
        self.commercial_volume_measuring_device_lots_id_fkey = <CommercialProductLot as crate::codegen::structs_codegen::tables::insertables::CommercialProductLotBuildable>::lot(
                self.commercial_volume_measuring_device_lots_id_fkey,
                lot,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| Self::Attributes::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    ///Sets the value of the `public.commercial_product_lots.product_model_id` column.
    fn product_model(
        mut self,
        product_model_id: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.commercial_volume_measuring_device_lots_id_fkey = <CommercialProductLot as crate::codegen::structs_codegen::tables::insertables::CommercialProductLotBuildable>::product_model(
                self.commercial_volume_measuring_device_lots_id_fkey,
                product_model_id,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| Self::Attributes::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
}
impl<
    CommercialProductLot: crate::codegen::structs_codegen::tables::insertables::CommercialProductLotBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductLotAttributes,
        >,
    VolumeMeasuringDeviceModel: crate::codegen::structs_codegen::tables::insertables::VolumeMeasuringDeviceModelBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableVolumeMeasuringDeviceModelAttributes,
        >,
> crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelBuildable
for InsertableCommercialVolumeMeasuringDeviceLotBuilder<
    CommercialProductLot,
    VolumeMeasuringDeviceModel,
> {
    #[inline]
    ///Sets the value of the `public.physical_asset_models.parent_model_id` column.
    fn parent_model(
        mut self,
        parent_model_id: Option<i32>,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.commercial_volume_measuring_device_lots_id_fkey = <CommercialProductLot as crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelBuildable>::parent_model(
                self.commercial_volume_measuring_device_lots_id_fkey,
                parent_model_id,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| Self::Attributes::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
}
impl<
    CommercialProductLot: crate::codegen::structs_codegen::tables::insertables::CommercialProductLotBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductLotAttributes,
        >,
    VolumeMeasuringDeviceModel: crate::codegen::structs_codegen::tables::insertables::VolumeMeasuringDeviceModelBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableVolumeMeasuringDeviceModelAttributes,
        >,
> crate::codegen::structs_codegen::tables::insertables::VolumeMeasuringDeviceModelBuildable
for InsertableCommercialVolumeMeasuringDeviceLotBuilder<
    CommercialProductLot,
    VolumeMeasuringDeviceModel,
> {}
impl<CommercialProductLot, VolumeMeasuringDeviceModel> web_common_traits::prelude::SetPrimaryKey
    for InsertableCommercialVolumeMeasuringDeviceLotBuilder<
        CommercialProductLot,
        VolumeMeasuringDeviceModel,
    >
where
    CommercialProductLot: web_common_traits::prelude::SetPrimaryKey<PrimaryKey = i32>,
    VolumeMeasuringDeviceModel: web_common_traits::prelude::SetPrimaryKey<PrimaryKey = i32>,
{
    type PrimaryKey = i32;
    fn set_primary_key(mut self, primary_key: Self::PrimaryKey) -> Self {
        self.commercial_volume_measuring_device_lots_id_fkey =
            self.commercial_volume_measuring_device_lots_id_fkey.set_primary_key(primary_key);
        self.commercial_volume_measuring_device_lots_id_fkey1 =
            self.commercial_volume_measuring_device_lots_id_fkey1.set_primary_key(primary_key);
        self
    }
}
impl<
    CommercialProductLot,
    VolumeMeasuringDeviceModel,
    C,
> web_common_traits::database::TryInsertGeneric<C>
for InsertableCommercialVolumeMeasuringDeviceLotBuilder<
    CommercialProductLot,
    VolumeMeasuringDeviceModel,
>
where
    Self: web_common_traits::database::InsertableVariant<
        C,
        UserId = i32,
        Row = crate::codegen::structs_codegen::tables::commercial_volume_measuring_device_lots::CommercialVolumeMeasuringDeviceLot,
        Error = web_common_traits::database::InsertError<
            InsertableCommercialVolumeMeasuringDeviceLotAttributes,
        >,
    >,
    CommercialProductLot: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
    VolumeMeasuringDeviceModel: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
{
    type Attributes = InsertableCommercialVolumeMeasuringDeviceLotAttributes;
    fn is_complete(&self) -> bool {
        self.commercial_volume_measuring_device_lots_id_fkey.is_complete()
            && self.commercial_volume_measuring_device_lots_id_fkey1.is_complete()
            && self.product_model_id.is_some()
    }
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<
        Self::PrimaryKey,
        web_common_traits::database::InsertError<Self::Attributes>,
    > {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::commercial_volume_measuring_device_lots::CommercialVolumeMeasuringDeviceLot = self
            .insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
