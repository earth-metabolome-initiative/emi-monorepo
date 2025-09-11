#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CommercialFreezerLotExtensionAttribute {
    CommercialProductLot(
        crate::codegen::structs_codegen::tables::insertables::CommercialProductLotAttribute,
    ),
    FreezerModel(crate::codegen::structs_codegen::tables::insertables::FreezerModelAttribute),
}
impl core::fmt::Display for CommercialFreezerLotExtensionAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::CommercialProductLot(e) => write!(f, "{e}"),
            Self::FreezerModel(e) => write!(f, "{e}"),
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::CommercialProductLotAttribute>
    for CommercialFreezerLotExtensionAttribute
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::CommercialProductLotAttribute,
    ) -> Self {
        Self::CommercialProductLot(attribute)
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::FreezerModelAttribute>
    for CommercialFreezerLotExtensionAttribute
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::FreezerModelAttribute,
    ) -> Self {
        Self::FreezerModel(attribute)
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CommercialFreezerLotAttribute {
    Extension(CommercialFreezerLotExtensionAttribute),
    Id,
    ProductModel,
}
impl core::str::FromStr for CommercialFreezerLotAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ProductModel" => Ok(Self::ProductModel),
            "product_model" => Ok(Self::ProductModel),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl
    web_common_traits::database::DefaultExtensionAttribute<
        crate::codegen::structs_codegen::tables::insertables::CommercialProductLotAttribute,
    > for CommercialFreezerLotAttribute
{
    /// Returns the default value for the target attribute.
    fn target_default() -> Self {
        Self::Extension(
            crate::codegen::structs_codegen::tables::insertables::CommercialProductLotAttribute::Id
                .into(),
        )
    }
}
impl<PhysicalAssetModel>
    web_common_traits::database::FromExtensionAttribute<
        crate::codegen::structs_codegen::tables::insertables::CommercialProductLotAttribute,
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductLotBuilder<
            PhysicalAssetModel,
        >,
    > for CommercialFreezerLotAttribute
{
    type EffectiveExtensionAttribute =
        crate::codegen::structs_codegen::tables::insertables::CommercialProductLotAttribute;
    fn from_extension_attribute(extension_attribute: Self::EffectiveExtensionAttribute) -> Self {
        Self::Extension(extension_attribute.into())
    }
}
impl
    web_common_traits::database::DefaultExtensionAttribute<
        crate::codegen::structs_codegen::tables::insertables::FreezerModelAttribute,
    > for CommercialFreezerLotAttribute
{
    /// Returns the default value for the target attribute.
    fn target_default() -> Self {
        Self::Extension(
            crate::codegen::structs_codegen::tables::insertables::FreezerModelAttribute::Id.into(),
        )
    }
}
impl<PhysicalAssetModel>
    web_common_traits::database::FromExtensionAttribute<
        crate::codegen::structs_codegen::tables::insertables::FreezerModelAttribute,
        crate::codegen::structs_codegen::tables::insertables::InsertableFreezerModelBuilder<
            PhysicalAssetModel,
        >,
    > for CommercialFreezerLotAttribute
{
    type EffectiveExtensionAttribute =
        crate::codegen::structs_codegen::tables::insertables::FreezerModelAttribute;
    fn from_extension_attribute(extension_attribute: Self::EffectiveExtensionAttribute) -> Self {
        Self::Extension(extension_attribute.into())
    }
}
impl core::fmt::Display for CommercialFreezerLotAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::Id => write!(f, "commercial_freezer_lots.id"),
            Self::ProductModel => write!(f, "commercial_freezer_lots.product_model"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::commercial_freezer_lots::commercial_freezer_lots
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableCommercialFreezerLot {
    pub(crate) id: i32,
    pub(crate) product_model: i32,
}
impl InsertableCommercialFreezerLot {
    pub fn commercial_freezer_lots_id_fkey<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot::read(
            self.id, conn,
        )
    }
    pub fn commercial_freezer_lots_id_fkey1<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::freezer_models::FreezerModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::freezer_models::FreezerModel:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::freezer_models::FreezerModel::read(self.id, conn)
    }
    #[cfg(feature = "postgres")]
    pub fn commercial_freezer_lots_id_product_model_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::asset_models::AssetModel,
        diesel::result::Error,
    > {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::asset_models::AssetModel::table()
            .filter(
                crate::codegen::diesel_codegen::tables::asset_models::asset_models::dsl::id
                    .eq(&self.id)
                    .and(
                        crate::codegen::diesel_codegen::tables::asset_models::asset_models::dsl::parent_model
                            .eq(&self.product_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::asset_models::AssetModel,
            >(conn)
    }
    pub fn product_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::commercial_freezer_models::CommercialFreezerModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::commercial_freezer_models::CommercialFreezerModel:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::commercial_freezer_models::CommercialFreezerModel::read(
            self.product_model,
            conn,
        )
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Hash, Ord, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableCommercialFreezerLotBuilder<
    CommercialProductLot
        = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductLotBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetModelBuilder<
                crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelBuilder,
            >,
        >,
    FreezerModel
        = crate::codegen::structs_codegen::tables::insertables::InsertableFreezerModelBuilder<
            Option<i32>,
        >,
> {
    pub(crate) product_model: Option<i32>,
    pub(crate) commercial_freezer_lots_id_fkey: CommercialProductLot,
    pub(crate) commercial_freezer_lots_id_fkey1: FreezerModel,
}
impl From<InsertableCommercialFreezerLotBuilder>
    for web_common_traits::database::IdOrBuilder<i32, InsertableCommercialFreezerLotBuilder>
{
    fn from(builder: InsertableCommercialFreezerLotBuilder) -> Self {
        Self::Builder(builder)
    }
}
impl<CommercialProductLot, FreezerModel> common_traits::builder::IsCompleteBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezerLotBuilder<
        CommercialProductLot,
        FreezerModel,
    >
where
    CommercialProductLot: common_traits::builder::IsCompleteBuilder,
    FreezerModel: common_traits::builder::IsCompleteBuilder,
{
    fn is_complete(&self) -> bool {
        self.commercial_freezer_lots_id_fkey.is_complete()
            && self.commercial_freezer_lots_id_fkey1.is_complete()
            && self.product_model.is_some()
    }
}
/// Trait defining setters for attributes of an instance of
/// `CommercialFreezerLot` or descendant tables.
pub trait CommercialFreezerLotSettable: Sized {
    /// Attributes required to build the insertable.
    type Attributes;
    /// Sets the value of the `public.commercial_freezer_lots.product_model`
    /// column.
    ///
    /// # Arguments
    /// * `product_model`: The value to set for the
    ///   `public.commercial_freezer_lots.product_model` column.
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
    fn product_model<PM>(
        self,
        product_model: PM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
}
impl<
    CommercialProductLot: crate::codegen::structs_codegen::tables::insertables::CommercialProductLotSettable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::CommercialProductLotAttribute,
        >
        + crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelSettable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::CommercialProductLotAttribute,
        >,
    FreezerModel,
> CommercialFreezerLotSettable
for InsertableCommercialFreezerLotBuilder<CommercialProductLot, FreezerModel> {
    type Attributes = crate::codegen::structs_codegen::tables::insertables::CommercialFreezerLotAttribute;
    ///Sets the value of the `public.commercial_freezer_lots.product_model` column.
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
    ///subgraph v5 ["`commercial_freezer_lots`"]
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
    fn product_model<PM>(
        mut self,
        product_model: PM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let product_model = <PM as web_common_traits::database::PrimaryKeyLike>::primary_key(
            &product_model,
        );
        self.commercial_freezer_lots_id_fkey = <CommercialProductLot as crate::codegen::structs_codegen::tables::insertables::CommercialProductLotSettable>::product_model(
                self.commercial_freezer_lots_id_fkey,
                product_model,
            )
            .map_err(|err| {
                err.into_field_name(|attribute| Self::Attributes::Extension(
                    attribute.into(),
                ))
            })?;
        self.commercial_freezer_lots_id_fkey = <CommercialProductLot as crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelSettable>::parent_model(
                self.commercial_freezer_lots_id_fkey,
                product_model,
            )
            .map_err(|err| {
                err.into_field_name(|attribute| Self::Attributes::Extension(
                    attribute.into(),
                ))
            })?;
        self.product_model = Some(product_model);
        Ok(self)
    }
}
impl<
    CommercialProductLot: crate::codegen::structs_codegen::tables::insertables::AssetModelSettable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::CommercialProductLotAttribute,
        >,
    FreezerModel,
> crate::codegen::structs_codegen::tables::insertables::AssetModelSettable
for InsertableCommercialFreezerLotBuilder<CommercialProductLot, FreezerModel>
where
    Self: crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelSettable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::CommercialFreezerLotAttribute,
    >,
{
    type Attributes = crate::codegen::structs_codegen::tables::insertables::CommercialFreezerLotAttribute;
    #[inline]
    ///Sets the value of the `public.asset_models.name` column.
    fn name<N>(
        mut self,
        name: N,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        N: TryInto<String>,
        validation_errors::SingleFieldError: From<<N as TryInto<String>>::Error>,
    {
        self.commercial_freezer_lots_id_fkey = <CommercialProductLot as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::name(
                self.commercial_freezer_lots_id_fkey,
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
    fn description<D>(
        mut self,
        description: D,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        D: TryInto<String>,
        validation_errors::SingleFieldError: From<<D as TryInto<String>>::Error>,
    {
        self.commercial_freezer_lots_id_fkey = <CommercialProductLot as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::description(
                self.commercial_freezer_lots_id_fkey,
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
    fn parent_model<PM>(
        self,
        parent_model: PM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
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
    fn created_by<CB>(
        mut self,
        created_by: CB,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        CB: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        self.commercial_freezer_lots_id_fkey = <CommercialProductLot as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_by(
                self.commercial_freezer_lots_id_fkey,
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
    fn created_at<CA>(
        mut self,
        created_at: CA,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        CA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::SingleFieldError: From<
            <CA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error,
        >,
    {
        self.commercial_freezer_lots_id_fkey = <CommercialProductLot as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_at(
                self.commercial_freezer_lots_id_fkey,
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
    fn updated_by<UB>(
        mut self,
        updated_by: UB,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        UB: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        self.commercial_freezer_lots_id_fkey = <CommercialProductLot as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_by(
                self.commercial_freezer_lots_id_fkey,
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
    fn updated_at<UA>(
        mut self,
        updated_at: UA,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        UA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::SingleFieldError: From<
            <UA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error,
        >,
    {
        self.commercial_freezer_lots_id_fkey = <CommercialProductLot as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_at(
                self.commercial_freezer_lots_id_fkey,
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
    CommercialProductLot: crate::codegen::structs_codegen::tables::insertables::CommercialProductLotSettable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::CommercialProductLotAttribute,
        >,
    FreezerModel,
> crate::codegen::structs_codegen::tables::insertables::CommercialProductLotSettable
for InsertableCommercialFreezerLotBuilder<CommercialProductLot, FreezerModel>
where
    Self: crate::codegen::structs_codegen::tables::insertables::CommercialFreezerLotSettable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::CommercialFreezerLotAttribute,
    >,
{
    type Attributes = crate::codegen::structs_codegen::tables::insertables::CommercialFreezerLotAttribute;
    #[inline]
    ///Sets the value of the `public.commercial_product_lots.lot` column.
    fn lot<L>(
        mut self,
        lot: L,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        L: TryInto<String>,
        validation_errors::SingleFieldError: From<<L as TryInto<String>>::Error>,
    {
        self.commercial_freezer_lots_id_fkey = <CommercialProductLot as crate::codegen::structs_codegen::tables::insertables::CommercialProductLotSettable>::lot(
                self.commercial_freezer_lots_id_fkey,
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
    ///subgraph v3 ["`commercial_freezer_lots`"]
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
    fn product_model<PM>(
        self,
        product_model: PM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        <Self as CommercialFreezerLotSettable>::product_model(self, product_model)
    }
}
impl<CommercialProductLot, FreezerModel>
    crate::codegen::structs_codegen::tables::insertables::FreezerModelSettable
    for InsertableCommercialFreezerLotBuilder<CommercialProductLot, FreezerModel>
{
    type Attributes =
        crate::codegen::structs_codegen::tables::insertables::CommercialFreezerLotAttribute;
}
impl<
    CommercialProductLot,
    FreezerModel,
> crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelSettable
for InsertableCommercialFreezerLotBuilder<CommercialProductLot, FreezerModel>
where
    Self: crate::codegen::structs_codegen::tables::insertables::CommercialFreezerLotSettable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::CommercialFreezerLotAttribute,
    >,
{
    type Attributes = crate::codegen::structs_codegen::tables::insertables::CommercialFreezerLotAttribute;
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
    ///subgraph v5 ["`commercial_freezer_lots`"]
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
    fn parent_model<PM>(
        self,
        parent_model: PM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PM: web_common_traits::database::MaybePrimaryKeyLike<PrimaryKey = i32>,
    {
        <Self as CommercialFreezerLotSettable>::product_model(
            self,
            <PM as web_common_traits::database::MaybePrimaryKeyLike>::maybe_primary_key(
                    &parent_model,
                )
                .ok_or(
                    common_traits::prelude::BuilderError::IncompleteBuild(
                        Self::Attributes::ProductModel,
                    ),
                )?,
        )
    }
}
impl<CommercialProductLot, FreezerModel> web_common_traits::database::MostConcreteTable
    for InsertableCommercialFreezerLotBuilder<CommercialProductLot, FreezerModel>
where
    CommercialProductLot: web_common_traits::database::MostConcreteTable,
    FreezerModel: web_common_traits::database::MostConcreteTable,
{
    fn set_most_concrete_table(&mut self, table_name: &str) {
        self.commercial_freezer_lots_id_fkey.set_most_concrete_table(table_name);
        self.commercial_freezer_lots_id_fkey1.set_most_concrete_table(table_name);
    }
}
impl<CommercialProductLot, FreezerModel> web_common_traits::prelude::SetPrimaryKey
    for InsertableCommercialFreezerLotBuilder<CommercialProductLot, FreezerModel>
where
    CommercialProductLot: web_common_traits::prelude::SetPrimaryKey<PrimaryKey = i32>,
    FreezerModel: web_common_traits::prelude::SetPrimaryKey<PrimaryKey = i32>,
{
    type PrimaryKey = i32;
    fn set_primary_key(mut self, primary_key: Self::PrimaryKey) -> Self {
        self.commercial_freezer_lots_id_fkey =
            self.commercial_freezer_lots_id_fkey.set_primary_key(primary_key);
        self.commercial_freezer_lots_id_fkey1 =
            self.commercial_freezer_lots_id_fkey1.set_primary_key(primary_key);
        self
    }
}
impl<
    CommercialProductLot,
    FreezerModel,
    C,
> web_common_traits::database::TryInsertGeneric<C>
for InsertableCommercialFreezerLotBuilder<CommercialProductLot, FreezerModel>
where
    Self: web_common_traits::database::InsertableVariant<
        C,
        UserId = i32,
        Row = crate::codegen::structs_codegen::tables::commercial_freezer_lots::CommercialFreezerLot,
        Error = web_common_traits::database::InsertError<CommercialFreezerLotAttribute>,
    >,
    CommercialProductLot: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
    FreezerModel: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
{
    type Attribute = CommercialFreezerLotAttribute;
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<
        Self::PrimaryKey,
        web_common_traits::database::InsertError<Self::Attribute>,
    > {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::commercial_freezer_lots::CommercialFreezerLot = self
            .insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
