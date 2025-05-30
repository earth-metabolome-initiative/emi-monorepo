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
    table_name = crate::codegen::diesel_codegen::tables::step_model_instrument_models::step_model_instrument_models
)]
pub struct StepModelInstrumentModel {
    pub id: i32,
    pub instrument_model_id: i32,
    pub created_by: i32,
    pub created_at: ::rosetta_timestamp::TimestampUTC,
    pub updated_by: i32,
    pub updated_at: ::rosetta_timestamp::TimestampUTC,
}
impl diesel::Identifiable for StepModelInstrumentModel {
    type Id = i32;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl StepModelInstrumentModel {
    #[cfg(feature = "postgres")]
    pub async fn id(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::step_model_instrument_categories::StepModelInstrumentCategory,
        diesel::result::Error,
    >{
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::step_model_instrument_categories::StepModelInstrumentCategory::table()
            .filter(
                crate::codegen::diesel_codegen::tables::step_model_instrument_categories::step_model_instrument_categories::dsl::id
                    .eq(&self.id),
            )
            .first::<
                crate::codegen::structs_codegen::tables::step_model_instrument_categories::StepModelInstrumentCategory,
            >(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn instrument_model(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel::table()
            .filter(
                crate::codegen::diesel_codegen::tables::instrument_models::instrument_models::dsl::id
                    .eq(&self.instrument_model_id),
            )
            .first::<
                crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel,
            >(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn created_by(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::users::User, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::users::User::table()
            .filter(
                crate::codegen::diesel_codegen::tables::users::users::dsl::id.eq(&self.created_by),
            )
            .first::<crate::codegen::structs_codegen::tables::users::User>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn updated_by(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::users::User, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::users::User::table()
            .filter(
                crate::codegen::diesel_codegen::tables::users::users::dsl::id.eq(&self.updated_by),
            )
            .first::<crate::codegen::structs_codegen::tables::users::User>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_id(
        conn: &mut diesel_async::AsyncPgConnection,
        id: &crate::codegen::structs_codegen::tables::step_model_instrument_categories::StepModelInstrumentCategory,
    ) -> Result<Self, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::step_model_instrument_models::step_model_instrument_models::dsl::id
                    .eq(id.id),
            )
            .first::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_instrument_model_id(
        conn: &mut diesel_async::AsyncPgConnection,
        instrument_model_id: &crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::step_model_instrument_models::step_model_instrument_models::dsl::instrument_model_id
                    .eq(instrument_model_id.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_created_by(
        conn: &mut diesel_async::AsyncPgConnection,
        created_by: &crate::codegen::structs_codegen::tables::users::User,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::step_model_instrument_models::step_model_instrument_models::dsl::created_by
                    .eq(created_by.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_updated_by(
        conn: &mut diesel_async::AsyncPgConnection,
        updated_by: &crate::codegen::structs_codegen::tables::users::User,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::step_model_instrument_models::step_model_instrument_models::dsl::updated_by
                    .eq(updated_by.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_created_at(
        created_at: &::rosetta_timestamp::TimestampUTC,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;

        use crate::codegen::diesel_codegen::tables::step_model_instrument_models::step_model_instrument_models;
        Self::table()
            .filter(step_model_instrument_models::created_at.eq(created_at))
            .order_by(step_model_instrument_models::id.asc())
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_updated_at(
        updated_at: &::rosetta_timestamp::TimestampUTC,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;

        use crate::codegen::diesel_codegen::tables::step_model_instrument_models::step_model_instrument_models;
        Self::table()
            .filter(step_model_instrument_models::updated_at.eq(updated_at))
            .order_by(step_model_instrument_models::id.asc())
            .load::<Self>(conn)
            .await
    }
}
