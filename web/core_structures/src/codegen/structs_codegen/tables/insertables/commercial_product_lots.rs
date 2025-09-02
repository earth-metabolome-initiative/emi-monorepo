#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableCommercialProductLotExtensionAttributes {
    PhysicalAssetModel(
        crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetModelAttributes,
    ),
}
impl core::fmt::Display for InsertableCommercialProductLotExtensionAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::PhysicalAssetModel(e) => write!(f, "{e}"),
        }
    }
}
impl From<
    crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetModelAttributes,
> for InsertableCommercialProductLotExtensionAttributes {
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetModelAttributes,
    ) -> Self {
        Self::PhysicalAssetModel(attribute)
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableCommercialProductLotAttributes {
    Extension(InsertableCommercialProductLotExtensionAttributes),
    Id,
    Lot,
    ProductModel,
}
impl core::str::FromStr for InsertableCommercialProductLotAttributes {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Lot" => Ok(Self::Lot),
            "ProductModel" => Ok(Self::ProductModel),
            "lot" => Ok(Self::Lot),
            "product_model" => Ok(Self::ProductModel),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl core::fmt::Display for InsertableCommercialProductLotAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::Id => write!(f, "id"),
            Self::Lot => write!(f, "lot"),
            Self::ProductModel => write!(f, "product_model"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::commercial_product_lots::commercial_product_lots
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableCommercialProductLot {
    pub(crate) id: i32,
    pub(crate) lot: String,
    pub(crate) product_model: i32,
}
impl InsertableCommercialProductLot {
    pub fn id<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel::table(),
                self.id,
            ),
            conn,
        )
    }
    pub fn product_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct::table(),
                self.product_model,
            ),
            conn,
        )
    }
}
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableCommercialProductLotBuilder<
    PhysicalAssetModel
        = crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetModelBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelBuilder,
        >,
