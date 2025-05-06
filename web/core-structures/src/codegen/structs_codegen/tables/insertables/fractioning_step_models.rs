#[derive(Clone, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableFractioningStepModelAttributes {
    Id,
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
pub struct InsertableFractioningStepModelBuilder {
    id: Option<i32>,
    expected_kilograms: Option<f32>,
    tolerance_kilograms: Option<f32>,
    created_by: Option<i32>,
    created_at: Option<rosetta_timestamp::TimestampUTC>,
    updated_by: Option<i32>,
    updated_at: Option<rosetta_timestamp::TimestampUTC>,
}
impl Default for InsertableFractioningStepModelBuilder {
    fn default() -> Self {
        Self {
            id: None,
            expected_kilograms: None,
            tolerance_kilograms: None,
            created_by: None,
            created_at: Some(rosetta_timestamp::TimestampUTC::default()),
            updated_by: None,
            updated_at: Some(rosetta_timestamp::TimestampUTC::default()),
        }
    }
}
impl InsertableFractioningStepModelBuilder {
    pub fn id<P: Into<i32>>(
        mut self,
        id: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let id = id.into();
        self.id = Some(id);
        Ok(self)
    }
    pub fn expected_kilograms<P: Into<f32>>(
        mut self,
        expected_kilograms: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let expected_kilograms = expected_kilograms.into();
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
    pub fn tolerance_kilograms<P: Into<f32>>(
        mut self,
        tolerance_kilograms: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let tolerance_kilograms = tolerance_kilograms.into();
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
    pub fn created_by<P: Into<i32>>(
        mut self,
        created_by: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let created_by = created_by.into();
        self.created_by = Some(created_by);
        self = self.updated_by(created_by)?;
        Ok(self)
    }
    pub fn created_at<P: Into<rosetta_timestamp::TimestampUTC>>(
        mut self,
        created_at: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let created_at = created_at.into();
        self.created_at = Some(created_at);
        Ok(self)
    }
    pub fn updated_by<P: Into<i32>>(
        mut self,
        updated_by: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let updated_by = updated_by.into();
        self.updated_by = Some(updated_by);
        Ok(self)
    }
    pub fn updated_at<P: Into<rosetta_timestamp::TimestampUTC>>(
        mut self,
        updated_at: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let updated_at = updated_at.into();
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
            id: self.id.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableFractioningStepModelAttributes::Id,
            ))?,
            expected_kilograms: self.expected_kilograms.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableFractioningStepModelAttributes::ExpectedKilograms,
                ),
            )?,
            tolerance_kilograms: self.tolerance_kilograms.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableFractioningStepModelAttributes::ToleranceKilograms,
                ),
            )?,
            created_by: self.created_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableFractioningStepModelAttributes::CreatedBy,
                ),
            )?,
            created_at: self.created_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableFractioningStepModelAttributes::CreatedAt,
                ),
            )?,
            updated_by: self.updated_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableFractioningStepModelAttributes::UpdatedBy,
                ),
            )?,
            updated_at: self.updated_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableFractioningStepModelAttributes::UpdatedAt,
                ),
            )?,
        })
    }
}
impl TryFrom<InsertableFractioningStepModel> for InsertableFractioningStepModelBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableFractioningStepModel) -> Result<Self, Self::Error> {
        Self::default()
            .id(insertable_variant.id)?
            .expected_kilograms(insertable_variant.expected_kilograms)?
            .tolerance_kilograms(insertable_variant.tolerance_kilograms)?
            .created_by(insertable_variant.created_by)?
            .created_at(insertable_variant.created_at)?
            .updated_by(insertable_variant.updated_by)?
            .updated_at(insertable_variant.updated_at)
    }
}
