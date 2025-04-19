#[derive(Clone, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableAliquotingStepAttributes {
    Id,
    SourceProcessableId,
    DestinationProcessableId,
    InstrumentId,
    CreatedBy,
    CreatedAt,
}
impl core::fmt::Display for InsertableAliquotingStepAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableAliquotingStepAttributes::Id => write!(f, "id"),
            InsertableAliquotingStepAttributes::SourceProcessableId => {
                write!(f, "source_processable_id")
            }
            InsertableAliquotingStepAttributes::DestinationProcessableId => {
                write!(f, "destination_processable_id")
            }
            InsertableAliquotingStepAttributes::InstrumentId => {
                write!(f, "instrument_id")
            }
            InsertableAliquotingStepAttributes::CreatedBy => write!(f, "created_by"),
            InsertableAliquotingStepAttributes::CreatedAt => write!(f, "created_at"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::aliquoting_steps::aliquoting_steps
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableAliquotingStep {
    id: rosetta_uuid::Uuid,
    source_processable_id: rosetta_uuid::Uuid,
    destination_processable_id: rosetta_uuid::Uuid,
    instrument_id: i32,
    created_by: i32,
    created_at: rosetta_timestamp::TimestampUTC,
}
impl InsertableAliquotingStep {
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
pub struct InsertableAliquotingStepBuilder {
    id: Option<rosetta_uuid::Uuid>,
    source_processable_id: Option<rosetta_uuid::Uuid>,
    destination_processable_id: Option<rosetta_uuid::Uuid>,
    instrument_id: Option<i32>,
    created_by: Option<i32>,
    created_at: Option<rosetta_timestamp::TimestampUTC>,
}
impl InsertableAliquotingStepBuilder {
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
    pub fn instrument_id(
        mut self,
        instrument_id: i32,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.instrument_id = Some(instrument_id);
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
impl common_traits::prelude::Builder for InsertableAliquotingStepBuilder {
    type Error = web_common_traits::database::InsertError<InsertableAliquotingStepAttributes>;
    type Object = InsertableAliquotingStep;
    type Attribute = InsertableAliquotingStepAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            id: self.id.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableAliquotingStepAttributes::Id,
                )
            })?,
            source_processable_id: self.source_processable_id.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableAliquotingStepAttributes::SourceProcessableId,
                )
            })?,
            destination_processable_id: self.destination_processable_id.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableAliquotingStepAttributes::DestinationProcessableId,
                )
            })?,
            instrument_id: self.instrument_id.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableAliquotingStepAttributes::InstrumentId,
                )
            })?,
            created_by: self.created_by.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableAliquotingStepAttributes::CreatedBy,
                )
            })?,
            created_at: self.created_at.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableAliquotingStepAttributes::CreatedAt,
                )
            })?,
        })
    }
}
impl TryFrom<InsertableAliquotingStep> for InsertableAliquotingStepBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableAliquotingStep) -> Result<Self, Self::Error> {
        Ok(Self::default()
            .id(insertable_variant.id)?
            .source_processable_id(insertable_variant.source_processable_id)?
            .destination_processable_id(insertable_variant.destination_processable_id)?
            .instrument_id(insertable_variant.instrument_id)?
            .created_by(insertable_variant.created_by)?
            .created_at(insertable_variant.created_at)?)
    }
}
