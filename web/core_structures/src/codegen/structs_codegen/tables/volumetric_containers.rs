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
    belongs_to(
        crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel,
        foreign_key = volumetric_container_model
    )
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
impl<'a> From<&'a VolumetricContainer>
    for web_common_traits::database::IdOrBuilder<
        ::rosetta_uuid::Uuid,
        crate::codegen::structs_codegen::tables::insertables::InsertableVolumetricContainerBuilder,
    >
{
    fn from(value: &'a VolumetricContainer) -> Self {
        web_common_traits::database::IdOrBuilder::Id(value.id)
    }
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
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer,
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
impl web_common_traits::database::PrimaryKeyLike for VolumetricContainer {
    type PrimaryKey = ::rosetta_uuid::Uuid;
    fn primary_key(&self) -> Self::PrimaryKey {
        self.id
    }
}
impl VolumetricContainer {
    pub fn volumetric_container_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel: web_common_traits::database::Read<
            C,
        >,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel::read(
            self.volumetric_container_model,
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn from_container_model(
        container_model: i32,
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
        model: i32,
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
    pub fn from_name_and_model(
        name: &str,
        model: i32,
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
        created_by: i32,
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
    pub fn from_updated_by(
        updated_by: i32,
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
}
impl AsRef<VolumetricContainer> for VolumetricContainer {
    fn as_ref(&self) -> &VolumetricContainer {
        self
    }
}
