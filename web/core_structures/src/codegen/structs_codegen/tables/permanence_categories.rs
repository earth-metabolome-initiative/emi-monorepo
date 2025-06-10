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
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::permanence_categories::permanence_categories
)]
pub struct PermanenceCategory {
    pub name: String,
    pub description: String,
    pub icon: String,
    pub color_id: i16,
    pub id: i16,
}
impl web_common_traits::prelude::TableName for PermanenceCategory {
    const TABLE_NAME: &'static str = "permanence_categories";
}
impl diesel::Identifiable for PermanenceCategory {
    type Id = i16;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl PermanenceCategory {
    pub fn color<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::colors::Color,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::colors::Color: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::colors::Color as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::colors::Color as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::colors::Color as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::colors::Color as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::colors::Color as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::colors::Color as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::colors::Color,
        >,
    {
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, RunQueryDsl};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::colors::Color::table(),
                self.color_id,
            ),
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn from_name(
        name: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use crate::codegen::diesel_codegen::tables::permanence_categories::permanence_categories;
        use diesel::OptionalExtension;
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{ExpressionMethods, QueryDsl};
        Self::table()
            .filter(permanence_categories::name.eq(name))
            .order_by(permanence_categories::id.asc())
            .first::<Self>(conn)
            .optional()
    }
    #[cfg(feature = "postgres")]
    pub fn from_description(
        description: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use crate::codegen::diesel_codegen::tables::permanence_categories::permanence_categories;
        use diesel::OptionalExtension;
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{ExpressionMethods, QueryDsl};
        Self::table()
            .filter(permanence_categories::description.eq(description))
            .order_by(permanence_categories::id.asc())
            .first::<Self>(conn)
            .optional()
    }
    #[cfg(feature = "postgres")]
    pub fn from_icon(
        icon: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use crate::codegen::diesel_codegen::tables::permanence_categories::permanence_categories;
        use diesel::OptionalExtension;
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{ExpressionMethods, QueryDsl};
        Self::table()
            .filter(permanence_categories::icon.eq(icon))
            .order_by(permanence_categories::id.asc())
            .first::<Self>(conn)
            .optional()
    }
    #[cfg(feature = "postgres")]
    pub fn from_color_id(
        color_id: &i16,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use crate::codegen::diesel_codegen::tables::permanence_categories::permanence_categories;
        use diesel::OptionalExtension;
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{ExpressionMethods, QueryDsl};
        Self::table()
            .filter(permanence_categories::color_id.eq(color_id))
            .order_by(permanence_categories::id.asc())
            .first::<Self>(conn)
            .optional()
    }
}
impl AsRef<PermanenceCategory> for PermanenceCategory {
    fn as_ref(&self) -> &PermanenceCategory {
        self
    }
}
