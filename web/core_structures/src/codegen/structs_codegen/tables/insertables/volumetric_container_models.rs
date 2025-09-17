#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum VolumetricContainerModelExtensionAttribute {
    ContainerModel(crate::codegen::structs_codegen::tables::insertables::ContainerModelAttribute),
}
impl core::fmt::Display for VolumetricContainerModelExtensionAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::ContainerModel(e) => write!(f, "volumetric_container_models({e})"),
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::ContainerModelAttribute>
    for VolumetricContainerModelExtensionAttribute
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::ContainerModelAttribute,
    ) -> Self {
        Self::ContainerModel(attribute)
    }
}
impl From<common_traits::builder::EmptyTuple> for VolumetricContainerModelExtensionAttribute {
    fn from(_attribute: common_traits::builder::EmptyTuple) -> Self {
        unreachable!("Some code generation error occurred to reach this point.")
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum VolumetricContainerModelAttribute {
    Extension(VolumetricContainerModelExtensionAttribute),
    Id,
    Liters,
}
impl core::str::FromStr for VolumetricContainerModelAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Liters" => Ok(Self::Liters),
            "liters" => Ok(Self::Liters),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl<T1> common_traits::builder::Attributed
for crate::codegen::structs_codegen::tables::insertables::InsertableVolumetricContainerModelBuilder<
    T1,
> {
    type Attribute = VolumetricContainerModelAttribute;
}
impl core::fmt::Display for VolumetricContainerModelAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::Id => write!(f, "volumetric_container_models.id"),
            Self::Liters => write!(f, "volumetric_container_models.liters"),
        }
    }
}
#[derive(Debug)]
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::volumetric_container_models::volumetric_container_models
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableVolumetricContainerModel {
    pub(crate) id: i32,
    pub(crate) liters: f32,
}
impl InsertableVolumetricContainerModel {}
#[derive(Clone, Debug, PartialEq, PartialOrd, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Builder for creating and inserting a new
/// [`VolumetricContainerModel`](crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel).
///
///
/// # Implementation details
/// While this builder implements several methods, a reasonably complete
/// **basic** usage example (*which may not apply to your own specific use case,
/// please adapt accordingly*) is as follows:
///
/// ```rust,ignore
/// use core_structures::VolumetricContainerModel;
/// use core_structures::tables::insertables::AssetModelSettable;
/// use core_structures::tables::insertables::PhysicalAssetModelSettable;
/// use core_structures::tables::insertables::VolumetricContainerModelSettable;
/// use web_common_traits::database::Insertable;
/// use web_common_traits::database::InsertableVariant;
///
/// let volumetric_container_model = VolumetricContainerModel::new()
///    // Set mandatory fields
///    .created_by(created_by)?
///    .description(description)?
///    .name(name)?
///    // Note: `updated_by` is automatically set by the `created by` column.
///    .updated_by(updated_by)?
///    .liters(liters)?
///    // Optionally set fields with default values
///    .created_at(created_at)?
///    .updated_at(updated_at)?
///    // Optionally set optional fields
///    .parent_model(parent_model)?
///    // Finally, insert the new record in the database
///    .insert(user.id, conn)?;
/// ```
pub struct InsertableVolumetricContainerModelBuilder<
    ContainerModel
        = crate::codegen::structs_codegen::tables::insertables::InsertableContainerModelBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetModelBuilder<
                crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelBuilder,
            >,
        >,
> {
    pub(crate) liters: Option<f32>,
    pub(crate) id: ContainerModel,
}
impl<ContainerModel> diesel::associations::HasTable
    for InsertableVolumetricContainerModelBuilder<ContainerModel>
{
    type Table = crate::codegen::diesel_codegen::tables::volumetric_container_models::volumetric_container_models::table;
    fn table() -> Self::Table {
        crate::codegen::diesel_codegen::tables::volumetric_container_models::volumetric_container_models::table
    }
}
impl From<InsertableVolumetricContainerModelBuilder>
    for web_common_traits::database::IdOrBuilder<i32, InsertableVolumetricContainerModelBuilder>
{
    fn from(builder: InsertableVolumetricContainerModelBuilder) -> Self {
        Self::Builder(builder)
    }
}
impl<ContainerModel> common_traits::builder::IsCompleteBuilder
for crate::codegen::structs_codegen::tables::insertables::InsertableVolumetricContainerModelBuilder<
    ContainerModel,
