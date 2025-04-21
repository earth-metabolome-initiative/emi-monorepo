#[derive(Clone, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableStepAttributes {
    Id,
    ProcedureId,
    StepModelId,
    BegunAt,
    FinishedAt,
    CreatedBy,
    CreatedAt,
}
impl core::fmt::Display for InsertableStepAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableStepAttributes::Id => write!(f, "id"),
            InsertableStepAttributes::ProcedureId => write!(f, "procedure_id"),
            InsertableStepAttributes::StepModelId => write!(f, "step_model_id"),
            InsertableStepAttributes::BegunAt => write!(f, "begun_at"),
            InsertableStepAttributes::FinishedAt => write!(f, "finished_at"),
            InsertableStepAttributes::CreatedBy => write!(f, "created_by"),
            InsertableStepAttributes::CreatedAt => write!(f, "created_at"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(table_name = crate::codegen::diesel_codegen::tables::steps::steps)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableStep {
    id: rosetta_uuid::Uuid,
    procedure_id: i32,
    step_model_id: i32,
    begun_at: rosetta_timestamp::TimestampUTC,
    finished_at: rosetta_timestamp::TimestampUTC,
    created_by: i32,
    created_at: rosetta_timestamp::TimestampUTC,
}
impl InsertableStep {
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
}
#[derive(Default)]
pub struct InsertableStepBuilder {
    id: Option<rosetta_uuid::Uuid>,
    procedure_id: Option<i32>,
    step_model_id: Option<i32>,
    begun_at: Option<rosetta_timestamp::TimestampUTC>,
    finished_at: Option<rosetta_timestamp::TimestampUTC>,
    created_by: Option<i32>,
    created_at: Option<rosetta_timestamp::TimestampUTC>,
}
impl InsertableStepBuilder {
    pub fn id(
        mut self,
        id: rosetta_uuid::Uuid,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.id = Some(id);
        Ok(self)
    }
    pub fn procedure_id(
        mut self,
        procedure_id: i32,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.procedure_id = Some(procedure_id);
        Ok(self)
    }
    pub fn step_model_id(
        mut self,
        step_model_id: i32,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.step_model_id = Some(step_model_id);
        Ok(self)
    }
    pub fn begun_at(
        mut self,
        begun_at: rosetta_timestamp::TimestampUTC,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.begun_at = Some(begun_at);
        Ok(self)
    }
    pub fn finished_at(
        mut self,
        finished_at: rosetta_timestamp::TimestampUTC,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.finished_at = Some(finished_at);
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
}
impl common_traits::prelude::Builder for InsertableStepBuilder {
    type Error = web_common_traits::database::InsertError<InsertableStepAttributes>;
    type Object = InsertableStep;
    type Attribute = InsertableStepAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            id: self.id.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableStepAttributes::Id,
            ))?,
            procedure_id: self.procedure_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableStepAttributes::ProcedureId,
                ),
            )?,
            step_model_id: self.step_model_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableStepAttributes::StepModelId,
                ),
            )?,
            begun_at: self.begun_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableStepAttributes::BegunAt,
                ),
            )?,
            finished_at: self.finished_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableStepAttributes::FinishedAt,
                ),
            )?,
            created_by: self.created_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableStepAttributes::CreatedBy,
                ),
            )?,
            created_at: self.created_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableStepAttributes::CreatedAt,
                ),
            )?,
        })
    }
}
impl TryFrom<InsertableStep> for InsertableStepBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableStep) -> Result<Self, Self::Error> {
        Self::default()
            .id(insertable_variant.id)?
            .procedure_id(insertable_variant.procedure_id)?
            .step_model_id(insertable_variant.step_model_id)?
            .begun_at(insertable_variant.begun_at)?
            .finished_at(insertable_variant.finished_at)?
            .created_by(insertable_variant.created_by)?
            .created_at(insertable_variant.created_at)
    }
}
