#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CommercialWeighingDeviceLotExtensionAttribute {
    CommercialProductLot(
        crate::codegen::structs_codegen::tables::insertables::CommercialProductLotAttribute,
    ),
    WeighingDeviceModel(
        crate::codegen::structs_codegen::tables::insertables::WeighingDeviceModelAttribute,
    ),
}
impl core::fmt::Display for CommercialWeighingDeviceLotExtensionAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::CommercialProductLot(e) => {
                write!(f, "commercial_weighing_device_lots({e})")
            }
            Self::WeighingDeviceModel(e) => {
                write!(f, "commercial_weighing_device_lots({e})")
            }
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::CommercialProductLotAttribute>
    for CommercialWeighingDeviceLotExtensionAttribute
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::CommercialProductLotAttribute,
    ) -> Self {
        Self::CommercialProductLot(attribute)
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::WeighingDeviceModelAttribute>
    for CommercialWeighingDeviceLotExtensionAttribute
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::WeighingDeviceModelAttribute,
    ) -> Self {
        Self::WeighingDeviceModel(attribute)
    }
}
impl From<common_traits::builder::EmptyTuple> for CommercialWeighingDeviceLotExtensionAttribute {
    fn from(_attribute: common_traits::builder::EmptyTuple) -> Self {
        unreachable!("Some code generation error occurred to reach this point.")
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CommercialWeighingDeviceLotAttribute {
    Extension(CommercialWeighingDeviceLotExtensionAttribute),
    Id,
    ProductModel,
}
impl core::str::FromStr for CommercialWeighingDeviceLotAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Id" => Ok(Self::Id),
            "ProductModel" => Ok(Self::ProductModel),
            "id" => Ok(Self::Id),
            "product_model" => Ok(Self::ProductModel),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl<T1, T2> common_traits::builder::Attributed
for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialWeighingDeviceLotBuilder<
    T1,
    T2,
> {
    type Attribute = CommercialWeighingDeviceLotAttribute;
}
impl web_common_traits::database::TableField for CommercialWeighingDeviceLotAttribute {}
impl web_common_traits::database::HasTableType for CommercialWeighingDeviceLotAttribute {
    type Table = crate::codegen::tables::table_names::TableName;
}
impl
    web_common_traits::database::FromExtension<
        crate::codegen::structs_codegen::tables::insertables::CommercialProductLotAttribute,
    > for CommercialWeighingDeviceLotAttribute
{
    fn from_extension(
        attribute: crate::codegen::structs_codegen::tables::insertables::CommercialProductLotAttribute,
    ) -> Self {
        CommercialWeighingDeviceLotAttribute::Extension(From::from(attribute))
    }
}
impl
    web_common_traits::database::FromExtension<
        crate::codegen::structs_codegen::tables::insertables::WeighingDeviceModelAttribute,
    > for CommercialWeighingDeviceLotAttribute
{
    fn from_extension(
        attribute: crate::codegen::structs_codegen::tables::insertables::WeighingDeviceModelAttribute,
    ) -> Self {
        CommercialWeighingDeviceLotAttribute::Extension(From::from(attribute))
    }
}
impl web_common_traits::database::FromExtension<common_traits::builder::EmptyTuple>
    for CommercialWeighingDeviceLotAttribute
{
    fn from_extension(attribute: common_traits::builder::EmptyTuple) -> Self {
        CommercialWeighingDeviceLotAttribute::Extension(From::from(attribute))
    }
}
impl core::fmt::Display for CommercialWeighingDeviceLotAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::Id => write!(f, "commercial_weighing_device_lots.id"),
            Self::ProductModel => {
                write!(f, "commercial_weighing_device_lots.product_model")
            }
        }
    }
}
#[derive(Debug)]
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::commercial_weighing_device_lots::commercial_weighing_device_lots
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableCommercialWeighingDeviceLot {
    pub(crate) id: i32,
    pub(crate) product_model: i32,
}
impl InsertableCommercialWeighingDeviceLot {
    pub fn product_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::commercial_weighing_device_models::CommercialWeighingDeviceModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::commercial_weighing_device_models::CommercialWeighingDeviceModel: web_common_traits::database::Read<
            C,
        >,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::commercial_weighing_device_models::CommercialWeighingDeviceModel::read(
            self.product_model,
            conn,
        )
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Hash, Ord, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Builder for creating and inserting a new
/// [`CommercialWeighingDeviceLot`](crate::codegen::structs_codegen::tables::commercial_weighing_device_lots::CommercialWeighingDeviceLot).
///
///
/// # Implementation details
/// While this builder implements several methods, a reasonably complete
/// **basic** usage example (*which may not apply to your own specific use case,
/// please adapt accordingly*) is as follows:
///
/// ```rust,ignore
/// use core_structures::CommercialWeighingDeviceLot;
/// use core_structures::tables::insertables::AssetModelSettable;
/// use core_structures::tables::insertables::CommercialProductLotSettable;
/// use core_structures::tables::insertables::CommercialWeighingDeviceLotSettable;
/// use web_common_traits::database::Insertable;
/// use web_common_traits::database::InsertableVariant;
///
/// let commercial_weighing_device_lot = CommercialWeighingDeviceLot::new()
///    // Set mandatory fields
///    .created_by(created_by)?
///    .description(description)?
///    .name(name)?
///    // Note: `updated_by` is automatically set by the `created by` column.
///    .updated_by(updated_by)?
///    .lot(lot)?
///    .product_model(product_model)?
///    // Optionally set fields with default values
///    .created_at(created_at)?
///    .updated_at(updated_at)?
///    // Finally, insert the new record in the database
///    .insert(user.id, conn)?;
/// ```
pub struct InsertableCommercialWeighingDeviceLotBuilder<
    CommercialProductLot
        = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductLotBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetModelBuilder<
                crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelBuilder,
            >,
        >,
    WeighingDeviceModel
        = crate::codegen::structs_codegen::tables::insertables::InsertableWeighingDeviceModelBuilder<
            Option<i32>,
        >,
> {
    pub(crate) product_model: Option<i32>,
    pub(crate) commercial_weighing_device_lots_id_fkey: CommercialProductLot,
    pub(crate) commercial_weighing_device_lots_id_fkey1: WeighingDeviceModel,
}
impl<CommercialProductLot, WeighingDeviceModel> diesel::associations::HasTable
    for InsertableCommercialWeighingDeviceLotBuilder<CommercialProductLot, WeighingDeviceModel>
{
    type Table = crate::codegen::diesel_codegen::tables::commercial_weighing_device_lots::commercial_weighing_device_lots::table;
    fn table() -> Self::Table {
        crate::codegen::diesel_codegen::tables::commercial_weighing_device_lots::commercial_weighing_device_lots::table
    }
}
impl From<InsertableCommercialWeighingDeviceLotBuilder>
    for web_common_traits::database::IdOrBuilder<i32, InsertableCommercialWeighingDeviceLotBuilder>
{
    fn from(builder: InsertableCommercialWeighingDeviceLotBuilder) -> Self {
        Self::Builder(builder)
    }
}
impl<CommercialProductLot, WeighingDeviceModel> common_traits::builder::IsCompleteBuilder
for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialWeighingDeviceLotBuilder<
    CommercialProductLot,
    WeighingDeviceModel,
>
where
    CommercialProductLot: common_traits::builder::IsCompleteBuilder,
    WeighingDeviceModel: common_traits::builder::IsCompleteBuilder,
{
    fn is_complete(&self) -> bool {
        self.commercial_weighing_device_lots_id_fkey.is_complete()
            && self.commercial_weighing_device_lots_id_fkey1.is_complete()
            && self.product_model.is_some()
    }
}
/// Trait defining setters for attributes of an instance of
/// `CommercialWeighingDeviceLot` or descendant tables.
pub trait CommercialWeighingDeviceLotSettable: Sized {
    /// Error type returned when setting attributes.
    type Error;
    /// Sets the value of the
    /// `public.commercial_weighing_device_lots.product_model` column.
    ///
    /// # Arguments
    /// * `product_model`: The value to set for the
    ///   `public.commercial_weighing_device_lots.product_model` column.
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
    fn product_model<PM>(self, product_model: PM) -> Result<Self, Self::Error>
    where
        PM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
}
impl<
    CommercialProductLot: crate::codegen::structs_codegen::tables::insertables::CommercialProductLotSettable<
            Error = web_common_traits::database::InsertError<
                crate::codegen::structs_codegen::tables::insertables::CommercialProductLotAttribute,
            >,
        >
        + crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelSettable<
            Error = web_common_traits::database::InsertError<
                crate::codegen::structs_codegen::tables::insertables::CommercialProductLotAttribute,
            >,
        >,
    WeighingDeviceModel,
> CommercialWeighingDeviceLotSettable
for InsertableCommercialWeighingDeviceLotBuilder<
    CommercialProductLot,
    WeighingDeviceModel,
>
where
    Self: common_traits::builder::Attributed<
        Attribute = crate::codegen::structs_codegen::tables::insertables::CommercialWeighingDeviceLotAttribute,
    >,
{
    type Error = web_common_traits::database::InsertError<
        <Self as common_traits::builder::Attributed>::Attribute,
    >;
    ///Sets the value of the `public.commercial_weighing_device_lots.product_model` column.
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
    ///subgraph v4 ["`asset_models`"]
    ///    v3@{shape: rounded, label: "parent_model"}
    ///class v3 undirectly-involved-column
    ///end
    ///subgraph v5 ["`commercial_product_lots`"]
    ///    v0@{shape: rounded, label: "product_model"}
    ///class v0 directly-involved-column
    ///end
    ///subgraph v6 ["`commercial_weighing_device_lots`"]
    ///    v1@{shape: rounded, label: "product_model"}
    ///class v1 column-of-interest
    ///end
    ///subgraph v7 ["`physical_asset_models`"]
    ///    v2@{shape: rounded, label: "parent_model"}
    ///class v2 directly-involved-column
    ///end
    ///v0 --->|"`ancestral same as`"| v3
    ///v0 -.->|"`inferred ancestral same as`"| v2
    ///v1 --->|"`ancestral same as`"| v3
    ///v1 -.->|"`inferred ancestral same as`"| v0
    ///v1 -.->|"`inferred ancestral same as`"| v2
    ///v2 --->|"`ancestral same as`"| v3
    ///v5 --->|"`extends`"| v7
    ///v6 --->|"`extends`"| v5
    ///v7 --->|"`extends`"| v4
    ///```
    fn product_model<PM>(mut self, product_model: PM) -> Result<Self, Self::Error>
    where
        PM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let product_model = <PM as web_common_traits::database::PrimaryKeyLike>::primary_key(
            &product_model,
        );
        self.commercial_weighing_device_lots_id_fkey = <CommercialProductLot as crate::codegen::structs_codegen::tables::insertables::CommercialProductLotSettable>::product_model(
                self.commercial_weighing_device_lots_id_fkey,
                product_model,
            )
            .map_err(|err| {
                err.into_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
                    attribute.into(),
                ))
            })?;
        self.commercial_weighing_device_lots_id_fkey = <CommercialProductLot as crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelSettable>::parent_model(
                self.commercial_weighing_device_lots_id_fkey,
                product_model,
            )
            .map_err(|err| {
                err.into_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
                    attribute.into(),
                ))
            })?;
        self.product_model = Some(product_model);
        Ok(self)
    }
}
impl<
    CommercialProductLot: crate::codegen::structs_codegen::tables::insertables::AssetModelSettable<
            Error = web_common_traits::database::InsertError<
                crate::codegen::structs_codegen::tables::insertables::CommercialProductLotAttribute,
            >,
        >,
    WeighingDeviceModel,
