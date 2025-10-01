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
        crate::codegen::structs_codegen::tables::placing_procedure_templates::PlacingProcedureTemplate,
        foreign_key = procedure_template
    )
)]
#[diesel(primary_key(procedure))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::placing_procedures::placing_procedures
)]
pub struct PlacingProcedure {
    pub procedure: ::rosetta_uuid::Uuid,
    pub procedure_template: i32,
}
impl web_common_traits::prelude::TableName for PlacingProcedure {
    const TABLE_NAME: &'static str = "placing_procedures";
}
impl<'a> From<&'a PlacingProcedure>
    for web_common_traits::database::IdOrBuilder<
        ::rosetta_uuid::Uuid,
        crate::codegen::structs_codegen::tables::insertables::InsertablePlacingProcedureBuilder,
    >
{
    fn from(value: &'a PlacingProcedure) -> Self {
        web_common_traits::database::IdOrBuilder::Id(value.procedure)
    }
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::geolocation_procedures::GeolocationProcedure,
    > for PlacingProcedure
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a ::rosetta_uuid::Uuid>,
{
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::procedures::Procedure,
    > for PlacingProcedure
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a ::rosetta_uuid::Uuid>,
{
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::placing_procedures::PlacingProcedure,
    > for PlacingProcedure
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a ::rosetta_uuid::Uuid>,
{
}
impl diesel::Identifiable for PlacingProcedure {
    type Id = ::rosetta_uuid::Uuid;
    fn id(self) -> Self::Id {
        self.procedure
    }
}
impl web_common_traits::database::PrimaryKeyLike for PlacingProcedure {
    type PrimaryKey = ::rosetta_uuid::Uuid;
    fn primary_key(&self) -> Self::PrimaryKey {
        self.procedure
    }
}
impl PlacingProcedure {
    pub fn procedure_template<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::placing_procedure_templates::PlacingProcedureTemplate,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::placing_procedure_templates::PlacingProcedureTemplate: web_common_traits::database::Read<
            C,
        >,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::placing_procedure_templates::PlacingProcedureTemplate::read(
            self.procedure_template,
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template(
        procedure_template: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            geolocation_procedures::geolocation_procedures, placing_procedures::placing_procedures,
        };
        Self::table()
            .inner_join(
                geolocation_procedures::table
                    .on(placing_procedures::procedure.eq(geolocation_procedures::procedure)),
            )
            .filter(geolocation_procedures::procedure_template.eq(procedure_template))
            .order_by(placing_procedures::procedure.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_geolocated_asset(
        geolocated_asset: ::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            geolocation_procedures::geolocation_procedures, placing_procedures::placing_procedures,
        };
        Self::table()
            .inner_join(
                geolocation_procedures::table
                    .on(placing_procedures::procedure.eq(geolocation_procedures::procedure)),
            )
            .filter(geolocation_procedures::geolocated_asset.eq(geolocated_asset))
            .order_by(placing_procedures::procedure.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_geolocated_asset_model(
        procedure_template_geolocated_asset_model: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            geolocation_procedures::geolocation_procedures, placing_procedures::placing_procedures,
        };
        Self::table()
            .inner_join(
                geolocation_procedures::table
                    .on(placing_procedures::procedure.eq(geolocation_procedures::procedure)),
            )
            .filter(
                geolocation_procedures::procedure_template_geolocated_asset_model
                    .eq(procedure_template_geolocated_asset_model),
            )
            .order_by(placing_procedures::procedure.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_geolocated_asset(
        procedure_geolocated_asset: ::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            geolocation_procedures::geolocation_procedures, placing_procedures::placing_procedures,
        };
        Self::table()
            .inner_join(
                geolocation_procedures::table
                    .on(placing_procedures::procedure.eq(geolocation_procedures::procedure)),
            )
            .filter(
                geolocation_procedures::procedure_geolocated_asset.eq(procedure_geolocated_asset),
            )
            .order_by(placing_procedures::procedure.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_geolocated_with(
        geolocated_with: ::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            geolocation_procedures::geolocation_procedures, placing_procedures::placing_procedures,
        };
        Self::table()
            .inner_join(
                geolocation_procedures::table
                    .on(placing_procedures::procedure.eq(geolocation_procedures::procedure)),
            )
            .filter(geolocation_procedures::geolocated_with.eq(geolocated_with))
            .order_by(placing_procedures::procedure.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_geolocated_with(
        procedure_geolocated_with: ::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            geolocation_procedures::geolocation_procedures, placing_procedures::placing_procedures,
        };
        Self::table()
            .inner_join(
                geolocation_procedures::table
                    .on(placing_procedures::procedure.eq(geolocation_procedures::procedure)),
            )
            .filter(geolocation_procedures::procedure_geolocated_with.eq(procedure_geolocated_with))
            .order_by(placing_procedures::procedure.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_geolocated_with_model(
        procedure_template_geolocated_with_model: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            geolocation_procedures::geolocation_procedures, placing_procedures::placing_procedures,
        };
        Self::table()
            .inner_join(
                geolocation_procedures::table
                    .on(placing_procedures::procedure.eq(geolocation_procedures::procedure)),
            )
            .filter(
                geolocation_procedures::procedure_template_geolocated_with_model
                    .eq(procedure_template_geolocated_with_model),
            )
            .order_by(placing_procedures::procedure.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_parent_procedure(
        parent_procedure: ::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            placing_procedures::placing_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(placing_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::parent_procedure.eq(parent_procedure))
            .order_by(placing_procedures::procedure.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_parent_procedure_template(
        parent_procedure_template: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            placing_procedures::placing_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(placing_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::parent_procedure_template.eq(parent_procedure_template))
            .order_by(placing_procedures::procedure.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_predecessor_procedure(
        predecessor_procedure: ::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            placing_procedures::placing_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(placing_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::predecessor_procedure.eq(predecessor_procedure))
            .order_by(placing_procedures::procedure.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_predecessor_procedure_template(
        predecessor_procedure_template: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            placing_procedures::placing_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(placing_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::predecessor_procedure_template.eq(predecessor_procedure_template))
            .order_by(placing_procedures::procedure.asc())
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
            placing_procedures::placing_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(placing_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::created_by.eq(created_by))
            .order_by(placing_procedures::procedure.asc())
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
            placing_procedures::placing_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(placing_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::updated_by.eq(updated_by))
            .order_by(placing_procedures::procedure.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
}
impl AsRef<PlacingProcedure> for PlacingProcedure {
    fn as_ref(&self) -> &PlacingProcedure {
        self
    }
}
