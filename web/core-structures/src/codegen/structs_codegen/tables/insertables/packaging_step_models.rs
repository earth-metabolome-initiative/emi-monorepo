#[derive(Clone, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertablePackagingStepModelAttributes {
    PackagingModelId,
    CreatedAt,
    UpdatedAt,
    CreatedBy,
    UpdatedBy,
}
impl core::fmt::Display for InsertablePackagingStepModelAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertablePackagingStepModelAttributes::PackagingModelId => {
                write!(f, "packaging_model_id")
            }
            InsertablePackagingStepModelAttributes::CreatedAt => write!(f, "created_at"),
            InsertablePackagingStepModelAttributes::UpdatedAt => write!(f, "updated_at"),
            InsertablePackagingStepModelAttributes::CreatedBy => write!(f, "created_by"),
            InsertablePackagingStepModelAttributes::UpdatedBy => write!(f, "updated_by"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::packaging_step_models::packaging_step_models
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertablePackagingStepModel {
    packaging_model_id: i32,
    created_at: rosetta_timestamp::TimestampUTC,
    updated_at: rosetta_timestamp::TimestampUTC,
    created_by: i32,
    updated_by: i32,
}
impl InsertablePackagingStepModel {
    #[cfg(feature = "postgres")]
    pub async fn packaging_model(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::packaging_models::PackagingModel,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::packaging_models::PackagingModel::table()
            .filter(
                crate::codegen::diesel_codegen::tables::packaging_models::packaging_models::dsl::id
                    .eq(&self.packaging_model_id),
            )
            .first::<crate::codegen::structs_codegen::tables::packaging_models::PackagingModel>(
                conn,
            )
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
pub struct InsertablePackagingStepModelBuilder {
    packaging_model_id: Option<i32>,
    created_at: Option<rosetta_timestamp::TimestampUTC>,
    updated_at: Option<rosetta_timestamp::TimestampUTC>,
    created_by: Option<i32>,
    updated_by: Option<i32>,
}
impl Default for InsertablePackagingStepModelBuilder {
    fn default() -> Self {
        Self {
            packaging_model_id: None,
            created_at: Some(rosetta_timestamp::TimestampUTC::default()),
            updated_at: Some(rosetta_timestamp::TimestampUTC::default()),
            created_by: None,
            updated_by: None,
        }
    }
}
impl InsertablePackagingStepModelBuilder {
    pub fn packaging_model_id<P>(
        mut self,
        packaging_model_id: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let packaging_model_id =
            packaging_model_id.try_into().map_err(|err: <P as TryInto<i32>>::Error| {
                Into::into(err)
                    .rename_field(InsertablePackagingStepModelAttributes::PackagingModelId)
            })?;
        self.packaging_model_id = Some(packaging_model_id);
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
                Into::into(err).rename_field(InsertablePackagingStepModelAttributes::CreatedAt)
            },
        )?;
        self.created_at = Some(created_at);
        Ok(self)
    }
    pub fn updated_at<P>(
        mut self,
        updated_at: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<rosetta_timestamp::TimestampUTC>,
        <P as TryInto<rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let updated_at = updated_at.try_into().map_err(
            |err: <P as TryInto<rosetta_timestamp::TimestampUTC>>::Error| {
                Into::into(err).rename_field(InsertablePackagingStepModelAttributes::UpdatedAt)
            },
        )?;
        self.updated_at = Some(updated_at);
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
            Into::into(err).rename_field(InsertablePackagingStepModelAttributes::CreatedBy)
        })?;
        self.created_by = Some(created_by);
        self = self.updated_by(created_by)?;
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
            Into::into(err).rename_field(InsertablePackagingStepModelAttributes::UpdatedBy)
        })?;
        self.updated_by = Some(updated_by);
        Ok(self)
    }
}
impl common_traits::prelude::Builder for InsertablePackagingStepModelBuilder {
    type Error = web_common_traits::database::InsertError<InsertablePackagingStepModelAttributes>;
    type Object = InsertablePackagingStepModel;
    type Attribute = InsertablePackagingStepModelAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            packaging_model_id: self.packaging_model_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertablePackagingStepModelAttributes::PackagingModelId,
                ),
            )?,
            created_at: self.created_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertablePackagingStepModelAttributes::CreatedAt,
                ),
            )?,
            updated_at: self.updated_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertablePackagingStepModelAttributes::UpdatedAt,
                ),
            )?,
            created_by: self.created_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertablePackagingStepModelAttributes::CreatedBy,
                ),
            )?,
            updated_by: self.updated_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertablePackagingStepModelAttributes::UpdatedBy,
                ),
            )?,
        })
    }
}
impl TryFrom<InsertablePackagingStepModel> for InsertablePackagingStepModelBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertablePackagingStepModel) -> Result<Self, Self::Error> {
        Self::default()
            .packaging_model_id(insertable_variant.packaging_model_id)?
            .created_at(insertable_variant.created_at)?
            .updated_at(insertable_variant.updated_at)?
            .created_by(insertable_variant.created_by)?
            .updated_by(insertable_variant.updated_by)
    }
}
