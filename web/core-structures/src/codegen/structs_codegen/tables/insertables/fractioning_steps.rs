#[derive(Clone, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableFractioningStepAttributes {
    Id,
    SourceProcessableId,
    DestinationProcessableId,
    FractioningStepModelId,
    InstrumentId,
    Kilograms,
    CreatedBy,
    CreatedAt,
}
impl core::fmt::Display for InsertableFractioningStepAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableFractioningStepAttributes::Id => write!(f, "id"),
            InsertableFractioningStepAttributes::SourceProcessableId => {
                write!(f, "source_processable_id")
            }
            InsertableFractioningStepAttributes::DestinationProcessableId => {
                write!(f, "destination_processable_id")
            }
            InsertableFractioningStepAttributes::FractioningStepModelId => {
                write!(f, "fractioning_step_model_id")
            }
            InsertableFractioningStepAttributes::InstrumentId => {
                write!(f, "instrument_id")
            }
            InsertableFractioningStepAttributes::Kilograms => write!(f, "kilograms"),
            InsertableFractioningStepAttributes::CreatedBy => write!(f, "created_by"),
            InsertableFractioningStepAttributes::CreatedAt => write!(f, "created_at"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::fractioning_steps::fractioning_steps
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableFractioningStep {
    id: rosetta_uuid::Uuid,
    source_processable_id: rosetta_uuid::Uuid,
    destination_processable_id: rosetta_uuid::Uuid,
    fractioning_step_model_id: i32,
    instrument_id: i32,
    kilograms: f32,
    created_by: i32,
    created_at: rosetta_timestamp::TimestampUTC,
}
impl InsertableFractioningStep {
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
    pub async fn source_processable(
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
                    .eq(&self.source_processable_id),
            )
            .first::<crate::codegen::structs_codegen::tables::processables::Processable>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn destination_processable(
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
                    .eq(&self.destination_processable_id),
            )
            .first::<crate::codegen::structs_codegen::tables::processables::Processable>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn fractioning_step_model(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::fractioning_step_models::FractioningStepModel,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::fractioning_step_models::FractioningStepModel::table()
            .filter(
                crate::codegen::diesel_codegen::tables::fractioning_step_models::fractioning_step_models::dsl::id
                    .eq(&self.fractioning_step_model_id),
            )
            .first::<
                crate::codegen::structs_codegen::tables::fractioning_step_models::FractioningStepModel,
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
#[derive(Default)]
pub struct InsertableFractioningStepBuilder {
    id: Option<rosetta_uuid::Uuid>,
    source_processable_id: Option<rosetta_uuid::Uuid>,
    destination_processable_id: Option<rosetta_uuid::Uuid>,
    fractioning_step_model_id: Option<i32>,
    instrument_id: Option<i32>,
    kilograms: Option<f32>,
    created_by: Option<i32>,
    created_at: Option<rosetta_timestamp::TimestampUTC>,
}
impl InsertableFractioningStepBuilder {
    pub fn id(
        mut self,
        id: rosetta_uuid::Uuid,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.id = Some(id);
        Ok(self)
    }
    pub fn source_processable_id(
        mut self,
        source_processable_id: rosetta_uuid::Uuid,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.source_processable_id = Some(source_processable_id);
        Ok(self)
    }
    pub fn destination_processable_id(
        mut self,
        destination_processable_id: rosetta_uuid::Uuid,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.destination_processable_id = Some(destination_processable_id);
        Ok(self)
    }
    pub fn fractioning_step_model_id(
        mut self,
        fractioning_step_model_id: i32,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.fractioning_step_model_id = Some(fractioning_step_model_id);
        Ok(self)
    }
    pub fn instrument_id(
        mut self,
        instrument_id: i32,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.instrument_id = Some(instrument_id);
        Ok(self)
    }
    pub fn kilograms(
        mut self,
        kilograms: f32,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        pgrx_validation::must_be_strictly_positive_f32(kilograms)
            .map_err(|e| e.rename_field(InsertableFractioningStepAttributes::Kilograms))?;
        self.kilograms = Some(kilograms);
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
impl common_traits::prelude::Builder for InsertableFractioningStepBuilder {
    type Error = web_common_traits::database::InsertError<InsertableFractioningStepAttributes>;
    type Object = InsertableFractioningStep;
    type Attribute = InsertableFractioningStepAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            id: self.id.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableFractioningStepAttributes::Id,
                )
            })?,
            source_processable_id: self.source_processable_id.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableFractioningStepAttributes::SourceProcessableId,
                )
            })?,
            destination_processable_id: self.destination_processable_id.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableFractioningStepAttributes::DestinationProcessableId,
                )
            })?,
            fractioning_step_model_id: self.fractioning_step_model_id.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableFractioningStepAttributes::FractioningStepModelId,
                )
            })?,
            instrument_id: self.instrument_id.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableFractioningStepAttributes::InstrumentId,
                )
            })?,
            kilograms: self.kilograms.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableFractioningStepAttributes::Kilograms,
                )
            })?,
            created_by: self.created_by.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableFractioningStepAttributes::CreatedBy,
                )
            })?,
            created_at: self.created_at.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableFractioningStepAttributes::CreatedAt,
                )
            })?,
        })
    }
}
impl TryFrom<InsertableFractioningStep> for InsertableFractioningStepBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableFractioningStep) -> Result<Self, Self::Error> {
        Ok(Self::default()
            .id(insertable_variant.id)?
            .source_processable_id(insertable_variant.source_processable_id)?
            .destination_processable_id(insertable_variant.destination_processable_id)?
            .fractioning_step_model_id(insertable_variant.fractioning_step_model_id)?
            .instrument_id(insertable_variant.instrument_id)?
            .kilograms(insertable_variant.kilograms)?
            .created_by(insertable_variant.created_by)?
            .created_at(insertable_variant.created_at)?)
    }
}
