#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CommercialBeadLotExtensionAttribute {
    CommercialProductLot(
        crate::codegen::structs_codegen::tables::insertables::CommercialProductLotAttribute,
    ),
    BeadModel(crate::codegen::structs_codegen::tables::insertables::BeadModelAttribute),
}
impl core::fmt::Display for CommercialBeadLotExtensionAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::CommercialProductLot(e) => write!(f, "commercial_bead_lots({e})"),
            Self::BeadModel(e) => write!(f, "commercial_bead_lots({e})"),
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::CommercialProductLotAttribute>
    for CommercialBeadLotExtensionAttribute
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::CommercialProductLotAttribute,
    ) -> Self {
        Self::CommercialProductLot(attribute)
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::BeadModelAttribute>
    for CommercialBeadLotExtensionAttribute
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::BeadModelAttribute,
    ) -> Self {
        Self::BeadModel(attribute)
    }
}
impl From<common_traits::builder::EmptyTuple> for CommercialBeadLotExtensionAttribute {
    fn from(_attribute: common_traits::builder::EmptyTuple) -> Self {
        unreachable!("Some code generation error occurred to reach this point.")
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CommercialBeadLotAttribute {
    Extension(CommercialBeadLotExtensionAttribute),
    Id,
    ProductModel,
}
impl core::str::FromStr for CommercialBeadLotAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ProductModel" => Ok(Self::ProductModel),
            "product_model" => Ok(Self::ProductModel),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl<T1, T2> common_traits::builder::Attributed
    for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBeadLotBuilder<
        T1,
        T2,
    >
{
    type Attribute = CommercialBeadLotAttribute;
}
impl core::fmt::Display for CommercialBeadLotAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::Id => write!(f, "commercial_bead_lots.id"),
            Self::ProductModel => write!(f, "commercial_bead_lots.product_model"),
        }
    }
}
#[derive(Debug)]
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::commercial_bead_lots::commercial_bead_lots
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableCommercialBeadLot {
    pub(crate) id: i32,
    pub(crate) product_model: i32,
}
impl InsertableCommercialBeadLot {
    pub fn product_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::commercial_bead_models::CommercialBeadModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::commercial_bead_models::CommercialBeadModel:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::commercial_bead_models::CommercialBeadModel::read(
            self.product_model,
            conn,
        )
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Hash, Ord, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Builder for creating and inserting a new
/// [`CommercialBeadLot`](crate::codegen::structs_codegen::tables::commercial_bead_lots::CommercialBeadLot).
///
///
/// # Implementation details
/// While this builder implements several methods, a reasonably complete
/// **basic** usage example (*which may not apply to your own specific use case,
/// please adapt accordingly*) is as follows:
///
/// ```rust,ignore
/// use core_structures::CommercialBeadLot;
/// use core_structures::tables::insertables::AssetModelSettable;
/// use core_structures::tables::insertables::BeadModelSettable;
/// use core_structures::tables::insertables::CommercialBeadLotSettable;
/// use core_structures::tables::insertables::CommercialProductLotSettable;
/// use web_common_traits::database::Insertable;
/// use web_common_traits::database::InsertableVariant;
///
/// let commercial_bead_lot = CommercialBeadLot::new()
///    // Set mandatory fields
///    .created_by(created_by)?
///    .description(description)?
///    .name(name)?
///    // Note: `updated_by` is automatically set by the `created by` column.
///    .updated_by(updated_by)?
///    .diameter_millimeters(diameter_millimeters)?
///    .product_model(product_model)?
///    .lot(lot)?
///    // Optionally set fields with default values
///    .created_at(created_at)?
///    .updated_at(updated_at)?
///    // Finally, insert the new record in the database
///    .insert(user.id, conn)?;
/// ```
pub struct InsertableCommercialBeadLotBuilder<
    CommercialProductLot
        = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductLotBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetModelBuilder<
                crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelBuilder,
            >,
        >,
    BeadModel
        = crate::codegen::structs_codegen::tables::insertables::InsertableBeadModelBuilder<
            Option<i32>,
        >,
> {
    pub(crate) product_model: Option<i32>,
    pub(crate) commercial_bead_lots_id_fkey: CommercialProductLot,
    pub(crate) commercial_bead_lots_id_fkey1: BeadModel,
}
impl<CommercialProductLot, BeadModel> diesel::associations::HasTable
    for InsertableCommercialBeadLotBuilder<CommercialProductLot, BeadModel>
{
    type Table =
        crate::codegen::diesel_codegen::tables::commercial_bead_lots::commercial_bead_lots::table;
    fn table() -> Self::Table {
        crate::codegen::diesel_codegen::tables::commercial_bead_lots::commercial_bead_lots::table
    }
}
impl From<InsertableCommercialBeadLotBuilder>
    for web_common_traits::database::IdOrBuilder<i32, InsertableCommercialBeadLotBuilder>
{
    fn from(builder: InsertableCommercialBeadLotBuilder) -> Self {
        Self::Builder(builder)
    }
}
impl<CommercialProductLot, BeadModel> common_traits::builder::IsCompleteBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBeadLotBuilder<
        CommercialProductLot,
        BeadModel,
    >
where
    BeadModel: common_traits::builder::IsCompleteBuilder,
    CommercialProductLot: common_traits::builder::IsCompleteBuilder,
{
    fn is_complete(&self) -> bool {
        self.commercial_bead_lots_id_fkey.is_complete()
            && self.commercial_bead_lots_id_fkey1.is_complete()
            && self.product_model.is_some()
    }
}
/// Trait defining setters for attributes of an instance of `CommercialBeadLot`
/// or descendant tables.
pub trait CommercialBeadLotSettable: Sized {
    /// Error type returned when setting attributes.
    type Error;
    /// Sets the value of the `public.commercial_bead_lots.product_model`
    /// column.
    ///
    /// # Arguments
    /// * `product_model`: The value to set for the
    ///   `public.commercial_bead_lots.product_model` column.
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
    BeadModel,
> CommercialBeadLotSettable
for InsertableCommercialBeadLotBuilder<CommercialProductLot, BeadModel>
where
    Self: common_traits::builder::Attributed<
        Attribute = crate::codegen::structs_codegen::tables::insertables::CommercialBeadLotAttribute,
    >,
{
    type Error = web_common_traits::database::InsertError<
        <Self as common_traits::builder::Attributed>::Attribute,
    >;
    ///Sets the value of the `public.commercial_bead_lots.product_model` column.
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
    ///subgraph v5 ["`commercial_bead_lots`"]
    ///    v0@{shape: rounded, label: "product_model"}
    ///class v0 column-of-interest
    ///end
    ///subgraph v6 ["`commercial_product_lots`"]
    ///    v1@{shape: rounded, label: "product_model"}
    ///class v1 directly-involved-column
    ///end
    ///subgraph v7 ["`physical_asset_models`"]
    ///    v2@{shape: rounded, label: "parent_model"}
    ///class v2 directly-involved-column
    ///end
    ///v0 --->|"`ancestral same as`"| v3
    ///v0 -.->|"`inferred ancestral same as`"| v1
    ///v0 -.->|"`inferred ancestral same as`"| v2
    ///v1 --->|"`ancestral same as`"| v3
    ///v1 -.->|"`inferred ancestral same as`"| v2
    ///v2 --->|"`ancestral same as`"| v3
    ///v5 --->|"`extends`"| v6
    ///v6 --->|"`extends`"| v7
    ///v7 --->|"`extends`"| v4
    ///```
    fn product_model<PM>(mut self, product_model: PM) -> Result<Self, Self::Error>
    where
        PM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let product_model = <PM as web_common_traits::database::PrimaryKeyLike>::primary_key(
            &product_model,
        );
        self.commercial_bead_lots_id_fkey = <CommercialProductLot as crate::codegen::structs_codegen::tables::insertables::CommercialProductLotSettable>::product_model(
                self.commercial_bead_lots_id_fkey,
                product_model,
            )
            .map_err(|err| {
                err.into_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
                    attribute.into(),
                ))
            })?;
        self.commercial_bead_lots_id_fkey = <CommercialProductLot as crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelSettable>::parent_model(
                self.commercial_bead_lots_id_fkey,
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
    BeadModel,
> crate::codegen::structs_codegen::tables::insertables::AssetModelSettable
for InsertableCommercialBeadLotBuilder<CommercialProductLot, BeadModel>
where
    Self: common_traits::builder::Attributed<
        Attribute = crate::codegen::structs_codegen::tables::insertables::CommercialBeadLotAttribute,
    >,
    Self: crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelSettable<
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialBeadLotAttribute,
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
        self.commercial_bead_lots_id_fkey = <CommercialProductLot as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::name(
                self.commercial_bead_lots_id_fkey,
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
        self.commercial_bead_lots_id_fkey = <CommercialProductLot as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::description(
                self.commercial_bead_lots_id_fkey,
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
        self.commercial_bead_lots_id_fkey = <CommercialProductLot as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_by(
                self.commercial_bead_lots_id_fkey,
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
        self.commercial_bead_lots_id_fkey = <CommercialProductLot as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_at(
                self.commercial_bead_lots_id_fkey,
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
        self.commercial_bead_lots_id_fkey = <CommercialProductLot as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_by(
                self.commercial_bead_lots_id_fkey,
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
        self.commercial_bead_lots_id_fkey = <CommercialProductLot as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_at(
                self.commercial_bead_lots_id_fkey,
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
    CommercialProductLot,
    BeadModel: crate::codegen::structs_codegen::tables::insertables::BeadModelSettable<
            Error = web_common_traits::database::InsertError<
                crate::codegen::structs_codegen::tables::insertables::BeadModelAttribute,
            >,
        >,
> crate::codegen::structs_codegen::tables::insertables::BeadModelSettable
for InsertableCommercialBeadLotBuilder<CommercialProductLot, BeadModel>
where
    Self: common_traits::builder::Attributed<
        Attribute = crate::codegen::structs_codegen::tables::insertables::CommercialBeadLotAttribute,
    >,
{
    type Error = web_common_traits::database::InsertError<
        <Self as common_traits::builder::Attributed>::Attribute,
    >;
    #[inline]
    ///Sets the value of the `public.bead_models.diameter_millimeters` column.
    fn diameter_millimeters<DM>(
        mut self,
        diameter_millimeters: DM,
    ) -> Result<Self, Self::Error>
    where
        DM: TryInto<f32>,
        validation_errors::SingleFieldError: From<<DM as TryInto<f32>>::Error>,
    {
        self.commercial_bead_lots_id_fkey1 = <BeadModel as crate::codegen::structs_codegen::tables::insertables::BeadModelSettable>::diameter_millimeters(
                self.commercial_bead_lots_id_fkey1,
                diameter_millimeters,
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
    BeadModel,
> crate::codegen::structs_codegen::tables::insertables::CommercialProductLotSettable
for InsertableCommercialBeadLotBuilder<CommercialProductLot, BeadModel>
where
    Self: common_traits::builder::Attributed<
        Attribute = crate::codegen::structs_codegen::tables::insertables::CommercialBeadLotAttribute,
    >,
    Self: crate::codegen::structs_codegen::tables::insertables::CommercialBeadLotSettable<
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialBeadLotAttribute,
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
        self.commercial_bead_lots_id_fkey = <CommercialProductLot as crate::codegen::structs_codegen::tables::insertables::CommercialProductLotSettable>::lot(
                self.commercial_bead_lots_id_fkey,
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
    ///subgraph v3 ["`commercial_bead_lots`"]
    ///    v0@{shape: rounded, label: "product_model"}
    ///class v0 directly-involved-column
    ///end
    ///subgraph v4 ["`commercial_product_lots`"]
    ///    v1@{shape: rounded, label: "product_model"}
    ///class v1 column-of-interest
    ///end
    ///subgraph v5 ["`physical_asset_models`"]
    ///    v2@{shape: rounded, label: "parent_model"}
    ///class v2 undirectly-involved-column
    ///end
    ///v0 -.->|"`inferred ancestral same as`"| v1
    ///v0 -.->|"`inferred ancestral same as`"| v2
    ///v1 -.->|"`inferred ancestral same as`"| v2
    ///v3 --->|"`extends`"| v4
    ///v4 --->|"`extends`"| v5
    ///```
    fn product_model<PM>(self, product_model: PM) -> Result<Self, Self::Error>
    where
        PM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        <Self as CommercialBeadLotSettable>::product_model(self, product_model)
    }
}
impl<
    CommercialProductLot,
    BeadModel,
> crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelSettable
for InsertableCommercialBeadLotBuilder<CommercialProductLot, BeadModel>
where
    Self: common_traits::builder::Attributed<
        Attribute = crate::codegen::structs_codegen::tables::insertables::CommercialBeadLotAttribute,
    >,
    Self: crate::codegen::structs_codegen::tables::insertables::CommercialBeadLotSettable<
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialBeadLotAttribute,
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
    ///subgraph v5 ["`commercial_bead_lots`"]
    ///    v0@{shape: rounded, label: "product_model"}
    ///class v0 directly-involved-column
    ///end
    ///subgraph v6 ["`commercial_product_lots`"]
    ///    v3@{shape: rounded, label: "product_model"}
    ///class v3 undirectly-involved-column
    ///end
    ///subgraph v7 ["`physical_asset_models`"]
    ///    v1@{shape: rounded, label: "parent_model"}
    ///class v1 column-of-interest
    ///end
    ///v0 --->|"`ancestral same as`"| v2
    ///v0 -.->|"`inferred ancestral same as`"| v3
    ///v0 -.->|"`inferred ancestral same as`"| v1
    ///v3 --->|"`ancestral same as`"| v2
    ///v3 -.->|"`inferred ancestral same as`"| v1
    ///v1 --->|"`ancestral same as`"| v2
    ///v5 --->|"`extends`"| v6
    ///v6 --->|"`extends`"| v7
    ///v7 --->|"`extends`"| v4
    ///```
    fn parent_model<PM>(self, parent_model: PM) -> Result<Self, Self::Error>
    where
        PM: web_common_traits::database::MaybePrimaryKeyLike<PrimaryKey = i32>,
    {
        <Self as CommercialBeadLotSettable>::product_model(
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
impl<CommercialProductLot, BeadModel> web_common_traits::database::MostConcreteTable
    for InsertableCommercialBeadLotBuilder<CommercialProductLot, BeadModel>
where
    CommercialProductLot: web_common_traits::database::MostConcreteTable,
    BeadModel: web_common_traits::database::MostConcreteTable,
{
    fn set_most_concrete_table(&mut self, table_name: &str) {
        self.commercial_bead_lots_id_fkey.set_most_concrete_table(table_name);
        self.commercial_bead_lots_id_fkey1.set_most_concrete_table(table_name);
    }
}
impl<CommercialProductLot, BeadModel> web_common_traits::prelude::SetPrimaryKey
    for InsertableCommercialBeadLotBuilder<CommercialProductLot, BeadModel>
where
    CommercialProductLot: web_common_traits::prelude::SetPrimaryKey<PrimaryKey = i32>,
    BeadModel: web_common_traits::prelude::SetPrimaryKey<PrimaryKey = i32>,
{
    type PrimaryKey = i32;
    fn set_primary_key(mut self, primary_key: Self::PrimaryKey) -> Self {
        self.commercial_bead_lots_id_fkey =
            self.commercial_bead_lots_id_fkey.set_primary_key(primary_key);
        self.commercial_bead_lots_id_fkey1 =
            self.commercial_bead_lots_id_fkey1.set_primary_key(primary_key);
        self
    }
}
impl<CommercialProductLot, BeadModel, C> web_common_traits::database::TryInsertGeneric<C>
    for InsertableCommercialBeadLotBuilder<CommercialProductLot, BeadModel>
where
    Self: web_common_traits::database::DispatchableInsertableVariant<
            C,
            Row = crate::codegen::structs_codegen::tables::commercial_bead_lots::CommercialBeadLot,
            Error = web_common_traits::database::InsertError<CommercialBeadLotAttribute>,
        >,
    CommercialProductLot: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
    BeadModel: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
{
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<
        Self::PrimaryKey,
        web_common_traits::database::InsertError<CommercialBeadLotAttribute>,
    > {
        use diesel::Identifiable;
        use web_common_traits::database::DispatchableInsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::commercial_bead_lots::CommercialBeadLot = self
            .insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
