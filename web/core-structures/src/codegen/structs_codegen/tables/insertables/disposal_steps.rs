#[derive(Clone, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableDisposalStepAttributes {
    Id,
    ProcessableId,
    CreatedBy,
    CreatedAt,
}
impl core::fmt::Display for InsertableDisposalStepAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableDisposalStepAttributes::Id => write!(f, "id"),
            InsertableDisposalStepAttributes::ProcessableId => {
                write!(f, "processable_id")
            }
            InsertableDisposalStepAttributes::CreatedBy => write!(f, "created_by"),
            InsertableDisposalStepAttributes::CreatedAt => write!(f, "created_at"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::disposal_steps::disposal_steps
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableDisposalStep {
    id: rosetta_uuid::Uuid,
    processable_id: rosetta_uuid::Uuid,
    created_by: i32,
    created_at: rosetta_timestamp::TimestampUTC,
}
impl InsertableDisposalStep {
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
pub struct InsertableDisposalStepBuilder {
    id: Option<rosetta_uuid::Uuid>,
    processable_id: Option<rosetta_uuid::Uuid>,
    created_by: Option<i32>,
    created_at: Option<rosetta_timestamp::TimestampUTC>,
}
impl Default for InsertableDisposalStepBuilder {
    fn default() -> Self {
        Self {
            id: None,
            processable_id: None,
            created_by: None,
            created_at: Some(rosetta_timestamp::TimestampUTC::default()),
        }
    }
}
impl InsertableDisposalStepBuilder {
    pub fn id<P>(mut self, id: P) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<rosetta_uuid::Uuid>,
        <P as TryInto<rosetta_uuid::Uuid>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let id = id.try_into().map_err(|err: <P as TryInto<rosetta_uuid::Uuid>>::Error| {
            Into::into(err).rename_field(InsertableDisposalStepAttributes::Id)
        })?;
        self.id = Some(id);
        Ok(self)
    }
    pub fn processable_id<P>(
        mut self,
        processable_id: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<rosetta_uuid::Uuid>,
        <P as TryInto<rosetta_uuid::Uuid>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let processable_id = processable_id.try_into().map_err(
            |err: <P as TryInto<rosetta_uuid::Uuid>>::Error| {
                Into::into(err).rename_field(InsertableDisposalStepAttributes::ProcessableId)
            },
        )?;
        self.processable_id = Some(processable_id);
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
            Into::into(err).rename_field(InsertableDisposalStepAttributes::CreatedBy)
        })?;
        self.created_by = Some(created_by);
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
                Into::into(err).rename_field(InsertableDisposalStepAttributes::CreatedAt)
            },
        )?;
        self.created_at = Some(created_at);
        Ok(self)
    }
}
impl common_traits::prelude::Builder for InsertableDisposalStepBuilder {
    type Error = web_common_traits::database::InsertError<InsertableDisposalStepAttributes>;
    type Object = InsertableDisposalStep;
    type Attribute = InsertableDisposalStepAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            id: self.id.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableDisposalStepAttributes::Id,
            ))?,
            processable_id: self.processable_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableDisposalStepAttributes::ProcessableId,
                ),
            )?,
            created_by: self.created_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableDisposalStepAttributes::CreatedBy,
                ),
            )?,
            created_at: self.created_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableDisposalStepAttributes::CreatedAt,
                ),
            )?,
        })
    }
}
impl TryFrom<InsertableDisposalStep> for InsertableDisposalStepBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableDisposalStep) -> Result<Self, Self::Error> {
        Self::default()
            .id(insertable_variant.id)?
            .processable_id(insertable_variant.processable_id)?
            .created_by(insertable_variant.created_by)?
            .created_at(insertable_variant.created_at)
    }
}
