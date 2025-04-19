#[derive(Clone, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableSamplingStepAttributes {
    Id,
    ProcessableId,
    TrackableId,
    CreatedBy,
    CreatedAt,
}
impl core::fmt::Display for InsertableSamplingStepAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableSamplingStepAttributes::Id => write!(f, "id"),
            InsertableSamplingStepAttributes::ProcessableId => {
                write!(f, "processable_id")
            }
            InsertableSamplingStepAttributes::TrackableId => write!(f, "trackable_id"),
            InsertableSamplingStepAttributes::CreatedBy => write!(f, "created_by"),
            InsertableSamplingStepAttributes::CreatedAt => write!(f, "created_at"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::sampling_steps::sampling_steps
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableSamplingStep {
    id: rosetta_uuid::Uuid,
    processable_id: rosetta_uuid::Uuid,
    trackable_id: rosetta_uuid::Uuid,
    created_by: i32,
    created_at: rosetta_timestamp::TimestampUTC,
}
impl InsertableSamplingStep {
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
    pub async fn trackable(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::trackables::Trackable, diesel::result::Error>
    {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::trackables::Trackable::table()
            .filter(
                crate::codegen::diesel_codegen::tables::trackables::trackables::dsl::id
                    .eq(&self.trackable_id),
            )
            .first::<crate::codegen::structs_codegen::tables::trackables::Trackable>(conn)
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
pub struct InsertableSamplingStepBuilder {
    id: Option<rosetta_uuid::Uuid>,
    processable_id: Option<rosetta_uuid::Uuid>,
    trackable_id: Option<rosetta_uuid::Uuid>,
    created_by: Option<i32>,
    created_at: Option<rosetta_timestamp::TimestampUTC>,
}
impl InsertableSamplingStepBuilder {
    pub fn id(
        mut self,
        id: rosetta_uuid::Uuid,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.id = Some(id);
        Ok(self)
    }
    pub fn processable_id(
        mut self,
        processable_id: rosetta_uuid::Uuid,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.processable_id = Some(processable_id);
        Ok(self)
    }
    pub fn trackable_id(
        mut self,
        trackable_id: rosetta_uuid::Uuid,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.trackable_id = Some(trackable_id);
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
impl common_traits::prelude::Builder for InsertableSamplingStepBuilder {
    type Error = web_common_traits::database::InsertError<InsertableSamplingStepAttributes>;
    type Object = InsertableSamplingStep;
    type Attribute = InsertableSamplingStepAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            id: self.id.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableSamplingStepAttributes::Id,
                )
            })?,
            processable_id: self.processable_id.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableSamplingStepAttributes::ProcessableId,
                )
            })?,
            trackable_id: self.trackable_id.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableSamplingStepAttributes::TrackableId,
                )
            })?,
            created_by: self.created_by.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableSamplingStepAttributes::CreatedBy,
                )
            })?,
            created_at: self.created_at.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableSamplingStepAttributes::CreatedAt,
                )
            })?,
        })
    }
}
impl TryFrom<InsertableSamplingStep> for InsertableSamplingStepBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableSamplingStep) -> Result<Self, Self::Error> {
        Ok(Self::default()
            .id(insertable_variant.id)?
            .processable_id(insertable_variant.processable_id)?
            .trackable_id(insertable_variant.trackable_id)?
            .created_by(insertable_variant.created_by)?
            .created_at(insertable_variant.created_at)?)
    }
}
