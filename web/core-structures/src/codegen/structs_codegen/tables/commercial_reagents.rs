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
    table_name = crate::codegen::diesel_codegen::tables::commercial_reagents::commercial_reagents
)]
pub struct CommercialReagent {
    pub id: rosetta_uuid::Uuid,
    pub commercial_product_lot_id: i32,
}
impl diesel::Identifiable for CommercialReagent {
    type Id = rosetta_uuid::Uuid;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl CommercialReagent {
    #[cfg(feature = "postgres")]
    pub async fn id(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::processables::Processable,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::processables::Processable::table()
            .filter(
                crate::codegen::diesel_codegen::tables::processables::processables::dsl::id
                    .eq(&self.id),
            )
            .first::<crate::codegen::structs_codegen::tables::processables::Processable>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn commercial_product_lot(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot::table()
            .filter(
                crate::codegen::diesel_codegen::tables::commercial_product_lots::commercial_product_lots::dsl::id
                    .eq(&self.commercial_product_lot_id),
            )
            .first::<
                crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot,
            >(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_id(
        conn: &mut diesel_async::AsyncPgConnection,
        id: &crate::codegen::structs_codegen::tables::processables::Processable,
    ) -> Result<Self, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::commercial_reagents::commercial_reagents::dsl::id
                    .eq(id.id),
            )
            .first::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_commercial_product_lot_id(
        conn: &mut diesel_async::AsyncPgConnection,
        commercial_product_lot_id: &crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::commercial_reagents::commercial_reagents::dsl::commercial_product_lot_id
                    .eq(commercial_product_lot_id.id),
            )
            .load::<Self>(conn)
            .await
    }
}
