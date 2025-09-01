#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableTaxonAttributes {
    Id,
    Name,
    ParentId,
    RankId,
}
impl core::str::FromStr for InsertableTaxonAttributes {
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
impl core::fmt::Display for InsertableTaxonAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Id => write!(f, "id"),
            Self::Name => write!(f, "name"),
            Self::ParentId => write!(f, "parent_id"),
            Self::RankId => write!(f, "rank_id"),
        }
    }
}
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
    ) -> Result<
        crate::codegen::structs_codegen::tables::ranks::Rank,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::ranks::Rank: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::ranks::Rank as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::ranks::Rank as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::ranks::Rank as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::ranks::Rank as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::ranks::Rank as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::ranks::Rank as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::ranks::Rank,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::ranks::Rank::table(),
                self.rank_id,
            ),
            conn,
        )
    }
}
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableTaxonBuilder {
    pub(crate) id: Option<i32>,
    pub(crate) name: Option<String>,
    pub(crate) parent_id: Option<i32>,
    pub(crate) rank_id: Option<i16>,
}
/// Trait defining setters for attributes of an instance of `Taxon` or
/// descendant tables.
pub trait TaxonBuildable: std::marker::Sized {
    /// Attributes required to build the insertable.
    type Attributes;
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
    fn id(
        self,
        id: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
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
    fn name<N>(
        self,
        name: N,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
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
    fn parent<PI>(
        self,
        parent_id: PI,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
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
    fn rank(
        self,
        rank_id: i16,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
}
impl TaxonBuildable for Option<i32> {
    type Attributes =
        crate::codegen::structs_codegen::tables::insertables::InsertableTaxonAttributes;
    fn id(
        self,
        id: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        Ok(Some(id.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err).rename_field(Self::Attributes::Id)
        })?))
    }
    fn name<N>(
        self,
        _name: N,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        N: TryInto<String>,
        validation_errors::SingleFieldError: From<<N as TryInto<String>>::Error>,
    {
        Ok(self)
    }
    fn parent<PI>(
        self,
        _parent_id: PI,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PI: TryInto<Option<i32>>,
        validation_errors::SingleFieldError: From<<PI as TryInto<Option<i32>>>::Error>,
    {
        Ok(self)
    }
    fn rank(
        self,
        _rank_id: i16,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        Ok(self)
    }
}
impl TaxonBuildable for InsertableTaxonBuilder {
    type Attributes =
        crate::codegen::structs_codegen::tables::insertables::InsertableTaxonAttributes;
    /// Sets the value of the `public.taxa.id` column.
    fn id(
        mut self,
        id: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        let id = id.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(InsertableTaxonAttributes::Id)
        })?;
        self.id = Some(id);
        Ok(self)
    }
    /// Sets the value of the `public.taxa.name` column.
    fn name<N>(
        mut self,
        name: N,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        N: TryInto<String>,
        validation_errors::SingleFieldError: From<<N as TryInto<String>>::Error>,
    {
        let name = name.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(InsertableTaxonAttributes::Name)
        })?;
        self.name = Some(name);
        Ok(self)
    }
    /// Sets the value of the `public.taxa.parent_id` column.
    fn parent<PI>(
        mut self,
        parent_id: PI,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PI: TryInto<Option<i32>>,
        validation_errors::SingleFieldError: From<<PI as TryInto<Option<i32>>>::Error>,
    {
        let parent_id = parent_id.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(InsertableTaxonAttributes::ParentId)
        })?;
        self.parent_id = parent_id;
        Ok(self)
    }
    /// Sets the value of the `public.taxa.rank_id` column.
    fn rank(
        mut self,
        rank_id: i16,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        let rank_id = rank_id.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(InsertableTaxonAttributes::RankId)
        })?;
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
    Self: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::codegen::structs_codegen::tables::taxa::Taxon,
            Error = web_common_traits::database::InsertError<InsertableTaxonAttributes>,
        >,
{
    type Attributes = InsertableTaxonAttributes;
    fn is_complete(&self) -> bool {
        self.id.is_some() && self.name.is_some() && self.rank_id.is_some()
    }
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, web_common_traits::database::InsertError<Self::Attributes>> {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::taxa::Taxon =
            self.insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
