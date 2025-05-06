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
    table_name = crate::codegen::diesel_codegen::tables::nameplate_categories::nameplate_categories
)]
pub struct NameplateCategory {
    pub name: String,
    pub permanence_category_id: i16,
    pub description: String,
    pub icon: String,
    pub color_id: i16,
    pub id: i16,
}
impl diesel::Identifiable for NameplateCategory {
    type Id = i16;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl NameplateCategory {
    #[cfg(feature = "postgres")]
    pub async fn permanence_category(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::permanence_categories::PermanenceCategory,
        diesel::result::Error,
    > {
        use diesel::associations::HasTable;
        use diesel::{ExpressionMethods, QueryDsl};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::permanence_categories::PermanenceCategory::table()
            .filter(
                crate::codegen::diesel_codegen::tables::permanence_categories::permanence_categories::dsl::id
                    .eq(&self.permanence_category_id),
            )
            .first::<
                crate::codegen::structs_codegen::tables::permanence_categories::PermanenceCategory,
            >(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn color(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::colors::Color, diesel::result::Error> {
        use diesel::associations::HasTable;
        use diesel::{ExpressionMethods, QueryDsl};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::colors::Color::table()
            .filter(
                crate::codegen::diesel_codegen::tables::colors::colors::dsl::id.eq(&self.color_id),
            )
            .first::<crate::codegen::structs_codegen::tables::colors::Color>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_permanence_category_id(
        conn: &mut diesel_async::AsyncPgConnection,
        permanence_category_id: &crate::codegen::structs_codegen::tables::permanence_categories::PermanenceCategory,
    ) -> Result<Self, diesel::result::Error> {
        use diesel::associations::HasTable;
        use diesel::{ExpressionMethods, QueryDsl};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::nameplate_categories::nameplate_categories::dsl::permanence_category_id
                    .eq(permanence_category_id.id),
            )
            .first::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_color_id(
        conn: &mut diesel_async::AsyncPgConnection,
        color_id: &crate::codegen::structs_codegen::tables::colors::Color,
    ) -> Result<Self, diesel::result::Error> {
        use diesel::associations::HasTable;
        use diesel::{ExpressionMethods, QueryDsl};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::nameplate_categories::nameplate_categories::dsl::color_id
                    .eq(color_id.id),
            )
            .first::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_name(
        name: &str,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::associations::HasTable;
        use diesel::{OptionalExtension, QueryDsl};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                diesel::ExpressionMethods::eq(
                    crate::codegen::diesel_codegen::tables::nameplate_categories::nameplate_categories::name,
                    name,
                ),
            )
            .first::<Self>(conn)
            .await
            .optional()
    }
    #[cfg(feature = "postgres")]
    pub async fn from_description(
        description: &str,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::associations::HasTable;
        use diesel::{OptionalExtension, QueryDsl};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                diesel::ExpressionMethods::eq(
                    crate::codegen::diesel_codegen::tables::nameplate_categories::nameplate_categories::description,
                    description,
                ),
            )
            .first::<Self>(conn)
            .await
            .optional()
    }
    #[cfg(feature = "postgres")]
    pub async fn from_icon(
        icon: &str,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::associations::HasTable;
        use diesel::{OptionalExtension, QueryDsl};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                diesel::ExpressionMethods::eq(
                    crate::codegen::diesel_codegen::tables::nameplate_categories::nameplate_categories::icon,
                    icon,
                ),
            )
            .first::<Self>(conn)
            .await
            .optional()
    }
}
