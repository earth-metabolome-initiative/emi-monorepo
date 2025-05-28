#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableStepModelContainerCategoryAttributes {
    StepModelId,
    ProcedureModelContainerCategoryId,
    CreatedBy,
    CreatedAt,
    UpdatedBy,
    UpdatedAt,
}
impl core::fmt::Display for InsertableStepModelContainerCategoryAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableStepModelContainerCategoryAttributes::StepModelId => {
                write!(f, "step_model_id")
            }
            InsertableStepModelContainerCategoryAttributes::ProcedureModelContainerCategoryId => {
                write!(f, "procedure_model_container_category_id")
            }
            InsertableStepModelContainerCategoryAttributes::CreatedBy => {
                write!(f, "created_by")
            }
            InsertableStepModelContainerCategoryAttributes::CreatedAt => {
                write!(f, "created_at")
            }
            InsertableStepModelContainerCategoryAttributes::UpdatedBy => {
                write!(f, "updated_by")
            }
            InsertableStepModelContainerCategoryAttributes::UpdatedAt => {
                write!(f, "updated_at")
            }
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::step_model_container_categories::step_model_container_categories
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableStepModelContainerCategory {
    step_model_id: i32,
    procedure_model_container_category_id: i32,
    created_by: i32,
    created_at: ::rosetta_timestamp::TimestampUTC,
    updated_by: i32,
    updated_at: ::rosetta_timestamp::TimestampUTC,
}
impl InsertableStepModelContainerCategory {
    #[cfg(feature = "postgres")]
    pub async fn step_model(
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
                    .eq(&self.step_model_id),
            )
            .first::<crate::codegen::structs_codegen::tables::step_models::StepModel>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn procedure_model_container_category(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_model_container_categories::ProcedureModelContainerCategory,
        diesel::result::Error,
    >{
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::procedure_model_container_categories::ProcedureModelContainerCategory::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_model_container_categories::procedure_model_container_categories::dsl::id
                    .eq(&self.procedure_model_container_category_id),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_model_container_categories::ProcedureModelContainerCategory,
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
pub struct InsertableStepModelContainerCategoryBuilder {
    step_model_id: Option<i32>,
    procedure_model_container_category_id: Option<i32>,
    created_by: Option<i32>,
    created_at: Option<::rosetta_timestamp::TimestampUTC>,
    updated_by: Option<i32>,
    updated_at: Option<::rosetta_timestamp::TimestampUTC>,
}
impl Default for InsertableStepModelContainerCategoryBuilder {
    fn default() -> Self {
        Self {
            step_model_id: None,
            procedure_model_container_category_id: None,
            created_by: None,
            created_at: Some(rosetta_timestamp::TimestampUTC::default()),
            updated_by: None,
            updated_at: Some(rosetta_timestamp::TimestampUTC::default()),
        }
    }
}
impl InsertableStepModelContainerCategoryBuilder {
    pub fn step_model_id<P>(
        mut self,
        step_model_id: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let step_model_id =
            step_model_id.try_into().map_err(|err: <P as TryInto<i32>>::Error| {
                Into::into(err)
                    .rename_field(InsertableStepModelContainerCategoryAttributes::StepModelId)
            })?;
        self.step_model_id = Some(step_model_id);
        Ok(self)
    }
    pub fn procedure_model_container_category_id<P>(
        mut self,
        procedure_model_container_category_id: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let procedure_model_container_category_id = procedure_model_container_category_id
            .try_into()
            .map_err(|err: <P as TryInto<i32>>::Error| {
                Into::into(err)
                    .rename_field(
                        InsertableStepModelContainerCategoryAttributes::ProcedureModelContainerCategoryId,
                    )
            })?;
        self.procedure_model_container_category_id = Some(procedure_model_container_category_id);
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
            Into::into(err).rename_field(InsertableStepModelContainerCategoryAttributes::CreatedBy)
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
                Into::into(err)
                    .rename_field(InsertableStepModelContainerCategoryAttributes::CreatedAt)
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
            Into::into(err).rename_field(InsertableStepModelContainerCategoryAttributes::UpdatedBy)
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
                Into::into(err)
                    .rename_field(InsertableStepModelContainerCategoryAttributes::UpdatedAt)
            },
        )?;
        self.updated_at = Some(updated_at);
        Ok(self)
    }
}
impl common_traits::prelude::Builder for InsertableStepModelContainerCategoryBuilder {
    type Error =
        web_common_traits::database::InsertError<InsertableStepModelContainerCategoryAttributes>;
    type Object = InsertableStepModelContainerCategory;
    type Attribute = InsertableStepModelContainerCategoryAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            step_model_id: self
                .step_model_id
                .ok_or(
                    common_traits::prelude::BuilderError::IncompleteBuild(
                        InsertableStepModelContainerCategoryAttributes::StepModelId,
                    ),
                )?,
            procedure_model_container_category_id: self
                .procedure_model_container_category_id
                .ok_or(
                    common_traits::prelude::BuilderError::IncompleteBuild(
                        InsertableStepModelContainerCategoryAttributes::ProcedureModelContainerCategoryId,
                    ),
                )?,
            created_by: self
                .created_by
                .ok_or(
                    common_traits::prelude::BuilderError::IncompleteBuild(
                        InsertableStepModelContainerCategoryAttributes::CreatedBy,
                    ),
                )?,
            created_at: self
                .created_at
                .ok_or(
                    common_traits::prelude::BuilderError::IncompleteBuild(
                        InsertableStepModelContainerCategoryAttributes::CreatedAt,
                    ),
                )?,
            updated_by: self
                .updated_by
                .ok_or(
                    common_traits::prelude::BuilderError::IncompleteBuild(
                        InsertableStepModelContainerCategoryAttributes::UpdatedBy,
                    ),
                )?,
            updated_at: self
                .updated_at
                .ok_or(
                    common_traits::prelude::BuilderError::IncompleteBuild(
                        InsertableStepModelContainerCategoryAttributes::UpdatedAt,
                    ),
                )?,
        })
    }
}
impl TryFrom<InsertableStepModelContainerCategory> for InsertableStepModelContainerCategoryBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(
        insertable_variant: InsertableStepModelContainerCategory,
    ) -> Result<Self, Self::Error> {
        Self::default()
            .step_model_id(insertable_variant.step_model_id)?
            .procedure_model_container_category_id(
                insertable_variant.procedure_model_container_category_id,
            )?
            .created_by(insertable_variant.created_by)?
            .created_at(insertable_variant.created_at)?
            .updated_by(insertable_variant.updated_by)?
            .updated_at(insertable_variant.updated_at)
    }
}
