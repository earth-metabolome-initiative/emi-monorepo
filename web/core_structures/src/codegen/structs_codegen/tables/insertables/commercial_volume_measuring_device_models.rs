#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CommercialVolumeMeasuringDeviceModelExtensionAttribute {
    VolumeMeasuringDeviceModel(
        crate::codegen::structs_codegen::tables::insertables::VolumeMeasuringDeviceModelAttribute,
    ),
    CommercialProduct(
        crate::codegen::structs_codegen::tables::insertables::CommercialProductAttribute,
    ),
}
impl core::fmt::Display for CommercialVolumeMeasuringDeviceModelExtensionAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::VolumeMeasuringDeviceModel(e) => {
                write!(f, "commercial_volume_measuring_device_models({e})")
            }
            Self::CommercialProduct(e) => {
                write!(f, "commercial_volume_measuring_device_models({e})")
            }
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::VolumeMeasuringDeviceModelAttribute>
    for CommercialVolumeMeasuringDeviceModelExtensionAttribute
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::VolumeMeasuringDeviceModelAttribute,
    ) -> Self {
        Self::VolumeMeasuringDeviceModel(attribute)
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::CommercialProductAttribute>
    for CommercialVolumeMeasuringDeviceModelExtensionAttribute
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::CommercialProductAttribute,
    ) -> Self {
        Self::CommercialProduct(attribute)
    }
}
impl From<common_traits::builder::EmptyTuple>
    for CommercialVolumeMeasuringDeviceModelExtensionAttribute
{
    fn from(_attribute: common_traits::builder::EmptyTuple) -> Self {
        unreachable!("Some code generation error occurred to reach this point.")
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CommercialVolumeMeasuringDeviceModelAttribute {
    Extension(CommercialVolumeMeasuringDeviceModelExtensionAttribute),
    Id,
    VolumeMeasuringDeviceModel,
}
impl core::str::FromStr for CommercialVolumeMeasuringDeviceModelAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Id" => Ok(Self::Id),
            "VolumeMeasuringDeviceModel" => Ok(Self::VolumeMeasuringDeviceModel),
            "id" => Ok(Self::Id),
            "volume_measuring_device_model" => Ok(Self::VolumeMeasuringDeviceModel),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl<T1, T2> common_traits::builder::Attributed
for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialVolumeMeasuringDeviceModelBuilder<
    T1,
    T2,
> {
    type Attribute = CommercialVolumeMeasuringDeviceModelAttribute;
}
impl web_common_traits::database::TableField for CommercialVolumeMeasuringDeviceModelAttribute {}
impl web_common_traits::database::HasTableType for CommercialVolumeMeasuringDeviceModelAttribute {
    type Table = crate::codegen::tables::table_names::TableName;
}
impl
    web_common_traits::database::FromExtension<
        crate::codegen::structs_codegen::tables::insertables::VolumeMeasuringDeviceModelAttribute,
    > for CommercialVolumeMeasuringDeviceModelAttribute
{
    fn from_extension(
        attribute: crate::codegen::structs_codegen::tables::insertables::VolumeMeasuringDeviceModelAttribute,
    ) -> Self {
        CommercialVolumeMeasuringDeviceModelAttribute::Extension(From::from(attribute))
    }
}
impl
    web_common_traits::database::FromExtension<
        crate::codegen::structs_codegen::tables::insertables::CommercialProductAttribute,
    > for CommercialVolumeMeasuringDeviceModelAttribute
{
    fn from_extension(
        attribute: crate::codegen::structs_codegen::tables::insertables::CommercialProductAttribute,
    ) -> Self {
        CommercialVolumeMeasuringDeviceModelAttribute::Extension(From::from(attribute))
    }
}
impl web_common_traits::database::FromExtension<common_traits::builder::EmptyTuple>
    for CommercialVolumeMeasuringDeviceModelAttribute
{
    fn from_extension(attribute: common_traits::builder::EmptyTuple) -> Self {
        CommercialVolumeMeasuringDeviceModelAttribute::Extension(From::from(attribute))
    }
}
impl core::fmt::Display for CommercialVolumeMeasuringDeviceModelAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::Id => write!(f, "commercial_volume_measuring_device_models.id"),
            Self::VolumeMeasuringDeviceModel => {
                write!(f, "commercial_volume_measuring_device_models.volume_measuring_device_model")
            }
        }
    }
}
#[derive(Debug)]
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::commercial_volume_measuring_device_models::commercial_volume_measuring_device_models
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableCommercialVolumeMeasuringDeviceModel {
    pub(crate) id: i32,
    pub(crate) volume_measuring_device_model: i32,
}
impl InsertableCommercialVolumeMeasuringDeviceModel {
    pub fn volume_measuring_device_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::volume_measuring_device_models::VolumeMeasuringDeviceModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::volume_measuring_device_models::VolumeMeasuringDeviceModel: web_common_traits::database::Read<
            C,
        >,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::volume_measuring_device_models::VolumeMeasuringDeviceModel::read(
            self.volume_measuring_device_model,
            conn,
        )
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Hash, Ord, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Builder for creating and inserting a new
/// [`CommercialVolumeMeasuringDeviceModel`](crate::codegen::structs_codegen::tables::commercial_volume_measuring_device_models::CommercialVolumeMeasuringDeviceModel).
///
///
/// # Implementation details
/// While this builder implements several methods, a reasonably complete
/// **basic** usage example (*which may not apply to your own specific use case,
/// please adapt accordingly*) is as follows:
///
/// ```rust,ignore
/// use core_structures::CommercialVolumeMeasuringDeviceModel;
/// use core_structures::tables::insertables::AssetModelSettable;
/// use core_structures::tables::insertables::CommercialProductSettable;
/// use core_structures::tables::insertables::CommercialVolumeMeasuringDeviceModelSettable;
/// use web_common_traits::database::Insertable;
/// use web_common_traits::database::InsertableVariant;
///
/// let commercial_volume_measuring_device_model = CommercialVolumeMeasuringDeviceModel::new()
///    // Set mandatory fields
///    .created_by(created_by)?
///    .description(description)?
///    .name(name)?
///    // Note: `updated_by` is automatically set by the `created by` column.
///    .updated_by(updated_by)?
///    .brand(brand_id)?
///    .volume_measuring_device_model(volume_measuring_device_model)?
///    // Optionally set fields with default values
///    .created_at(created_at)?
///    .updated_at(updated_at)?
///    // Optionally set optional fields
///    .parent_model(parent_model)?
///    .deprecation_date(deprecation_date)?
///    // Finally, insert the new record in the database
///    .insert(user.id, conn)?;
/// ```
pub struct InsertableCommercialVolumeMeasuringDeviceModelBuilder<
    VolumeMeasuringDeviceModel
        = crate::codegen::structs_codegen::tables::insertables::InsertableVolumeMeasuringDeviceModelBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetModelBuilder<
                crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelBuilder,
            >,
        >,
    CommercialProduct
        = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductBuilder<
            Option<i32>,
        >,
> {
    pub(crate) volume_measuring_device_model: Option<i32>,
    pub(crate) commercial_volume_measuring_device_models_id_fkey: VolumeMeasuringDeviceModel,
    pub(crate) commercial_volume_measuring_device_models_id_fkey1: CommercialProduct,
}
impl<VolumeMeasuringDeviceModel, CommercialProduct> diesel::associations::HasTable
    for InsertableCommercialVolumeMeasuringDeviceModelBuilder<
        VolumeMeasuringDeviceModel,
        CommercialProduct,
    >
{
    type Table = crate::codegen::diesel_codegen::tables::commercial_volume_measuring_device_models::commercial_volume_measuring_device_models::table;
    fn table() -> Self::Table {
        crate::codegen::diesel_codegen::tables::commercial_volume_measuring_device_models::commercial_volume_measuring_device_models::table
    }
}
impl From<InsertableCommercialVolumeMeasuringDeviceModelBuilder>
    for web_common_traits::database::IdOrBuilder<
        i32,
        InsertableCommercialVolumeMeasuringDeviceModelBuilder,
    >
{
    fn from(builder: InsertableCommercialVolumeMeasuringDeviceModelBuilder) -> Self {
        Self::Builder(builder)
    }
}
impl<
    VolumeMeasuringDeviceModel,
    CommercialProduct,
> common_traits::builder::IsCompleteBuilder
for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialVolumeMeasuringDeviceModelBuilder<
    VolumeMeasuringDeviceModel,
    CommercialProduct,
>
where
    CommercialProduct: common_traits::builder::IsCompleteBuilder,
    VolumeMeasuringDeviceModel: common_traits::builder::IsCompleteBuilder,
{
    fn is_complete(&self) -> bool {
        self.commercial_volume_measuring_device_models_id_fkey.is_complete()
            && self.commercial_volume_measuring_device_models_id_fkey1.is_complete()
            && self.volume_measuring_device_model.is_some()
    }
}
/// Trait defining setters for attributes of an instance of
/// `CommercialVolumeMeasuringDeviceModel` or descendant tables.
pub trait CommercialVolumeMeasuringDeviceModelSettable: Sized {
    /// Error type returned when setting attributes.
    type Error;
    /// Sets the value of the
    /// `public.commercial_volume_measuring_device_models.
    /// volume_measuring_device_model` column.
    ///
    /// # Arguments
    /// * `volume_measuring_device_model`: The value to set for the
    ///   `public.commercial_volume_measuring_device_models.
    ///   volume_measuring_device_model` column.
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
    fn volume_measuring_device_model<VMDM>(
        self,
        volume_measuring_device_model: VMDM,
    ) -> Result<Self, Self::Error>
    where
        VMDM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
}
impl<
    VolumeMeasuringDeviceModel: crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelSettable<
            Error = web_common_traits::database::InsertError<
                crate::codegen::structs_codegen::tables::insertables::VolumeMeasuringDeviceModelAttribute,
            >,
        >,
    CommercialProduct,
> CommercialVolumeMeasuringDeviceModelSettable
for InsertableCommercialVolumeMeasuringDeviceModelBuilder<
    VolumeMeasuringDeviceModel,
    CommercialProduct,
>
where
    Self: common_traits::builder::Attributed<
        Attribute = crate::codegen::structs_codegen::tables::insertables::CommercialVolumeMeasuringDeviceModelAttribute,
    >,
{
    type Error = web_common_traits::database::InsertError<
        <Self as common_traits::builder::Attributed>::Attribute,
    >;
    ///Sets the value of the `public.commercial_volume_measuring_device_models.volume_measuring_device_model` column.
    ///
    ///# Implementation notes
    ///This method also set the values of other columns, due to
    ///same-as relationships or inferred values.
    ///
    ///## Mermaid illustration
    ///
    ///```mermaid
    ///flowchart BT
    ///classDef column-of-interest stroke: #f0746c,fill: #f49f9a
    ///classDef directly-involved-column stroke: #6c74f0,fill: #9a9ff4
    ///classDef undirectly-involved-column stroke: #a7eff0,stroke-dasharray: 5, 5,fill: #d2f6f7
    ///subgraph v3 ["`asset_models`"]
    ///    v2@{shape: rounded, label: "parent_model"}
    ///class v2 undirectly-involved-column
    ///end
    ///subgraph v4 ["`commercial_volume_measuring_device_models`"]
    ///    v0@{shape: rounded, label: "volume_measuring_device_model"}
    ///class v0 column-of-interest
    ///end
    ///subgraph v5 ["`physical_asset_models`"]
    ///    v1@{shape: rounded, label: "parent_model"}
    ///class v1 directly-involved-column
    ///end
    ///v0 --->|"`ancestral same as`"| v2
    ///v0 -.->|"`inferred ancestral same as`"| v1
    ///v1 --->|"`ancestral same as`"| v2
    ///v5 --->|"`extends`"| v3
    ///```
    fn volume_measuring_device_model<VMDM>(
        mut self,
        volume_measuring_device_model: VMDM,
    ) -> Result<Self, Self::Error>
    where
        VMDM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let volume_measuring_device_model = <VMDM as web_common_traits::database::PrimaryKeyLike>::primary_key(
            &volume_measuring_device_model,
        );
        self.commercial_volume_measuring_device_models_id_fkey = <VolumeMeasuringDeviceModel as crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelSettable>::parent_model(
                self.commercial_volume_measuring_device_models_id_fkey,
                volume_measuring_device_model,
            )
            .map_err(|err| {
                err.replace_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
                    attribute.into(),
                ))
            })?;
        self.volume_measuring_device_model = Some(volume_measuring_device_model);
        Ok(self)
    }
}
impl<
    VolumeMeasuringDeviceModel: crate::codegen::structs_codegen::tables::insertables::AssetModelSettable<
            Error = web_common_traits::database::InsertError<
                crate::codegen::structs_codegen::tables::insertables::VolumeMeasuringDeviceModelAttribute,
            >,
        >,
    CommercialProduct,
