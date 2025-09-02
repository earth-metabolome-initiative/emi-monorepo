#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    diesel::Selectable,
    diesel::Insertable,
    diesel::AsChangeset,
    diesel::Queryable,
    diesel::Identifiable,
)]
#[cfg_attr(feature = "yew", derive(yew::prelude::Properties))]
#[diesel(primary_key(id))]
#[diesel(table_name = crate::codegen::diesel_codegen::tables::taxa::taxa)]
pub struct Taxon {
    pub id: i32,
    pub name: String,
    pub parent_id: Option<i32>,
    pub rank_id: i16,
}
impl web_common_traits::prelude::TableName for Taxon {
    const TABLE_NAME: &'static str = "taxa";
}
impl
    web_common_traits::prelude::ExtensionTable<crate::codegen::structs_codegen::tables::taxa::Taxon>
    for Taxon
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{
}
impl diesel::Identifiable for Taxon {
    type Id = i32;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl Taxon {
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
    #[cfg(feature = "postgres")]
    pub fn from_name(
        name: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::taxa::taxa;
        Self::table().filter(taxa::name.eq(name)).order_by(taxa::id.asc()).load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_parent_id(
        parent_id: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::taxa::taxa;
        Self::table()
            .filter(taxa::parent_id.eq(parent_id))
            .order_by(taxa::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_rank_id(
        rank_id: &i16,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::taxa::taxa;
        Self::table().filter(taxa::rank_id.eq(rank_id)).order_by(taxa::id.asc()).load::<Self>(conn)
    }
}
impl AsRef<Taxon> for Taxon {
    fn as_ref(&self) -> &Taxon {
        self
    }
}
