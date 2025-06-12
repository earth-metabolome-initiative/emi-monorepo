#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableWeighingInstrumentModelAttributes {
    Id(crate::codegen::structs_codegen::tables::insertables::InsertableInstrumentModelAttributes),
    ErrorKilograms,
    MinimumMeasurableKilograms,
    MaximumMeasurableKilograms,
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertableInstrumentModelAttributes>
    for InsertableWeighingInstrumentModelAttributes
{
    fn from(
        extension: crate::codegen::structs_codegen::tables::insertables::InsertableInstrumentModelAttributes,
    ) -> Self {
        Self::Id(extension)
    }
}
impl core::fmt::Display for InsertableWeighingInstrumentModelAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableWeighingInstrumentModelAttributes::Id(id) => write!(f, "{}", id),
            InsertableWeighingInstrumentModelAttributes::ErrorKilograms => {
                write!(f, "error_kilograms")
            }
            InsertableWeighingInstrumentModelAttributes::MinimumMeasurableKilograms => {
                write!(f, "minimum_measurable_kilograms")
            }
            InsertableWeighingInstrumentModelAttributes::MaximumMeasurableKilograms => {
                write!(f, "maximum_measurable_kilograms")
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
    id: ::rosetta_uuid::Uuid,
    error_kilograms: f32,
    minimum_measurable_kilograms: f32,
    maximum_measurable_kilograms: f32,
}
impl InsertableWeighingInstrumentModel {
    pub fn id<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel::table(
                ),
                self.id,
            ),
            conn,
        )
    }
}
#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableWeighingInstrumentModelBuilder {
    pub(crate) id:
        crate::codegen::structs_codegen::tables::insertables::InsertableInstrumentModelBuilder,
    pub(crate) error_kilograms: Option<f32>,
    pub(crate) minimum_measurable_kilograms: Option<f32>,
    pub(crate) maximum_measurable_kilograms: Option<f32>,
}
impl InsertableWeighingInstrumentModelBuilder {
    pub fn error_kilograms<P>(
        mut self,
        error_kilograms: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableWeighingInstrumentModelAttributes>,
    >
    where
        P: TryInto<f32>,
        <P as TryInto<f32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let error_kilograms =
            error_kilograms.try_into().map_err(|err: <P as TryInto<f32>>::Error| {
                Into::into(err)
                    .rename_field(InsertableWeighingInstrumentModelAttributes::ErrorKilograms)
            })?;
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
    pub fn minimum_measurable_kilograms<P>(
        mut self,
        minimum_measurable_kilograms: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableWeighingInstrumentModelAttributes>,
    >
    where
        P: TryInto<f32>,
        <P as TryInto<f32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let minimum_measurable_kilograms = minimum_measurable_kilograms.try_into().map_err(
            |err: <P as TryInto<f32>>::Error| {
                Into::into(err).rename_field(
                    InsertableWeighingInstrumentModelAttributes::MinimumMeasurableKilograms,
                )
            },
        )?;
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
    pub fn maximum_measurable_kilograms<P>(
        mut self,
        maximum_measurable_kilograms: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableWeighingInstrumentModelAttributes>,
    >
    where
        P: TryInto<f32>,
        <P as TryInto<f32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let maximum_measurable_kilograms = maximum_measurable_kilograms.try_into().map_err(
            |err: <P as TryInto<f32>>::Error| {
                Into::into(err).rename_field(
                    InsertableWeighingInstrumentModelAttributes::MaximumMeasurableKilograms,
                )
            },
        )?;
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
    pub fn deprecation_date<P>(
        mut self,
        deprecation_date: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableWeighingInstrumentModelAttributes>,
    >
    where
        P: TryInto<Option<::rosetta_timestamp::TimestampUTC>>,
        <P as TryInto<Option<::rosetta_timestamp::TimestampUTC>>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.id =
            self.id.deprecation_date(deprecation_date).map_err(|err| err.into_field_name())?;
        Ok(self)
    }
    pub fn brand_id<P>(
        mut self,
        brand_id: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableWeighingInstrumentModelAttributes>,
    >
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.brand_id(brand_id).map_err(|err| err.into_field_name())?;
        Ok(self)
    }
    pub fn id<P>(
        mut self,
        id: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableWeighingInstrumentModelAttributes>,
    >
    where
        P: TryInto<::rosetta_uuid::Uuid>,
        <P as TryInto<::rosetta_uuid::Uuid>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.id(id).map_err(|err| err.into_field_name())?;
        Ok(self)
    }
    pub fn name<P>(
        mut self,
        name: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableWeighingInstrumentModelAttributes>,
    >
    where
        P: TryInto<Option<String>>,
        <P as TryInto<Option<String>>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.name(name).map_err(|err| err.into_field_name())?;
        Ok(self)
    }
    pub fn description<P>(
        mut self,
        description: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableWeighingInstrumentModelAttributes>,
    >
    where
        P: TryInto<Option<String>>,
        <P as TryInto<Option<String>>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.description(description).map_err(|err| err.into_field_name())?;
        Ok(self)
    }
    pub fn photograph_id<P>(
        mut self,
        photograph_id: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableWeighingInstrumentModelAttributes>,
    >
    where
        P: TryInto<Option<::rosetta_uuid::Uuid>>,
        <P as TryInto<Option<::rosetta_uuid::Uuid>>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.photograph_id(photograph_id).map_err(|err| err.into_field_name())?;
        Ok(self)
    }
    pub fn parent_id<P>(
        mut self,
        parent_id: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableWeighingInstrumentModelAttributes>,
    >
    where
        P: TryInto<Option<::rosetta_uuid::Uuid>>,
        <P as TryInto<Option<::rosetta_uuid::Uuid>>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.parent_id(parent_id).map_err(|err| err.into_field_name())?;
        Ok(self)
    }
    pub fn created_by<P>(
        mut self,
        created_by: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableWeighingInstrumentModelAttributes>,
    >
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.created_by(created_by).map_err(|err| err.into_field_name())?;
        Ok(self)
    }
    pub fn created_at<P>(
        mut self,
        created_at: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableWeighingInstrumentModelAttributes>,
    >
    where
        P: TryInto<::rosetta_timestamp::TimestampUTC>,
        <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.created_at(created_at).map_err(|err| err.into_field_name())?;
        Ok(self)
    }
    pub fn updated_by<P>(
        mut self,
        updated_by: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableWeighingInstrumentModelAttributes>,
    >
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.updated_by(updated_by).map_err(|err| err.into_field_name())?;
        Ok(self)
    }
    pub fn updated_at<P>(
        mut self,
        updated_at: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableWeighingInstrumentModelAttributes>,
    >
    where
        P: TryInto<::rosetta_timestamp::TimestampUTC>,
        <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.updated_at(updated_at).map_err(|err| err.into_field_name())?;
        Ok(self)
    }
}
impl InsertableWeighingInstrumentModelBuilder {
    pub(crate) fn try_insert<C>(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<
        InsertableWeighingInstrumentModel,
        web_common_traits::database::InsertError<
            InsertableWeighingInstrumentModelAttributes,
        >,
    >
    where
        crate::codegen::structs_codegen::tables::insertables::InsertableInstrumentModelBuilder: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel,
            Error = web_common_traits::database::InsertError<
                crate::codegen::structs_codegen::tables::insertables::InsertableInstrumentModelAttributes,
            >,
        >,
    {
        use diesel::associations::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let error_kilograms =
            self.error_kilograms.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableWeighingInstrumentModelAttributes::ErrorKilograms,
            ))?;
        let minimum_measurable_kilograms = self.minimum_measurable_kilograms.ok_or(
            common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableWeighingInstrumentModelAttributes::MinimumMeasurableKilograms,
            ),
        )?;
        let maximum_measurable_kilograms = self.maximum_measurable_kilograms.ok_or(
            common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableWeighingInstrumentModelAttributes::MaximumMeasurableKilograms,
            ),
        )?;
        let id = self.id.insert(user_id, conn).map_err(|err| err.into_field_name())?.id();
        Ok(InsertableWeighingInstrumentModel {
            id,
            error_kilograms,
            minimum_measurable_kilograms,
            maximum_measurable_kilograms,
        })
    }
}
