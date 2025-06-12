#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableAliquotingInstrumentModelAttributes {
    Id(crate::codegen::structs_codegen::tables::insertables::InsertableInstrumentModelAttributes),
    ErrorLiters,
    MinimumMeasurableLiters,
    MaximumMeasurableLiters,
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertableInstrumentModelAttributes>
    for InsertableAliquotingInstrumentModelAttributes
{
    fn from(
        extension: crate::codegen::structs_codegen::tables::insertables::InsertableInstrumentModelAttributes,
    ) -> Self {
        Self::Id(extension)
    }
}
impl core::fmt::Display for InsertableAliquotingInstrumentModelAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableAliquotingInstrumentModelAttributes::Id(id) => write!(f, "{}", id),
            InsertableAliquotingInstrumentModelAttributes::ErrorLiters => {
                write!(f, "error_liters")
            }
            InsertableAliquotingInstrumentModelAttributes::MinimumMeasurableLiters => {
                write!(f, "minimum_measurable_liters")
            }
            InsertableAliquotingInstrumentModelAttributes::MaximumMeasurableLiters => {
                write!(f, "maximum_measurable_liters")
            }
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::aliquoting_instrument_models::aliquoting_instrument_models
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableAliquotingInstrumentModel {
    id: ::rosetta_uuid::Uuid,
    error_liters: f32,
    minimum_measurable_liters: f32,
    maximum_measurable_liters: f32,
}
impl InsertableAliquotingInstrumentModel {
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
pub struct InsertableAliquotingInstrumentModelBuilder {
    pub(crate) id:
        crate::codegen::structs_codegen::tables::insertables::InsertableInstrumentModelBuilder,
    pub(crate) error_liters: Option<f32>,
    pub(crate) minimum_measurable_liters: Option<f32>,
    pub(crate) maximum_measurable_liters: Option<f32>,
}
impl InsertableAliquotingInstrumentModelBuilder {
    pub fn error_liters<P>(
        mut self,
        error_liters: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableAliquotingInstrumentModelAttributes>,
    >
    where
        P: TryInto<f32>,
        <P as TryInto<f32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let error_liters = error_liters.try_into().map_err(|err: <P as TryInto<f32>>::Error| {
            Into::into(err).rename_field(InsertableAliquotingInstrumentModelAttributes::ErrorLiters)
        })?;
        if let Some(minimum_measurable_liters) = self.minimum_measurable_liters {
            pgrx_validation::must_be_strictly_smaller_than_f32(
                error_liters,
                minimum_measurable_liters,
            )
            .map_err(|e| {
                e.rename_fields(
                    InsertableAliquotingInstrumentModelAttributes::ErrorLiters,
                    InsertableAliquotingInstrumentModelAttributes::MinimumMeasurableLiters,
                )
            })?;
        }
        pgrx_validation::must_be_strictly_positive_f32(error_liters).map_err(|e| {
            e.rename_field(InsertableAliquotingInstrumentModelAttributes::ErrorLiters)
        })?;
        self.error_liters = Some(error_liters);
        Ok(self)
    }
    pub fn minimum_measurable_liters<P>(
        mut self,
        minimum_measurable_liters: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableAliquotingInstrumentModelAttributes>,
    >
    where
        P: TryInto<f32>,
        <P as TryInto<f32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let minimum_measurable_liters =
            minimum_measurable_liters.try_into().map_err(|err: <P as TryInto<f32>>::Error| {
                Into::into(err).rename_field(
                    InsertableAliquotingInstrumentModelAttributes::MinimumMeasurableLiters,
                )
            })?;
        if let Some(error_liters) = self.error_liters {
            pgrx_validation::must_be_strictly_smaller_than_f32(
                error_liters,
                minimum_measurable_liters,
            )
            .map_err(|e| {
                e.rename_fields(
                    InsertableAliquotingInstrumentModelAttributes::ErrorLiters,
                    InsertableAliquotingInstrumentModelAttributes::MinimumMeasurableLiters,
                )
            })?;
        }
        if let Some(maximum_measurable_liters) = self.maximum_measurable_liters {
            pgrx_validation::must_be_strictly_smaller_than_f32(
                minimum_measurable_liters,
                maximum_measurable_liters,
            )
            .map_err(|e| {
                e.rename_fields(
                    InsertableAliquotingInstrumentModelAttributes::MinimumMeasurableLiters,
                    InsertableAliquotingInstrumentModelAttributes::MaximumMeasurableLiters,
                )
            })?;
        }
        pgrx_validation::must_be_strictly_positive_f32(minimum_measurable_liters).map_err(|e| {
            e.rename_field(InsertableAliquotingInstrumentModelAttributes::MinimumMeasurableLiters)
        })?;
        self.minimum_measurable_liters = Some(minimum_measurable_liters);
        Ok(self)
    }
    pub fn maximum_measurable_liters<P>(
        mut self,
        maximum_measurable_liters: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableAliquotingInstrumentModelAttributes>,
    >
    where
        P: TryInto<f32>,
        <P as TryInto<f32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let maximum_measurable_liters =
            maximum_measurable_liters.try_into().map_err(|err: <P as TryInto<f32>>::Error| {
                Into::into(err).rename_field(
                    InsertableAliquotingInstrumentModelAttributes::MaximumMeasurableLiters,
                )
            })?;
        if let Some(minimum_measurable_liters) = self.minimum_measurable_liters {
            pgrx_validation::must_be_strictly_smaller_than_f32(
                minimum_measurable_liters,
                maximum_measurable_liters,
            )
            .map_err(|e| {
                e.rename_fields(
                    InsertableAliquotingInstrumentModelAttributes::MinimumMeasurableLiters,
                    InsertableAliquotingInstrumentModelAttributes::MaximumMeasurableLiters,
                )
            })?;
        }
        pgrx_validation::must_be_strictly_positive_f32(maximum_measurable_liters).map_err(|e| {
            e.rename_field(InsertableAliquotingInstrumentModelAttributes::MaximumMeasurableLiters)
        })?;
        self.maximum_measurable_liters = Some(maximum_measurable_liters);
        Ok(self)
    }
    pub fn deprecation_date<P>(
        mut self,
        deprecation_date: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableAliquotingInstrumentModelAttributes>,
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
        web_common_traits::database::InsertError<InsertableAliquotingInstrumentModelAttributes>,
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
        web_common_traits::database::InsertError<InsertableAliquotingInstrumentModelAttributes>,
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
        web_common_traits::database::InsertError<InsertableAliquotingInstrumentModelAttributes>,
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
        web_common_traits::database::InsertError<InsertableAliquotingInstrumentModelAttributes>,
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
        web_common_traits::database::InsertError<InsertableAliquotingInstrumentModelAttributes>,
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
        web_common_traits::database::InsertError<InsertableAliquotingInstrumentModelAttributes>,
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
        web_common_traits::database::InsertError<InsertableAliquotingInstrumentModelAttributes>,
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
        web_common_traits::database::InsertError<InsertableAliquotingInstrumentModelAttributes>,
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
        web_common_traits::database::InsertError<InsertableAliquotingInstrumentModelAttributes>,
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
        web_common_traits::database::InsertError<InsertableAliquotingInstrumentModelAttributes>,
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
impl InsertableAliquotingInstrumentModelBuilder {
    pub(crate) fn try_insert<C>(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<
        InsertableAliquotingInstrumentModel,
        web_common_traits::database::InsertError<
            InsertableAliquotingInstrumentModelAttributes,
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
        let error_liters =
            self.error_liters.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableAliquotingInstrumentModelAttributes::ErrorLiters,
            ))?;
        let minimum_measurable_liters = self.minimum_measurable_liters.ok_or(
            common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableAliquotingInstrumentModelAttributes::MinimumMeasurableLiters,
            ),
        )?;
        let maximum_measurable_liters = self.maximum_measurable_liters.ok_or(
            common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableAliquotingInstrumentModelAttributes::MaximumMeasurableLiters,
            ),
        )?;
        let id = self.id.insert(user_id, conn).map_err(|err| err.into_field_name())?.id();
        Ok(InsertableAliquotingInstrumentModel {
            id,
            error_liters,
            minimum_measurable_liters,
            maximum_measurable_liters,
        })
    }
}
