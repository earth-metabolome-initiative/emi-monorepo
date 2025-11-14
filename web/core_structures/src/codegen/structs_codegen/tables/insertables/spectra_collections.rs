#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SpectraCollectionExtensionAttribute {
    DigitalAsset(crate::codegen::structs_codegen::tables::insertables::DigitalAssetAttribute),
}
impl core::fmt::Display for SpectraCollectionExtensionAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::DigitalAsset(e) => write!(f, "spectra_collections({e})"),
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::DigitalAssetAttribute>
    for SpectraCollectionExtensionAttribute
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::DigitalAssetAttribute,
    ) -> Self {
        Self::DigitalAsset(attribute)
    }
}
impl From<common_traits::builder::EmptyTuple> for SpectraCollectionExtensionAttribute {
    fn from(_attribute: common_traits::builder::EmptyTuple) -> Self {
        unreachable!("Some code generation error occurred to reach this point.")
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SpectraCollectionAttribute {
    Extension(SpectraCollectionExtensionAttribute),
    Id,
}
impl core::str::FromStr for SpectraCollectionAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Id" => Ok(Self::Id),
            "id" => Ok(Self::Id),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl<T1> common_traits::builder::Attributed
    for crate::codegen::structs_codegen::tables::insertables::InsertableSpectraCollectionBuilder<T1>
{
    type Attribute = SpectraCollectionAttribute;
}
impl web_common_traits::database::TableField for SpectraCollectionAttribute {}
impl web_common_traits::database::HasTableType for SpectraCollectionAttribute {
    type Table = crate::codegen::tables::table_names::TableName;
}
impl
    web_common_traits::database::FromExtension<
        crate::codegen::structs_codegen::tables::insertables::DigitalAssetAttribute,
    > for SpectraCollectionAttribute
{
    fn from_extension(
        attribute: crate::codegen::structs_codegen::tables::insertables::DigitalAssetAttribute,
    ) -> Self {
        SpectraCollectionAttribute::Extension(From::from(attribute))
    }
}
impl web_common_traits::database::FromExtension<common_traits::builder::EmptyTuple>
    for SpectraCollectionAttribute
{
    fn from_extension(attribute: common_traits::builder::EmptyTuple) -> Self {
        SpectraCollectionAttribute::Extension(From::from(attribute))
    }
}
impl core::fmt::Display for SpectraCollectionAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::Id => write!(f, "spectra_collections.id"),
        }
    }
}
#[derive(Debug)]
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::spectra_collections::spectra_collections
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableSpectraCollection {
    pub(crate) id: ::rosetta_uuid::Uuid,
}
impl InsertableSpectraCollection {}
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Hash, Ord, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Builder for creating and inserting a new
/// [`SpectraCollection`](crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection).
///
///
/// # Implementation details
/// While this builder implements several methods, a reasonably complete
/// **basic** usage example (*which may not apply to your own specific use case,
/// please adapt accordingly*) is as follows:
///
/// ```rust,ignore
/// use core_structures::SpectraCollection;
/// use core_structures::tables::insertables::AssetSettable;
/// use core_structures::tables::insertables::DigitalAssetSettable;
/// use web_common_traits::database::Insertable;
/// use web_common_traits::database::InsertableVariant;
///
/// let spectra_collection = SpectraCollection::new()
///    // Set mandatory fields
///    .created_by(created_by)?
///    // Note: `updated_by` is automatically set by the `created by` column.
///    .updated_by(updated_by)?
///    .model(model)?
///    // Optionally set fields with default values
///    .created_at(created_at)?
///    .id(id)?
///    .updated_at(updated_at)?
///    // Optionally set optional fields
///    .description(description)?
///    .name(name)?
///    // Finally, insert the new record in the database
///    .insert(user.id, conn)?;
/// ```
pub struct InsertableSpectraCollectionBuilder<
    DigitalAsset
        = crate::codegen::structs_codegen::tables::insertables::InsertableDigitalAssetBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertableAssetBuilder,
        >,
