#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableAliquotingInstrumentModelAttributes {
    Id,
    ErrorLiters,
    MinimumMeasurableLiters,
    MaximumMeasurableLiters,
    CreatedBy,
    CreatedAt,
    UpdatedBy,
    UpdatedAt,
}
impl core::fmt::Display for InsertableAliquotingInstrumentModelAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableAliquotingInstrumentModelAttributes::Id => write!(f, "id"),
            InsertableAliquotingInstrumentModelAttributes::ErrorLiters => {
                write!(f, "error_liters")
            }
            InsertableAliquotingInstrumentModelAttributes::MinimumMeasurableLiters => {
                write!(f, "minimum_measurable_liters")
            }
            InsertableAliquotingInstrumentModelAttributes::MaximumMeasurableLiters => {
                write!(f, "maximum_measurable_liters")
            }
            InsertableAliquotingInstrumentModelAttributes::CreatedBy => {
                write!(f, "created_by")
            }
            InsertableAliquotingInstrumentModelAttributes::CreatedAt => {
                write!(f, "created_at")
            }
            InsertableAliquotingInstrumentModelAttributes::UpdatedBy => {
                write!(f, "updated_by")
            }
            InsertableAliquotingInstrumentModelAttributes::UpdatedAt => {
                write!(f, "updated_at")
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
    id: i32,
    error_liters: f32,
    minimum_measurable_liters: f32,
    maximum_measurable_liters: f32,
    created_by: i32,
    created_at: ::rosetta_timestamp::TimestampUTC,
    updated_by: i32,
    updated_at: ::rosetta_timestamp::TimestampUTC,
}
impl InsertableAliquotingInstrumentModel {
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
pub struct InsertableAliquotingInstrumentModelBuilder {
    id: Option<i32>,
    error_liters: Option<f32>,
    minimum_measurable_liters: Option<f32>,
    maximum_measurable_liters: Option<f32>,
    created_by: Option<i32>,
    created_at: Option<::rosetta_timestamp::TimestampUTC>,
    updated_by: Option<i32>,
    updated_at: Option<::rosetta_timestamp::TimestampUTC>,
}
impl Default for InsertableAliquotingInstrumentModelBuilder {
    fn default() -> Self {
        Self {
            id: None,
            error_liters: None,
            minimum_measurable_liters: None,
            maximum_measurable_liters: None,
            created_by: None,
            created_at: Some(rosetta_timestamp::TimestampUTC::default()),
            updated_by: None,
            updated_at: Some(rosetta_timestamp::TimestampUTC::default()),
        }
    }
}
impl InsertableAliquotingInstrumentModelBuilder {
    pub fn id<P>(mut self, id: P) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let id = id.try_into().map_err(|err: <P as TryInto<i32>>::Error| {
            Into::into(err).rename_field(InsertableAliquotingInstrumentModelAttributes::Id)
        })?;
        self.id = Some(id);
        Ok(self)
    }
    pub fn error_liters<P>(
        mut self,
        error_liters: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
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
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
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
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
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
    pub fn created_by<P>(
        mut self,
        created_by: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error>
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let created_by = created_by.try_into().map_err(|err: <P as TryInto<i32>>::Error| {
            Into::into(err).rename_field(InsertableAliquotingInstrumentModelAttributes::CreatedBy)
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
                    .rename_field(InsertableAliquotingInstrumentModelAttributes::CreatedAt)
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
            Into::into(err).rename_field(InsertableAliquotingInstrumentModelAttributes::UpdatedBy)
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
                    .rename_field(InsertableAliquotingInstrumentModelAttributes::UpdatedAt)
            },
        )?;
        self.updated_at = Some(updated_at);
        Ok(self)
    }
}
impl common_traits::prelude::Builder for InsertableAliquotingInstrumentModelBuilder {
    type Error =
        web_common_traits::database::InsertError<InsertableAliquotingInstrumentModelAttributes>;
    type Object = InsertableAliquotingInstrumentModel;
    type Attribute = InsertableAliquotingInstrumentModelAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            id: self.id.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableAliquotingInstrumentModelAttributes::Id,
            ))?,
            error_liters: self.error_liters.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableAliquotingInstrumentModelAttributes::ErrorLiters,
                ),
            )?,
            minimum_measurable_liters: self.minimum_measurable_liters.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableAliquotingInstrumentModelAttributes::MinimumMeasurableLiters,
                ),
            )?,
            maximum_measurable_liters: self.maximum_measurable_liters.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableAliquotingInstrumentModelAttributes::MaximumMeasurableLiters,
                ),
            )?,
            created_by: self.created_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableAliquotingInstrumentModelAttributes::CreatedBy,
                ),
            )?,
            created_at: self.created_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableAliquotingInstrumentModelAttributes::CreatedAt,
                ),
            )?,
            updated_by: self.updated_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableAliquotingInstrumentModelAttributes::UpdatedBy,
                ),
            )?,
            updated_at: self.updated_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableAliquotingInstrumentModelAttributes::UpdatedAt,
                ),
            )?,
        })
    }
}
impl TryFrom<InsertableAliquotingInstrumentModel> for InsertableAliquotingInstrumentModelBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(
        insertable_variant: InsertableAliquotingInstrumentModel,
    ) -> Result<Self, Self::Error> {
        Self::default()
            .id(insertable_variant.id)?
            .error_liters(insertable_variant.error_liters)?
            .minimum_measurable_liters(insertable_variant.minimum_measurable_liters)?
            .maximum_measurable_liters(insertable_variant.maximum_measurable_liters)?
            .created_by(insertable_variant.created_by)?
            .created_at(insertable_variant.created_at)?
            .updated_by(insertable_variant.updated_by)?
            .updated_at(insertable_variant.updated_at)
    }
}
