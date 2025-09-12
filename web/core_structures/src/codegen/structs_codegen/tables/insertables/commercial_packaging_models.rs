#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CommercialPackagingModelExtensionAttribute {
    PackagingModel(crate::codegen::structs_codegen::tables::insertables::PackagingModelAttribute),
    CommercialProduct(
        crate::codegen::structs_codegen::tables::insertables::CommercialProductAttribute,
    ),
}
impl core::fmt::Display for CommercialPackagingModelExtensionAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::PackagingModel(e) => write!(f, "{e}"),
            Self::CommercialProduct(e) => write!(f, "{e}"),
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::PackagingModelAttribute>
    for CommercialPackagingModelExtensionAttribute
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::PackagingModelAttribute,
    ) -> Self {
        Self::PackagingModel(attribute)
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::CommercialProductAttribute>
    for CommercialPackagingModelExtensionAttribute
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::CommercialProductAttribute,
    ) -> Self {
        Self::CommercialProduct(attribute)
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CommercialPackagingModelAttribute {
    Extension(CommercialPackagingModelExtensionAttribute),
    Id,
    PackagingModel,
}
impl core::str::FromStr for CommercialPackagingModelAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PackagingModel" => Ok(Self::PackagingModel),
            "packaging_model" => Ok(Self::PackagingModel),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl
    web_common_traits::database::DefaultExtensionAttribute<
        crate::codegen::structs_codegen::tables::insertables::PackagingModelAttribute,
    > for CommercialPackagingModelAttribute
{
    /// Returns the default value for the target attribute.
    fn target_default() -> Self {
        Self::Extension(
            crate::codegen::structs_codegen::tables::insertables::PackagingModelAttribute::Id
                .into(),
        )
    }
}
impl<PhysicalAssetModel>
    web_common_traits::database::FromExtensionAttribute<
        crate::codegen::structs_codegen::tables::insertables::PackagingModelAttribute,
        crate::codegen::structs_codegen::tables::insertables::InsertablePackagingModelBuilder<
            PhysicalAssetModel,
        >,
    > for CommercialPackagingModelAttribute
{
    type EffectiveExtensionAttribute =
        crate::codegen::structs_codegen::tables::insertables::PackagingModelAttribute;
    fn from_extension_attribute(extension_attribute: Self::EffectiveExtensionAttribute) -> Self {
        Self::Extension(extension_attribute.into())
    }
}
impl
    web_common_traits::database::DefaultExtensionAttribute<
        crate::codegen::structs_codegen::tables::insertables::CommercialProductAttribute,
    > for CommercialPackagingModelAttribute
{
    /// Returns the default value for the target attribute.
    fn target_default() -> Self {
        Self::Extension(
            crate::codegen::structs_codegen::tables::insertables::CommercialProductAttribute::Id
                .into(),
        )
    }
}
impl<AssetModel>
    web_common_traits::database::FromExtensionAttribute<
        crate::codegen::structs_codegen::tables::insertables::CommercialProductAttribute,
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductBuilder<
            AssetModel,
        >,
    > for CommercialPackagingModelAttribute
{
    type EffectiveExtensionAttribute =
        crate::codegen::structs_codegen::tables::insertables::CommercialProductAttribute;
    fn from_extension_attribute(extension_attribute: Self::EffectiveExtensionAttribute) -> Self {
        Self::Extension(extension_attribute.into())
    }
}
impl core::fmt::Display for CommercialPackagingModelAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::Id => write!(f, "commercial_packaging_models.id"),
            Self::PackagingModel => {
                write!(f, "commercial_packaging_models.packaging_model")
            }
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::commercial_packaging_models::commercial_packaging_models
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableCommercialPackagingModel {
    pub(crate) id: i32,
    pub(crate) packaging_model: i32,
}
impl InsertableCommercialPackagingModel {
    pub fn commercial_packaging_models_id_fkey<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::packaging_models::PackagingModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::packaging_models::PackagingModel:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::packaging_models::PackagingModel::read(
            self.id, conn,
        )
    }
    pub fn commercial_packaging_models_id_fkey1<C: diesel::connection::LoadConnection>(
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
    pub fn commercial_packaging_models_id_packaging_model_fkey(
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
                            .eq(&self.packaging_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::asset_models::AssetModel,
            >(conn)
    }
    pub fn packaging_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::packaging_models::PackagingModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::packaging_models::PackagingModel:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::packaging_models::PackagingModel::read(
            self.packaging_model,
            conn,
        )
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Hash, Ord, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Builder for creating and inserting a new
/// [`CommercialPackagingModel`](crate::codegen::structs_codegen::tables::commercial_packaging_models::CommercialPackagingModel).
///
///
/// # Implementation details
/// While this builder implements several methods, a reasonably complete
/// **basic** usage example (*which may not apply to your own specific use case,
/// please adapt accordingly*) is as follows:
///
/// ```rust,ignore
/// use core_structures::CommercialPackagingModel;
/// use core_structures::tables::insertables::AssetModelSettable;
/// use core_structures::tables::insertables::CommercialPackagingModelSettable;
/// use core_structures::tables::insertables::CommercialProductSettable;
/// use web_common_traits::database::Insertable;
/// use web_common_traits::database::InsertableVariant;
///
/// let commercial_packaging_model = CommercialPackagingModel::new()
///    // Set mandatory fields
///    .created_by(created_by)?
///    .description(description)?
///    .name(name)?
///    // Note: `updated_by` is automatically set by the `created by` column.
///    .updated_by(updated_by)?
///    .packaging_model(packaging_model)?
///    .brand(brand_id)?
///    // Optionally set fields with default values
///    .created_at(created_at)?
///    .updated_at(updated_at)?
///    // Optionally set optional fields
///    .parent_model(parent_model)?
///    .deprecation_date(deprecation_date)?
///    // Finally, insert the new record in the database
///    .insert(user.id, conn)?;
/// ```
pub struct InsertableCommercialPackagingModelBuilder<
    CommercialProduct
        = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelBuilder,
        >,
    PackagingModel
        = crate::codegen::structs_codegen::tables::insertables::InsertablePackagingModelBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetModelBuilder<
                Option<i32>,
            >,
        >,
> {
    pub(crate) packaging_model: Option<i32>,
    pub(crate) commercial_packaging_models_id_fkey: PackagingModel,
    pub(crate) commercial_packaging_models_id_fkey1: CommercialProduct,
}
impl From<InsertableCommercialPackagingModelBuilder>
    for web_common_traits::database::IdOrBuilder<i32, InsertableCommercialPackagingModelBuilder>
{
    fn from(builder: InsertableCommercialPackagingModelBuilder) -> Self {
        Self::Builder(builder)
    }
}
impl<PackagingModel, CommercialProduct> common_traits::builder::IsCompleteBuilder
for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPackagingModelBuilder<
    PackagingModel,
    CommercialProduct,
>
where
    CommercialProduct: common_traits::builder::IsCompleteBuilder,
    PackagingModel: common_traits::builder::IsCompleteBuilder,
{
    fn is_complete(&self) -> bool {
        self.commercial_packaging_models_id_fkey.is_complete()
            && self.commercial_packaging_models_id_fkey1.is_complete()
            && self.packaging_model.is_some()
    }
}
/// Trait defining setters for attributes of an instance of
/// `CommercialPackagingModel` or descendant tables.
pub trait CommercialPackagingModelSettable: Sized {
    /// Attributes required to build the insertable.
    type Attributes;
    /// Sets the value of the
    /// `public.commercial_packaging_models.packaging_model` column.
    ///
    /// # Arguments
    /// * `packaging_model`: The value to set for the
    ///   `public.commercial_packaging_models.packaging_model` column.
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
    fn packaging_model<PM>(
        self,
        packaging_model: PM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
}
impl<
    CommercialProduct,
    PackagingModel: crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelSettable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::PackagingModelAttribute,
        >,
> CommercialPackagingModelSettable
for InsertableCommercialPackagingModelBuilder<CommercialProduct, PackagingModel> {
    type Attributes = crate::codegen::structs_codegen::tables::insertables::CommercialPackagingModelAttribute;
    ///Sets the value of the `public.commercial_packaging_models.packaging_model` column.
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
    ///subgraph v4 ["`commercial_packaging_models`"]
    ///    v0@{shape: rounded, label: "packaging_model"}
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
    fn packaging_model<PM>(
        mut self,
        packaging_model: PM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let packaging_model = <PM as web_common_traits::database::PrimaryKeyLike>::primary_key(
            &packaging_model,
        );
        self.commercial_packaging_models_id_fkey = <PackagingModel as crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelSettable>::parent_model(
                self.commercial_packaging_models_id_fkey,
                packaging_model,
            )
            .map_err(|err| {
                err.into_field_name(|attribute| Self::Attributes::Extension(
                    attribute.into(),
                ))
            })?;
        self.packaging_model = Some(packaging_model);
        Ok(self)
    }
}
impl<
    CommercialProduct: crate::codegen::structs_codegen::tables::insertables::AssetModelSettable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::CommercialProductAttribute,
        >,
    PackagingModel,
> crate::codegen::structs_codegen::tables::insertables::AssetModelSettable
for InsertableCommercialPackagingModelBuilder<CommercialProduct, PackagingModel>
where
    Self: crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelSettable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::CommercialPackagingModelAttribute,
    >,
{
    type Attributes = crate::codegen::structs_codegen::tables::insertables::CommercialPackagingModelAttribute;
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
        self.commercial_packaging_models_id_fkey1 = <CommercialProduct as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::name(
                self.commercial_packaging_models_id_fkey1,
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
        self.commercial_packaging_models_id_fkey1 = <CommercialProduct as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::description(
                self.commercial_packaging_models_id_fkey1,
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
        self.commercial_packaging_models_id_fkey1 = <CommercialProduct as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_by(
                self.commercial_packaging_models_id_fkey1,
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
        self.commercial_packaging_models_id_fkey1 = <CommercialProduct as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_at(
                self.commercial_packaging_models_id_fkey1,
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
        self.commercial_packaging_models_id_fkey1 = <CommercialProduct as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_by(
                self.commercial_packaging_models_id_fkey1,
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
        self.commercial_packaging_models_id_fkey1 = <CommercialProduct as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_at(
                self.commercial_packaging_models_id_fkey1,
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
    PackagingModel,
> crate::codegen::structs_codegen::tables::insertables::CommercialProductSettable
for InsertableCommercialPackagingModelBuilder<CommercialProduct, PackagingModel> {
    type Attributes = crate::codegen::structs_codegen::tables::insertables::CommercialPackagingModelAttribute;
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
        self.commercial_packaging_models_id_fkey1 = <CommercialProduct as crate::codegen::structs_codegen::tables::insertables::CommercialProductSettable>::deprecation_date(
                self.commercial_packaging_models_id_fkey1,
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
    fn brand<BI>(
        mut self,
        brand_id: BI,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        BI: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        self.commercial_packaging_models_id_fkey1 = <CommercialProduct as crate::codegen::structs_codegen::tables::insertables::CommercialProductSettable>::brand(
                self.commercial_packaging_models_id_fkey1,
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
impl<CommercialProduct, PackagingModel>
    crate::codegen::structs_codegen::tables::insertables::PackagingModelSettable
    for InsertableCommercialPackagingModelBuilder<CommercialProduct, PackagingModel>
{
    type Attributes =
        crate::codegen::structs_codegen::tables::insertables::CommercialPackagingModelAttribute;
}
impl<
    CommercialProduct,
    PackagingModel,
> crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelSettable
for InsertableCommercialPackagingModelBuilder<CommercialProduct, PackagingModel>
where
    Self: crate::codegen::structs_codegen::tables::insertables::CommercialPackagingModelSettable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::CommercialPackagingModelAttribute,
    >,
{
    type Attributes = crate::codegen::structs_codegen::tables::insertables::CommercialPackagingModelAttribute;
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
    ///subgraph v4 ["`commercial_packaging_models`"]
    ///    v0@{shape: rounded, label: "packaging_model"}
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
    fn parent_model<PM>(
        self,
        parent_model: PM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PM: web_common_traits::database::MaybePrimaryKeyLike<PrimaryKey = i32>,
    {
        <Self as CommercialPackagingModelSettable>::packaging_model(
            self,
            <PM as web_common_traits::database::MaybePrimaryKeyLike>::maybe_primary_key(
                    &parent_model,
                )
                .ok_or(
                    common_traits::prelude::BuilderError::IncompleteBuild(
                        Self::Attributes::PackagingModel,
                    ),
                )?,
        )
    }
}
impl<PackagingModel, CommercialProduct> web_common_traits::database::MostConcreteTable
    for InsertableCommercialPackagingModelBuilder<PackagingModel, CommercialProduct>
where
    PackagingModel: web_common_traits::database::MostConcreteTable,
    CommercialProduct: web_common_traits::database::MostConcreteTable,
{
    fn set_most_concrete_table(&mut self, table_name: &str) {
        self.commercial_packaging_models_id_fkey.set_most_concrete_table(table_name);
        self.commercial_packaging_models_id_fkey1.set_most_concrete_table(table_name);
    }
}
impl<PackagingModel, CommercialProduct> web_common_traits::prelude::SetPrimaryKey
    for InsertableCommercialPackagingModelBuilder<PackagingModel, CommercialProduct>
where
    PackagingModel: web_common_traits::prelude::SetPrimaryKey<PrimaryKey = i32>,
    CommercialProduct: web_common_traits::prelude::SetPrimaryKey<PrimaryKey = i32>,
{
    type PrimaryKey = i32;
    fn set_primary_key(mut self, primary_key: Self::PrimaryKey) -> Self {
        self.commercial_packaging_models_id_fkey =
            self.commercial_packaging_models_id_fkey.set_primary_key(primary_key);
        self.commercial_packaging_models_id_fkey1 =
            self.commercial_packaging_models_id_fkey1.set_primary_key(primary_key);
        self
    }
}
impl<
    CommercialProduct,
    PackagingModel,
    C,
> web_common_traits::database::TryInsertGeneric<C>
for InsertableCommercialPackagingModelBuilder<CommercialProduct, PackagingModel>
where
    Self: web_common_traits::database::InsertableVariant<
        C,
        UserId = i32,
        Row = crate::codegen::structs_codegen::tables::commercial_packaging_models::CommercialPackagingModel,
        Error = web_common_traits::database::InsertError<
            CommercialPackagingModelAttribute,
        >,
    >,
    CommercialProduct: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
    PackagingModel: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
{
    type Attribute = CommercialPackagingModelAttribute;
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
        let insertable: crate::codegen::structs_codegen::tables::commercial_packaging_models::CommercialPackagingModel = self
            .insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