> crate::codegen::structs_codegen::tables::insertables::AssetModelSettable
for InsertableCommercialVolumeMeasuringDeviceModelBuilder<
    VolumeMeasuringDeviceModel,
    CommercialProduct,
>
where
    Self: common_traits::builder::Attributed<
        Attribute = crate::codegen::structs_codegen::tables::insertables::CommercialVolumeMeasuringDeviceModelAttribute,
    >,
    Self: crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelSettable<
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialVolumeMeasuringDeviceModelAttribute,
        >,
    >,
{
    type Error = web_common_traits::database::InsertError<
        <Self as common_traits::builder::Attributed>::Attribute,
    >;
    #[inline]
    ///Sets the value of the `public.asset_models.name` column.
    fn name<N>(mut self, name: N) -> Result<Self, Self::Error>
    where
        N: TryInto<String>,
        validation_errors::prelude::SingleFieldError: From<<N as TryInto<String>>::Error>,
    {
        self.commercial_volume_measuring_device_models_id_fkey = <VolumeMeasuringDeviceModel as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::name(
                self.commercial_volume_measuring_device_models_id_fkey,
                name,
            )
            .map_err(|e| {
                e
                    .replace_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    ///Sets the value of the `public.asset_models.description` column.
    fn description<D>(mut self, description: D) -> Result<Self, Self::Error>
    where
        D: TryInto<String>,
        validation_errors::prelude::SingleFieldError: From<<D as TryInto<String>>::Error>,
    {
        self.commercial_volume_measuring_device_models_id_fkey = <VolumeMeasuringDeviceModel as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::description(
                self.commercial_volume_measuring_device_models_id_fkey,
                description,
            )
            .map_err(|e| {
                e
                    .replace_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    ///Sets the value of the `public.asset_models.parent_model` column.
    ///
    ///# Implementation notes
    ///This method also set the values of other columns, due to
    ///same-as relationships or inferred values.
    ///
    ///## Mermaid illustration
    ///
    ///```mermaid
    ///flowchart BT
    ///classDef column-of-interest stroke: #f0746c,fill: #f49f9a
    ///classDef directly-involved-column stroke: #6c74f0,fill: #9a9ff4
    ///subgraph v2 ["`asset_models`"]
    ///    v0@{shape: rounded, label: "parent_model"}
    ///class v0 column-of-interest
    ///end
    ///subgraph v3 ["`physical_asset_models`"]
    ///    v1@{shape: rounded, label: "parent_model"}
    ///class v1 directly-involved-column
    ///end
    ///v1 --->|"`ancestral same as`"| v0
    ///v3 --->|"`extends`"| v2
    ///```
    fn parent_model<PM>(self, parent_model: PM) -> Result<Self, Self::Error>
    where
        PM: web_common_traits::database::MaybePrimaryKeyLike<PrimaryKey = i32>,
    {
        <Self as crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelSettable>::parent_model(
            self,
            parent_model,
        )
    }
    #[inline]
    ///Sets the value of the `public.asset_models.created_by` column.
    fn created_by<CB>(mut self, created_by: CB) -> Result<Self, Self::Error>
    where
        CB: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        self.commercial_volume_measuring_device_models_id_fkey = <VolumeMeasuringDeviceModel as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_by(
                self.commercial_volume_measuring_device_models_id_fkey,
                created_by,
            )
            .map_err(|e| {
                e
                    .replace_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    ///Sets the value of the `public.asset_models.created_at` column.
    fn created_at<CA>(mut self, created_at: CA) -> Result<Self, Self::Error>
    where
        CA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::prelude::SingleFieldError: From<
            <CA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error,
        >,
    {
        self.commercial_volume_measuring_device_models_id_fkey = <VolumeMeasuringDeviceModel as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_at(
                self.commercial_volume_measuring_device_models_id_fkey,
                created_at,
            )
            .map_err(|e| {
                e
                    .replace_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    ///Sets the value of the `public.asset_models.updated_by` column.
    fn updated_by<UB>(mut self, updated_by: UB) -> Result<Self, Self::Error>
    where
        UB: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        self.commercial_volume_measuring_device_models_id_fkey = <VolumeMeasuringDeviceModel as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_by(
                self.commercial_volume_measuring_device_models_id_fkey,
                updated_by,
            )
            .map_err(|e| {
                e
                    .replace_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    ///Sets the value of the `public.asset_models.updated_at` column.
    fn updated_at<UA>(mut self, updated_at: UA) -> Result<Self, Self::Error>
    where
        UA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::prelude::SingleFieldError: From<
            <UA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error,
        >,
    {
        self.commercial_volume_measuring_device_models_id_fkey = <VolumeMeasuringDeviceModel as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_at(
                self.commercial_volume_measuring_device_models_id_fkey,
                updated_at,
            )
            .map_err(|e| {
                e
                    .replace_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
}
impl<
    VolumeMeasuringDeviceModel,
    CommercialProduct: crate::codegen::structs_codegen::tables::insertables::CommercialProductSettable<
            Error = web_common_traits::database::InsertError<
                crate::codegen::structs_codegen::tables::insertables::CommercialProductAttribute,
            >,
        >,
> crate::codegen::structs_codegen::tables::insertables::CommercialProductSettable
for InsertableCommercialVolumeMeasuringDeviceModelBuilder<
    VolumeMeasuringDeviceModel,
    CommercialProduct,
>
where
    Self: common_traits::builder::Attributed<
        Attribute = crate::codegen::structs_codegen::tables::insertables::CommercialVolumeMeasuringDeviceModelAttribute,
    >,
{
    type Error = web_common_traits::database::InsertError<
        <Self as common_traits::builder::Attributed>::Attribute,
    >;
    #[inline]
    ///Sets the value of the `public.commercial_products.deprecation_date` column.
    fn deprecation_date<DD>(mut self, deprecation_date: DD) -> Result<Self, Self::Error>
    where
        DD: TryInto<Option<::rosetta_timestamp::TimestampUTC>>,
        validation_errors::prelude::SingleFieldError: From<
            <DD as TryInto<Option<::rosetta_timestamp::TimestampUTC>>>::Error,
        >,
    {
        self.commercial_volume_measuring_device_models_id_fkey1 = <CommercialProduct as crate::codegen::structs_codegen::tables::insertables::CommercialProductSettable>::deprecation_date(
                self.commercial_volume_measuring_device_models_id_fkey1,
                deprecation_date,
            )
            .map_err(|e| {
                e
                    .replace_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    ///Sets the value of the `public.commercial_products.brand_id` column.
    fn brand<BI>(mut self, brand_id: BI) -> Result<Self, Self::Error>
    where
        BI: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        self.commercial_volume_measuring_device_models_id_fkey1 = <CommercialProduct as crate::codegen::structs_codegen::tables::insertables::CommercialProductSettable>::brand(
                self.commercial_volume_measuring_device_models_id_fkey1,
                brand_id,
            )
            .map_err(|e| {
                e
                    .replace_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
}
impl<
    VolumeMeasuringDeviceModel,
    CommercialProduct,
> crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelSettable
for InsertableCommercialVolumeMeasuringDeviceModelBuilder<
    VolumeMeasuringDeviceModel,
    CommercialProduct,
>
where
    Self: common_traits::builder::Attributed<
        Attribute = crate::codegen::structs_codegen::tables::insertables::CommercialVolumeMeasuringDeviceModelAttribute,
    >,
    Self: crate::codegen::structs_codegen::tables::insertables::CommercialVolumeMeasuringDeviceModelSettable<
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialVolumeMeasuringDeviceModelAttribute,
        >,
    >,
{
    type Error = web_common_traits::database::InsertError<
        <Self as common_traits::builder::Attributed>::Attribute,
    >;
    #[inline]
    ///Sets the value of the `public.physical_asset_models.parent_model` column.
    ///
    ///# Implementation notes
    ///This method also set the values of other columns, due to
    ///same-as relationships or inferred values.
    ///
    ///## Mermaid illustration
    ///
    ///```mermaid
    ///flowchart BT
    ///classDef column-of-interest stroke: #f0746c,fill: #f49f9a
    ///classDef directly-involved-column stroke: #6c74f0,fill: #9a9ff4
    ///classDef undirectly-involved-column stroke: #a7eff0,stroke-dasharray: 5, 5,fill: #d2f6f7
    ///subgraph v3 ["`asset_models`"]
    ///    v2@{shape: rounded, label: "parent_model"}
    ///class v2 undirectly-involved-column
    ///end
    ///subgraph v4 ["`commercial_volume_measuring_device_models`"]
    ///    v0@{shape: rounded, label: "volume_measuring_device_model"}
    ///class v0 directly-involved-column
    ///end
    ///subgraph v5 ["`physical_asset_models`"]
    ///    v1@{shape: rounded, label: "parent_model"}
    ///class v1 column-of-interest
    ///end
    ///v0 --->|"`ancestral same as`"| v2
    ///v0 -.->|"`inferred ancestral same as`"| v1
    ///v1 --->|"`ancestral same as`"| v2
    ///v5 --->|"`extends`"| v3
    ///```
    fn parent_model<PM>(self, parent_model: PM) -> Result<Self, Self::Error>
    where
        PM: web_common_traits::database::MaybePrimaryKeyLike<PrimaryKey = i32>,
    {
        <Self as CommercialVolumeMeasuringDeviceModelSettable>::volume_measuring_device_model(
            self,
            <PM as web_common_traits::database::MaybePrimaryKeyLike>::maybe_primary_key(
                    &parent_model,
                )
                .ok_or(
                    common_traits::prelude::BuilderError::IncompleteBuild(
                        <Self as common_traits::builder::Attributed>::Attribute::VolumeMeasuringDeviceModel,
                    ),
                )?,
        )
    }
}
impl<
    VolumeMeasuringDeviceModel,
    CommercialProduct,
> crate::codegen::structs_codegen::tables::insertables::VolumeMeasuringDeviceModelSettable
for InsertableCommercialVolumeMeasuringDeviceModelBuilder<
    VolumeMeasuringDeviceModel,
    CommercialProduct,
>
where
    Self: common_traits::builder::Attributed<
        Attribute = crate::codegen::structs_codegen::tables::insertables::CommercialVolumeMeasuringDeviceModelAttribute,
    >,
{
    type Error = web_common_traits::database::InsertError<
        <Self as common_traits::builder::Attributed>::Attribute,
    >;
}
impl<VolumeMeasuringDeviceModel, CommercialProduct> web_common_traits::database::MostConcreteTable
    for InsertableCommercialVolumeMeasuringDeviceModelBuilder<
        VolumeMeasuringDeviceModel,
        CommercialProduct,
    >
where
    VolumeMeasuringDeviceModel: web_common_traits::database::MostConcreteTable,
    CommercialProduct: web_common_traits::database::MostConcreteTable,
{
    fn set_most_concrete_table(&mut self, table_name: &str) {
        self.commercial_volume_measuring_device_models_id_fkey.set_most_concrete_table(table_name);
        self.commercial_volume_measuring_device_models_id_fkey1.set_most_concrete_table(table_name);
    }
}
impl<VolumeMeasuringDeviceModel, CommercialProduct> web_common_traits::prelude::SetPrimaryKey
    for InsertableCommercialVolumeMeasuringDeviceModelBuilder<
        VolumeMeasuringDeviceModel,
        CommercialProduct,
    >
where
    VolumeMeasuringDeviceModel: web_common_traits::prelude::SetPrimaryKey<PrimaryKey = i32>,
    CommercialProduct: web_common_traits::prelude::SetPrimaryKey<PrimaryKey = i32>,
{
    type PrimaryKey = i32;
    fn set_primary_key(mut self, primary_key: Self::PrimaryKey) -> Self {
        self.commercial_volume_measuring_device_models_id_fkey =
            self.commercial_volume_measuring_device_models_id_fkey.set_primary_key(primary_key);
        self.commercial_volume_measuring_device_models_id_fkey1 =
            self.commercial_volume_measuring_device_models_id_fkey1.set_primary_key(primary_key);
        self
    }
}
impl<
    VolumeMeasuringDeviceModel,
    CommercialProduct,
    C,
> web_common_traits::database::TryInsertGeneric<C>
for InsertableCommercialVolumeMeasuringDeviceModelBuilder<
    VolumeMeasuringDeviceModel,
    CommercialProduct,
>
where
    Self: web_common_traits::database::DispatchableInsertableVariant<
            C,
            Row = crate::codegen::structs_codegen::tables::commercial_volume_measuring_device_models::CommercialVolumeMeasuringDeviceModel,
            Error = web_common_traits::database::InsertError<
                CommercialVolumeMeasuringDeviceModelAttribute,
            >,
        > + web_common_traits::database::SetPrimaryKey<PrimaryKey = i32>
        + common_traits::builder::IsCompleteBuilder,
{
    type Error = web_common_traits::database::InsertError<
        CommercialVolumeMeasuringDeviceModelAttribute,
    >;
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, Self::Error> {
        use diesel::Identifiable;
        use web_common_traits::database::DispatchableInsertableVariant;
        Ok(self.insert(user_id, conn)?.id())
    }
}