> {
    pub(crate) id: DigitalAsset,
}
impl<DigitalAsset> diesel::associations::HasTable
    for InsertableSpectraCollectionBuilder<DigitalAsset>
{
    type Table =
        crate::codegen::diesel_codegen::tables::spectra_collections::spectra_collections::table;
    fn table() -> Self::Table {
        crate::codegen::diesel_codegen::tables::spectra_collections::spectra_collections::table
    }
}
impl From<InsertableSpectraCollectionBuilder>
    for web_common_traits::database::IdOrBuilder<
        ::rosetta_uuid::Uuid,
        InsertableSpectraCollectionBuilder,
    >
{
    fn from(builder: InsertableSpectraCollectionBuilder) -> Self {
        Self::Builder(builder)
    }
}
impl<DigitalAsset> common_traits::builder::IsCompleteBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableSpectraCollectionBuilder<
        DigitalAsset,
    >
where
    DigitalAsset: common_traits::builder::IsCompleteBuilder,
{
    fn is_complete(&self) -> bool {
        self.id.is_complete()
    }
}
/// Trait defining setters for attributes of an instance of `SpectraCollection`
/// or descendant tables.
pub trait SpectraCollectionSettable: Sized {
    /// Error type returned when setting attributes.
    type Error;
}
impl<DigitalAsset> SpectraCollectionSettable
for InsertableSpectraCollectionBuilder<DigitalAsset>
where
    Self: common_traits::builder::Attributed<
        Attribute = crate::codegen::structs_codegen::tables::insertables::SpectraCollectionAttribute,
    >,
{
    type Error = web_common_traits::database::InsertError<
        <Self as common_traits::builder::Attributed>::Attribute,
    >;
}
impl<
    DigitalAsset: crate::codegen::structs_codegen::tables::insertables::AssetSettable<
            Error = web_common_traits::database::InsertError<
                crate::codegen::structs_codegen::tables::insertables::DigitalAssetAttribute,
            >,
        >,
