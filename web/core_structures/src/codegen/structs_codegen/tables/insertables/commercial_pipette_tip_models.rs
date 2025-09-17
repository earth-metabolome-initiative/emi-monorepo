#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CommercialPipetteTipModelExtensionAttribute {
    PipetteTipModel(crate::codegen::structs_codegen::tables::insertables::PipetteTipModelAttribute),
    CommercialProduct(
        crate::codegen::structs_codegen::tables::insertables::CommercialProductAttribute,
    ),
}
impl core::fmt::Display for CommercialPipetteTipModelExtensionAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::PipetteTipModel(e) => write!(f, "commercial_pipette_tip_models({e})"),
            Self::CommercialProduct(e) => write!(f, "commercial_pipette_tip_models({e})"),
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::PipetteTipModelAttribute>
    for CommercialPipetteTipModelExtensionAttribute
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::PipetteTipModelAttribute,
    ) -> Self {
        Self::PipetteTipModel(attribute)
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::CommercialProductAttribute>
    for CommercialPipetteTipModelExtensionAttribute
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::CommercialProductAttribute,
    ) -> Self {
        Self::CommercialProduct(attribute)
    }
}
impl From<common_traits::builder::EmptyTuple> for CommercialPipetteTipModelExtensionAttribute {
    fn from(_attribute: common_traits::builder::EmptyTuple) -> Self {
        unreachable!("Some code generation error occurred to reach this point.")
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CommercialPipetteTipModelAttribute {
    Extension(CommercialPipetteTipModelExtensionAttribute),
    Id,
    PipetteTipModel,
}
impl core::str::FromStr for CommercialPipetteTipModelAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PipetteTipModel" => Ok(Self::PipetteTipModel),
            "pipette_tip_model" => Ok(Self::PipetteTipModel),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl<T1, T2> common_traits::builder::Attributed
for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteTipModelBuilder<
    T1,
    T2,
> {
    type Attribute = CommercialPipetteTipModelAttribute;
}
impl core::fmt::Display for CommercialPipetteTipModelAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::Id => write!(f, "commercial_pipette_tip_models.id"),
            Self::PipetteTipModel => {
                write!(f, "commercial_pipette_tip_models.pipette_tip_model")
            }
        }
    }
}
#[derive(Debug)]
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::commercial_pipette_tip_models::commercial_pipette_tip_models
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableCommercialPipetteTipModel {
    pub(crate) id: i32,
    pub(crate) pipette_tip_model: i32,
}
impl InsertableCommercialPipetteTipModel {
    pub fn pipette_tip_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel::read(
            self.pipette_tip_model,
            conn,
        )
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Hash, Ord, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Builder for creating and inserting a new
/// [`CommercialPipetteTipModel`](crate::codegen::structs_codegen::tables::commercial_pipette_tip_models::CommercialPipetteTipModel).
///
///
/// # Implementation details
/// While this builder implements several methods, a reasonably complete
/// **basic** usage example (*which may not apply to your own specific use case,
/// please adapt accordingly*) is as follows:
///
/// ```rust,ignore
/// use core_structures::CommercialPipetteTipModel;
/// use core_structures::tables::insertables::AssetModelSettable;
/// use core_structures::tables::insertables::CommercialPipetteTipModelSettable;
/// use core_structures::tables::insertables::CommercialProductSettable;
/// use web_common_traits::database::Insertable;
/// use web_common_traits::database::InsertableVariant;
///
/// let commercial_pipette_tip_model = CommercialPipetteTipModel::new()
///    // Set mandatory fields
///    .created_by(created_by)?
///    .description(description)?
///    .name(name)?
///    // Note: `updated_by` is automatically set by the `created by` column.
///    .updated_by(updated_by)?
///    .pipette_tip_model(pipette_tip_model)?
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
pub struct InsertableCommercialPipetteTipModelBuilder<
    PipetteTipModel
        = crate::codegen::structs_codegen::tables::insertables::InsertablePipetteTipModelBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetModelBuilder<
                crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelBuilder,
            >,
        >,
    CommercialProduct
        = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductBuilder<
            Option<i32>,
        >,
> {
    pub(crate) pipette_tip_model: Option<i32>,
    pub(crate) commercial_pipette_tip_models_id_fkey: PipetteTipModel,
    pub(crate) commercial_pipette_tip_models_id_fkey1: CommercialProduct,
}
impl<PipetteTipModel, CommercialProduct> diesel::associations::HasTable
    for InsertableCommercialPipetteTipModelBuilder<PipetteTipModel, CommercialProduct>
{
    type Table = crate::codegen::diesel_codegen::tables::commercial_pipette_tip_models::commercial_pipette_tip_models::table;
    fn table() -> Self::Table {
        crate::codegen::diesel_codegen::tables::commercial_pipette_tip_models::commercial_pipette_tip_models::table
    }
}
impl From<InsertableCommercialPipetteTipModelBuilder>
    for web_common_traits::database::IdOrBuilder<i32, InsertableCommercialPipetteTipModelBuilder>
{
    fn from(builder: InsertableCommercialPipetteTipModelBuilder) -> Self {
        Self::Builder(builder)
    }
}
impl<PipetteTipModel, CommercialProduct> common_traits::builder::IsCompleteBuilder
for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteTipModelBuilder<
    PipetteTipModel,
    CommercialProduct,
>
where
    CommercialProduct: common_traits::builder::IsCompleteBuilder,
    PipetteTipModel: common_traits::builder::IsCompleteBuilder,
{
    fn is_complete(&self) -> bool {
        self.commercial_pipette_tip_models_id_fkey.is_complete()
            && self.commercial_pipette_tip_models_id_fkey1.is_complete()
            && self.pipette_tip_model.is_some()
    }
}
/// Trait defining setters for attributes of an instance of
/// `CommercialPipetteTipModel` or descendant tables.
pub trait CommercialPipetteTipModelSettable: Sized {
    /// Error type returned when setting attributes.
    type Error;
    /// Sets the value of the
    /// `public.commercial_pipette_tip_models.pipette_tip_model` column.
    ///
    /// # Arguments
    /// * `pipette_tip_model`: The value to set for the
    ///   `public.commercial_pipette_tip_models.pipette_tip_model` column.
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
    fn pipette_tip_model<PTM>(self, pipette_tip_model: PTM) -> Result<Self, Self::Error>
    where
        PTM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
}
impl<
    PipetteTipModel: crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelSettable<
            Error = web_common_traits::database::InsertError<
                crate::codegen::structs_codegen::tables::insertables::PipetteTipModelAttribute,
            >,
        >,
    CommercialProduct,
> CommercialPipetteTipModelSettable
for InsertableCommercialPipetteTipModelBuilder<PipetteTipModel, CommercialProduct>
where
    Self: common_traits::builder::Attributed<
        Attribute = crate::codegen::structs_codegen::tables::insertables::CommercialPipetteTipModelAttribute,
    >,
{
    type Error = web_common_traits::database::InsertError<
        <Self as common_traits::builder::Attributed>::Attribute,
    >;
    ///Sets the value of the `public.commercial_pipette_tip_models.pipette_tip_model` column.
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
    ///subgraph v4 ["`commercial_pipette_tip_models`"]
    ///    v0@{shape: rounded, label: "pipette_tip_model"}
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
    fn pipette_tip_model<PTM>(
        mut self,
        pipette_tip_model: PTM,
    ) -> Result<Self, Self::Error>
    where
        PTM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let pipette_tip_model = <PTM as web_common_traits::database::PrimaryKeyLike>::primary_key(
            &pipette_tip_model,
        );
        self.commercial_pipette_tip_models_id_fkey = <PipetteTipModel as crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelSettable>::parent_model(
                self.commercial_pipette_tip_models_id_fkey,
                pipette_tip_model,
            )
            .map_err(|err| {
                err.into_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
                    attribute.into(),
                ))
            })?;
        self.pipette_tip_model = Some(pipette_tip_model);
        Ok(self)
    }
}
impl<
    PipetteTipModel: crate::codegen::structs_codegen::tables::insertables::AssetModelSettable<
            Error = web_common_traits::database::InsertError<
                crate::codegen::structs_codegen::tables::insertables::PipetteTipModelAttribute,
            >,
        >,
    CommercialProduct,
