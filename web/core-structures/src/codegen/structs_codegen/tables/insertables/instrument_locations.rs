#[derive(Clone, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableInstrumentLocationAttributes {
    InstrumentId,
    RoomId,
    CreatedAt,
    CreatedBy,
}
impl core::fmt::Display for InsertableInstrumentLocationAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableInstrumentLocationAttributes::InstrumentId => {
                write!(f, "instrument_id")
            }
            InsertableInstrumentLocationAttributes::RoomId => write!(f, "room_id"),
            InsertableInstrumentLocationAttributes::CreatedAt => write!(f, "created_at"),
            InsertableInstrumentLocationAttributes::CreatedBy => write!(f, "created_by"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::instrument_locations::instrument_locations
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableInstrumentLocation {
    instrument_id: i32,
    room_id: i32,
    created_at: rosetta_timestamp::TimestampUTC,
    created_by: i32,
}
impl InsertableInstrumentLocation {
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
    pub async fn room(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::rooms::Room, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::rooms::Room::table()
            .filter(crate::codegen::diesel_codegen::tables::rooms::rooms::dsl::id.eq(&self.room_id))
            .first::<crate::codegen::structs_codegen::tables::rooms::Room>(conn)
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
pub struct InsertableInstrumentLocationBuilder {
    instrument_id: Option<i32>,
    room_id: Option<i32>,
    created_at: Option<rosetta_timestamp::TimestampUTC>,
    created_by: Option<i32>,
}
impl Default for InsertableInstrumentLocationBuilder {
    fn default() -> Self {
        Self {
            instrument_id: None,
            room_id: None,
            created_at: Some(rosetta_timestamp::TimestampUTC::default()),
            created_by: None,
        }
    }
}
impl InsertableInstrumentLocationBuilder {
    pub fn instrument_id<P>(
        mut self,
        instrument_id: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let instrument_id =
            instrument_id.try_into().map_err(|err: <P as TryInto<i32>>::Error| {
                Into::into(err).rename_field(InsertableInstrumentLocationAttributes::InstrumentId)
            })?;
        self.instrument_id = Some(instrument_id);
        Ok(self)
    }
    pub fn room_id<P>(
        mut self,
        room_id: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let room_id = room_id.try_into().map_err(|err: <P as TryInto<i32>>::Error| {
            Into::into(err).rename_field(InsertableInstrumentLocationAttributes::RoomId)
        })?;
        self.room_id = Some(room_id);
        Ok(self)
    }
    pub fn created_at<P>(
        mut self,
        created_at: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<rosetta_timestamp::TimestampUTC>,
        <P as TryInto<rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let created_at = created_at.try_into().map_err(
            |err: <P as TryInto<rosetta_timestamp::TimestampUTC>>::Error| {
                Into::into(err).rename_field(InsertableInstrumentLocationAttributes::CreatedAt)
            },
        )?;
        self.created_at = Some(created_at);
        Ok(self)
    }
    pub fn created_by<P>(
        mut self,
        created_by: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let created_by = created_by.try_into().map_err(|err: <P as TryInto<i32>>::Error| {
            Into::into(err).rename_field(InsertableInstrumentLocationAttributes::CreatedBy)
        })?;
        self.created_by = Some(created_by);
        Ok(self)
    }
}
impl common_traits::prelude::Builder for InsertableInstrumentLocationBuilder {
    type Error = web_common_traits::database::InsertError<InsertableInstrumentLocationAttributes>;
    type Object = InsertableInstrumentLocation;
    type Attribute = InsertableInstrumentLocationAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            instrument_id: self.instrument_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableInstrumentLocationAttributes::InstrumentId,
                ),
            )?,
            room_id: self.room_id.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableInstrumentLocationAttributes::RoomId,
            ))?,
            created_at: self.created_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableInstrumentLocationAttributes::CreatedAt,
                ),
            )?,
            created_by: self.created_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableInstrumentLocationAttributes::CreatedBy,
                ),
            )?,
        })
    }
}
impl TryFrom<InsertableInstrumentLocation> for InsertableInstrumentLocationBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableInstrumentLocation) -> Result<Self, Self::Error> {
        Self::default()
            .instrument_id(insertable_variant.instrument_id)?
            .room_id(insertable_variant.room_id)?
            .created_at(insertable_variant.created_at)?
            .created_by(insertable_variant.created_by)
    }
}