>
where
    ContainerModel: common_traits::builder::IsCompleteBuilder,
{
    fn is_complete(&self) -> bool {
        self.id.is_complete() && self.liters.is_some()
    }
}
/// Trait defining setters for attributes of an instance of
/// `VolumetricContainerModel` or descendant tables.
pub trait VolumetricContainerModelSettable: Sized {
    /// Error type returned when setting attributes.
    type Error;
    /// Sets the value of the `public.volumetric_container_models.liters`
    /// column.
    ///
    /// # Arguments
    /// * `liters`: The value to set for the
    ///   `public.volumetric_container_models.liters` column.
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
    /// * If the provided value cannot be converted to the required type `f32`.
    /// * If the provided value does not pass schema-defined validation.
    fn liters<L>(self, liters: L) -> Result<Self, Self::Error>
    where
        L: TryInto<f32>,
        validation_errors::SingleFieldError: From<<L as TryInto<f32>>::Error>;
}
impl<ContainerModel> VolumetricContainerModelSettable
for InsertableVolumetricContainerModelBuilder<ContainerModel>
where
    Self: common_traits::builder::Attributed<
        Attribute = crate::codegen::structs_codegen::tables::insertables::VolumetricContainerModelAttribute,
    >,
{
    type Error = web_common_traits::database::InsertError<
        <Self as common_traits::builder::Attributed>::Attribute,
    >;
    ///Sets the value of the `public.volumetric_container_models.liters` column.
    fn liters<L>(mut self, liters: L) -> Result<Self, Self::Error>
    where
        L: TryInto<f32>,
        validation_errors::SingleFieldError: From<<L as TryInto<f32>>::Error>,
    {
        let liters = liters
            .try_into()
            .map_err(|err| {
                validation_errors::SingleFieldError::from(err)
                    .rename_field(VolumetricContainerModelAttribute::Liters)
            })?;
        pgrx_validation::must_be_strictly_positive_f32(liters)
            .map_err(|e| {
                e
                    .rename_field(
                        crate::codegen::structs_codegen::tables::insertables::VolumetricContainerModelAttribute::Liters,
                    )
            })?;
        self.liters = Some(liters);
        Ok(self)
    }
}
impl<
    ContainerModel: crate::codegen::structs_codegen::tables::insertables::AssetModelSettable<
            Error = web_common_traits::database::InsertError<
                crate::codegen::structs_codegen::tables::insertables::ContainerModelAttribute,
            >,
        >,
> crate::codegen::structs_codegen::tables::insertables::AssetModelSettable
for InsertableVolumetricContainerModelBuilder<ContainerModel>
where
    Self: common_traits::builder::Attributed<
        Attribute = crate::codegen::structs_codegen::tables::insertables::VolumetricContainerModelAttribute,
    >,
    Self: crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelSettable<
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::VolumetricContainerModelAttribute,
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
        self.id = <ContainerModel as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::name(
                self.id,
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
        self.id = <ContainerModel as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::description(
                self.id,
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
        self.id = <ContainerModel as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_by(
                self.id,
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
        self.id = <ContainerModel as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::created_at(
                self.id,
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
        self.id = <ContainerModel as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_by(
                self.id,
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
        self.id = <ContainerModel as crate::codegen::structs_codegen::tables::insertables::AssetModelSettable>::updated_at(
                self.id,
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
    ContainerModel,
> crate::codegen::structs_codegen::tables::insertables::ContainerModelSettable
for InsertableVolumetricContainerModelBuilder<ContainerModel>
where
    Self: common_traits::builder::Attributed<
        Attribute = crate::codegen::structs_codegen::tables::insertables::VolumetricContainerModelAttribute,
    >,
{
    type Error = web_common_traits::database::InsertError<
        <Self as common_traits::builder::Attributed>::Attribute,
    >;
}
impl<
    ContainerModel: crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelSettable<
            Error = web_common_traits::database::InsertError<
                crate::codegen::structs_codegen::tables::insertables::ContainerModelAttribute,
            >,
        >,
> crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelSettable
for InsertableVolumetricContainerModelBuilder<ContainerModel>
where
    Self: common_traits::builder::Attributed<
        Attribute = crate::codegen::structs_codegen::tables::insertables::VolumetricContainerModelAttribute,
    >,
{
    type Error = web_common_traits::database::InsertError<
        <Self as common_traits::builder::Attributed>::Attribute,
    >;
    #[inline]
    ///Sets the value of the `public.physical_asset_models.parent_model` column.
    fn parent_model<PM>(mut self, parent_model: PM) -> Result<Self, Self::Error>
    where
        PM: web_common_traits::database::MaybePrimaryKeyLike<PrimaryKey = i32>,
    {
        self.id = <ContainerModel as crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelSettable>::parent_model(
                self.id,
                parent_model,
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
impl<ContainerModel> web_common_traits::database::MostConcreteTable
    for InsertableVolumetricContainerModelBuilder<ContainerModel>
where
    ContainerModel: web_common_traits::database::MostConcreteTable,
{
    fn set_most_concrete_table(&mut self, table_name: &str) {
        self.id.set_most_concrete_table(table_name);
    }
}
impl<ContainerModel> web_common_traits::prelude::SetPrimaryKey
    for InsertableVolumetricContainerModelBuilder<ContainerModel>
where
    ContainerModel: web_common_traits::prelude::SetPrimaryKey<PrimaryKey = i32>,
{
    type PrimaryKey = i32;
    fn set_primary_key(mut self, primary_key: Self::PrimaryKey) -> Self {
        self.id = self.id.set_primary_key(primary_key);
        self
    }
}
impl<ContainerModel, C> web_common_traits::database::TryInsertGeneric<C>
for InsertableVolumetricContainerModelBuilder<ContainerModel>
where
    Self: web_common_traits::database::DispatchableInsertableVariant<
        C,
        Row = crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel,
        Error = web_common_traits::database::InsertError<
            VolumetricContainerModelAttribute,
        >,
    >,
    ContainerModel: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
{
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<
        Self::PrimaryKey,
        web_common_traits::database::InsertError<VolumetricContainerModelAttribute>,
    > {
        use diesel::Identifiable;
        use web_common_traits::database::DispatchableInsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel = self
            .insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
