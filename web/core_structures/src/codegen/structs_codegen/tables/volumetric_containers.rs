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
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::volumetric_containers::volumetric_containers
)]
pub struct VolumetricContainer {
    pub id: ::rosetta_uuid::Uuid,
    pub volumetric_container_model_id: i32,
}
impl web_common_traits::prelude::TableName for VolumetricContainer {
    const TABLE_NAME: &'static str = "volumetric_containers";
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::assets::Asset,
    > for VolumetricContainer
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a ::rosetta_uuid::Uuid>,
{
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::containers::Container,
    > for VolumetricContainer
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a ::rosetta_uuid::Uuid>,
{
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset,
    > for VolumetricContainer
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a ::rosetta_uuid::Uuid>,
{
}
impl diesel::Identifiable for VolumetricContainer {
    type Id = ::rosetta_uuid::Uuid;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl VolumetricContainer {
    pub fn id<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::containers::Container,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::containers::Container: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::containers::Container as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::containers::Container as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::containers::Container as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::containers::Container as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::containers::Container as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::containers::Container as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::containers::Container,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::containers::Container::table(),
                self.id,
            ),
            conn,
        )
    }
    pub fn volumetric_container_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel::table(),
                self.volumetric_container_model_id,
            ),
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn from_volumetric_container_model_id(
        volumetric_container_model_id: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::volumetric_containers::volumetric_containers;
        Self::table()
            .filter(
                volumetric_containers::volumetric_container_model_id
                    .eq(volumetric_container_model_id),
            )
            .order_by(volumetric_containers::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_id_and_volumetric_container_model_id(
        id: &::rosetta_uuid::Uuid,
        volumetric_container_model_id: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::volumetric_containers::volumetric_containers;
        Self::table()
            .filter(
                volumetric_containers::id.eq(id).and(
                    volumetric_containers::volumetric_container_model_id
                        .eq(volumetric_container_model_id),
                ),
            )
            .order_by(volumetric_containers::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_container_model_id(
        container_model_id: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            containers::containers, volumetric_containers::volumetric_containers,
        };
        Self::table()
            .inner_join(containers::table.on(volumetric_containers::id.eq(containers::id)))
            .filter(containers::container_model_id.eq(container_model_id))
            .order_by(volumetric_containers::id.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_model_id(
        model_id: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            physical_assets::physical_assets, volumetric_containers::volumetric_containers,
        };
        Self::table()
            .inner_join(
                physical_assets::table.on(volumetric_containers::id.eq(physical_assets::id)),
            )
            .filter(physical_assets::model_id.eq(model_id))
            .order_by(volumetric_containers::id.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_model_id_and_id(
        model_id: &i32,
        id: &::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl,
            SelectableHelper, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            assets::assets, volumetric_containers::volumetric_containers,
        };
        Self::table()
            .inner_join(assets::table.on(volumetric_containers::id.eq(assets::id)))
            .filter(assets::model_id.eq(model_id).and(assets::id.eq(id)))
            .order_by(volumetric_containers::id.asc())
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

        use crate::codegen::diesel_codegen::tables::{
            assets::assets, volumetric_containers::volumetric_containers,
        };
        Self::table()
            .inner_join(assets::table.on(volumetric_containers::id.eq(assets::id)))
            .filter(assets::most_concrete_table.eq(most_concrete_table))
            .order_by(volumetric_containers::id.asc())
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

        use crate::codegen::diesel_codegen::tables::{
            assets::assets, volumetric_containers::volumetric_containers,
        };
        Self::table()
            .inner_join(assets::table.on(volumetric_containers::id.eq(assets::id)))
            .filter(assets::description.eq(description))
            .order_by(volumetric_containers::id.asc())
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
            assets::assets, volumetric_containers::volumetric_containers,
        };
        Self::table()
            .inner_join(assets::table.on(volumetric_containers::id.eq(assets::id)))
            .filter(assets::created_by.eq(created_by))
            .order_by(volumetric_containers::id.asc())
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
            assets::assets, volumetric_containers::volumetric_containers,
        };
        Self::table()
            .inner_join(assets::table.on(volumetric_containers::id.eq(assets::id)))
            .filter(assets::created_at.eq(created_at))
            .order_by(volumetric_containers::id.asc())
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
            assets::assets, volumetric_containers::volumetric_containers,
        };
        Self::table()
            .inner_join(assets::table.on(volumetric_containers::id.eq(assets::id)))
            .filter(assets::updated_by.eq(updated_by))
            .order_by(volumetric_containers::id.asc())
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
            assets::assets, volumetric_containers::volumetric_containers,
        };
        Self::table()
            .inner_join(assets::table.on(volumetric_containers::id.eq(assets::id)))
            .filter(assets::updated_at.eq(updated_at))
            .order_by(volumetric_containers::id.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
}
impl AsRef<VolumetricContainer> for VolumetricContainer {
    fn as_ref(&self) -> &VolumetricContainer {
        self
    }
}