> crate::codegen::structs_codegen::tables::insertables::AssetModelSettable
for InsertableCommercialWeighingDeviceLotBuilder<
    CommercialProductLot,
    WeighingDeviceModel,
>
where
    Self: common_traits::builder::Attributed<
        Attribute = crate::codegen::structs_codegen::tables::insertables::CommercialWeighingDeviceLotAttribute,
    >,
    Self: crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelSettable<
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialWeighingDeviceLotAttribute,
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
        validation_errors::SingleFieldError: From<<N as TryInto<String>>::Error>,
    {
        self.commercial_weighing_device_lots_id_fkey = <CommercialProductLot as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::name(
                self.commercial_weighing_device_lots_id_fkey,
                name,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
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
        validation_errors::SingleFieldError: From<<D as TryInto<String>>::Error>,
    {
        self.commercial_weighing_device_lots_id_fkey = <CommercialProductLot as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::description(
                self.commercial_weighing_device_lots_id_fkey,
                description,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
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
        self.commercial_weighing_device_lots_id_fkey = <CommercialProductLot as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_by(
                self.commercial_weighing_device_lots_id_fkey,
                created_by,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
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
        validation_errors::SingleFieldError: From<
            <CA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error,
        >,
    {
        self.commercial_weighing_device_lots_id_fkey = <CommercialProductLot as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_at(
                self.commercial_weighing_device_lots_id_fkey,
                created_at,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
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
        self.commercial_weighing_device_lots_id_fkey = <CommercialProductLot as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_by(
                self.commercial_weighing_device_lots_id_fkey,
                updated_by,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
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
        validation_errors::SingleFieldError: From<
            <UA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error,
        >,
    {
        self.commercial_weighing_device_lots_id_fkey = <CommercialProductLot as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_at(
                self.commercial_weighing_device_lots_id_fkey,
                updated_at,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
}
impl<
    CommercialProductLot: crate::codegen::structs_codegen::tables::insertables::CommercialProductLotSettable<
            Error = web_common_traits::database::InsertError<
                crate::codegen::structs_codegen::tables::insertables::CommercialProductLotAttribute,
            >,
        >,
    WeighingDeviceModel,
> crate::codegen::structs_codegen::tables::insertables::CommercialProductLotSettable
for InsertableCommercialWeighingDeviceLotBuilder<
    CommercialProductLot,
    WeighingDeviceModel,
>
where
    Self: common_traits::builder::Attributed<
        Attribute = crate::codegen::structs_codegen::tables::insertables::CommercialWeighingDeviceLotAttribute,
    >,
    Self: crate::codegen::structs_codegen::tables::insertables::CommercialWeighingDeviceLotSettable<
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialWeighingDeviceLotAttribute,
        >,
    >,
{
    type Error = web_common_traits::database::InsertError<
        <Self as common_traits::builder::Attributed>::Attribute,
    >;
    #[inline]
    ///Sets the value of the `public.commercial_product_lots.lot` column.
    fn lot<L>(mut self, lot: L) -> Result<Self, Self::Error>
    where
        L: TryInto<String>,
        validation_errors::SingleFieldError: From<<L as TryInto<String>>::Error>,
    {
        self.commercial_weighing_device_lots_id_fkey = <CommercialProductLot as crate::codegen::structs_codegen::tables::insertables::CommercialProductLotSettable>::lot(
                self.commercial_weighing_device_lots_id_fkey,
                lot,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    ///Sets the value of the `public.commercial_product_lots.product_model` column.
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
    ///subgraph v3 ["`commercial_product_lots`"]
    ///    v0@{shape: rounded, label: "product_model"}
    ///class v0 column-of-interest
    ///end
    ///subgraph v4 ["`commercial_weighing_device_lots`"]
    ///    v1@{shape: rounded, label: "product_model"}
    ///class v1 directly-involved-column
    ///end
    ///subgraph v5 ["`physical_asset_models`"]
    ///    v2@{shape: rounded, label: "parent_model"}
    ///class v2 undirectly-involved-column
    ///end
    ///v0 -.->|"`inferred ancestral same as`"| v2
    ///v1 -.->|"`inferred ancestral same as`"| v0
    ///v1 -.->|"`inferred ancestral same as`"| v2
    ///v3 --->|"`extends`"| v5
    ///v4 --->|"`extends`"| v3
    ///```
    fn product_model<PM>(self, product_model: PM) -> Result<Self, Self::Error>
    where
        PM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        <Self as CommercialWeighingDeviceLotSettable>::product_model(self, product_model)
    }
}
impl<
    CommercialProductLot,
    WeighingDeviceModel,
> crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelSettable
for InsertableCommercialWeighingDeviceLotBuilder<
    CommercialProductLot,
    WeighingDeviceModel,
>
where
    Self: common_traits::builder::Attributed<
        Attribute = crate::codegen::structs_codegen::tables::insertables::CommercialWeighingDeviceLotAttribute,
    >,
    Self: crate::codegen::structs_codegen::tables::insertables::CommercialWeighingDeviceLotSettable<
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialWeighingDeviceLotAttribute,
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
    ///subgraph v4 ["`asset_models`"]
    ///    v2@{shape: rounded, label: "parent_model"}
    ///class v2 undirectly-involved-column
    ///end
    ///subgraph v5 ["`commercial_product_lots`"]
    ///    v3@{shape: rounded, label: "product_model"}
    ///class v3 undirectly-involved-column
    ///end
    ///subgraph v6 ["`commercial_weighing_device_lots`"]
    ///    v0@{shape: rounded, label: "product_model"}
    ///class v0 directly-involved-column
    ///end
    ///subgraph v7 ["`physical_asset_models`"]
    ///    v1@{shape: rounded, label: "parent_model"}
    ///class v1 column-of-interest
    ///end
    ///v3 --->|"`ancestral same as`"| v2
    ///v3 -.->|"`inferred ancestral same as`"| v1
    ///v0 --->|"`ancestral same as`"| v2
    ///v0 -.->|"`inferred ancestral same as`"| v3
    ///v0 -.->|"`inferred ancestral same as`"| v1
    ///v1 --->|"`ancestral same as`"| v2
    ///v5 --->|"`extends`"| v7
    ///v6 --->|"`extends`"| v5
    ///v7 --->|"`extends`"| v4
    ///```
    fn parent_model<PM>(self, parent_model: PM) -> Result<Self, Self::Error>
    where
        PM: web_common_traits::database::MaybePrimaryKeyLike<PrimaryKey = i32>,
    {
        <Self as CommercialWeighingDeviceLotSettable>::product_model(
            self,
            <PM as web_common_traits::database::MaybePrimaryKeyLike>::maybe_primary_key(
                    &parent_model,
                )
                .ok_or(
                    common_traits::prelude::BuilderError::IncompleteBuild(
                        <Self as common_traits::builder::Attributed>::Attribute::ProductModel,
                    ),
                )?,
        )
    }
}
impl<
    CommercialProductLot,
    WeighingDeviceModel,
> crate::codegen::structs_codegen::tables::insertables::WeighingDeviceModelSettable
for InsertableCommercialWeighingDeviceLotBuilder<
    CommercialProductLot,
    WeighingDeviceModel,
>
where
    Self: common_traits::builder::Attributed<
        Attribute = crate::codegen::structs_codegen::tables::insertables::CommercialWeighingDeviceLotAttribute,
    >,
{
    type Error = web_common_traits::database::InsertError<
        <Self as common_traits::builder::Attributed>::Attribute,
    >;
}
impl<CommercialProductLot, WeighingDeviceModel> web_common_traits::database::MostConcreteTable
    for InsertableCommercialWeighingDeviceLotBuilder<CommercialProductLot, WeighingDeviceModel>
where
    CommercialProductLot: web_common_traits::database::MostConcreteTable,
    WeighingDeviceModel: web_common_traits::database::MostConcreteTable,
{
    fn set_most_concrete_table(&mut self, table_name: &str) {
        self.commercial_weighing_device_lots_id_fkey.set_most_concrete_table(table_name);
        self.commercial_weighing_device_lots_id_fkey1.set_most_concrete_table(table_name);
    }
}
impl<CommercialProductLot, WeighingDeviceModel> web_common_traits::prelude::SetPrimaryKey
    for InsertableCommercialWeighingDeviceLotBuilder<CommercialProductLot, WeighingDeviceModel>
where
    CommercialProductLot: web_common_traits::prelude::SetPrimaryKey<PrimaryKey = i32>,
    WeighingDeviceModel: web_common_traits::prelude::SetPrimaryKey<PrimaryKey = i32>,
{
    type PrimaryKey = i32;
    fn set_primary_key(mut self, primary_key: Self::PrimaryKey) -> Self {
        self.commercial_weighing_device_lots_id_fkey =
            self.commercial_weighing_device_lots_id_fkey.set_primary_key(primary_key);
        self.commercial_weighing_device_lots_id_fkey1 =
            self.commercial_weighing_device_lots_id_fkey1.set_primary_key(primary_key);
        self
    }
}
impl<
    CommercialProductLot,
    WeighingDeviceModel,
    C,
> web_common_traits::database::TryInsertGeneric<C>
for InsertableCommercialWeighingDeviceLotBuilder<
    CommercialProductLot,
    WeighingDeviceModel,
>
where
    Self: web_common_traits::database::DispatchableInsertableVariant<
            C,
            Row = crate::codegen::structs_codegen::tables::commercial_weighing_device_lots::CommercialWeighingDeviceLot,
            Error = web_common_traits::database::InsertError<
                CommercialWeighingDeviceLotAttribute,
            >,
        > + web_common_traits::database::SetPrimaryKey<PrimaryKey = i32>
        + common_traits::builder::IsCompleteBuilder,
{
    type Error = web_common_traits::database::InsertError<
        CommercialWeighingDeviceLotAttribute,
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
