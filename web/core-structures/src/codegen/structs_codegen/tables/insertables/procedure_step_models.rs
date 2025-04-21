#[derive(Clone, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableProcedureStepModelAttributes {
    ProcedureModelId,
    StepModelId,
    NextProcedureStepModelId,
    PrevProcedureStepModelId,
    CreatedBy,
    CreatedAt,
    UpdatedBy,
    UpdatedAt,
}
impl core::fmt::Display for InsertableProcedureStepModelAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableProcedureStepModelAttributes::ProcedureModelId => {
                write!(f, "procedure_model_id")
            }
            InsertableProcedureStepModelAttributes::StepModelId => {
                write!(f, "step_model_id")
            }
            InsertableProcedureStepModelAttributes::NextProcedureStepModelId => {
                write!(f, "next_procedure_step_model_id")
            }
            InsertableProcedureStepModelAttributes::PrevProcedureStepModelId => {
                write!(f, "prev_procedure_step_model_id")
            }
            InsertableProcedureStepModelAttributes::CreatedBy => write!(f, "created_by"),
            InsertableProcedureStepModelAttributes::CreatedAt => write!(f, "created_at"),
            InsertableProcedureStepModelAttributes::UpdatedBy => write!(f, "updated_by"),
            InsertableProcedureStepModelAttributes::UpdatedAt => write!(f, "updated_at"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::procedure_step_models::procedure_step_models
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableProcedureStepModel {
    procedure_model_id: i32,
    step_model_id: i32,
    next_procedure_step_model_id: Option<i32>,
    prev_procedure_step_model_id: Option<i32>,
    created_by: i32,
    created_at: rosetta_timestamp::TimestampUTC,
    updated_by: i32,
    updated_at: rosetta_timestamp::TimestampUTC,
}
impl InsertableProcedureStepModel {
    #[cfg(feature = "postgres")]
    pub async fn procedure_model(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_models::procedure_models::dsl::id
                    .eq(&self.procedure_model_id),
            )
            .first::<crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel>(
                conn,
            )
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
    pub async fn next_procedure_step_model(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::procedure_step_models::ProcedureStepModel>,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        let Some(next_procedure_step_model_id) = self.next_procedure_step_model_id.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::procedure_step_models::ProcedureStepModel::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_step_models::procedure_step_models::dsl::id
                    .eq(next_procedure_step_model_id),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_step_models::ProcedureStepModel,
            >(conn)
            .await
            .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub async fn prev_procedure_step_model(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::procedure_step_models::ProcedureStepModel>,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        let Some(prev_procedure_step_model_id) = self.prev_procedure_step_model_id.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::procedure_step_models::ProcedureStepModel::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_step_models::procedure_step_models::dsl::id
                    .eq(prev_procedure_step_model_id),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_step_models::ProcedureStepModel,
            >(conn)
            .await
            .map(Some)
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
}
#[derive(Default)]
pub struct InsertableProcedureStepModelBuilder {
    procedure_model_id: Option<i32>,
    step_model_id: Option<i32>,
    next_procedure_step_model_id: Option<i32>,
    prev_procedure_step_model_id: Option<i32>,
    created_by: Option<i32>,
    created_at: Option<rosetta_timestamp::TimestampUTC>,
    updated_by: Option<i32>,
    updated_at: Option<rosetta_timestamp::TimestampUTC>,
}
impl InsertableProcedureStepModelBuilder {
    pub fn procedure_model_id(
        mut self,
        procedure_model_id: i32,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.procedure_model_id = Some(procedure_model_id);
        Ok(self)
    }
    pub fn step_model_id(
        mut self,
        step_model_id: i32,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.step_model_id = Some(step_model_id);
        Ok(self)
    }
    pub fn next_procedure_step_model_id(
        mut self,
        next_procedure_step_model_id: Option<i32>,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        if let (Some(next_procedure_step_model_id), Some(prev_procedure_step_model_id)) =
            (next_procedure_step_model_id, self.prev_procedure_step_model_id)
        {
            pgrx_validation::must_be_distinct_i32(
                next_procedure_step_model_id,
                prev_procedure_step_model_id,
            )
            .map_err(|e| {
                e.rename_fields(
                    InsertableProcedureStepModelAttributes::NextProcedureStepModelId,
                    InsertableProcedureStepModelAttributes::PrevProcedureStepModelId,
                )
            })?;
        }
        self.next_procedure_step_model_id = next_procedure_step_model_id;
        Ok(self)
    }
    pub fn prev_procedure_step_model_id(
        mut self,
        prev_procedure_step_model_id: Option<i32>,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        if let (Some(next_procedure_step_model_id), Some(prev_procedure_step_model_id)) =
            (self.next_procedure_step_model_id, prev_procedure_step_model_id)
        {
            pgrx_validation::must_be_distinct_i32(
                next_procedure_step_model_id,
                prev_procedure_step_model_id,
            )
            .map_err(|e| {
                e.rename_fields(
                    InsertableProcedureStepModelAttributes::NextProcedureStepModelId,
                    InsertableProcedureStepModelAttributes::PrevProcedureStepModelId,
                )
            })?;
        }
        self.prev_procedure_step_model_id = prev_procedure_step_model_id;
        Ok(self)
    }
    pub fn created_by(
        mut self,
        created_by: i32,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.created_by = Some(created_by);
        Ok(self)
    }
    pub fn created_at(
        mut self,
        created_at: rosetta_timestamp::TimestampUTC,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.created_at = Some(created_at);
        Ok(self)
    }
    pub fn updated_by(
        mut self,
        updated_by: i32,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.updated_by = Some(updated_by);
        Ok(self)
    }
    pub fn updated_at(
        mut self,
        updated_at: rosetta_timestamp::TimestampUTC,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.updated_at = Some(updated_at);
        Ok(self)
    }
}
impl common_traits::prelude::Builder for InsertableProcedureStepModelBuilder {
    type Error = web_common_traits::database::InsertError<InsertableProcedureStepModelAttributes>;
    type Object = InsertableProcedureStepModel;
    type Attribute = InsertableProcedureStepModelAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            procedure_model_id: self.procedure_model_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableProcedureStepModelAttributes::ProcedureModelId,
                ),
            )?,
            step_model_id: self.step_model_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableProcedureStepModelAttributes::StepModelId,
                ),
            )?,
            next_procedure_step_model_id: self.next_procedure_step_model_id,
            prev_procedure_step_model_id: self.prev_procedure_step_model_id,
            created_by: self.created_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableProcedureStepModelAttributes::CreatedBy,
                ),
            )?,
            created_at: self.created_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableProcedureStepModelAttributes::CreatedAt,
                ),
            )?,
            updated_by: self.updated_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableProcedureStepModelAttributes::UpdatedBy,
                ),
            )?,
            updated_at: self.updated_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableProcedureStepModelAttributes::UpdatedAt,
                ),
            )?,
        })
    }
}
impl TryFrom<InsertableProcedureStepModel> for InsertableProcedureStepModelBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableProcedureStepModel) -> Result<Self, Self::Error> {
        Self::default()
            .procedure_model_id(insertable_variant.procedure_model_id)?
            .step_model_id(insertable_variant.step_model_id)?
            .next_procedure_step_model_id(insertable_variant.next_procedure_step_model_id)?
            .prev_procedure_step_model_id(insertable_variant.prev_procedure_step_model_id)?
            .created_by(insertable_variant.created_by)?
            .created_at(insertable_variant.created_at)?
            .updated_by(insertable_variant.updated_by)?
            .updated_at(insertable_variant.updated_at)
    }
}
