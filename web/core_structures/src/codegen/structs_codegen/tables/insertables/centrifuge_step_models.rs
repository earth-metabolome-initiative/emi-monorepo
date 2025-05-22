#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableCentrifugeStepModelAttributes {
    Id,
    Seconds,
    RotationPerMinute,
    CreatedBy,
    CreatedAt,
    UpdatedBy,
    UpdatedAt,
}
impl core::fmt::Display for InsertableCentrifugeStepModelAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableCentrifugeStepModelAttributes::Id => write!(f, "id"),
            InsertableCentrifugeStepModelAttributes::Seconds => write!(f, "seconds"),
            InsertableCentrifugeStepModelAttributes::RotationPerMinute => {
                write!(f, "rotation_per_minute")
            }
            InsertableCentrifugeStepModelAttributes::CreatedBy => write!(f, "created_by"),
            InsertableCentrifugeStepModelAttributes::CreatedAt => write!(f, "created_at"),
            InsertableCentrifugeStepModelAttributes::UpdatedBy => write!(f, "updated_by"),
            InsertableCentrifugeStepModelAttributes::UpdatedAt => write!(f, "updated_at"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::centrifuge_step_models::centrifuge_step_models
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableCentrifugeStepModel {
    id: i32,
    seconds: f32,
    rotation_per_minute: f32,
    created_by: i32,
    created_at: ::rosetta_timestamp::TimestampUTC,
    updated_by: i32,
    updated_at: ::rosetta_timestamp::TimestampUTC,
}
impl InsertableCentrifugeStepModel {
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
pub struct InsertableCentrifugeStepModelBuilder {
    id: Option<i32>,
    seconds: Option<f32>,
    rotation_per_minute: Option<f32>,
    created_by: Option<i32>,
    created_at: Option<::rosetta_timestamp::TimestampUTC>,
    updated_by: Option<i32>,
    updated_at: Option<::rosetta_timestamp::TimestampUTC>,
}
impl Default for InsertableCentrifugeStepModelBuilder {
    fn default() -> Self {
        Self {
            id: None,
            seconds: None,
            rotation_per_minute: None,
            created_by: None,
            created_at: Some(rosetta_timestamp::TimestampUTC::default()),
            updated_by: None,
            updated_at: Some(rosetta_timestamp::TimestampUTC::default()),
        }
    }
}
impl InsertableCentrifugeStepModelBuilder {
    pub fn id<P>(mut self, id: P) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let id = id.try_into().map_err(|err: <P as TryInto<i32>>::Error| {
            Into::into(err).rename_field(InsertableCentrifugeStepModelAttributes::Id)
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
            Into::into(err).rename_field(InsertableCentrifugeStepModelAttributes::Seconds)
        })?;
        pgrx_validation::must_be_strictly_positive_f32(seconds)
            .map_err(|e| e.rename_field(InsertableCentrifugeStepModelAttributes::Seconds))?;
        pgrx_validation::must_be_strictly_smaller_than_f32(seconds, 3600f32)
            .map_err(|e| e.rename_field(InsertableCentrifugeStepModelAttributes::Seconds))?;
        pgrx_validation::must_be_greater_than_f32(seconds, 30f32)
            .map_err(|e| e.rename_field(InsertableCentrifugeStepModelAttributes::Seconds))?;
        self.seconds = Some(seconds);
        Ok(self)
    }
    pub fn rotation_per_minute<P>(
        mut self,
        rotation_per_minute: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<f32>,
        <P as TryInto<f32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let rotation_per_minute =
            rotation_per_minute.try_into().map_err(|err: <P as TryInto<f32>>::Error| {
                Into::into(err)
                    .rename_field(InsertableCentrifugeStepModelAttributes::RotationPerMinute)
            })?;
        pgrx_validation::must_be_strictly_positive_f32(rotation_per_minute).map_err(|e| {
            e.rename_field(InsertableCentrifugeStepModelAttributes::RotationPerMinute)
        })?;
        pgrx_validation::must_be_smaller_than_f32(rotation_per_minute, 30000f32).map_err(|e| {
            e.rename_field(InsertableCentrifugeStepModelAttributes::RotationPerMinute)
        })?;
        pgrx_validation::must_be_greater_than_f32(rotation_per_minute, 5000f32).map_err(|e| {
            e.rename_field(InsertableCentrifugeStepModelAttributes::RotationPerMinute)
        })?;
        self.rotation_per_minute = Some(rotation_per_minute);
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
            Into::into(err).rename_field(InsertableCentrifugeStepModelAttributes::CreatedBy)
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
                Into::into(err).rename_field(InsertableCentrifugeStepModelAttributes::CreatedAt)
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
            Into::into(err).rename_field(InsertableCentrifugeStepModelAttributes::UpdatedBy)
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
                Into::into(err).rename_field(InsertableCentrifugeStepModelAttributes::UpdatedAt)
            },
        )?;
        self.updated_at = Some(updated_at);
        Ok(self)
    }
}
impl common_traits::prelude::Builder for InsertableCentrifugeStepModelBuilder {
    type Error = web_common_traits::database::InsertError<InsertableCentrifugeStepModelAttributes>;
    type Object = InsertableCentrifugeStepModel;
    type Attribute = InsertableCentrifugeStepModelAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            id: self.id.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableCentrifugeStepModelAttributes::Id,
            ))?,
            seconds: self.seconds.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableCentrifugeStepModelAttributes::Seconds,
            ))?,
            rotation_per_minute: self.rotation_per_minute.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableCentrifugeStepModelAttributes::RotationPerMinute,
                ),
            )?,
            created_by: self.created_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableCentrifugeStepModelAttributes::CreatedBy,
                ),
            )?,
            created_at: self.created_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableCentrifugeStepModelAttributes::CreatedAt,
                ),
            )?,
            updated_by: self.updated_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableCentrifugeStepModelAttributes::UpdatedBy,
                ),
            )?,
            updated_at: self.updated_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableCentrifugeStepModelAttributes::UpdatedAt,
                ),
            )?,
        })
    }
}
impl TryFrom<InsertableCentrifugeStepModel> for InsertableCentrifugeStepModelBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableCentrifugeStepModel) -> Result<Self, Self::Error> {
        Self::default()
            .id(insertable_variant.id)?
            .seconds(insertable_variant.seconds)?
            .rotation_per_minute(insertable_variant.rotation_per_minute)?
            .created_by(insertable_variant.created_by)?
            .created_at(insertable_variant.created_at)?
            .updated_by(insertable_variant.updated_by)?
            .updated_at(insertable_variant.updated_at)
    }
}
