#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SpectrumExtensionAttribute {
    DigitalAsset(crate::codegen::structs_codegen::tables::insertables::DigitalAssetAttribute),
}
impl core::fmt::Display for SpectrumExtensionAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::DigitalAsset(e) => write!(f, "{e}"),
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::DigitalAssetAttribute>
    for SpectrumExtensionAttribute
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::DigitalAssetAttribute,
    ) -> Self {
        Self::DigitalAsset(attribute)
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SpectrumAttribute {
    Extension(SpectrumExtensionAttribute),
    Id,
    SpectraCollectionId,
}
impl core::str::FromStr for SpectrumAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SpectraCollectionId" => Ok(Self::SpectraCollectionId),
            "spectra_collection_id" => Ok(Self::SpectraCollectionId),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl
    web_common_traits::database::DefaultExtensionAttribute<
        crate::codegen::structs_codegen::tables::insertables::DigitalAssetAttribute,
    > for SpectrumAttribute
{
    /// Returns the default value for the target attribute.
    fn target_default() -> Self {
        Self::Extension(
            crate::codegen::structs_codegen::tables::insertables::DigitalAssetAttribute::Id.into(),
        )
    }
}
impl<Asset>
    web_common_traits::database::FromExtensionAttribute<
        crate::codegen::structs_codegen::tables::insertables::DigitalAssetAttribute,
        crate::codegen::structs_codegen::tables::insertables::InsertableDigitalAssetBuilder<Asset>,
    > for SpectrumAttribute
{
    type EffectiveExtensionAttribute =
        crate::codegen::structs_codegen::tables::insertables::DigitalAssetAttribute;
    fn from_extension_attribute(extension_attribute: Self::EffectiveExtensionAttribute) -> Self {
        Self::Extension(extension_attribute.into())
    }
}
impl core::fmt::Display for SpectrumAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::Id => write!(f, "spectra.id"),
            Self::SpectraCollectionId => write!(f, "spectra.spectra_collection_id"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(table_name = crate::codegen::diesel_codegen::tables::spectra::spectra)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableSpectrum {
    pub(crate) id: ::rosetta_uuid::Uuid,
    pub(crate) spectra_collection_id: ::rosetta_uuid::Uuid,
}
impl InsertableSpectrum {
    pub fn id<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::digital_assets::DigitalAsset,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::digital_assets::DigitalAsset:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::digital_assets::DigitalAsset::read(self.id, conn)
    }
    pub fn spectra_collection<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection::read(
            self.spectra_collection_id,
            conn,
        )
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Hash, Ord, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Builder for creating and inserting a new
/// [`Spectrum`](crate::codegen::structs_codegen::tables::spectra::Spectrum).
///
/// # Implementation details
/// While this builder implements several methods, a reasonably complete
/// **basic** usage example (*which may not apply to your own specific use case,
/// please adapt accordingly*) is as follows:
///
/// ```rust,ignore
/// use core_structures::Spectrum;
/// use core_structures::tables::insertables::AssetSettable;
/// use core_structures::tables::insertables::DigitalAssetSettable;
/// use core_structures::tables::insertables::SpectrumSettable;
/// use web_common_traits::database::Insertable;
/// use web_common_traits::database::InsertableVariant;
///
/// let spectrum = Spectrum::new()
///    // Set mandatory fields
///    .created_by(created_by)?
///    // Note: `updated_by` is automatically set by the `created by` column.
///    .updated_by(updated_by)?
///    .model(model)?
///    .spectra_collection(spectra_collection_id)?
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
pub struct InsertableSpectrumBuilder<
    DigitalAsset
        = crate::codegen::structs_codegen::tables::insertables::InsertableDigitalAssetBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertableAssetBuilder,
        >,
> {
    pub(crate) spectra_collection_id: Option<::rosetta_uuid::Uuid>,
    pub(crate) id: DigitalAsset,
}
impl From<InsertableSpectrumBuilder>
    for web_common_traits::database::IdOrBuilder<::rosetta_uuid::Uuid, InsertableSpectrumBuilder>
{
    fn from(builder: InsertableSpectrumBuilder) -> Self {
        Self::Builder(builder)
    }
}
impl<DigitalAsset> common_traits::builder::IsCompleteBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableSpectrumBuilder<
        DigitalAsset,
    >
where
    DigitalAsset: common_traits::builder::IsCompleteBuilder,
{
    fn is_complete(&self) -> bool {
        self.id.is_complete() && self.spectra_collection_id.is_some()
    }
}
/// Trait defining setters for attributes of an instance of `Spectrum` or
/// descendant tables.
pub trait SpectrumSettable: Sized {
    /// Attributes required to build the insertable.
    type Attributes;
    /// Sets the value of the `public.spectra.spectra_collection_id` column.
    ///
    /// # Arguments
    /// * `spectra_collection_id`: The value to set for the
    ///   `public.spectra.spectra_collection_id` column.
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
    ///   `::rosetta_uuid::Uuid`.
    /// * If the provided value does not pass schema-defined validation.
    fn spectra_collection<SCI>(
        self,
        spectra_collection_id: SCI,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        SCI: web_common_traits::database::PrimaryKeyLike<PrimaryKey = ::rosetta_uuid::Uuid>;
}
impl<DigitalAsset> SpectrumSettable for InsertableSpectrumBuilder<DigitalAsset> {
    type Attributes = crate::codegen::structs_codegen::tables::insertables::SpectrumAttribute;
    /// Sets the value of the `public.spectra.spectra_collection_id` column.
    fn spectra_collection<SCI>(
        mut self,
        spectra_collection_id: SCI,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        SCI: web_common_traits::database::PrimaryKeyLike<PrimaryKey = ::rosetta_uuid::Uuid>,
    {
        let spectra_collection_id =
            <SCI as web_common_traits::database::PrimaryKeyLike>::primary_key(
                &spectra_collection_id,
            );
        self.spectra_collection_id = Some(spectra_collection_id);
        Ok(self)
    }
}
impl<
    DigitalAsset: crate::codegen::structs_codegen::tables::insertables::AssetSettable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::DigitalAssetAttribute,
        >,
> crate::codegen::structs_codegen::tables::insertables::AssetSettable
for InsertableSpectrumBuilder<DigitalAsset>
where
    Self: crate::codegen::structs_codegen::tables::insertables::DigitalAssetSettable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::SpectrumAttribute,
    >,
{
    type Attributes = crate::codegen::structs_codegen::tables::insertables::SpectrumAttribute;
    #[inline]
    ///Sets the value of the `public.assets.id` column.
    fn id<I>(
        mut self,
        id: I,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
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
                    .into_field_name(|attribute| Self::Attributes::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    ///Sets the value of the `public.assets.name` column.
    fn name<N>(
        mut self,
        name: N,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        N: TryInto<Option<String>>,
        validation_errors::SingleFieldError: From<<N as TryInto<Option<String>>>::Error>,
    {
        self.id = <DigitalAsset as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::name(
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
    ///Sets the value of the `public.assets.description` column.
    fn description<D>(
        mut self,
        description: D,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        D: TryInto<Option<String>>,
        validation_errors::SingleFieldError: From<<D as TryInto<Option<String>>>::Error>,
    {
        self.id = <DigitalAsset as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::description(
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
    fn model<M>(
        self,
        model: M,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
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
    fn created_by<CB>(
        mut self,
        created_by: CB,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        CB: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        self.id = <DigitalAsset as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::created_by(
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
    ///Sets the value of the `public.assets.created_at` column.
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
        self.id = <DigitalAsset as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::created_at(
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
    ///Sets the value of the `public.assets.updated_by` column.
    fn updated_by<UB>(
        mut self,
        updated_by: UB,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        UB: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        self.id = <DigitalAsset as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::updated_by(
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
    ///Sets the value of the `public.assets.updated_at` column.
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
        self.id = <DigitalAsset as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::updated_at(
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
    DigitalAsset: crate::codegen::structs_codegen::tables::insertables::DigitalAssetSettable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::DigitalAssetAttribute,
        >,
> crate::codegen::structs_codegen::tables::insertables::DigitalAssetSettable
for InsertableSpectrumBuilder<DigitalAsset> {
    type Attributes = crate::codegen::structs_codegen::tables::insertables::SpectrumAttribute;
    #[inline]
    ///Sets the value of the `public.digital_assets.model` column.
    fn model<M>(
        mut self,
        model: M,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        M: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        self.id = <DigitalAsset as crate::codegen::structs_codegen::tables::insertables::DigitalAssetSettable>::model(
                self.id,
                model,
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
impl<DigitalAsset> web_common_traits::database::MostConcreteTable
    for InsertableSpectrumBuilder<DigitalAsset>
where
    DigitalAsset: web_common_traits::database::MostConcreteTable,
{
    fn set_most_concrete_table(&mut self, table_name: &str) {
        self.id.set_most_concrete_table(table_name);
    }
}
impl<DigitalAsset> web_common_traits::prelude::SetPrimaryKey
    for InsertableSpectrumBuilder<DigitalAsset>
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
    for InsertableSpectrumBuilder<DigitalAsset>
where
    Self: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::codegen::structs_codegen::tables::spectra::Spectrum,
            Error = web_common_traits::database::InsertError<SpectrumAttribute>,
        >,
    DigitalAsset:
        web_common_traits::database::TryInsertGeneric<C, PrimaryKey = ::rosetta_uuid::Uuid>,
{
    type Attribute = SpectrumAttribute;
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, web_common_traits::database::InsertError<Self::Attribute>> {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::spectra::Spectrum =
            self.insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
