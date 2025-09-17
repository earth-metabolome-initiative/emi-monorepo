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
        crate::codegen::structs_codegen::tables::commercial_centrifuge_lots::CommercialCentrifugeLot,
        foreign_key = model
    )
)]
#[diesel(primary_key(id))]
#[diesel(table_name = crate::codegen::diesel_codegen::tables::centrifuges::centrifuges)]
pub struct Centrifuge {
    pub id: ::rosetta_uuid::Uuid,
    pub model: i32,
}
impl web_common_traits::prelude::TableName for Centrifuge {
    const TABLE_NAME: &'static str = "centrifuges";
}
impl<'a> From<&'a Centrifuge>
    for web_common_traits::database::IdOrBuilder<
        ::rosetta_uuid::Uuid,
        crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeBuilder,
    >
{
    fn from(value: &'a Centrifuge) -> Self {
        web_common_traits::database::IdOrBuilder::Id(value.id)
    }
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::assets::Asset,
    > for Centrifuge
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a ::rosetta_uuid::Uuid>,
{
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset,
    > for Centrifuge
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a ::rosetta_uuid::Uuid>,
{
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::centrifuges::Centrifuge,
    > for Centrifuge
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a ::rosetta_uuid::Uuid>,
{
}
impl diesel::Identifiable for Centrifuge {
    type Id = ::rosetta_uuid::Uuid;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl web_common_traits::database::PrimaryKeyLike for Centrifuge {
    type PrimaryKey = ::rosetta_uuid::Uuid;
    fn primary_key(&self) -> Self::PrimaryKey {
        self.id
    }
}
impl Centrifuge {
    pub fn model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::commercial_centrifuge_lots::CommercialCentrifugeLot,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::commercial_centrifuge_lots::CommercialCentrifugeLot: web_common_traits::database::Read<
            C,
        >,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::commercial_centrifuge_lots::CommercialCentrifugeLot::read(
            self.model,
            conn,
        )
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
            centrifuges::centrifuges, physical_assets::physical_assets,
        };
        Self::table()
            .inner_join(physical_assets::table.on(centrifuges::id.eq(physical_assets::id)))
            .filter(physical_assets::model.eq(model))
            .order_by(centrifuges::id.asc())
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

        use crate::codegen::diesel_codegen::tables::{assets::assets, centrifuges::centrifuges};
        Self::table()
            .inner_join(assets::table.on(centrifuges::id.eq(assets::id)))
            .filter(assets::name.eq(name).and(assets::model.eq(model)))
            .order_by(centrifuges::id.asc())
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

        use crate::codegen::diesel_codegen::tables::{assets::assets, centrifuges::centrifuges};
        Self::table()
            .inner_join(assets::table.on(centrifuges::id.eq(assets::id)))
            .filter(assets::name.eq(name))
            .order_by(centrifuges::id.asc())
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

        use crate::codegen::diesel_codegen::tables::{assets::assets, centrifuges::centrifuges};
        Self::table()
            .inner_join(assets::table.on(centrifuges::id.eq(assets::id)))
            .filter(assets::description.eq(description))
            .order_by(centrifuges::id.asc())
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

        use crate::codegen::diesel_codegen::tables::{assets::assets, centrifuges::centrifuges};
        Self::table()
            .inner_join(assets::table.on(centrifuges::id.eq(assets::id)))
            .filter(assets::created_by.eq(created_by))
            .order_by(centrifuges::id.asc())
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

        use crate::codegen::diesel_codegen::tables::{assets::assets, centrifuges::centrifuges};
        Self::table()
            .inner_join(assets::table.on(centrifuges::id.eq(assets::id)))
            .filter(assets::updated_by.eq(updated_by))
            .order_by(centrifuges::id.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
}
impl AsRef<Centrifuge> for Centrifuge {
    fn as_ref(&self) -> &Centrifuge {
        self
    }
}
