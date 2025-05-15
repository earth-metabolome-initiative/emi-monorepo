#[derive(Clone, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableStepToolModelAttributes {
    Id,
    StepId,
    ToolModelId,
    CreatedBy,
    CreatedAt,
}
impl core::fmt::Display for InsertableStepToolModelAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableStepToolModelAttributes::Id => write!(f, "id"),
            InsertableStepToolModelAttributes::StepId => write!(f, "step_id"),
            InsertableStepToolModelAttributes::ToolModelId => write!(f, "tool_model_id"),
            InsertableStepToolModelAttributes::CreatedBy => write!(f, "created_by"),
            InsertableStepToolModelAttributes::CreatedAt => write!(f, "created_at"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::step_tool_models::step_tool_models
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableStepToolModel {
    id: rosetta_uuid::Uuid,
    step_id: rosetta_uuid::Uuid,
    tool_model_id: i32,
    created_by: i32,
    created_at: rosetta_timestamp::TimestampUTC,
}
impl InsertableStepToolModel {
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
    pub async fn tool_model(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::tool_models::ToolModel,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::tool_models::ToolModel::table()
            .filter(
                crate::codegen::diesel_codegen::tables::tool_models::tool_models::dsl::id
                    .eq(&self.tool_model_id),
            )
            .first::<crate::codegen::structs_codegen::tables::tool_models::ToolModel>(conn)
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
pub struct InsertableStepToolModelBuilder {
    id: Option<rosetta_uuid::Uuid>,
    step_id: Option<rosetta_uuid::Uuid>,
    tool_model_id: Option<i32>,
    created_by: Option<i32>,
    created_at: Option<rosetta_timestamp::TimestampUTC>,
}
impl Default for InsertableStepToolModelBuilder {
    fn default() -> Self {
        Self {
            id: None,
            step_id: None,
            tool_model_id: None,
            created_by: None,
            created_at: Some(rosetta_timestamp::TimestampUTC::default()),
        }
    }
}
impl InsertableStepToolModelBuilder {
    pub fn id<P>(mut self, id: P) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<rosetta_uuid::Uuid>,
        <P as TryInto<rosetta_uuid::Uuid>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let id = id.try_into().map_err(|err: <P as TryInto<rosetta_uuid::Uuid>>::Error| {
            Into::into(err).rename_field(InsertableStepToolModelAttributes::Id)
        })?;
        self.id = Some(id);
        Ok(self)
    }
    pub fn step_id<P>(
        mut self,
        step_id: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<rosetta_uuid::Uuid>,
        <P as TryInto<rosetta_uuid::Uuid>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let step_id =
            step_id.try_into().map_err(|err: <P as TryInto<rosetta_uuid::Uuid>>::Error| {
                Into::into(err).rename_field(InsertableStepToolModelAttributes::StepId)
            })?;
        self.step_id = Some(step_id);
        Ok(self)
    }
    pub fn tool_model_id<P>(
        mut self,
        tool_model_id: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let tool_model_id =
            tool_model_id.try_into().map_err(|err: <P as TryInto<i32>>::Error| {
                Into::into(err).rename_field(InsertableStepToolModelAttributes::ToolModelId)
            })?;
        self.tool_model_id = Some(tool_model_id);
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
            Into::into(err).rename_field(InsertableStepToolModelAttributes::CreatedBy)
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
                Into::into(err).rename_field(InsertableStepToolModelAttributes::CreatedAt)
            },
        )?;
        self.created_at = Some(created_at);
        Ok(self)
    }
}
impl common_traits::prelude::Builder for InsertableStepToolModelBuilder {
    type Error = web_common_traits::database::InsertError<InsertableStepToolModelAttributes>;
    type Object = InsertableStepToolModel;
    type Attribute = InsertableStepToolModelAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            id: self.id.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableStepToolModelAttributes::Id,
            ))?,
            step_id: self.step_id.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableStepToolModelAttributes::StepId,
            ))?,
            tool_model_id: self.tool_model_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableStepToolModelAttributes::ToolModelId,
                ),
            )?,
            created_by: self.created_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableStepToolModelAttributes::CreatedBy,
                ),
            )?,
            created_at: self.created_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableStepToolModelAttributes::CreatedAt,
                ),
            )?,
        })
    }
}
impl TryFrom<InsertableStepToolModel> for InsertableStepToolModelBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableStepToolModel) -> Result<Self, Self::Error> {
        Self::default()
            .id(insertable_variant.id)?
            .step_id(insertable_variant.step_id)?
            .tool_model_id(insertable_variant.tool_model_id)?
            .created_by(insertable_variant.created_by)?
            .created_at(insertable_variant.created_at)
    }
}
