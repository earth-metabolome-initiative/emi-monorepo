#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableBallMillStepModelAttributes {
    Id,
    Seconds,
    Hertz,
    CreatedBy,
    CreatedAt,
    UpdatedBy,
    UpdatedAt,
}
impl core::fmt::Display for InsertableBallMillStepModelAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableBallMillStepModelAttributes::Id => write!(f, "id"),
            InsertableBallMillStepModelAttributes::Seconds => write!(f, "seconds"),
            InsertableBallMillStepModelAttributes::Hertz => write!(f, "hertz"),
            InsertableBallMillStepModelAttributes::CreatedBy => write!(f, "created_by"),
            InsertableBallMillStepModelAttributes::CreatedAt => write!(f, "created_at"),
            InsertableBallMillStepModelAttributes::UpdatedBy => write!(f, "updated_by"),
            InsertableBallMillStepModelAttributes::UpdatedAt => write!(f, "updated_at"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::ball_mill_step_models::ball_mill_step_models
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableBallMillStepModel {
    id: i32,
    seconds: f32,
    hertz: f32,
    created_by: i32,
    created_at: ::rosetta_timestamp::TimestampUTC,
    updated_by: i32,
    updated_at: ::rosetta_timestamp::TimestampUTC,
}
impl InsertableBallMillStepModel {
    #[cfg(feature = "postgres")]
    pub async fn id(
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
                    .eq(&self.id),
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
pub struct InsertableBallMillStepModelBuilder {
    id: Option<i32>,
    seconds: Option<f32>,
    hertz: Option<f32>,
    created_by: Option<i32>,
    created_at: Option<::rosetta_timestamp::TimestampUTC>,
    updated_by: Option<i32>,
    updated_at: Option<::rosetta_timestamp::TimestampUTC>,
}
impl Default for InsertableBallMillStepModelBuilder {
    fn default() -> Self {
        Self {
            id: None,
            seconds: None,
            hertz: None,
            created_by: None,
            created_at: Some(rosetta_timestamp::TimestampUTC::default()),
            updated_by: None,
            updated_at: Some(rosetta_timestamp::TimestampUTC::default()),
        }
    }
}
impl InsertableBallMillStepModelBuilder {
    pub fn id<P>(mut self, id: P) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let id = id.try_into().map_err(|err: <P as TryInto<i32>>::Error| {
            Into::into(err).rename_field(InsertableBallMillStepModelAttributes::Id)
        })?;
        self.id = Some(id);
        Ok(self)
    }
    pub fn seconds<P>(
        mut self,
        seconds: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<f32>,
        <P as TryInto<f32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let seconds = seconds.try_into().map_err(|err: <P as TryInto<f32>>::Error| {
            Into::into(err).rename_field(InsertableBallMillStepModelAttributes::Seconds)
        })?;
        pgrx_validation::must_be_strictly_positive_f32(seconds)
            .map_err(|e| e.rename_field(InsertableBallMillStepModelAttributes::Seconds))?;
        pgrx_validation::must_be_strictly_smaller_than_f32(seconds, 1800f32)
            .map_err(|e| e.rename_field(InsertableBallMillStepModelAttributes::Seconds))?;
        pgrx_validation::must_be_strictly_greater_than_f32(seconds, 10f32)
            .map_err(|e| e.rename_field(InsertableBallMillStepModelAttributes::Seconds))?;
        self.seconds = Some(seconds);
        Ok(self)
    }
    pub fn hertz<P>(
        mut self,
        hertz: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<f32>,
        <P as TryInto<f32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let hertz = hertz.try_into().map_err(|err: <P as TryInto<f32>>::Error| {
            Into::into(err).rename_field(InsertableBallMillStepModelAttributes::Hertz)
        })?;
        pgrx_validation::must_be_strictly_positive_f32(hertz)
            .map_err(|e| e.rename_field(InsertableBallMillStepModelAttributes::Hertz))?;
        pgrx_validation::must_be_strictly_smaller_than_f32(hertz, 100f32)
            .map_err(|e| e.rename_field(InsertableBallMillStepModelAttributes::Hertz))?;
        pgrx_validation::must_be_strictly_greater_than_f32(hertz, 0f32)
            .map_err(|e| e.rename_field(InsertableBallMillStepModelAttributes::Hertz))?;
        self.hertz = Some(hertz);
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
            Into::into(err).rename_field(InsertableBallMillStepModelAttributes::CreatedBy)
        })?;
        self.created_by = Some(created_by);
        self = self.updated_by(created_by)?;
        Ok(self)
    }
    pub fn created_at<P>(
        mut self,
        created_at: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<::rosetta_timestamp::TimestampUTC>,
        <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let created_at = created_at.try_into().map_err(
            |err: <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error| {
                Into::into(err).rename_field(InsertableBallMillStepModelAttributes::CreatedAt)
            },
        )?;
        self.created_at = Some(created_at);
        Ok(self)
    }
    pub fn updated_by<P>(
        mut self,
        updated_by: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let updated_by = updated_by.try_into().map_err(|err: <P as TryInto<i32>>::Error| {
            Into::into(err).rename_field(InsertableBallMillStepModelAttributes::UpdatedBy)
        })?;
        self.updated_by = Some(updated_by);
        Ok(self)
    }
    pub fn updated_at<P>(
        mut self,
        updated_at: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<::rosetta_timestamp::TimestampUTC>,
        <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let updated_at = updated_at.try_into().map_err(
            |err: <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error| {
                Into::into(err).rename_field(InsertableBallMillStepModelAttributes::UpdatedAt)
            },
        )?;
        self.updated_at = Some(updated_at);
        Ok(self)
    }
}
impl common_traits::prelude::Builder for InsertableBallMillStepModelBuilder {
    type Error = web_common_traits::database::InsertError<InsertableBallMillStepModelAttributes>;
    type Object = InsertableBallMillStepModel;
    type Attribute = InsertableBallMillStepModelAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            id: self.id.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableBallMillStepModelAttributes::Id,
            ))?,
            seconds: self.seconds.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableBallMillStepModelAttributes::Seconds,
            ))?,
            hertz: self.hertz.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableBallMillStepModelAttributes::Hertz,
            ))?,
            created_by: self.created_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableBallMillStepModelAttributes::CreatedBy,
                ),
            )?,
            created_at: self.created_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableBallMillStepModelAttributes::CreatedAt,
                ),
            )?,
            updated_by: self.updated_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableBallMillStepModelAttributes::UpdatedBy,
                ),
            )?,
            updated_at: self.updated_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableBallMillStepModelAttributes::UpdatedAt,
                ),
            )?,
        })
    }
}
impl TryFrom<InsertableBallMillStepModel> for InsertableBallMillStepModelBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableBallMillStepModel) -> Result<Self, Self::Error> {
        Self::default()
            .id(insertable_variant.id)?
            .seconds(insertable_variant.seconds)?
            .hertz(insertable_variant.hertz)?
            .created_by(insertable_variant.created_by)?
            .created_at(insertable_variant.created_at)?
            .updated_by(insertable_variant.updated_by)?
            .updated_at(insertable_variant.updated_at)
    }
}
