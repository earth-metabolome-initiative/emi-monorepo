#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(diesel::Selectable, diesel::Queryable, diesel::Identifiable)]
#[diesel(primary_key(id))]
#[diesel(table_name = crate::codegen::diesel_codegen::tables::steps::steps)]
pub struct Step {
    pub id: rosetta_uuid::Uuid,
    pub procedure_id: i32,
    pub step_model_id: i32,
    pub begun_at: rosetta_timestamp::TimestampUTC,
    pub finished_at: rosetta_timestamp::TimestampUTC,
    pub created_by: i32,
    pub created_at: rosetta_timestamp::TimestampUTC,
}
impl Step {
    #[cfg(feature = "postgres")]
    pub async fn procedure(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::procedures::Procedure, diesel::result::Error>
    {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::procedures::Procedure::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedures::procedures::dsl::id
                    .eq(&self.procedure_id),
            )
            .first::<crate::codegen::structs_codegen::tables::procedures::Procedure>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn step_model(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::step_models::StepModel,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::step_models::StepModel::table()
            .filter(
                crate::codegen::diesel_codegen::tables::step_models::step_models::dsl::id
                    .eq(&self.step_model_id),
            )
            .first::<crate::codegen::structs_codegen::tables::step_models::StepModel>(conn)
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
    pub async fn from_procedure_id(
        conn: &mut diesel_async::AsyncPgConnection,
        procedure_id: &crate::codegen::structs_codegen::tables::procedures::Procedure,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::steps::steps::dsl::procedure_id
                    .eq(procedure_id.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_step_model_id(
        conn: &mut diesel_async::AsyncPgConnection,
        step_model_id: &crate::codegen::structs_codegen::tables::step_models::StepModel,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::steps::steps::dsl::step_model_id
                    .eq(step_model_id.id),
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
                crate::codegen::diesel_codegen::tables::steps::steps::dsl::created_by
                    .eq(created_by.id),
            )
            .load::<Self>(conn)
            .await
    }
}
