#[derive(Clone, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableStepModelContainerCategoryAttributes {
    StepModelId,
    ContainerCategoryId,
    ExpectedKelvin,
    ToleranceKelvin,
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
            InsertableStepModelContainerCategoryAttributes::ContainerCategoryId => {
                write!(f, "container_category_id")
            }
            InsertableStepModelContainerCategoryAttributes::ExpectedKelvin => {
                write!(f, "expected_kelvin")
            }
            InsertableStepModelContainerCategoryAttributes::ToleranceKelvin => {
                write!(f, "tolerance_kelvin")
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
    container_category_id: i16,
    expected_kelvin: f32,
    tolerance_kelvin: f32,
    created_by: i32,
    created_at: rosetta_timestamp::TimestampUTC,
    updated_by: i32,
    updated_at: rosetta_timestamp::TimestampUTC,
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
    pub async fn container_category(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::container_categories::ContainerCategory,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::container_categories::ContainerCategory::table()
            .filter(
                crate::codegen::diesel_codegen::tables::container_categories::container_categories::dsl::id
                    .eq(&self.container_category_id),
            )
            .first::<
                crate::codegen::structs_codegen::tables::container_categories::ContainerCategory,
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
pub struct InsertableStepModelContainerCategoryBuilder {
    step_model_id: Option<i32>,
    container_category_id: Option<i16>,
    expected_kelvin: Option<f32>,
    tolerance_kelvin: Option<f32>,
    created_by: Option<i32>,
    created_at: Option<rosetta_timestamp::TimestampUTC>,
    updated_by: Option<i32>,
    updated_at: Option<rosetta_timestamp::TimestampUTC>,
}
impl InsertableStepModelContainerCategoryBuilder {
    pub fn step_model_id(
        mut self,
        step_model_id: i32,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.step_model_id = Some(step_model_id);
        Ok(self)
    }
    pub fn container_category_id(
        mut self,
        container_category_id: i16,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.container_category_id = Some(container_category_id);
        Ok(self)
    }
    pub fn expected_kelvin(
        mut self,
        expected_kelvin: f32,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        if let Some(tolerance_kelvin) = self.tolerance_kelvin {
            pgrx_validation::must_be_strictly_smaller_than_f32(tolerance_kelvin, expected_kelvin)
                .map_err(|e| {
                e.rename_fields(
                    InsertableStepModelContainerCategoryAttributes::ToleranceKelvin,
                    InsertableStepModelContainerCategoryAttributes::ExpectedKelvin,
                )
            })?;
        }
        pgrx_validation::must_be_strictly_positive_f32(expected_kelvin).map_err(|e| {
            e.rename_field(InsertableStepModelContainerCategoryAttributes::ExpectedKelvin)
        })?;
        self.expected_kelvin = Some(expected_kelvin);
        Ok(self)
    }
    pub fn tolerance_kelvin(
        mut self,
        tolerance_kelvin: f32,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        if let Some(expected_kelvin) = self.expected_kelvin {
            pgrx_validation::must_be_strictly_smaller_than_f32(tolerance_kelvin, expected_kelvin)
                .map_err(|e| {
                e.rename_fields(
                    InsertableStepModelContainerCategoryAttributes::ToleranceKelvin,
                    InsertableStepModelContainerCategoryAttributes::ExpectedKelvin,
                )
            })?;
        }
        pgrx_validation::must_be_strictly_positive_f32(tolerance_kelvin).map_err(|e| {
            e.rename_field(InsertableStepModelContainerCategoryAttributes::ToleranceKelvin)
        })?;
        self.tolerance_kelvin = Some(tolerance_kelvin);
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
impl common_traits::prelude::Builder for InsertableStepModelContainerCategoryBuilder {
    type Error =
        web_common_traits::database::InsertError<InsertableStepModelContainerCategoryAttributes>;
    type Object = InsertableStepModelContainerCategory;
    type Attribute = InsertableStepModelContainerCategoryAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            step_model_id: self.step_model_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableStepModelContainerCategoryAttributes::StepModelId,
                ),
            )?,
            container_category_id: self.container_category_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableStepModelContainerCategoryAttributes::ContainerCategoryId,
                ),
            )?,
            expected_kelvin: self.expected_kelvin.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableStepModelContainerCategoryAttributes::ExpectedKelvin,
                ),
            )?,
            tolerance_kelvin: self.tolerance_kelvin.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableStepModelContainerCategoryAttributes::ToleranceKelvin,
                ),
            )?,
            created_by: self.created_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableStepModelContainerCategoryAttributes::CreatedBy,
                ),
            )?,
            created_at: self.created_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableStepModelContainerCategoryAttributes::CreatedAt,
                ),
            )?,
            updated_by: self.updated_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableStepModelContainerCategoryAttributes::UpdatedBy,
                ),
            )?,
            updated_at: self.updated_at.ok_or(
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
            .container_category_id(insertable_variant.container_category_id)?
            .expected_kelvin(insertable_variant.expected_kelvin)?
            .tolerance_kelvin(insertable_variant.tolerance_kelvin)?
            .created_by(insertable_variant.created_by)?
            .created_at(insertable_variant.created_at)?
            .updated_by(insertable_variant.updated_by)?
            .updated_at(insertable_variant.updated_at)
    }
}
