#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    diesel::Selectable,
    diesel::Insertable,
    diesel::AsChangeset,
    diesel::Queryable,
    diesel::Identifiable,
    diesel::Associations,
)]
#[cfg_attr(feature = "yew", derive(yew::prelude::Properties))]
#[diesel(belongs_to(crate::Rank, foreign_key = rank_id))]
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
impl web_common_traits::prelude::ExtensionTable<crate::Taxon> for Taxon where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>
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
    ) -> Result<crate::Rank, diesel::result::Error>
    where
        crate::Rank: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::Rank::read(self.rank_id, conn)
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
        parent_id: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::taxa::taxa;
        Self::table()
            .filter(taxa::parent_id.eq(parent_id))
            .order_by(taxa::id.asc())
            .load::<Self>(conn)
    }
}
impl AsRef<Taxon> for Taxon {
    fn as_ref(&self) -> &Taxon {
        self
    }
}
