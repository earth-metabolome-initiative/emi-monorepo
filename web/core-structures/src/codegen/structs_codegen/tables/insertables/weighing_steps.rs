#[derive(Clone, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableWeighingStepAttributes {
    Id,
    ProcessableId,
    WeighingStepModelId,
    InstrumentId,
    Kilograms,
    CreatedBy,
    CreatedAt,
}
impl core::fmt::Display for InsertableWeighingStepAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableWeighingStepAttributes::Id => write!(f, "id"),
            InsertableWeighingStepAttributes::ProcessableId => {
                write!(f, "processable_id")
            }
            InsertableWeighingStepAttributes::WeighingStepModelId => {
                write!(f, "weighing_step_model_id")
            }
            InsertableWeighingStepAttributes::InstrumentId => write!(f, "instrument_id"),
            InsertableWeighingStepAttributes::Kilograms => write!(f, "kilograms"),
            InsertableWeighingStepAttributes::CreatedBy => write!(f, "created_by"),
            InsertableWeighingStepAttributes::CreatedAt => write!(f, "created_at"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::weighing_steps::weighing_steps
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableWeighingStep {
    id: rosetta_uuid::Uuid,
    processable_id: rosetta_uuid::Uuid,
    weighing_step_model_id: i32,
    instrument_id: i32,
    kilograms: f32,
    created_by: i32,
    created_at: rosetta_timestamp::TimestampUTC,
}
impl InsertableWeighingStep {
    #[cfg(feature = "postgres")]
    pub async fn id(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::steps::Step, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::steps::Step::table()
            .filter(crate::codegen::diesel_codegen::tables::steps::steps::dsl::id.eq(&self.id))
            .first::<crate::codegen::structs_codegen::tables::steps::Step>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn processable(
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
                    .eq(&self.processable_id),
            )
            .first::<crate::codegen::structs_codegen::tables::processables::Processable>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn weighing_step_model(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::weighing_step_models::WeighingStepModel,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::weighing_step_models::WeighingStepModel::table()
            .filter(
                crate::codegen::diesel_codegen::tables::weighing_step_models::weighing_step_models::dsl::id
                    .eq(&self.weighing_step_model_id),
            )
            .first::<
                crate::codegen::structs_codegen::tables::weighing_step_models::WeighingStepModel,
            >(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn instrument(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::instruments::Instrument,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::instruments::Instrument::table()
            .filter(
                crate::codegen::diesel_codegen::tables::instruments::instruments::dsl::id
                    .eq(&self.instrument_id),
            )
            .first::<crate::codegen::structs_codegen::tables::instruments::Instrument>(conn)
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
pub struct InsertableWeighingStepBuilder {
    id: Option<rosetta_uuid::Uuid>,
    processable_id: Option<rosetta_uuid::Uuid>,
    weighing_step_model_id: Option<i32>,
    instrument_id: Option<i32>,
    kilograms: Option<f32>,
    created_by: Option<i32>,
    created_at: Option<rosetta_timestamp::TimestampUTC>,
}
impl Default for InsertableWeighingStepBuilder {
    fn default() -> Self {
        Self {
            id: None,
            processable_id: None,
            weighing_step_model_id: None,
            instrument_id: None,
            kilograms: None,
            created_by: None,
            created_at: Some(rosetta_timestamp::TimestampUTC::default()),
        }
    }
}
impl InsertableWeighingStepBuilder {
    pub fn id<P: Into<rosetta_uuid::Uuid>>(
        mut self,
        id: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let id = id.into();
        self.id = Some(id);
        Ok(self)
    }
    pub fn processable_id<P: Into<rosetta_uuid::Uuid>>(
        mut self,
        processable_id: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let processable_id = processable_id.into();
        self.processable_id = Some(processable_id);
        Ok(self)
    }
    pub fn weighing_step_model_id<P: Into<i32>>(
        mut self,
        weighing_step_model_id: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let weighing_step_model_id = weighing_step_model_id.into();
        self.weighing_step_model_id = Some(weighing_step_model_id);
        Ok(self)
    }
    pub fn instrument_id<P: Into<i32>>(
        mut self,
        instrument_id: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let instrument_id = instrument_id.into();
        self.instrument_id = Some(instrument_id);
        Ok(self)
    }
    pub fn kilograms<P: Into<f32>>(
        mut self,
        kilograms: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let kilograms = kilograms.into();
        pgrx_validation::must_be_strictly_positive_f32(kilograms)
            .map_err(|e| e.rename_field(InsertableWeighingStepAttributes::Kilograms))?;
        self.kilograms = Some(kilograms);
        Ok(self)
    }
    pub fn created_by<P: Into<i32>>(
        mut self,
        created_by: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let created_by = created_by.into();
        self.created_by = Some(created_by);
        Ok(self)
    }
    pub fn created_at<P: Into<rosetta_timestamp::TimestampUTC>>(
        mut self,
        created_at: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let created_at = created_at.into();
        self.created_at = Some(created_at);
        Ok(self)
    }
}
impl common_traits::prelude::Builder for InsertableWeighingStepBuilder {
    type Error = web_common_traits::database::InsertError<InsertableWeighingStepAttributes>;
    type Object = InsertableWeighingStep;
    type Attribute = InsertableWeighingStepAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            id: self.id.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableWeighingStepAttributes::Id,
            ))?,
            processable_id: self.processable_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableWeighingStepAttributes::ProcessableId,
                ),
            )?,
            weighing_step_model_id: self.weighing_step_model_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableWeighingStepAttributes::WeighingStepModelId,
                ),
            )?,
            instrument_id: self.instrument_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableWeighingStepAttributes::InstrumentId,
                ),
            )?,
            kilograms: self.kilograms.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableWeighingStepAttributes::Kilograms,
                ),
            )?,
            created_by: self.created_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableWeighingStepAttributes::CreatedBy,
                ),
            )?,
            created_at: self.created_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableWeighingStepAttributes::CreatedAt,
                ),
            )?,
        })
    }
}
impl TryFrom<InsertableWeighingStep> for InsertableWeighingStepBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableWeighingStep) -> Result<Self, Self::Error> {
        Self::default()
            .id(insertable_variant.id)?
            .processable_id(insertable_variant.processable_id)?
            .weighing_step_model_id(insertable_variant.weighing_step_model_id)?
            .instrument_id(insertable_variant.instrument_id)?
            .kilograms(insertable_variant.kilograms)?
            .created_by(insertable_variant.created_by)?
            .created_at(insertable_variant.created_at)
    }
}
