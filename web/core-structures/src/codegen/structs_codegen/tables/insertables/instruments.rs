#[derive(Clone, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableInstrumentAttributes {
    InstrumentModelId,
    InstrumentStateId,
    Qrcode,
    CreatedBy,
    CreatedAt,
    UpdatedBy,
    UpdatedAt,
}
impl core::fmt::Display for InsertableInstrumentAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableInstrumentAttributes::InstrumentModelId => {
                write!(f, "instrument_model_id")
            }
            InsertableInstrumentAttributes::InstrumentStateId => {
                write!(f, "instrument_state_id")
            }
            InsertableInstrumentAttributes::Qrcode => write!(f, "qrcode"),
            InsertableInstrumentAttributes::CreatedBy => write!(f, "created_by"),
            InsertableInstrumentAttributes::CreatedAt => write!(f, "created_at"),
            InsertableInstrumentAttributes::UpdatedBy => write!(f, "updated_by"),
            InsertableInstrumentAttributes::UpdatedAt => write!(f, "updated_at"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(table_name = crate::codegen::diesel_codegen::tables::instruments::instruments)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableInstrument {
    instrument_model_id: i32,
    instrument_state_id: i16,
    qrcode: rosetta_uuid::Uuid,
    created_by: i32,
    created_at: rosetta_timestamp::TimestampUTC,
    updated_by: i32,
    updated_at: rosetta_timestamp::TimestampUTC,
}
impl InsertableInstrument {
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
    pub async fn instrument_state(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::instrument_states::InstrumentState,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::instrument_states::InstrumentState::table()
            .filter(
                crate::codegen::diesel_codegen::tables::instrument_states::instrument_states::dsl::id
                    .eq(&self.instrument_state_id),
            )
            .first::<
                crate::codegen::structs_codegen::tables::instrument_states::InstrumentState,
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
}
#[derive(Default)]
pub struct InsertableInstrumentBuilder {
    instrument_model_id: Option<i32>,
    instrument_state_id: Option<i16>,
    qrcode: Option<rosetta_uuid::Uuid>,
    created_by: Option<i32>,
    created_at: Option<rosetta_timestamp::TimestampUTC>,
    updated_by: Option<i32>,
    updated_at: Option<rosetta_timestamp::TimestampUTC>,
}
impl InsertableInstrumentBuilder {
    pub fn instrument_model_id(
        mut self,
        instrument_model_id: i32,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.instrument_model_id = Some(instrument_model_id);
        Ok(self)
    }
    pub fn instrument_state_id(
        mut self,
        instrument_state_id: i16,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.instrument_state_id = Some(instrument_state_id);
        Ok(self)
    }
    pub fn qrcode(
        mut self,
        qrcode: rosetta_uuid::Uuid,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.qrcode = Some(qrcode);
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
impl common_traits::prelude::Builder for InsertableInstrumentBuilder {
    type Error = web_common_traits::database::InsertError<InsertableInstrumentAttributes>;
    type Object = InsertableInstrument;
    type Attribute = InsertableInstrumentAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            instrument_model_id: self.instrument_model_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableInstrumentAttributes::InstrumentModelId,
                ),
            )?,
            instrument_state_id: self.instrument_state_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableInstrumentAttributes::InstrumentStateId,
                ),
            )?,
            qrcode: self.qrcode.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableInstrumentAttributes::Qrcode,
            ))?,
            created_by: self.created_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableInstrumentAttributes::CreatedBy,
                ),
            )?,
            created_at: self.created_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableInstrumentAttributes::CreatedAt,
                ),
            )?,
            updated_by: self.updated_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableInstrumentAttributes::UpdatedBy,
                ),
            )?,
            updated_at: self.updated_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableInstrumentAttributes::UpdatedAt,
                ),
            )?,
        })
    }
}
impl TryFrom<InsertableInstrument> for InsertableInstrumentBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableInstrument) -> Result<Self, Self::Error> {
        Self::default()
            .instrument_model_id(insertable_variant.instrument_model_id)?
            .instrument_state_id(insertable_variant.instrument_state_id)?
            .qrcode(insertable_variant.qrcode)?
            .created_by(insertable_variant.created_by)?
            .created_at(insertable_variant.created_at)?
            .updated_by(insertable_variant.updated_by)?
            .updated_at(insertable_variant.updated_at)?
    }
}
