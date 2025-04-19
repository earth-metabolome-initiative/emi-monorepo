#[derive(Clone, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableFractioningStepModelAttributes {
    Id,
    StepModelInstrumentCategoryId,
    ExpectedKilograms,
    ToleranceKilograms,
    CreatedBy,
    CreatedAt,
    UpdatedBy,
    UpdatedAt,
}
impl core::fmt::Display for InsertableFractioningStepModelAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableFractioningStepModelAttributes::Id => write!(f, "id"),
            InsertableFractioningStepModelAttributes::StepModelInstrumentCategoryId => {
                write!(f, "step_model_instrument_category_id")
            }
            InsertableFractioningStepModelAttributes::ExpectedKilograms => {
                write!(f, "expected_kilograms")
            }
            InsertableFractioningStepModelAttributes::ToleranceKilograms => {
                write!(f, "tolerance_kilograms")
            }
            InsertableFractioningStepModelAttributes::CreatedBy => {
                write!(f, "created_by")
            }
            InsertableFractioningStepModelAttributes::CreatedAt => {
                write!(f, "created_at")
            }
            InsertableFractioningStepModelAttributes::UpdatedBy => {
                write!(f, "updated_by")
            }
            InsertableFractioningStepModelAttributes::UpdatedAt => {
                write!(f, "updated_at")
            }
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::fractioning_step_models::fractioning_step_models
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableFractioningStepModel {
    id: i32,
    step_model_instrument_category_id: i32,
    expected_kilograms: f32,
    tolerance_kilograms: f32,
    created_by: i32,
    created_at: rosetta_timestamp::TimestampUTC,
    updated_by: i32,
    updated_at: rosetta_timestamp::TimestampUTC,
}
impl InsertableFractioningStepModel {
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
    pub async fn step_model_instrument_category(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::step_model_instrument_categories::StepModelInstrumentCategory,
        diesel::result::Error,
    >{
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::step_model_instrument_categories::StepModelInstrumentCategory::table()
            .filter(
                crate::codegen::diesel_codegen::tables::step_model_instrument_categories::step_model_instrument_categories::dsl::id
                    .eq(&self.step_model_instrument_category_id),
            )
            .first::<
                crate::codegen::structs_codegen::tables::step_model_instrument_categories::StepModelInstrumentCategory,
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
pub struct InsertableFractioningStepModelBuilder {
    id: Option<i32>,
    step_model_instrument_category_id: Option<i32>,
    expected_kilograms: Option<f32>,
    tolerance_kilograms: Option<f32>,
    created_by: Option<i32>,
    created_at: Option<rosetta_timestamp::TimestampUTC>,
    updated_by: Option<i32>,
    updated_at: Option<rosetta_timestamp::TimestampUTC>,
}
impl InsertableFractioningStepModelBuilder {
    pub fn id(mut self, id: i32) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.id = Some(id);
        Ok(self)
    }
    pub fn step_model_instrument_category_id(
        mut self,
        step_model_instrument_category_id: i32,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.step_model_instrument_category_id = Some(step_model_instrument_category_id);
        Ok(self)
    }
    pub fn expected_kilograms(
        mut self,
        expected_kilograms: f32,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        if let Some(tolerance_kilograms) = self.tolerance_kilograms {
            pgrx_validation::must_be_strictly_smaller_than_f32(
                tolerance_kilograms,
                expected_kilograms,
            )
            .map_err(|e| {
                e.rename_fields(
                    InsertableFractioningStepModelAttributes::ToleranceKilograms,
                    InsertableFractioningStepModelAttributes::ExpectedKilograms,
                )
            })?;
        }
        pgrx_validation::must_be_strictly_positive_f32(expected_kilograms).map_err(|e| {
            e.rename_field(InsertableFractioningStepModelAttributes::ExpectedKilograms)
        })?;
        self.expected_kilograms = Some(expected_kilograms);
        Ok(self)
    }
    pub fn tolerance_kilograms(
        mut self,
        tolerance_kilograms: f32,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        if let Some(expected_kilograms) = self.expected_kilograms {
            pgrx_validation::must_be_strictly_smaller_than_f32(
                tolerance_kilograms,
                expected_kilograms,
            )
            .map_err(|e| {
                e.rename_fields(
                    InsertableFractioningStepModelAttributes::ToleranceKilograms,
                    InsertableFractioningStepModelAttributes::ExpectedKilograms,
                )
            })?;
        }
        pgrx_validation::must_be_strictly_positive_f32(tolerance_kilograms).map_err(|e| {
            e.rename_field(InsertableFractioningStepModelAttributes::ToleranceKilograms)
        })?;
        self.tolerance_kilograms = Some(tolerance_kilograms);
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
impl common_traits::prelude::Builder for InsertableFractioningStepModelBuilder {
    type Error = web_common_traits::database::InsertError<InsertableFractioningStepModelAttributes>;
    type Object = InsertableFractioningStepModel;
    type Attribute = InsertableFractioningStepModelAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            id: self.id.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableFractioningStepModelAttributes::Id,
                )
            })?,
            step_model_instrument_category_id: self.step_model_instrument_category_id.ok_or_else(
                || {
                    common_traits::prelude::BuilderError::IncompleteBuild(
                        InsertableFractioningStepModelAttributes::StepModelInstrumentCategoryId,
                    )
                },
            )?,
            expected_kilograms: self.expected_kilograms.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableFractioningStepModelAttributes::ExpectedKilograms,
                )
            })?,
            tolerance_kilograms: self.tolerance_kilograms.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableFractioningStepModelAttributes::ToleranceKilograms,
                )
            })?,
            created_by: self.created_by.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableFractioningStepModelAttributes::CreatedBy,
                )
            })?,
            created_at: self.created_at.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableFractioningStepModelAttributes::CreatedAt,
                )
            })?,
            updated_by: self.updated_by.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableFractioningStepModelAttributes::UpdatedBy,
                )
            })?,
            updated_at: self.updated_at.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableFractioningStepModelAttributes::UpdatedAt,
                )
            })?,
        })
    }
}
impl TryFrom<InsertableFractioningStepModel> for InsertableFractioningStepModelBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableFractioningStepModel) -> Result<Self, Self::Error> {
        Ok(Self::default()
            .id(insertable_variant.id)?
            .step_model_instrument_category_id(
                insertable_variant.step_model_instrument_category_id,
            )?
            .expected_kilograms(insertable_variant.expected_kilograms)?
            .tolerance_kilograms(insertable_variant.tolerance_kilograms)?
            .created_by(insertable_variant.created_by)?
            .created_at(insertable_variant.created_at)?
            .updated_by(insertable_variant.updated_by)?
            .updated_at(insertable_variant.updated_at)?)
    }
}
