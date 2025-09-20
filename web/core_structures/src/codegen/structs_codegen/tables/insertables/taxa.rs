#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TaxonAttribute {
    Id,
    Name,
    ParentId,
    RankId,
}
impl core::str::FromStr for TaxonAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Id" => Ok(Self::Id),
            "Name" => Ok(Self::Name),
            "ParentId" => Ok(Self::ParentId),
            "RankId" => Ok(Self::RankId),
            "id" => Ok(Self::Id),
            "name" => Ok(Self::Name),
            "parent_id" => Ok(Self::ParentId),
            "rank_id" => Ok(Self::RankId),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl common_traits::builder::Attributed
    for crate::codegen::structs_codegen::tables::insertables::InsertableTaxonBuilder
{
    type Attribute = TaxonAttribute;
}
impl web_common_traits::database::TableField for TaxonAttribute {}
impl web_common_traits::database::HasTableType for TaxonAttribute {
    type Table = crate::codegen::tables::table_names::TableName;
}
impl core::fmt::Display for TaxonAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Id => write!(f, "taxa.id"),
            Self::Name => write!(f, "taxa.name"),
            Self::ParentId => write!(f, "taxa.parent_id"),
            Self::RankId => write!(f, "taxa.rank_id"),
        }
    }
}
#[derive(Debug)]
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(table_name = crate::codegen::diesel_codegen::tables::taxa::taxa)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableTaxon {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) parent_id: Option<i32>,
    pub(crate) rank_id: i16,
}
impl InsertableTaxon {
    pub fn rank<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::codegen::structs_codegen::tables::ranks::Rank, diesel::result::Error>
    where
        crate::codegen::structs_codegen::tables::ranks::Rank: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::ranks::Rank::read(self.rank_id, conn)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Hash, Ord, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Builder for creating and inserting a new
/// [`Taxon`](crate::codegen::structs_codegen::tables::taxa::Taxon).
///
/// # Implementation details
/// While this builder implements several methods, a reasonably complete
/// **basic** usage example (*which may not apply to your own specific use case,
/// please adapt accordingly*) is as follows:
///
/// ```rust,ignore
/// use core_structures::Taxon;
/// use core_structures::tables::insertables::TaxonSettable;
/// use web_common_traits::database::Insertable;
/// use web_common_traits::database::InsertableVariant;
///
/// let taxon = Taxon::new()
///    // Set mandatory fields
///    .id(id)?
///    .name(name)?
///    .rank(rank_id)?
///    // Optionally set optional fields
///    .parent(parent_id)?
///    // Finally, insert the new record in the database
///    .insert(user.id, conn)?;
/// ```
pub struct InsertableTaxonBuilder {
    pub(crate) id: Option<i32>,
    pub(crate) name: Option<String>,
    pub(crate) parent_id: Option<i32>,
    pub(crate) rank_id: Option<i16>,
}
impl diesel::associations::HasTable for InsertableTaxonBuilder {
    type Table = crate::codegen::diesel_codegen::tables::taxa::taxa::table;
    fn table() -> Self::Table {
        crate::codegen::diesel_codegen::tables::taxa::taxa::table
    }
}
impl From<InsertableTaxonBuilder>
    for web_common_traits::database::IdOrBuilder<i32, InsertableTaxonBuilder>
{
    fn from(builder: InsertableTaxonBuilder) -> Self {
        Self::Builder(builder)
    }
}
impl common_traits::builder::IsCompleteBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableTaxonBuilder
{
    fn is_complete(&self) -> bool {
        self.id.is_some() && self.name.is_some() && self.rank_id.is_some()
    }
}
/// Trait defining setters for attributes of an instance of `Taxon` or
/// descendant tables.
pub trait TaxonSettable: Sized {
    /// Error type returned when setting attributes.
    type Error;
    /// Sets the value of the `public.taxa.id` column.
    ///
    /// # Arguments
    /// * `id`: The value to set for the `public.taxa.id` column.
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
    fn id<I>(self, id: I) -> Result<Self, Self::Error>
    where
        I: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
    /// Sets the value of the `public.taxa.name` column.
    ///
    /// # Arguments
    /// * `name`: The value to set for the `public.taxa.name` column.
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
    fn name<N>(self, name: N) -> Result<Self, Self::Error>
    where
        N: TryInto<String>,
        validation_errors::SingleFieldError: From<<N as TryInto<String>>::Error>;
    /// Sets the value of the `public.taxa.parent_id` column.
    ///
    /// # Arguments
    /// * `parent_id`: The value to set for the `public.taxa.parent_id` column.
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
    fn parent<PI>(self, parent_id: PI) -> Result<Self, Self::Error>
    where
        PI: TryInto<Option<i32>>,
        validation_errors::SingleFieldError: From<<PI as TryInto<Option<i32>>>::Error>;
    /// Sets the value of the `public.taxa.rank_id` column.
    ///
    /// # Arguments
    /// * `rank_id`: The value to set for the `public.taxa.rank_id` column.
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
    /// * If the provided value cannot be converted to the required type `i16`.
    /// * If the provided value does not pass schema-defined validation.
    fn rank<RI>(self, rank_id: RI) -> Result<Self, Self::Error>
    where
        RI: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i16>;
}
impl TaxonSettable for InsertableTaxonBuilder
where
    Self: common_traits::builder::Attributed<
            Attribute = crate::codegen::structs_codegen::tables::insertables::TaxonAttribute,
        >,
{
    type Error = web_common_traits::database::InsertError<
        <Self as common_traits::builder::Attributed>::Attribute,
    >;
    /// Sets the value of the `public.taxa.id` column.
    fn id<I>(mut self, id: I) -> Result<Self, Self::Error>
    where
        I: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let id = <I as web_common_traits::database::PrimaryKeyLike>::primary_key(&id);
        self.id = Some(id);
        Ok(self)
    }
    /// Sets the value of the `public.taxa.name` column.
    fn name<N>(mut self, name: N) -> Result<Self, Self::Error>
    where
        N: TryInto<String>,
        validation_errors::SingleFieldError: From<<N as TryInto<String>>::Error>,
    {
        let name = name.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err).rename_field(TaxonAttribute::Name)
        })?;
        self.name = Some(name);
        Ok(self)
    }
    /// Sets the value of the `public.taxa.parent_id` column.
    fn parent<PI>(mut self, parent_id: PI) -> Result<Self, Self::Error>
    where
        PI: TryInto<Option<i32>>,
        validation_errors::SingleFieldError: From<<PI as TryInto<Option<i32>>>::Error>,
    {
        let parent_id = parent_id.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err).rename_field(TaxonAttribute::ParentId)
        })?;
        self.parent_id = parent_id;
        Ok(self)
    }
    /// Sets the value of the `public.taxa.rank_id` column.
    fn rank<RI>(mut self, rank_id: RI) -> Result<Self, Self::Error>
    where
        RI: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i16>,
    {
        let rank_id = <RI as web_common_traits::database::PrimaryKeyLike>::primary_key(&rank_id);
        self.rank_id = Some(rank_id);
        Ok(self)
    }
}
impl web_common_traits::prelude::SetPrimaryKey for InsertableTaxonBuilder {
    type PrimaryKey = i32;
    fn set_primary_key(self, _primary_key: Self::PrimaryKey) -> Self {
        self
    }
}
impl<C> web_common_traits::database::TryInsertGeneric<C> for InsertableTaxonBuilder
where
    Self: web_common_traits::database::DispatchableInsertableVariant<
            C,
            Row = crate::codegen::structs_codegen::tables::taxa::Taxon,
            Error = web_common_traits::database::InsertError<TaxonAttribute>,
        > + web_common_traits::database::SetPrimaryKey<PrimaryKey = i32>
        + common_traits::builder::IsCompleteBuilder,
{
    type Error = web_common_traits::database::InsertError<TaxonAttribute>;
    fn mint_primary_key(self, user_id: i32, conn: &mut C) -> Result<Self::PrimaryKey, Self::Error> {
        use diesel::Identifiable;
        use web_common_traits::database::DispatchableInsertableVariant;
        Ok(self.insert(user_id, conn)?.id())
    }
}