> crate::codegen::structs_codegen::tables::insertables::AssetSettable
for InsertableSpectraCollectionBuilder<DigitalAsset>
where
    Self: common_traits::builder::Attributed<
        Attribute = crate::codegen::structs_codegen::tables::insertables::SpectraCollectionAttribute,
    >,
    Self: crate::codegen::structs_codegen::tables::insertables::DigitalAssetSettable<
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::SpectraCollectionAttribute,
        >,
    >,
{
    type Error = web_common_traits::database::InsertError<
        <Self as common_traits::builder::Attributed>::Attribute,
    >;
    #[inline]
    ///Sets the value of the `public.assets.id` column.
    fn id<I>(mut self, id: I) -> Result<Self, Self::Error>
    where
        I: web_common_traits::database::PrimaryKeyLike<
            PrimaryKey = ::rosetta_uuid::Uuid,
        >,
    {
        self.id = <DigitalAsset as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::id(
                self.id,
                id,
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
    ///Sets the value of the `public.assets.name` column.
    fn name<N>(mut self, name: N) -> Result<Self, Self::Error>
    where
        N: TryInto<Option<String>>,
        validation_errors::prelude::SingleFieldError: From<<N as TryInto<Option<String>>>::Error>,
    {
        self.id = <DigitalAsset as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::name(
                self.id,
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
    ///Sets the value of the `public.assets.description` column.
    fn description<D>(mut self, description: D) -> Result<Self, Self::Error>
    where
        D: TryInto<Option<String>>,
        validation_errors::prelude::SingleFieldError: From<<D as TryInto<Option<String>>>::Error>,
    {
        self.id = <DigitalAsset as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::description(
                self.id,
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
    ///Sets the value of the `public.assets.model` column.
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
    ///subgraph v2 ["`assets`"]
    ///    v0@{shape: rounded, label: "model"}
    ///class v0 column-of-interest
    ///end
    ///subgraph v3 ["`digital_assets`"]
    ///    v1@{shape: rounded, label: "model"}
    ///class v1 directly-involved-column
    ///end
    ///v1 --->|"`ancestral same as`"| v0
    ///v3 --->|"`extends`"| v2
    ///```
    fn model<M>(self, model: M) -> Result<Self, Self::Error>
    where
        M: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        <Self as crate::codegen::structs_codegen::tables::insertables::DigitalAssetSettable>::model(
            self,
            model,
        )
    }
    #[inline]
    ///Sets the value of the `public.assets.created_by` column.
    fn created_by<CB>(mut self, created_by: CB) -> Result<Self, Self::Error>
    where
        CB: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        self.id = <DigitalAsset as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::created_by(
                self.id,
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
    ///Sets the value of the `public.assets.created_at` column.
    fn created_at<CA>(mut self, created_at: CA) -> Result<Self, Self::Error>
    where
        CA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::prelude::SingleFieldError: From<
            <CA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error,
        >,
    {
        self.id = <DigitalAsset as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::created_at(
                self.id,
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
    ///Sets the value of the `public.assets.updated_by` column.
    fn updated_by<UB>(mut self, updated_by: UB) -> Result<Self, Self::Error>
    where
        UB: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        self.id = <DigitalAsset as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::updated_by(
                self.id,
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
    ///Sets the value of the `public.assets.updated_at` column.
    fn updated_at<UA>(mut self, updated_at: UA) -> Result<Self, Self::Error>
    where
        UA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::prelude::SingleFieldError: From<
            <UA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error,
        >,
    {
        self.id = <DigitalAsset as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::updated_at(
                self.id,
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
    DigitalAsset: crate::codegen::structs_codegen::tables::insertables::DigitalAssetSettable<
            Error = web_common_traits::database::InsertError<
                crate::codegen::structs_codegen::tables::insertables::DigitalAssetAttribute,
            >,
        >,
> crate::codegen::structs_codegen::tables::insertables::DigitalAssetSettable
for InsertableSpectraCollectionBuilder<DigitalAsset>
where
    Self: common_traits::builder::Attributed<
        Attribute = crate::codegen::structs_codegen::tables::insertables::SpectraCollectionAttribute,
    >,
{
    type Error = web_common_traits::database::InsertError<
        <Self as common_traits::builder::Attributed>::Attribute,
    >;
    #[inline]
    ///Sets the value of the `public.digital_assets.model` column.
    fn model<M>(mut self, model: M) -> Result<Self, Self::Error>
    where
        M: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        self.id = <DigitalAsset as crate::codegen::structs_codegen::tables::insertables::DigitalAssetSettable>::model(
                self.id,
                model,
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
impl<DigitalAsset> web_common_traits::database::MostConcreteTable
    for InsertableSpectraCollectionBuilder<DigitalAsset>
where
    DigitalAsset: web_common_traits::database::MostConcreteTable,
{
    fn set_most_concrete_table(&mut self, table_name: &str) {
        self.id.set_most_concrete_table(table_name);
    }
}
impl<DigitalAsset> web_common_traits::prelude::SetPrimaryKey
    for InsertableSpectraCollectionBuilder<DigitalAsset>
where
    DigitalAsset: web_common_traits::prelude::SetPrimaryKey<PrimaryKey = ::rosetta_uuid::Uuid>,
{
    type PrimaryKey = ::rosetta_uuid::Uuid;
    fn set_primary_key(mut self, primary_key: Self::PrimaryKey) -> Self {
        self.id = self.id.set_primary_key(primary_key);
        self
    }
}
impl<DigitalAsset, C> web_common_traits::database::TryInsertGeneric<C>
    for InsertableSpectraCollectionBuilder<DigitalAsset>
where
    Self: web_common_traits::database::DispatchableInsertableVariant<
            C,
            Row = crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection,
            Error = web_common_traits::database::InsertError<SpectraCollectionAttribute>,
        > + web_common_traits::database::SetPrimaryKey<PrimaryKey = ::rosetta_uuid::Uuid>
        + common_traits::builder::IsCompleteBuilder,
{
    type Error = web_common_traits::database::InsertError<SpectraCollectionAttribute>;
    fn mint_primary_key(self, user_id: i32, conn: &mut C) -> Result<Self::PrimaryKey, Self::Error> {
        use diesel::Identifiable;
        use web_common_traits::database::DispatchableInsertableVariant;
        Ok(self.insert(user_id, conn)?.id())
    }
}
