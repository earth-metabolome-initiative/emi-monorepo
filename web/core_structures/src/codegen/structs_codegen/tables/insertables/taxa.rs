#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableTaxonAttributes {
    Id,
    Name,
    ParentId,
    RankId,
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
impl web_common_traits::database::ExtendableBuilder for InsertableTaxonBuilder {
    type Attributes = InsertableTaxonAttributes;
    fn extend_builder(
        mut self,
        other: Self,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let Some(id) = other.id {
            self = self.id(id)?;
        }
        if let Some(name) = other.name {
            self = self.name(name)?;
        }
        if let Some(parent_id) = other.parent_id {
            self = self.parent(Some(parent_id))?;
        }
        if let Some(rank_id) = other.rank_id {
            self = self.rank(rank_id)?;
        }
        Ok(self)
    }
}
impl web_common_traits::prelude::SetPrimaryKey for InsertableTaxonBuilder {
    type PrimaryKey = i32;
    fn set_primary_key(self, _primary_key: Self::PrimaryKey) -> Self {
        self
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableTaxonBuilder {
    /// Sets the value of the `taxa.id` column from table `taxa`.
    pub fn id<Id>(
        mut self,
        id: Id,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableTaxonAttributes>>
    where
        Id: TryInto<i32>,
        <Id as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let id = id.try_into().map_err(|err: <Id as TryInto<i32>>::Error| {
            Into::into(err).rename_field(InsertableTaxonAttributes::Id)
        })?;
        self.id = Some(id);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableTaxonBuilder {
    /// Sets the value of the `taxa.name` column from table `taxa`.
    pub fn name<Name>(
        mut self,
        name: Name,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableTaxonAttributes>>
    where
        Name: TryInto<String>,
        <Name as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let name = name.try_into().map_err(|err: <Name as TryInto<String>>::Error| {
            Into::into(err).rename_field(InsertableTaxonAttributes::Name)
        })?;
        self.name = Some(name);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableTaxonBuilder {
    /// Sets the value of the `taxa.parent_id` column from table `taxa`.
    pub fn parent<ParentId>(
        mut self,
        parent_id: ParentId,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableTaxonAttributes>>
    where
        ParentId: TryInto<Option<i32>>,
        <ParentId as TryInto<Option<i32>>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let parent_id =
            parent_id.try_into().map_err(|err: <ParentId as TryInto<Option<i32>>>::Error| {
                Into::into(err).rename_field(InsertableTaxonAttributes::ParentId)
            })?;
        self.parent_id = parent_id;
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableTaxonBuilder {
    /// Sets the value of the `taxa.rank_id` column from table `taxa`.
    pub fn rank(
        mut self,
        rank_id: i16,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableTaxonAttributes>> {
        self.rank_id = Some(rank_id);
        Ok(self)
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
