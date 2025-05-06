#[derive(Clone, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableStepInstrumentAttributes {
    Id,
    StepId,
    InstrumentId,
    CreatedBy,
    CreatedAt,
}
impl core::fmt::Display for InsertableStepInstrumentAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableStepInstrumentAttributes::Id => write!(f, "id"),
            InsertableStepInstrumentAttributes::StepId => write!(f, "step_id"),
            InsertableStepInstrumentAttributes::InstrumentId => {
                write!(f, "instrument_id")
            }
            InsertableStepInstrumentAttributes::CreatedBy => write!(f, "created_by"),
            InsertableStepInstrumentAttributes::CreatedAt => write!(f, "created_at"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::step_instruments::step_instruments
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableStepInstrument {
    id: rosetta_uuid::Uuid,
    step_id: rosetta_uuid::Uuid,
    instrument_id: i32,
    created_by: i32,
    created_at: rosetta_timestamp::TimestampUTC,
}
impl InsertableStepInstrument {
    #[cfg(feature = "postgres")]
    pub async fn step(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::steps::Step, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::steps::Step::table()
            .filter(crate::codegen::diesel_codegen::tables::steps::steps::dsl::id.eq(&self.step_id))
            .first::<crate::codegen::structs_codegen::tables::steps::Step>(conn)
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
pub struct InsertableStepInstrumentBuilder {
    id: Option<rosetta_uuid::Uuid>,
    step_id: Option<rosetta_uuid::Uuid>,
    instrument_id: Option<i32>,
    created_by: Option<i32>,
    created_at: Option<rosetta_timestamp::TimestampUTC>,
}
impl Default for InsertableStepInstrumentBuilder {
    fn default() -> Self {
        Self {
            id: None,
            step_id: None,
            instrument_id: None,
            created_by: None,
            created_at: Some(rosetta_timestamp::TimestampUTC::default()),
        }
    }
}
impl InsertableStepInstrumentBuilder {
    pub fn id<P: Into<rosetta_uuid::Uuid>>(
        mut self,
        id: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let id = id.into();
        self.id = Some(id);
        Ok(self)
    }
    pub fn step_id<P: Into<rosetta_uuid::Uuid>>(
        mut self,
        step_id: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let step_id = step_id.into();
        self.step_id = Some(step_id);
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
impl common_traits::prelude::Builder for InsertableStepInstrumentBuilder {
    type Error = web_common_traits::database::InsertError<InsertableStepInstrumentAttributes>;
    type Object = InsertableStepInstrument;
    type Attribute = InsertableStepInstrumentAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            id: self.id.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableStepInstrumentAttributes::Id,
            ))?,
            step_id: self.step_id.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableStepInstrumentAttributes::StepId,
            ))?,
            instrument_id: self.instrument_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableStepInstrumentAttributes::InstrumentId,
                ),
            )?,
            created_by: self.created_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableStepInstrumentAttributes::CreatedBy,
                ),
            )?,
            created_at: self.created_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableStepInstrumentAttributes::CreatedAt,
                ),
            )?,
        })
    }
}
impl TryFrom<InsertableStepInstrument> for InsertableStepInstrumentBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableStepInstrument) -> Result<Self, Self::Error> {
        Self::default()
            .id(insertable_variant.id)?
            .step_id(insertable_variant.step_id)?
            .instrument_id(insertable_variant.instrument_id)?
            .created_by(insertable_variant.created_by)?
            .created_at(insertable_variant.created_at)
    }
}