> {
    pub(crate) lot: Option<String>,
    pub(crate) product_model: Option<i32>,
    pub(crate) id: PhysicalAssetModel,
}
/// Trait defining setters for attributes of an instance of
/// `CommercialProductLot` or descendant tables.
pub trait CommercialProductLotBuildable: Sized {
    /// Attributes required to build the insertable.
    type Attributes;
    /// Sets the value of the `public.commercial_product_lots.lot` column.
    ///
    /// # Arguments
    /// * `lot`: The value to set for the `public.commercial_product_lots.lot`
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
    /// * If the provided value cannot be converted to the required type
    ///   `String`.
    /// * If the provided value does not pass schema-defined validation.
    fn lot<L>(
        self,
        lot: L,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        L: TryInto<String>,
        validation_errors::SingleFieldError: From<<L as TryInto<String>>::Error>;
    /// Sets the value of the `public.commercial_product_lots.product_model`
    /// column.
    ///
    /// # Arguments
    /// * `product_model`: The value to set for the
    ///   `public.commercial_product_lots.product_model` column.
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
        product_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
}
impl<
    PhysicalAssetModel: crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetModelAttributes,
        >,
> CommercialProductLotBuildable
for InsertableCommercialProductLotBuilder<PhysicalAssetModel> {
    type Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductLotAttributes;
    ///Sets the value of the `public.commercial_product_lots.lot` column.
    fn lot<L>(
        mut self,
        lot: L,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        L: TryInto<String>,
        validation_errors::SingleFieldError: From<<L as TryInto<String>>::Error>,
    {
        let lot = lot
            .try_into()
            .map_err(|err| {
                validation_errors::SingleFieldError::from(err)
                    .rename_field(InsertableCommercialProductLotAttributes::Lot)
            })?;
        self.lot = Some(lot);
        Ok(self)
    }
    ///Sets the value of the `public.commercial_product_lots.product_model` column.
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
    ///classDef undirectly-involved-column stroke: #a7eff0,stroke-dasharray: 5, 5,fill: #d2f6f7
    ///subgraph v3 ["`asset_models`"]
    ///    v2@{shape: rounded, label: "parent_model"}
    ///class v2 undirectly-involved-column
    ///end
    ///subgraph v4 ["`commercial_product_lots`"]
    ///    v0@{shape: rounded, label: "product_model"}
    ///class v0 column-of-interest
    ///end
    ///subgraph v5 ["`physical_asset_models`"]
    ///    v1@{shape: rounded, label: "parent_model"}
    ///class v1 directly-involved-column
    ///end
    ///v0 --->|"`ancestral same as`"| v2
    ///v0 -.->|"`inferred ancestral same as`"| v1
    ///v1 --->|"`ancestral same as`"| v2
    ///v4 --->|"`extends`"| v5
    ///v4 -.->|"`descendant of`"| v3
    ///v5 --->|"`extends`"| v3
    ///```
    fn product_model(
        mut self,
        product_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        let product_model = product_model
            .try_into()
            .map_err(|err| {
                validation_errors::SingleFieldError::from(err)
                    .rename_field(InsertableCommercialProductLotAttributes::ProductModel)
            })?;
        self.id = <PhysicalAssetModel as crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelBuildable>::parent_model(
                self.id,
                Some(product_model),
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
    PhysicalAssetModel: crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetModelAttributes,
        >,
> crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable
for InsertableCommercialProductLotBuilder<PhysicalAssetModel>
where
    Self: crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelBuildable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductLotAttributes,
    >,
{
    type Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductLotAttributes;
    #[inline]
    ///Sets the value of the `public.asset_models.name` column.
    fn name<N>(
        mut self,
        name: N,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        N: TryInto<Option<String>>,
        validation_errors::SingleFieldError: From<<N as TryInto<Option<String>>>::Error>,
    {
        self.id = <PhysicalAssetModel as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::name(
                self.id,
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
        D: TryInto<Option<String>>,
        validation_errors::SingleFieldError: From<<D as TryInto<Option<String>>>::Error>,
    {
        self.id = <PhysicalAssetModel as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::description(
                self.id,
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
    ///flowchart LR
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
    fn parent_model(
        self,
        parent_model: Option<i32>,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        <Self as crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelBuildable>::parent_model(
            self,
            parent_model,
        )
    }
    #[inline]
    ///Sets the value of the `public.asset_models.created_by` column.
    fn created_by(
        mut self,
        created_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.id = <PhysicalAssetModel as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::created_by(
                self.id,
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
        self.id = <PhysicalAssetModel as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::created_at(
                self.id,
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
        self.id = <PhysicalAssetModel as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::updated_by(
                self.id,
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
        self.id = <PhysicalAssetModel as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::updated_at(
                self.id,
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
    PhysicalAssetModel,
> crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelBuildable
for InsertableCommercialProductLotBuilder<PhysicalAssetModel>
where
    Self: crate::codegen::structs_codegen::tables::insertables::CommercialProductLotBuildable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductLotAttributes,
    >,
{
    type Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductLotAttributes;
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
    ///flowchart LR
    ///classDef column-of-interest stroke: #f0746c,fill: #f49f9a
    ///classDef directly-involved-column stroke: #6c74f0,fill: #9a9ff4
    ///classDef undirectly-involved-column stroke: #a7eff0,stroke-dasharray: 5, 5,fill: #d2f6f7
    ///subgraph v3 ["`asset_models`"]
    ///    v2@{shape: rounded, label: "parent_model"}
    ///class v2 undirectly-involved-column
    ///end
    ///subgraph v4 ["`commercial_product_lots`"]
    ///    v1@{shape: rounded, label: "product_model"}
    ///class v1 directly-involved-column
    ///end
    ///subgraph v5 ["`physical_asset_models`"]
    ///    v0@{shape: rounded, label: "parent_model"}
    ///class v0 column-of-interest
    ///end
    ///v1 --->|"`ancestral same as`"| v2
    ///v1 -.->|"`inferred ancestral same as`"| v0
    ///v0 --->|"`ancestral same as`"| v2
    ///v4 --->|"`extends`"| v5
    ///v4 -.->|"`descendant of`"| v3
    ///v5 --->|"`extends`"| v3
    ///```
    fn parent_model(
        self,
        parent_model: Option<i32>,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        <Self as CommercialProductLotBuildable>::product_model(
            self,
            parent_model
                .ok_or(
                    common_traits::prelude::BuilderError::IncompleteBuild(
                        Self::Attributes::ProductModel,
                    ),
                )?,
        )
    }
}
impl<PhysicalAssetModel> web_common_traits::database::MostConcreteTable
    for InsertableCommercialProductLotBuilder<PhysicalAssetModel>
where
    PhysicalAssetModel: web_common_traits::database::MostConcreteTable,
{
    fn set_most_concrete_table(&mut self, table_name: &str) {
        self.id.set_most_concrete_table(table_name);
    }
}
impl<PhysicalAssetModel> web_common_traits::prelude::SetPrimaryKey
    for InsertableCommercialProductLotBuilder<PhysicalAssetModel>
where
    PhysicalAssetModel: web_common_traits::prelude::SetPrimaryKey<PrimaryKey = i32>,
{
    type PrimaryKey = i32;
    fn set_primary_key(mut self, primary_key: Self::PrimaryKey) -> Self {
        self.id = self.id.set_primary_key(primary_key);
        self
    }
}
impl<PhysicalAssetModel, C> web_common_traits::database::TryInsertGeneric<C>
for InsertableCommercialProductLotBuilder<PhysicalAssetModel>
where
    Self: web_common_traits::database::InsertableVariant<
        C,
        UserId = i32,
        Row = crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot,
        Error = web_common_traits::database::InsertError<
            InsertableCommercialProductLotAttributes,
        >,
    >,
    PhysicalAssetModel: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
{
    type Attributes = InsertableCommercialProductLotAttributes;
    fn is_complete(&self) -> bool {
        self.id.is_complete() && self.lot.is_some() && self.product_model.is_some()
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
        let insertable: crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot = self
            .insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