> crate::codegen::structs_codegen::tables::insertables::AssetModelSettable
for InsertableCommercialPipetteTipModelBuilder<PipetteTipModel, CommercialProduct>
where
    Self: common_traits::builder::Attributed<
        Attribute = crate::codegen::structs_codegen::tables::insertables::CommercialPipetteTipModelAttribute,
    >,
    Self: crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelSettable<
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialPipetteTipModelAttribute,
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
        self.commercial_pipette_tip_models_id_fkey = <PipetteTipModel as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::name(
                self.commercial_pipette_tip_models_id_fkey,
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
        self.commercial_pipette_tip_models_id_fkey = <PipetteTipModel as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::description(
                self.commercial_pipette_tip_models_id_fkey,
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
        self.commercial_pipette_tip_models_id_fkey = <PipetteTipModel as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_by(
                self.commercial_pipette_tip_models_id_fkey,
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
        self.commercial_pipette_tip_models_id_fkey = <PipetteTipModel as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_at(
                self.commercial_pipette_tip_models_id_fkey,
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
        self.commercial_pipette_tip_models_id_fkey = <PipetteTipModel as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_by(
                self.commercial_pipette_tip_models_id_fkey,
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
        self.commercial_pipette_tip_models_id_fkey = <PipetteTipModel as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_at(
                self.commercial_pipette_tip_models_id_fkey,
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
    PipetteTipModel,
    CommercialProduct: crate::codegen::structs_codegen::tables::insertables::CommercialProductSettable<
            Error = web_common_traits::database::InsertError<
                crate::codegen::structs_codegen::tables::insertables::CommercialProductAttribute,
            >,
        >,
> crate::codegen::structs_codegen::tables::insertables::CommercialProductSettable
for InsertableCommercialPipetteTipModelBuilder<PipetteTipModel, CommercialProduct>
where
    Self: common_traits::builder::Attributed<
        Attribute = crate::codegen::structs_codegen::tables::insertables::CommercialPipetteTipModelAttribute,
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
        validation_errors::SingleFieldError: From<
            <DD as TryInto<Option<::rosetta_timestamp::TimestampUTC>>>::Error,
        >,
    {
        self.commercial_pipette_tip_models_id_fkey1 = <CommercialProduct as crate::codegen::structs_codegen::tables::insertables::CommercialProductSettable>::deprecation_date(
                self.commercial_pipette_tip_models_id_fkey1,
                deprecation_date,
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
    ///Sets the value of the `public.commercial_products.brand_id` column.
    fn brand<BI>(mut self, brand_id: BI) -> Result<Self, Self::Error>
    where
        BI: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        self.commercial_pipette_tip_models_id_fkey1 = <CommercialProduct as crate::codegen::structs_codegen::tables::insertables::CommercialProductSettable>::brand(
                self.commercial_pipette_tip_models_id_fkey1,
                brand_id,
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
    PipetteTipModel,
    CommercialProduct,
> crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelSettable
for InsertableCommercialPipetteTipModelBuilder<PipetteTipModel, CommercialProduct>
where
    Self: common_traits::builder::Attributed<
        Attribute = crate::codegen::structs_codegen::tables::insertables::CommercialPipetteTipModelAttribute,
    >,
    Self: crate::codegen::structs_codegen::tables::insertables::CommercialPipetteTipModelSettable<
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialPipetteTipModelAttribute,
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
    ///subgraph v4 ["`commercial_pipette_tip_models`"]
    ///    v0@{shape: rounded, label: "pipette_tip_model"}
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
        <Self as CommercialPipetteTipModelSettable>::pipette_tip_model(
            self,
            <PM as web_common_traits::database::MaybePrimaryKeyLike>::maybe_primary_key(
                    &parent_model,
                )
                .ok_or(
                    common_traits::prelude::BuilderError::IncompleteBuild(
                        <Self as common_traits::builder::Attributed>::Attribute::PipetteTipModel,
                    ),
                )?,
        )
    }
}
impl<
    PipetteTipModel,
    CommercialProduct,
> crate::codegen::structs_codegen::tables::insertables::PipetteTipModelSettable
for InsertableCommercialPipetteTipModelBuilder<PipetteTipModel, CommercialProduct>
where
    Self: common_traits::builder::Attributed<
        Attribute = crate::codegen::structs_codegen::tables::insertables::CommercialPipetteTipModelAttribute,
    >,
{
    type Error = web_common_traits::database::InsertError<
        <Self as common_traits::builder::Attributed>::Attribute,
    >;
}
impl<PipetteTipModel, CommercialProduct> web_common_traits::database::MostConcreteTable
    for InsertableCommercialPipetteTipModelBuilder<PipetteTipModel, CommercialProduct>
where
    PipetteTipModel: web_common_traits::database::MostConcreteTable,
    CommercialProduct: web_common_traits::database::MostConcreteTable,
{
    fn set_most_concrete_table(&mut self, table_name: &str) {
        self.commercial_pipette_tip_models_id_fkey.set_most_concrete_table(table_name);
        self.commercial_pipette_tip_models_id_fkey1.set_most_concrete_table(table_name);
    }
}
impl<PipetteTipModel, CommercialProduct> web_common_traits::prelude::SetPrimaryKey
    for InsertableCommercialPipetteTipModelBuilder<PipetteTipModel, CommercialProduct>
where
    PipetteTipModel: web_common_traits::prelude::SetPrimaryKey<PrimaryKey = i32>,
    CommercialProduct: web_common_traits::prelude::SetPrimaryKey<PrimaryKey = i32>,
{
    type PrimaryKey = i32;
    fn set_primary_key(mut self, primary_key: Self::PrimaryKey) -> Self {
        self.commercial_pipette_tip_models_id_fkey =
            self.commercial_pipette_tip_models_id_fkey.set_primary_key(primary_key);
        self.commercial_pipette_tip_models_id_fkey1 =
            self.commercial_pipette_tip_models_id_fkey1.set_primary_key(primary_key);
        self
    }
}
impl<
    PipetteTipModel,
    CommercialProduct,
    C,
> web_common_traits::database::TryInsertGeneric<C>
for InsertableCommercialPipetteTipModelBuilder<PipetteTipModel, CommercialProduct>
where
    Self: web_common_traits::database::DispatchableInsertableVariant<
        C,
        Row = crate::codegen::structs_codegen::tables::commercial_pipette_tip_models::CommercialPipetteTipModel,
        Error = web_common_traits::database::InsertError<
            CommercialPipetteTipModelAttribute,
        >,
    >,
    PipetteTipModel: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
    CommercialProduct: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
{
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<
        Self::PrimaryKey,
        web_common_traits::database::InsertError<CommercialPipetteTipModelAttribute>,
    > {
        use diesel::Identifiable;
        use web_common_traits::database::DispatchableInsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::commercial_pipette_tip_models::CommercialPipetteTipModel = self
            .insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
