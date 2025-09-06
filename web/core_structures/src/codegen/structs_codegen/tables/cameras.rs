#[derive(Debug, Clone, PartialEq, Copy, Eq, PartialOrd, Ord, Hash)]
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
#[diesel(table_name = crate::codegen::diesel_codegen::tables::cameras::cameras)]
pub struct Camera {
    pub id: ::rosetta_uuid::Uuid,
    pub model: i32,
}
impl web_common_traits::prelude::TableName for Camera {
    const TABLE_NAME: &'static str = "cameras";
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::assets::Asset,
    > for Camera
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a ::rosetta_uuid::Uuid>,
{
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset,
    > for Camera
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a ::rosetta_uuid::Uuid>,
{
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::cameras::Camera,
    > for Camera
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a ::rosetta_uuid::Uuid>,
{
}
impl diesel::Identifiable for Camera {
    type Id = ::rosetta_uuid::Uuid;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl Camera {
    pub fn id<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset::table(),
                self.id,
            ),
            conn,
        )
    }
    pub fn model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::commercial_camera_lots::CommercialCameraLot,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::commercial_camera_lots::CommercialCameraLot: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::commercial_camera_lots::CommercialCameraLot as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::commercial_camera_lots::CommercialCameraLot as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::commercial_camera_lots::CommercialCameraLot as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::commercial_camera_lots::CommercialCameraLot as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::commercial_camera_lots::CommercialCameraLot as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::commercial_camera_lots::CommercialCameraLot as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::commercial_camera_lots::CommercialCameraLot,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::commercial_camera_lots::CommercialCameraLot::table(),
                self.model,
            ),
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn cameras_id_model_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::assets::Asset, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::assets::Asset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::assets::assets::dsl::id.eq(&self.id).and(
                    crate::codegen::diesel_codegen::tables::assets::assets::dsl::model
                        .eq(&self.model),
                ),
            )
            .first::<crate::codegen::structs_codegen::tables::assets::Asset>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_model(
        model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::cameras::cameras;
        Self::table()
            .filter(cameras::model.eq(model))
            .order_by(cameras::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_id_and_model(
        id: &::rosetta_uuid::Uuid,
        model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::cameras::cameras;
        Self::table()
            .filter(cameras::id.eq(id).and(cameras::model.eq(model)))
            .order_by(cameras::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_model_and_id(
        model: &i32,
        id: &::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl,
            SelectableHelper, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{assets::assets, cameras::cameras};
        Self::table()
            .inner_join(assets::table.on(cameras::id.eq(assets::id)))
            .filter(assets::model.eq(model).and(assets::id.eq(id)))
            .order_by(cameras::id.asc())
            .select(Self::as_select())
            .first::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_name_and_model(
        name: &str,
        model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl,
            SelectableHelper, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{assets::assets, cameras::cameras};
        Self::table()
            .inner_join(assets::table.on(cameras::id.eq(assets::id)))
            .filter(assets::name.eq(name).and(assets::model.eq(model)))
            .order_by(cameras::id.asc())
            .select(Self::as_select())
            .first::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_most_concrete_table(
        most_concrete_table: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{assets::assets, cameras::cameras};
        Self::table()
            .inner_join(assets::table.on(cameras::id.eq(assets::id)))
            .filter(assets::most_concrete_table.eq(most_concrete_table))
            .order_by(cameras::id.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_name(
        name: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{assets::assets, cameras::cameras};
        Self::table()
            .inner_join(assets::table.on(cameras::id.eq(assets::id)))
            .filter(assets::name.eq(name))
            .order_by(cameras::id.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
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

        use crate::codegen::diesel_codegen::tables::{assets::assets, cameras::cameras};
        Self::table()
            .inner_join(assets::table.on(cameras::id.eq(assets::id)))
            .filter(assets::description.eq(description))
            .order_by(cameras::id.asc())
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

        use crate::codegen::diesel_codegen::tables::{assets::assets, cameras::cameras};
        Self::table()
            .inner_join(assets::table.on(cameras::id.eq(assets::id)))
            .filter(assets::created_by.eq(created_by))
            .order_by(cameras::id.asc())
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

        use crate::codegen::diesel_codegen::tables::{assets::assets, cameras::cameras};
        Self::table()
            .inner_join(assets::table.on(cameras::id.eq(assets::id)))
            .filter(assets::created_at.eq(created_at))
            .order_by(cameras::id.asc())
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

        use crate::codegen::diesel_codegen::tables::{assets::assets, cameras::cameras};
        Self::table()
            .inner_join(assets::table.on(cameras::id.eq(assets::id)))
            .filter(assets::updated_by.eq(updated_by))
            .order_by(cameras::id.asc())
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

        use crate::codegen::diesel_codegen::tables::{assets::assets, cameras::cameras};
        Self::table()
            .inner_join(assets::table.on(cameras::id.eq(assets::id)))
            .filter(assets::updated_at.eq(updated_at))
            .order_by(cameras::id.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
}
impl AsRef<Camera> for Camera {
    fn as_ref(&self) -> &Camera {
        self
    }
}
