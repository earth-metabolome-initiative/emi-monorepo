#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableProcedureModelContainerCategoryAttributes {
    ProcedureModelId,
    ContainerCategory,
    ExpectedKelvin,
    ToleranceKelvin,
    CreatedBy,
    CreatedAt,
    UpdatedBy,
    UpdatedAt,
}
impl core::fmt::Display for InsertableProcedureModelContainerCategoryAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableProcedureModelContainerCategoryAttributes::ProcedureModelId => {
                write!(f, "procedure_model_id")
            }
            InsertableProcedureModelContainerCategoryAttributes::ContainerCategory => {
                write!(f, "container_category")
            }
            InsertableProcedureModelContainerCategoryAttributes::ExpectedKelvin => {
                write!(f, "expected_kelvin")
            }
            InsertableProcedureModelContainerCategoryAttributes::ToleranceKelvin => {
                write!(f, "tolerance_kelvin")
            }
            InsertableProcedureModelContainerCategoryAttributes::CreatedBy => {
                write!(f, "created_by")
            }
            InsertableProcedureModelContainerCategoryAttributes::CreatedAt => {
                write!(f, "created_at")
            }
            InsertableProcedureModelContainerCategoryAttributes::UpdatedBy => {
                write!(f, "updated_by")
            }
            InsertableProcedureModelContainerCategoryAttributes::UpdatedAt => {
                write!(f, "updated_at")
            }
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::procedure_model_container_categories::procedure_model_container_categories
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableProcedureModelContainerCategory {
    procedure_model_id: i32,
    container_category: ::container_categories::ContainerCategory,
    expected_kelvin: f32,
    tolerance_kelvin: f32,
    created_by: i32,
    created_at: ::rosetta_timestamp::TimestampUTC,
    updated_by: i32,
    updated_at: ::rosetta_timestamp::TimestampUTC,
}
impl InsertableProcedureModelContainerCategory {
    #[cfg(feature = "postgres")]
    pub async fn procedure_model(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_models::procedure_models::dsl::id
                    .eq(&self.procedure_model_id),
            )
            .first::<crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel>(
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
pub struct InsertableProcedureModelContainerCategoryBuilder {
    procedure_model_id: Option<i32>,
    container_category: Option<::container_categories::ContainerCategory>,
    expected_kelvin: Option<f32>,
    tolerance_kelvin: Option<f32>,
    created_by: Option<i32>,
    created_at: Option<::rosetta_timestamp::TimestampUTC>,
    updated_by: Option<i32>,
    updated_at: Option<::rosetta_timestamp::TimestampUTC>,
}
impl Default for InsertableProcedureModelContainerCategoryBuilder {
    fn default() -> Self {
        Self {
            procedure_model_id: None,
            container_category: None,
            expected_kelvin: Some(293.15f32),
            tolerance_kelvin: Some(20f32),
            created_by: None,
            created_at: Some(rosetta_timestamp::TimestampUTC::default()),
            updated_by: None,
            updated_at: Some(rosetta_timestamp::TimestampUTC::default()),
        }
    }
}
impl InsertableProcedureModelContainerCategoryBuilder {
    pub fn procedure_model_id<P>(
        mut self,
        procedure_model_id: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let procedure_model_id =
            procedure_model_id.try_into().map_err(|err: <P as TryInto<i32>>::Error| {
                Into::into(err).rename_field(
                    InsertableProcedureModelContainerCategoryAttributes::ProcedureModelId,
                )
            })?;
        self.procedure_model_id = Some(procedure_model_id);
        Ok(self)
    }
    pub fn container_category<P>(
        mut self,
        container_category: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<::container_categories::ContainerCategory>,
        <P as TryInto<::container_categories::ContainerCategory>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let container_category = container_category.try_into().map_err(
            |err: <P as TryInto<::container_categories::ContainerCategory>>::Error| {
                Into::into(err).rename_field(
                    InsertableProcedureModelContainerCategoryAttributes::ContainerCategory,
                )
            },
        )?;
        self.container_category = Some(container_category);
        Ok(self)
    }
    pub fn expected_kelvin<P>(
        mut self,
        expected_kelvin: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<f32>,
        <P as TryInto<f32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let expected_kelvin =
            expected_kelvin.try_into().map_err(|err: <P as TryInto<f32>>::Error| {
                Into::into(err).rename_field(
                    InsertableProcedureModelContainerCategoryAttributes::ExpectedKelvin,
                )
            })?;
        if let Some(tolerance_kelvin) = self.tolerance_kelvin {
            pgrx_validation::must_be_strictly_smaller_than_f32(tolerance_kelvin, expected_kelvin)
                .map_err(|e| {
                e.rename_fields(
                    InsertableProcedureModelContainerCategoryAttributes::ToleranceKelvin,
                    InsertableProcedureModelContainerCategoryAttributes::ExpectedKelvin,
                )
            })?;
        }
        pgrx_validation::must_be_strictly_positive_f32(expected_kelvin).map_err(|e| {
            e.rename_field(InsertableProcedureModelContainerCategoryAttributes::ExpectedKelvin)
        })?;
        self.expected_kelvin = Some(expected_kelvin);
        Ok(self)
    }
    pub fn tolerance_kelvin<P>(
        mut self,
        tolerance_kelvin: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<f32>,
        <P as TryInto<f32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let tolerance_kelvin =
            tolerance_kelvin.try_into().map_err(|err: <P as TryInto<f32>>::Error| {
                Into::into(err).rename_field(
                    InsertableProcedureModelContainerCategoryAttributes::ToleranceKelvin,
                )
            })?;
        if let Some(expected_kelvin) = self.expected_kelvin {
            pgrx_validation::must_be_strictly_smaller_than_f32(tolerance_kelvin, expected_kelvin)
                .map_err(|e| {
                e.rename_fields(
                    InsertableProcedureModelContainerCategoryAttributes::ToleranceKelvin,
                    InsertableProcedureModelContainerCategoryAttributes::ExpectedKelvin,
                )
            })?;
        }
        pgrx_validation::must_be_strictly_positive_f32(tolerance_kelvin).map_err(|e| {
            e.rename_field(InsertableProcedureModelContainerCategoryAttributes::ToleranceKelvin)
        })?;
        self.tolerance_kelvin = Some(tolerance_kelvin);
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
            Into::into(err)
                .rename_field(InsertableProcedureModelContainerCategoryAttributes::CreatedBy)
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
                    .rename_field(InsertableProcedureModelContainerCategoryAttributes::CreatedAt)
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
            Into::into(err)
                .rename_field(InsertableProcedureModelContainerCategoryAttributes::UpdatedBy)
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
                    .rename_field(InsertableProcedureModelContainerCategoryAttributes::UpdatedAt)
            },
        )?;
        self.updated_at = Some(updated_at);
        Ok(self)
    }
}
impl common_traits::prelude::Builder for InsertableProcedureModelContainerCategoryBuilder {
    type Error = web_common_traits::database::InsertError<
        InsertableProcedureModelContainerCategoryAttributes,
    >;
    type Object = InsertableProcedureModelContainerCategory;
    type Attribute = InsertableProcedureModelContainerCategoryAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            procedure_model_id: self.procedure_model_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableProcedureModelContainerCategoryAttributes::ProcedureModelId,
                ),
            )?,
            container_category: self.container_category.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableProcedureModelContainerCategoryAttributes::ContainerCategory,
                ),
            )?,
            expected_kelvin: self.expected_kelvin.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableProcedureModelContainerCategoryAttributes::ExpectedKelvin,
                ),
            )?,
            tolerance_kelvin: self.tolerance_kelvin.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableProcedureModelContainerCategoryAttributes::ToleranceKelvin,
                ),
            )?,
            created_by: self.created_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableProcedureModelContainerCategoryAttributes::CreatedBy,
                ),
            )?,
            created_at: self.created_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableProcedureModelContainerCategoryAttributes::CreatedAt,
                ),
            )?,
            updated_by: self.updated_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableProcedureModelContainerCategoryAttributes::UpdatedBy,
                ),
            )?,
            updated_at: self.updated_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableProcedureModelContainerCategoryAttributes::UpdatedAt,
                ),
            )?,
        })
    }
}
impl TryFrom<InsertableProcedureModelContainerCategory>
    for InsertableProcedureModelContainerCategoryBuilder
{
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(
        insertable_variant: InsertableProcedureModelContainerCategory,
    ) -> Result<Self, Self::Error> {
        Self::default()
            .procedure_model_id(insertable_variant.procedure_model_id)?
            .container_category(insertable_variant.container_category)?
            .expected_kelvin(insertable_variant.expected_kelvin)?
            .tolerance_kelvin(insertable_variant.tolerance_kelvin)?
            .created_by(insertable_variant.created_by)?
            .created_at(insertable_variant.created_at)?
            .updated_by(insertable_variant.updated_by)?
            .updated_at(insertable_variant.updated_at)
    }
}
