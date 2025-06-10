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
    table_name = crate::codegen::diesel_codegen::tables::instrument_states::instrument_states
)]
pub struct InstrumentState {
    pub name: String,
    pub description: String,
    pub icon: String,
    pub color_id: i16,
    pub id: i16,
}
impl web_common_traits::prelude::TableName for InstrumentState {
    const TABLE_NAME: &'static str = "instrument_states";
}
impl diesel::Identifiable for InstrumentState {
    type Id = i16;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl InstrumentState {
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
        use crate::codegen::diesel_codegen::tables::instrument_states::instrument_states;
        use diesel::OptionalExtension;
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{ExpressionMethods, QueryDsl};
        Self::table()
            .filter(instrument_states::name.eq(name))
            .order_by(instrument_states::id.asc())
            .first::<Self>(conn)
            .optional()
    }
    #[cfg(feature = "postgres")]
    pub fn from_description(
        description: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use crate::codegen::diesel_codegen::tables::instrument_states::instrument_states;
        use diesel::OptionalExtension;
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{ExpressionMethods, QueryDsl};
        Self::table()
            .filter(instrument_states::description.eq(description))
            .order_by(instrument_states::id.asc())
            .first::<Self>(conn)
            .optional()
    }
    #[cfg(feature = "postgres")]
    pub fn from_icon(
        icon: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use crate::codegen::diesel_codegen::tables::instrument_states::instrument_states;
        use diesel::OptionalExtension;
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{ExpressionMethods, QueryDsl};
        Self::table()
            .filter(instrument_states::icon.eq(icon))
            .order_by(instrument_states::id.asc())
            .first::<Self>(conn)
            .optional()
    }
    #[cfg(feature = "postgres")]
    pub fn from_color_id(
        color_id: &i16,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use crate::codegen::diesel_codegen::tables::instrument_states::instrument_states;
        use diesel::OptionalExtension;
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{ExpressionMethods, QueryDsl};
        Self::table()
            .filter(instrument_states::color_id.eq(color_id))
            .order_by(instrument_states::id.asc())
            .first::<Self>(conn)
            .optional()
    }
}
impl AsRef<InstrumentState> for InstrumentState {
    fn as_ref(&self) -> &InstrumentState {
        self
    }
}
