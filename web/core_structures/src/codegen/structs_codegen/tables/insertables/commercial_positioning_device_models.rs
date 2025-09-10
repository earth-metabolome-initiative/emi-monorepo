#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CommercialPositioningDeviceModelExtensionAttribute {
    PositioningDeviceModel(
        crate::codegen::structs_codegen::tables::insertables::PositioningDeviceModelAttribute,
    ),
    CommercialProduct(
        crate::codegen::structs_codegen::tables::insertables::CommercialProductAttribute,
    ),
}
impl core::fmt::Display for CommercialPositioningDeviceModelExtensionAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::PositioningDeviceModel(e) => write!(f, "{e}"),
            Self::CommercialProduct(e) => write!(f, "{e}"),
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::PositioningDeviceModelAttribute>
    for CommercialPositioningDeviceModelExtensionAttribute
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::PositioningDeviceModelAttribute,
    ) -> Self {
        Self::PositioningDeviceModel(attribute)
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::CommercialProductAttribute>
    for CommercialPositioningDeviceModelExtensionAttribute
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::CommercialProductAttribute,
    ) -> Self {
        Self::CommercialProduct(attribute)
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CommercialPositioningDeviceModelAttribute {
    Extension(CommercialPositioningDeviceModelExtensionAttribute),
    Id,
    PositioningDeviceModel,
}
impl core::str::FromStr for CommercialPositioningDeviceModelAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PositioningDeviceModel" => Ok(Self::PositioningDeviceModel),
            "positioning_device_model" => Ok(Self::PositioningDeviceModel),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl core::fmt::Display for CommercialPositioningDeviceModelAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::Id => write!(f, "commercial_positioning_device_models.id"),
            Self::PositioningDeviceModel => {
                write!(f, "commercial_positioning_device_models.positioning_device_model")
            }
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::commercial_positioning_device_models::commercial_positioning_device_models
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableCommercialPositioningDeviceModel {
    pub(crate) id: i32,
    pub(crate) positioning_device_model: i32,
}
impl InsertableCommercialPositioningDeviceModel {
    pub fn positioning_device_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::positioning_device_models::PositioningDeviceModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::positioning_device_models::PositioningDeviceModel:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::positioning_device_models::PositioningDeviceModel::read(
            self.positioning_device_model,
            conn,
        )
    }
    pub fn commercial_positioning_device_models_id_fkey<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::positioning_device_models::PositioningDeviceModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::positioning_device_models::PositioningDeviceModel:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::positioning_device_models::PositioningDeviceModel::read(
            self.id,
            conn,
        )
    }
    pub fn commercial_positioning_device_models_id_fkey1<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct::read(
            self.id, conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn commercial_positioning_device_id_positioning_device_model_fkey(
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
                            .eq(&self.positioning_device_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::asset_models::AssetModel,
            >(conn)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Hash, Ord, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableCommercialPositioningDeviceModelBuilder<
    CommercialProduct
        = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelBuilder,
        >,
    PositioningDeviceModel
        = crate::codegen::structs_codegen::tables::insertables::InsertablePositioningDeviceModelBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetModelBuilder<
                Option<i32>,
            >,
        >,
> {
    pub(crate) positioning_device_model: Option<i32>,
    pub(crate) commercial_positioning_device_models_id_fkey: PositioningDeviceModel,
    pub(crate) commercial_positioning_device_models_id_fkey1: CommercialProduct,
}
impl From<InsertableCommercialPositioningDeviceModelBuilder>
    for web_common_traits::database::IdOrBuilder<
        i32,
        InsertableCommercialPositioningDeviceModelBuilder,
    >
{
    fn from(builder: InsertableCommercialPositioningDeviceModelBuilder) -> Self {
        Self::Builder(builder)
    }
}
impl<PositioningDeviceModel, CommercialProduct> common_traits::builder::IsCompleteBuilder
for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPositioningDeviceModelBuilder<
    PositioningDeviceModel,
    CommercialProduct,
>
where
    CommercialProduct: common_traits::builder::IsCompleteBuilder,
    PositioningDeviceModel: common_traits::builder::IsCompleteBuilder,
{
    fn is_complete(&self) -> bool {
        self.commercial_positioning_device_models_id_fkey.is_complete()
            && self.commercial_positioning_device_models_id_fkey1.is_complete()
            && self.positioning_device_model.is_some()
    }
}
/// Trait defining setters for attributes of an instance of
/// `CommercialPositioningDeviceModel` or descendant tables.
pub trait CommercialPositioningDeviceModelSettable: Sized {
    /// Attributes required to build the insertable.
    type Attributes;
    /// Sets the value of the
    /// `public.commercial_positioning_device_models.positioning_device_model`
    /// column.
    ///
    /// # Arguments
    /// * `positioning_device_model`: The value to set for the
    ///   `public.commercial_positioning_device_models.positioning_device_model`
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
    fn positioning_device_model(
        self,
        positioning_device_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
}
impl<
    CommercialProduct,
    PositioningDeviceModel: crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelSettable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::PositioningDeviceModelAttribute,
        >,
> CommercialPositioningDeviceModelSettable
for InsertableCommercialPositioningDeviceModelBuilder<
    CommercialProduct,
    PositioningDeviceModel,
> {
    type Attributes = crate::codegen::structs_codegen::tables::insertables::CommercialPositioningDeviceModelAttribute;
    ///Sets the value of the `public.commercial_positioning_device_models.positioning_device_model` column.
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
    ///subgraph v4 ["`commercial_positioning_device_models`"]
    ///    v0@{shape: rounded, label: "positioning_device_model"}
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
    fn positioning_device_model(
        mut self,
        positioning_device_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.commercial_positioning_device_models_id_fkey = <PositioningDeviceModel as crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelSettable>::parent_model(
                self.commercial_positioning_device_models_id_fkey,
                Some(positioning_device_model),
            )
            .map_err(|err| {
                err.into_field_name(|attribute| Self::Attributes::Extension(
                    attribute.into(),
                ))
            })?;
        self.positioning_device_model = Some(positioning_device_model);
        Ok(self)
    }
}
impl<
    CommercialProduct: crate::codegen::structs_codegen::tables::insertables::AssetModelSettable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::CommercialProductAttribute,
        >,
    PositioningDeviceModel,
> crate::codegen::structs_codegen::tables::insertables::AssetModelSettable
for InsertableCommercialPositioningDeviceModelBuilder<
    CommercialProduct,
    PositioningDeviceModel,
>
where
    Self: crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelSettable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::CommercialPositioningDeviceModelAttribute,
    >,
{
    type Attributes = crate::codegen::structs_codegen::tables::insertables::CommercialPositioningDeviceModelAttribute;
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
        self.commercial_positioning_device_models_id_fkey1 = <CommercialProduct as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::name(
                self.commercial_positioning_device_models_id_fkey1,
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
        self.commercial_positioning_device_models_id_fkey1 = <CommercialProduct as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::description(
                self.commercial_positioning_device_models_id_fkey1,
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
    fn parent_model(
        self,
        parent_model: Option<i32>,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        <Self as crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelSettable>::parent_model(
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
        self.commercial_positioning_device_models_id_fkey1 = <CommercialProduct as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_by(
                self.commercial_positioning_device_models_id_fkey1,
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
        self.commercial_positioning_device_models_id_fkey1 = <CommercialProduct as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_at(
                self.commercial_positioning_device_models_id_fkey1,
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
        self.commercial_positioning_device_models_id_fkey1 = <CommercialProduct as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_by(
                self.commercial_positioning_device_models_id_fkey1,
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
        self.commercial_positioning_device_models_id_fkey1 = <CommercialProduct as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_at(
                self.commercial_positioning_device_models_id_fkey1,
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
    CommercialProduct: crate::codegen::structs_codegen::tables::insertables::CommercialProductSettable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::CommercialProductAttribute,
        >,
    PositioningDeviceModel,
> crate::codegen::structs_codegen::tables::insertables::CommercialProductSettable
for InsertableCommercialPositioningDeviceModelBuilder<
    CommercialProduct,
    PositioningDeviceModel,
> {
    type Attributes = crate::codegen::structs_codegen::tables::insertables::CommercialPositioningDeviceModelAttribute;
    #[inline]
    ///Sets the value of the `public.commercial_products.deprecation_date` column.
    fn deprecation_date<DD>(
        mut self,
        deprecation_date: DD,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        DD: TryInto<Option<::rosetta_timestamp::TimestampUTC>>,
        validation_errors::SingleFieldError: From<
            <DD as TryInto<Option<::rosetta_timestamp::TimestampUTC>>>::Error,
        >,
    {
        self.commercial_positioning_device_models_id_fkey1 = <CommercialProduct as crate::codegen::structs_codegen::tables::insertables::CommercialProductSettable>::deprecation_date(
                self.commercial_positioning_device_models_id_fkey1,
                deprecation_date,
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
    ///Sets the value of the `public.commercial_products.brand_id` column.
    fn brand(
        mut self,
        brand_id: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.commercial_positioning_device_models_id_fkey1 = <CommercialProduct as crate::codegen::structs_codegen::tables::insertables::CommercialProductSettable>::brand(
                self.commercial_positioning_device_models_id_fkey1,
                brand_id,
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
    CommercialProduct,
    PositioningDeviceModel,
> crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelSettable
for InsertableCommercialPositioningDeviceModelBuilder<
    CommercialProduct,
    PositioningDeviceModel,
>
where
    Self: crate::codegen::structs_codegen::tables::insertables::CommercialPositioningDeviceModelSettable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::CommercialPositioningDeviceModelAttribute,
    >,
{
    type Attributes = crate::codegen::structs_codegen::tables::insertables::CommercialPositioningDeviceModelAttribute;
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
    ///subgraph v4 ["`commercial_positioning_device_models`"]
    ///    v0@{shape: rounded, label: "positioning_device_model"}
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
    fn parent_model(
        self,
        parent_model: Option<i32>,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        <Self as CommercialPositioningDeviceModelSettable>::positioning_device_model(
            self,
            parent_model
                .ok_or(
                    common_traits::prelude::BuilderError::IncompleteBuild(
                        Self::Attributes::PositioningDeviceModel,
                    ),
                )?,
        )
    }
}
impl<CommercialProduct, PositioningDeviceModel>
    crate::codegen::structs_codegen::tables::insertables::PositioningDeviceModelSettable
    for InsertableCommercialPositioningDeviceModelBuilder<CommercialProduct, PositioningDeviceModel>
{
    type Attributes = crate::codegen::structs_codegen::tables::insertables::CommercialPositioningDeviceModelAttribute;
}
impl<PositioningDeviceModel, CommercialProduct> web_common_traits::database::MostConcreteTable
    for InsertableCommercialPositioningDeviceModelBuilder<PositioningDeviceModel, CommercialProduct>
where
    PositioningDeviceModel: web_common_traits::database::MostConcreteTable,
    CommercialProduct: web_common_traits::database::MostConcreteTable,
{
    fn set_most_concrete_table(&mut self, table_name: &str) {
        self.commercial_positioning_device_models_id_fkey.set_most_concrete_table(table_name);
        self.commercial_positioning_device_models_id_fkey1.set_most_concrete_table(table_name);
    }
}
impl<PositioningDeviceModel, CommercialProduct> web_common_traits::prelude::SetPrimaryKey
    for InsertableCommercialPositioningDeviceModelBuilder<PositioningDeviceModel, CommercialProduct>
where
    PositioningDeviceModel: web_common_traits::prelude::SetPrimaryKey<PrimaryKey = i32>,
    CommercialProduct: web_common_traits::prelude::SetPrimaryKey<PrimaryKey = i32>,
{
    type PrimaryKey = i32;
    fn set_primary_key(mut self, primary_key: Self::PrimaryKey) -> Self {
        self.commercial_positioning_device_models_id_fkey =
            self.commercial_positioning_device_models_id_fkey.set_primary_key(primary_key);
        self.commercial_positioning_device_models_id_fkey1 =
            self.commercial_positioning_device_models_id_fkey1.set_primary_key(primary_key);
        self
    }
}
impl<
    CommercialProduct,
    PositioningDeviceModel,
    C,
> web_common_traits::database::TryInsertGeneric<C>
for InsertableCommercialPositioningDeviceModelBuilder<
    CommercialProduct,
    PositioningDeviceModel,
>
where
    Self: web_common_traits::database::InsertableVariant<
        C,
        UserId = i32,
        Row = crate::codegen::structs_codegen::tables::commercial_positioning_device_models::CommercialPositioningDeviceModel,
        Error = web_common_traits::database::InsertError<
            CommercialPositioningDeviceModelAttribute,
        >,
    >,
    CommercialProduct: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
    PositioningDeviceModel: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
{
    type Attributes = CommercialPositioningDeviceModelAttribute;
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
        let insertable: crate::codegen::structs_codegen::tables::commercial_positioning_device_models::CommercialPositioningDeviceModel = self
            .insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
