#[derive(Clone, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableWeighingInstrumentModelAttributes {
    Id,
    ErrorKilograms,
    MinimumMeasurableKilograms,
    MaximumMeasurableKilograms,
    CreatedBy,
    CreatedAt,
    UpdatedBy,
    UpdatedAt,
}
impl core::fmt::Display for InsertableWeighingInstrumentModelAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableWeighingInstrumentModelAttributes::Id => write!(f, "id"),
            InsertableWeighingInstrumentModelAttributes::ErrorKilograms => {
                write!(f, "error_kilograms")
            }
            InsertableWeighingInstrumentModelAttributes::MinimumMeasurableKilograms => {
                write!(f, "minimum_measurable_kilograms")
            }
            InsertableWeighingInstrumentModelAttributes::MaximumMeasurableKilograms => {
                write!(f, "maximum_measurable_kilograms")
            }
            InsertableWeighingInstrumentModelAttributes::CreatedBy => {
                write!(f, "created_by")
            }
            InsertableWeighingInstrumentModelAttributes::CreatedAt => {
                write!(f, "created_at")
            }
            InsertableWeighingInstrumentModelAttributes::UpdatedBy => {
                write!(f, "updated_by")
            }
            InsertableWeighingInstrumentModelAttributes::UpdatedAt => {
                write!(f, "updated_at")
            }
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::weighing_instrument_models::weighing_instrument_models
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableWeighingInstrumentModel {
    id: i32,
    error_kilograms: f32,
    minimum_measurable_kilograms: f32,
    maximum_measurable_kilograms: f32,
    created_by: i32,
    created_at: rosetta_timestamp::TimestampUTC,
    updated_by: i32,
    updated_at: rosetta_timestamp::TimestampUTC,
}
impl InsertableWeighingInstrumentModel {
    #[cfg(feature = "postgres")]
    pub async fn id(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel::table()
            .filter(
                crate::codegen::diesel_codegen::tables::instrument_models::instrument_models::dsl::id
                    .eq(&self.id),
            )
            .first::<
                crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel,
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
pub struct InsertableWeighingInstrumentModelBuilder {
    id: Option<i32>,
    error_kilograms: Option<f32>,
    minimum_measurable_kilograms: Option<f32>,
    maximum_measurable_kilograms: Option<f32>,
    created_by: Option<i32>,
    created_at: Option<rosetta_timestamp::TimestampUTC>,
    updated_by: Option<i32>,
    updated_at: Option<rosetta_timestamp::TimestampUTC>,
}
impl InsertableWeighingInstrumentModelBuilder {
    pub fn id(mut self, id: i32) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.id = Some(id);
        Ok(self)
    }
    pub fn error_kilograms(
        mut self,
        error_kilograms: f32,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        if let Some(minimum_measurable_kilograms) = self.minimum_measurable_kilograms {
            pgrx_validation::must_be_strictly_smaller_than_f32(
                error_kilograms,
                minimum_measurable_kilograms,
            )
            .map_err(|e| {
                e.rename_fields(
                    InsertableWeighingInstrumentModelAttributes::ErrorKilograms,
                    InsertableWeighingInstrumentModelAttributes::MinimumMeasurableKilograms,
                )
            })?;
        }
        pgrx_validation::must_be_strictly_positive_f32(error_kilograms).map_err(|e| {
            e.rename_field(InsertableWeighingInstrumentModelAttributes::ErrorKilograms)
        })?;
        self.error_kilograms = Some(error_kilograms);
        Ok(self)
    }
    pub fn minimum_measurable_kilograms(
        mut self,
        minimum_measurable_kilograms: f32,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        if let Some(error_kilograms) = self.error_kilograms {
            pgrx_validation::must_be_strictly_smaller_than_f32(
                error_kilograms,
                minimum_measurable_kilograms,
            )
            .map_err(|e| {
                e.rename_fields(
                    InsertableWeighingInstrumentModelAttributes::ErrorKilograms,
                    InsertableWeighingInstrumentModelAttributes::MinimumMeasurableKilograms,
                )
            })?;
        }
        if let Some(maximum_measurable_kilograms) = self.maximum_measurable_kilograms {
            pgrx_validation::must_be_strictly_smaller_than_f32(
                minimum_measurable_kilograms,
                maximum_measurable_kilograms,
            )
            .map_err(|e| {
                e.rename_fields(
                    InsertableWeighingInstrumentModelAttributes::MinimumMeasurableKilograms,
                    InsertableWeighingInstrumentModelAttributes::MaximumMeasurableKilograms,
                )
            })?;
        }
        pgrx_validation::must_be_strictly_positive_f32(minimum_measurable_kilograms).map_err(
            |e| {
                e.rename_field(
                    InsertableWeighingInstrumentModelAttributes::MinimumMeasurableKilograms,
                )
            },
        )?;
        self.minimum_measurable_kilograms = Some(minimum_measurable_kilograms);
        Ok(self)
    }
    pub fn maximum_measurable_kilograms(
        mut self,
        maximum_measurable_kilograms: f32,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        if let Some(minimum_measurable_kilograms) = self.minimum_measurable_kilograms {
            pgrx_validation::must_be_strictly_smaller_than_f32(
                minimum_measurable_kilograms,
                maximum_measurable_kilograms,
            )
            .map_err(|e| {
                e.rename_fields(
                    InsertableWeighingInstrumentModelAttributes::MinimumMeasurableKilograms,
                    InsertableWeighingInstrumentModelAttributes::MaximumMeasurableKilograms,
                )
            })?;
        }
        pgrx_validation::must_be_strictly_positive_f32(maximum_measurable_kilograms).map_err(
            |e| {
                e.rename_field(
                    InsertableWeighingInstrumentModelAttributes::MaximumMeasurableKilograms,
                )
            },
        )?;
        self.maximum_measurable_kilograms = Some(maximum_measurable_kilograms);
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
impl common_traits::prelude::Builder for InsertableWeighingInstrumentModelBuilder {
    type Error =
        web_common_traits::database::InsertError<InsertableWeighingInstrumentModelAttributes>;
    type Object = InsertableWeighingInstrumentModel;
    type Attribute = InsertableWeighingInstrumentModelAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            id: self.id.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableWeighingInstrumentModelAttributes::Id,
            ))?,
            error_kilograms: self.error_kilograms.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableWeighingInstrumentModelAttributes::ErrorKilograms,
                ),
            )?,
            minimum_measurable_kilograms: self.minimum_measurable_kilograms.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableWeighingInstrumentModelAttributes::MinimumMeasurableKilograms,
                ),
            )?,
            maximum_measurable_kilograms: self.maximum_measurable_kilograms.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableWeighingInstrumentModelAttributes::MaximumMeasurableKilograms,
                ),
            )?,
            created_by: self.created_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableWeighingInstrumentModelAttributes::CreatedBy,
                ),
            )?,
            created_at: self.created_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableWeighingInstrumentModelAttributes::CreatedAt,
                ),
            )?,
            updated_by: self.updated_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableWeighingInstrumentModelAttributes::UpdatedBy,
                ),
            )?,
            updated_at: self.updated_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableWeighingInstrumentModelAttributes::UpdatedAt,
                ),
            )?,
        })
    }
}
impl TryFrom<InsertableWeighingInstrumentModel> for InsertableWeighingInstrumentModelBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(
        insertable_variant: InsertableWeighingInstrumentModel,
    ) -> Result<Self, Self::Error> {
        Self::default()
            .id(insertable_variant.id)?
            .error_kilograms(insertable_variant.error_kilograms)?
            .minimum_measurable_kilograms(insertable_variant.minimum_measurable_kilograms)?
            .maximum_measurable_kilograms(insertable_variant.maximum_measurable_kilograms)?
            .created_by(insertable_variant.created_by)?
            .created_at(insertable_variant.created_at)?
            .updated_by(insertable_variant.updated_by)?
            .updated_at(insertable_variant.updated_at)?
    }
}
