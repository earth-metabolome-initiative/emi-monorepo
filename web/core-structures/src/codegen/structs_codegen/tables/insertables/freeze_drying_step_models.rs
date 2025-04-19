#[derive(Clone, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableFreezeDryingStepModelAttributes {
    Id,
    StepModelInstrumentCategoryId,
    ExpectedKelvin,
    ExpectedPascal,
    ExpectedSeconds,
    CreatedBy,
    CreatedAt,
    UpdatedBy,
    UpdatedAt,
}
impl core::fmt::Display for InsertableFreezeDryingStepModelAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableFreezeDryingStepModelAttributes::Id => write!(f, "id"),
            InsertableFreezeDryingStepModelAttributes::StepModelInstrumentCategoryId => {
                write!(f, "step_model_instrument_category_id")
            }
            InsertableFreezeDryingStepModelAttributes::ExpectedKelvin => {
                write!(f, "expected_kelvin")
            }
            InsertableFreezeDryingStepModelAttributes::ExpectedPascal => {
                write!(f, "expected_pascal")
            }
            InsertableFreezeDryingStepModelAttributes::ExpectedSeconds => {
                write!(f, "expected_seconds")
            }
            InsertableFreezeDryingStepModelAttributes::CreatedBy => {
                write!(f, "created_by")
            }
            InsertableFreezeDryingStepModelAttributes::CreatedAt => {
                write!(f, "created_at")
            }
            InsertableFreezeDryingStepModelAttributes::UpdatedBy => {
                write!(f, "updated_by")
            }
            InsertableFreezeDryingStepModelAttributes::UpdatedAt => {
                write!(f, "updated_at")
            }
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::freeze_drying_step_models::freeze_drying_step_models
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableFreezeDryingStepModel {
    id: i32,
    step_model_instrument_category_id: i32,
    expected_kelvin: f32,
    expected_pascal: f32,
    expected_seconds: f32,
    created_by: i32,
    created_at: rosetta_timestamp::TimestampUTC,
    updated_by: i32,
    updated_at: rosetta_timestamp::TimestampUTC,
}
impl InsertableFreezeDryingStepModel {
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
pub struct InsertableFreezeDryingStepModelBuilder {
    id: Option<i32>,
    step_model_instrument_category_id: Option<i32>,
    expected_kelvin: Option<f32>,
    expected_pascal: Option<f32>,
    expected_seconds: Option<f32>,
    created_by: Option<i32>,
    created_at: Option<rosetta_timestamp::TimestampUTC>,
    updated_by: Option<i32>,
    updated_at: Option<rosetta_timestamp::TimestampUTC>,
}
impl InsertableFreezeDryingStepModelBuilder {
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
    pub fn expected_kelvin(
        mut self,
        expected_kelvin: f32,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        pgrx_validation::must_be_strictly_positive_f32(expected_kelvin).map_err(|e| {
            e.rename_field(InsertableFreezeDryingStepModelAttributes::ExpectedKelvin)
        })?;
        pgrx_validation::must_be_strictly_smaller_than_f32(expected_kelvin, 250f32).map_err(
            |e| e.rename_field(InsertableFreezeDryingStepModelAttributes::ExpectedKelvin),
        )?;
        self.expected_kelvin = Some(expected_kelvin);
        Ok(self)
    }
    pub fn expected_pascal(
        mut self,
        expected_pascal: f32,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        pgrx_validation::must_be_strictly_positive_f32(expected_pascal).map_err(|e| {
            e.rename_field(InsertableFreezeDryingStepModelAttributes::ExpectedPascal)
        })?;
        pgrx_validation::must_be_strictly_smaller_than_f32(expected_pascal, 100f32).map_err(
            |e| e.rename_field(InsertableFreezeDryingStepModelAttributes::ExpectedPascal),
        )?;
        self.expected_pascal = Some(expected_pascal);
        Ok(self)
    }
    pub fn expected_seconds(
        mut self,
        expected_seconds: f32,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        pgrx_validation::must_be_strictly_positive_f32(expected_seconds).map_err(|e| {
            e.rename_field(InsertableFreezeDryingStepModelAttributes::ExpectedSeconds)
        })?;
        pgrx_validation::must_be_strictly_greater_than_f32(expected_seconds, 7200f32).map_err(
            |e| e.rename_field(InsertableFreezeDryingStepModelAttributes::ExpectedSeconds),
        )?;
        pgrx_validation::must_be_strictly_smaller_than_f32(expected_seconds, 604800f32).map_err(
            |e| e.rename_field(InsertableFreezeDryingStepModelAttributes::ExpectedSeconds),
        )?;
        self.expected_seconds = Some(expected_seconds);
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
impl common_traits::prelude::Builder for InsertableFreezeDryingStepModelBuilder {
    type Error =
        web_common_traits::database::InsertError<InsertableFreezeDryingStepModelAttributes>;
    type Object = InsertableFreezeDryingStepModel;
    type Attribute = InsertableFreezeDryingStepModelAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            id: self.id.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableFreezeDryingStepModelAttributes::Id,
                )
            })?,
            step_model_instrument_category_id: self.step_model_instrument_category_id.ok_or_else(
                || {
                    common_traits::prelude::BuilderError::IncompleteBuild(
                        InsertableFreezeDryingStepModelAttributes::StepModelInstrumentCategoryId,
                    )
                },
            )?,
            expected_kelvin: self.expected_kelvin.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableFreezeDryingStepModelAttributes::ExpectedKelvin,
                )
            })?,
            expected_pascal: self.expected_pascal.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableFreezeDryingStepModelAttributes::ExpectedPascal,
                )
            })?,
            expected_seconds: self.expected_seconds.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableFreezeDryingStepModelAttributes::ExpectedSeconds,
                )
            })?,
            created_by: self.created_by.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableFreezeDryingStepModelAttributes::CreatedBy,
                )
            })?,
            created_at: self.created_at.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableFreezeDryingStepModelAttributes::CreatedAt,
                )
            })?,
            updated_by: self.updated_by.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableFreezeDryingStepModelAttributes::UpdatedBy,
                )
            })?,
            updated_at: self.updated_at.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableFreezeDryingStepModelAttributes::UpdatedAt,
                )
            })?,
        })
    }
}
impl TryFrom<InsertableFreezeDryingStepModel> for InsertableFreezeDryingStepModelBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableFreezeDryingStepModel) -> Result<Self, Self::Error> {
        Ok(Self::default()
            .id(insertable_variant.id)?
            .step_model_instrument_category_id(
                insertable_variant.step_model_instrument_category_id,
            )?
            .expected_kelvin(insertable_variant.expected_kelvin)?
            .expected_pascal(insertable_variant.expected_pascal)?
            .expected_seconds(insertable_variant.expected_seconds)?
            .created_by(insertable_variant.created_by)?
            .created_at(insertable_variant.created_at)?
            .updated_by(insertable_variant.updated_by)?
            .updated_at(insertable_variant.updated_at)?)
    }
}
