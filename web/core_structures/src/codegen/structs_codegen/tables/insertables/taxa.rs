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
            InsertableTaxonAttributes::Id => write!(f, "id"),
            InsertableTaxonAttributes::Name => write!(f, "name"),
            InsertableTaxonAttributes::ParentId => write!(f, "parent_id"),
            InsertableTaxonAttributes::RankId => write!(f, "rank_id"),
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
    id: i32,
    name: String,
    parent_id: Option<i32>,
    rank_id: i16,
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
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, RunQueryDsl};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::ranks::Rank::table(),
                self.rank_id,
            ),
            conn,
        )
    }
}
#[derive(Default)]
pub struct InsertableTaxonBuilder {
    pub(crate) id: Option<i32>,
    pub(crate) name: Option<String>,
    pub(crate) parent_id: Option<i32>,
    pub(crate) rank_id: Option<i16>,
}
impl InsertableTaxonBuilder {
    pub fn id<P>(
        mut self,
        id: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableTaxonAttributes>>
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let id = id.try_into().map_err(|err: <P as TryInto<i32>>::Error| {
            Into::into(err).rename_field(InsertableTaxonAttributes::Id)
        })?;
        self.id = Some(id);
        Ok(self)
    }
    pub fn name<P>(
        mut self,
        name: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableTaxonAttributes>>
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let name = name.try_into().map_err(|err: <P as TryInto<String>>::Error| {
            Into::into(err).rename_field(InsertableTaxonAttributes::Name)
        })?;
        self.name = Some(name);
        Ok(self)
    }
    pub fn parent_id<P>(
        mut self,
        parent_id: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableTaxonAttributes>>
    where
        P: TryInto<Option<i32>>,
        <P as TryInto<Option<i32>>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let parent_id =
            parent_id.try_into().map_err(|err: <P as TryInto<Option<i32>>>::Error| {
                Into::into(err).rename_field(InsertableTaxonAttributes::ParentId)
            })?;
        self.parent_id = parent_id;
        Ok(self)
    }
    pub fn rank_id<P>(
        mut self,
        rank_id: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableTaxonAttributes>>
    where
        P: TryInto<i16>,
        <P as TryInto<i16>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let rank_id = rank_id.try_into().map_err(|err: <P as TryInto<i16>>::Error| {
            Into::into(err).rename_field(InsertableTaxonAttributes::RankId)
        })?;
        self.rank_id = Some(rank_id);
        Ok(self)
    }
}
impl TryFrom<InsertableTaxonBuilder> for InsertableTaxon {
    type Error = common_traits::prelude::BuilderError<InsertableTaxonAttributes>;
    fn try_from(builder: InsertableTaxonBuilder) -> Result<InsertableTaxon, Self::Error> {
        Ok(Self {
            id: builder.id.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableTaxonAttributes::Id,
            ))?,
            name: builder.name.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableTaxonAttributes::Name,
            ))?,
            parent_id: builder.parent_id,
            rank_id: builder.rank_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableTaxonAttributes::RankId,
                ),
            )?,
        })
    }
}
