#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SpectrumExtensionAttribute {
    DigitalAsset(crate::codegen::structs_codegen::tables::insertables::DigitalAssetAttribute),
}
impl core::fmt::Display for SpectrumExtensionAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::DigitalAsset(e) => write!(f, "spectra({e})"),
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
impl From<common_traits::builder::EmptyTuple> for SpectrumExtensionAttribute {
    fn from(_attribute: common_traits::builder::EmptyTuple) -> Self {
        unreachable!("Some code generation error occurred to reach this point.")
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
            "Id" => Ok(Self::Id),
            "SpectraCollectionId" => Ok(Self::SpectraCollectionId),
            "id" => Ok(Self::Id),
            "spectra_collection_id" => Ok(Self::SpectraCollectionId),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl<T1> common_traits::builder::Attributed
    for crate::codegen::structs_codegen::tables::insertables::InsertableSpectrumBuilder<T1>
{
    type Attribute = SpectrumAttribute;
}
impl web_common_traits::database::TableField for SpectrumAttribute {}
impl web_common_traits::database::HasTableType for SpectrumAttribute {
    type Table = crate::codegen::tables::table_names::TableName;
}
impl
    web_common_traits::database::FromExtension<
        crate::codegen::structs_codegen::tables::insertables::DigitalAssetAttribute,
    > for SpectrumAttribute
{
    fn from_extension(
        attribute: crate::codegen::structs_codegen::tables::insertables::DigitalAssetAttribute,
    ) -> Self {
        SpectrumAttribute::Extension(From::from(attribute))
    }
}
impl web_common_traits::database::FromExtension<common_traits::builder::EmptyTuple>
    for SpectrumAttribute
{
    fn from_extension(attribute: common_traits::builder::EmptyTuple) -> Self {
        SpectrumAttribute::Extension(From::from(attribute))
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
#[derive(Debug)]
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
impl<DigitalAsset> diesel::associations::HasTable for InsertableSpectrumBuilder<DigitalAsset> {
    type Table = crate::codegen::diesel_codegen::tables::spectra::spectra::table;
    fn table() -> Self::Table {
        crate::codegen::diesel_codegen::tables::spectra::spectra::table
    }
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
    /// Error type returned when setting attributes.
    type Error;
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
    fn spectra_collection<SCI>(self, spectra_collection_id: SCI) -> Result<Self, Self::Error>
    where
        SCI: web_common_traits::database::PrimaryKeyLike<PrimaryKey = ::rosetta_uuid::Uuid>;
}
impl<DigitalAsset> SpectrumSettable for InsertableSpectrumBuilder<DigitalAsset>
where
    Self: common_traits::builder::Attributed<
            Attribute = crate::codegen::structs_codegen::tables::insertables::SpectrumAttribute,
        >,
{
    type Error = web_common_traits::database::InsertError<
        <Self as common_traits::builder::Attributed>::Attribute,
    >;
    /// Sets the value of the `public.spectra.spectra_collection_id` column.
    fn spectra_collection<SCI>(mut self, spectra_collection_id: SCI) -> Result<Self, Self::Error>
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
            Error = web_common_traits::database::InsertError<
                crate::codegen::structs_codegen::tables::insertables::DigitalAssetAttribute,
            >,
        >,
> crate::codegen::structs_codegen::tables::insertables::AssetSettable
    for InsertableSpectrumBuilder<DigitalAsset>
where
    Self: common_traits::builder::Attributed<
            Attribute = crate::codegen::structs_codegen::tables::insertables::SpectrumAttribute,
        >,
    Self: crate::codegen::structs_codegen::tables::insertables::DigitalAssetSettable<
            Error = web_common_traits::database::InsertError<
                crate::codegen::structs_codegen::tables::insertables::SpectrumAttribute,
            >,
        >,
{
    type Error = web_common_traits::database::InsertError<
        <Self as common_traits::builder::Attributed>::Attribute,
    >;
    #[inline]
    /// Sets the value of the `public.assets.id` column.
    fn id<I>(mut self, id: I) -> Result<Self, Self::Error>
    where
        I: web_common_traits::database::PrimaryKeyLike<PrimaryKey = ::rosetta_uuid::Uuid>,
    {
        self.id = <DigitalAsset as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::id(
                self.id,
                id,
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
    /// Sets the value of the `public.assets.name` column.
    fn name<N>(mut self, name: N) -> Result<Self, Self::Error>
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
                    .into_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    /// Sets the value of the `public.assets.description` column.
    fn description<D>(mut self, description: D) -> Result<Self, Self::Error>
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
                    .into_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    /// Sets the value of the `public.assets.model` column.
    ///
    /// # Implementation notes
    /// This method also set the values of other columns, due to
    /// same-as relationships or inferred values.
    ///
    /// ## Mermaid illustration
    ///
    /// ```mermaid
    /// flowchart BT
    /// classDef column-of-interest stroke: #f0746c,fill: #f49f9a
    /// classDef directly-involved-column stroke: #6c74f0,fill: #9a9ff4
    /// subgraph v2 ["`assets`"]
    ///    v0@{shape: rounded, label: "model"}
    /// class v0 column-of-interest
    /// end
    /// subgraph v3 ["`digital_assets`"]
    ///    v1@{shape: rounded, label: "model"}
    /// class v1 directly-involved-column
    /// end
    /// v1 --->|"`ancestral same as`"| v0
    /// v3 --->|"`extends`"| v2
    /// ```
    fn model<M>(self, model: M) -> Result<Self, Self::Error>
    where
        M: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        <Self as crate::codegen::structs_codegen::tables::insertables::DigitalAssetSettable>::model(
            self, model,
        )
    }
    #[inline]
    /// Sets the value of the `public.assets.created_by` column.
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
                    .into_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    /// Sets the value of the `public.assets.created_at` column.
    fn created_at<CA>(mut self, created_at: CA) -> Result<Self, Self::Error>
    where
        CA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::SingleFieldError:
            From<<CA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error>,
    {
        self.id = <DigitalAsset as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::created_at(
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
    /// Sets the value of the `public.assets.updated_by` column.
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
                    .into_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    /// Sets the value of the `public.assets.updated_at` column.
    fn updated_at<UA>(mut self, updated_at: UA) -> Result<Self, Self::Error>
    where
        UA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::SingleFieldError:
            From<<UA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error>,
    {
        self.id = <DigitalAsset as crate::codegen::structs_codegen::tables::insertables::AssetSettable>::updated_at(
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
    DigitalAsset: crate::codegen::structs_codegen::tables::insertables::DigitalAssetSettable<
            Error = web_common_traits::database::InsertError<
                crate::codegen::structs_codegen::tables::insertables::DigitalAssetAttribute,
            >,
        >,
> crate::codegen::structs_codegen::tables::insertables::DigitalAssetSettable
    for InsertableSpectrumBuilder<DigitalAsset>
where
    Self: common_traits::builder::Attributed<
            Attribute = crate::codegen::structs_codegen::tables::insertables::SpectrumAttribute,
        >,
{
    type Error = web_common_traits::database::InsertError<
        <Self as common_traits::builder::Attributed>::Attribute,
    >;
    #[inline]
    /// Sets the value of the `public.digital_assets.model` column.
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
                    .into_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
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
    Self: web_common_traits::database::DispatchableInsertableVariant<
            C,
            Row = crate::codegen::structs_codegen::tables::spectra::Spectrum,
            Error = web_common_traits::database::InsertError<SpectrumAttribute>,
        > + web_common_traits::database::SetPrimaryKey<PrimaryKey = ::rosetta_uuid::Uuid>
        + common_traits::builder::IsCompleteBuilder,
{
    type Error = web_common_traits::database::InsertError<SpectrumAttribute>;
    fn mint_primary_key(self, user_id: i32, conn: &mut C) -> Result<Self::PrimaryKey, Self::Error> {
        use diesel::Identifiable;
        use web_common_traits::database::DispatchableInsertableVariant;
        Ok(self.insert(user_id, conn)?.id())
    }
}
