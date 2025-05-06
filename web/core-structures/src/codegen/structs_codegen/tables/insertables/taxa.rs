#[derive(Clone, core::fmt::Debug)]
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
    #[cfg(feature = "postgres")]
    pub async fn rank(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::ranks::Rank, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::ranks::Rank::table()
            .filter(crate::codegen::diesel_codegen::tables::ranks::ranks::dsl::id.eq(&self.rank_id))
            .first::<crate::codegen::structs_codegen::tables::ranks::Rank>(conn)
            .await
    }
}
#[derive(Default)]
pub struct InsertableTaxonBuilder {
    id: Option<i32>,
    name: Option<String>,
    parent_id: Option<i32>,
    rank_id: Option<i16>,
}
impl InsertableTaxonBuilder {
    pub fn id<P: Into<i32>>(
        mut self,
        id: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let id = id.into();
        self.id = Some(id);
        Ok(self)
    }
    pub fn name<P: Into<String>>(
        mut self,
        name: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let name = name.into();
        self.name = Some(name);
        Ok(self)
    }
    pub fn parent_id<P: Into<Option<i32>>>(
        mut self,
        parent_id: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let parent_id = parent_id.into();
        self.parent_id = parent_id;
        Ok(self)
    }
    pub fn rank_id<P: Into<i16>>(
        mut self,
        rank_id: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let rank_id = rank_id.into();
        self.rank_id = Some(rank_id);
        Ok(self)
    }
}
impl common_traits::prelude::Builder for InsertableTaxonBuilder {
    type Error = web_common_traits::database::InsertError<InsertableTaxonAttributes>;
    type Object = InsertableTaxon;
    type Attribute = InsertableTaxonAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            id: self.id.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableTaxonAttributes::Id,
            ))?,
            name: self.name.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableTaxonAttributes::Name,
            ))?,
            parent_id: self.parent_id,
            rank_id: self.rank_id.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableTaxonAttributes::RankId,
            ))?,
        })
    }
}
impl TryFrom<InsertableTaxon> for InsertableTaxonBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableTaxon) -> Result<Self, Self::Error> {
        Self::default()
            .id(insertable_variant.id)?
            .name(insertable_variant.name)?
            .parent_id(insertable_variant.parent_id)?
            .rank_id(insertable_variant.rank_id)
    }
}
