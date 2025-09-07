#[derive(Debug, Clone, PartialEq, Copy, Eq, PartialOrd, Ord, Hash)]
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
#[diesel(
    belongs_to(crate::VolumetricContainerModel, foreign_key = volumetric_container_model)
)]
#[diesel(primary_key(id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::volumetric_containers::volumetric_containers
)]
pub struct VolumetricContainer {
    pub id: ::rosetta_uuid::Uuid,
    pub volumetric_container_model: i32,
}
impl web_common_traits::prelude::TableName for VolumetricContainer {
    const TABLE_NAME: &'static str = "volumetric_containers";
}
impl web_common_traits::prelude::ExtensionTable<crate::Asset> for VolumetricContainer where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a ::rosetta_uuid::Uuid>
{
}
impl web_common_traits::prelude::ExtensionTable<crate::Container> for VolumetricContainer where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a ::rosetta_uuid::Uuid>
{
}
impl web_common_traits::prelude::ExtensionTable<crate::PhysicalAsset> for VolumetricContainer where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a ::rosetta_uuid::Uuid>
{
}
impl web_common_traits::prelude::ExtensionTable<crate::VolumetricContainer> for VolumetricContainer where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a ::rosetta_uuid::Uuid>
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
    ) -> Result<crate::Container, diesel::result::Error>
    where
        crate::Container: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::Container::read(self.id, conn)
    }
    #[cfg(feature = "postgres")]
    pub fn volumetric_containers_id_volumetric_container_model_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::Asset, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::Asset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::assets::assets::dsl::id.eq(&self.id).and(
                    crate::codegen::diesel_codegen::tables::assets::assets::dsl::model
                        .eq(&self.volumetric_container_model),
                ),
            )
            .first::<crate::Asset>(conn)
    }
    pub fn volumetric_container_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::VolumetricContainerModel, diesel::result::Error>
    where
        crate::VolumetricContainerModel: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::VolumetricContainerModel::read(self.volumetric_container_model, conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_id(
        id: &::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::volumetric_containers::volumetric_containers;
        Self::table()
            .filter(volumetric_containers::id.eq(id))
            .order_by(volumetric_containers::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_id_and_volumetric_container_model(
        id: &::rosetta_uuid::Uuid,
        volumetric_container_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::volumetric_containers::volumetric_containers;
        Self::table()
            .filter(volumetric_containers::id.eq(id).and(
                volumetric_containers::volumetric_container_model.eq(volumetric_container_model),
            ))
            .order_by(volumetric_containers::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_container_model(
        container_model: &i32,
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
            .filter(containers::container_model.eq(container_model))
            .order_by(volumetric_containers::id.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_model(
        model: &i32,
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
            .filter(physical_assets::model.eq(model))
            .order_by(volumetric_containers::id.asc())
            .select(Self::as_select())
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

        use crate::codegen::diesel_codegen::tables::{
            assets::assets, volumetric_containers::volumetric_containers,
        };
        Self::table()
            .inner_join(assets::table.on(volumetric_containers::id.eq(assets::id)))
            .filter(assets::model.eq(model).and(assets::id.eq(id)))
            .order_by(volumetric_containers::id.asc())
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

        use crate::codegen::diesel_codegen::tables::{
            assets::assets, volumetric_containers::volumetric_containers,
        };
        Self::table()
            .inner_join(assets::table.on(volumetric_containers::id.eq(assets::id)))
            .filter(assets::name.eq(name).and(assets::model.eq(model)))
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
    pub fn from_name(
        name: &str,
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
            .filter(assets::name.eq(name))
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
