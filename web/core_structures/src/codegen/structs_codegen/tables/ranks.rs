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
#[diesel(table_name = crate::codegen::diesel_codegen::tables::ranks::ranks)]
pub struct Rank {
    pub name: String,
    pub description: String,
    pub id: i16,
}
impl web_common_traits::prelude::TableName for Rank {
    const TABLE_NAME: &'static str = "ranks";
}
impl<'a> From<&'a Rank>
    for web_common_traits::database::IdOrBuilder<
        i16,
        crate::codegen::structs_codegen::tables::insertables::InsertableRankBuilder,
    >
{
    fn from(value: &'a Rank) -> Self {
        web_common_traits::database::IdOrBuilder::Id(value.id)
    }
}
impl web_common_traits::prelude::ExtensionTable<crate::Rank> for Rank where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i16>
{
}
impl diesel::Identifiable for Rank {
    type Id = i16;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl Rank {
    #[cfg(feature = "postgres")]
    pub fn from_name(
        name: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::ranks::ranks;
        Self::table().filter(ranks::name.eq(name)).order_by(ranks::id.asc()).first::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_description(
        description: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::ranks::ranks;
        Self::table()
            .filter(ranks::description.eq(description))
            .order_by(ranks::id.asc())
            .first::<Self>(conn)
    }
}
impl AsRef<Rank> for Rank {
    fn as_ref(&self) -> &Rank {
        self
    }
}
