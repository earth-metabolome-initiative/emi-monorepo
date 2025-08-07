#[derive(Debug, Clone, PartialEq, Copy)]
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
    table_name = crate::codegen::diesel_codegen::tables::volumetric_processables::volumetric_processables
)]
pub struct VolumetricProcessable {
    pub id: ::rosetta_uuid::Uuid,
    pub liters: f32,
}
impl web_common_traits::prelude::TableName for VolumetricProcessable {
    const TABLE_NAME: &'static str = "volumetric_processables";
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::processables::Processable,
    > for VolumetricProcessable
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a ::rosetta_uuid::Uuid>,
{
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::trackables::Trackable,
    > for VolumetricProcessable
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a ::rosetta_uuid::Uuid>,
{
}
impl diesel::Identifiable for VolumetricProcessable {
    type Id = ::rosetta_uuid::Uuid;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl VolumetricProcessable {
    pub fn id<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::processables::Processable,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::processables::Processable: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::processables::Processable as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::processables::Processable as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::processables::Processable as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::processables::Processable as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::processables::Processable as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::processables::Processable as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::processables::Processable,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::processables::Processable::table(),
                self.id,
            ),
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn from_name(
        name: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, OptionalExtension, QueryDsl, RunQueryDsl,
            SelectableHelper, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            trackables::trackables, volumetric_processables::volumetric_processables,
        };
        Self::table()
            .inner_join(trackables::table.on(volumetric_processables::id.eq(trackables::id)))
            .filter(trackables::name.eq(name))
            .order_by(volumetric_processables::id.asc())
            .select(Self::as_select())
            .first::<Self>(conn)
            .optional()
    }
    #[cfg(feature = "postgres")]
    pub fn from_description(
        description: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            trackables::trackables, volumetric_processables::volumetric_processables,
        };
        Self::table()
            .inner_join(trackables::table.on(volumetric_processables::id.eq(trackables::id)))
            .filter(trackables::description.eq(description))
            .order_by(volumetric_processables::id.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_photograph_id(
        photograph_id: &::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            trackables::trackables, volumetric_processables::volumetric_processables,
        };
        Self::table()
            .inner_join(trackables::table.on(volumetric_processables::id.eq(trackables::id)))
            .filter(trackables::photograph_id.eq(photograph_id))
            .order_by(volumetric_processables::id.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_created_by(
        created_by: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            trackables::trackables, volumetric_processables::volumetric_processables,
        };
        Self::table()
            .inner_join(trackables::table.on(volumetric_processables::id.eq(trackables::id)))
            .filter(trackables::created_by.eq(created_by))
            .order_by(volumetric_processables::id.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_created_at(
        created_at: &::rosetta_timestamp::TimestampUTC,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            trackables::trackables, volumetric_processables::volumetric_processables,
        };
        Self::table()
            .inner_join(trackables::table.on(volumetric_processables::id.eq(trackables::id)))
            .filter(trackables::created_at.eq(created_at))
            .order_by(volumetric_processables::id.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_updated_by(
        updated_by: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            trackables::trackables, volumetric_processables::volumetric_processables,
        };
        Self::table()
            .inner_join(trackables::table.on(volumetric_processables::id.eq(trackables::id)))
            .filter(trackables::updated_by.eq(updated_by))
            .order_by(volumetric_processables::id.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_updated_at(
        updated_at: &::rosetta_timestamp::TimestampUTC,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            trackables::trackables, volumetric_processables::volumetric_processables,
        };
        Self::table()
            .inner_join(trackables::table.on(volumetric_processables::id.eq(trackables::id)))
            .filter(trackables::updated_at.eq(updated_at))
            .order_by(volumetric_processables::id.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
}
impl AsRef<VolumetricProcessable> for VolumetricProcessable {
    fn as_ref(&self) -> &VolumetricProcessable {
        self
    }
}
