#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableFractioningProcedureModelAttributes {
    Id(
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelAttributes,
    ),
    Kilograms,
    TolerancePercentage,
    WeighedWith(
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes,
    ),
    Source(
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes,
    ),
    Destination(
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes,
    ),
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelAttributes>
    for InsertableFractioningProcedureModelAttributes
{
    fn from(
        extension: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelAttributes,
    ) -> Self {
        Self::Id(extension)
    }
}
impl From<
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes,
> for InsertableFractioningProcedureModelAttributes {
    fn from(
        foreign: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes,
    ) -> Self {
        Self::Source(foreign)
    }
}
impl core::fmt::Display for InsertableFractioningProcedureModelAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableFractioningProcedureModelAttributes::Id(id) => write!(f, "{}", id),
            InsertableFractioningProcedureModelAttributes::Kilograms => {
                write!(f, "kilograms")
            }
            InsertableFractioningProcedureModelAttributes::TolerancePercentage => {
                write!(f, "tolerance_percentage")
            }
            InsertableFractioningProcedureModelAttributes::WeighedWith(weighed_with) => {
                write!(f, "{}", weighed_with)
            }
            InsertableFractioningProcedureModelAttributes::Source(source) => {
                write!(f, "{}", source)
            }
            InsertableFractioningProcedureModelAttributes::Destination(destination) => {
                write!(f, "{}", destination)
            }
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::fractioning_procedure_models::fractioning_procedure_models
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableFractioningProcedureModel {
    id: i32,
    kilograms: f32,
    tolerance_percentage: f32,
    weighed_with: i32,
    source: i32,
    destination: i32,
}
impl InsertableFractioningProcedureModel {
    pub fn destination<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable::table(),
                self.destination,
            ),
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn fractioning_procedure_models_destination_id_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable,
        diesel::result::Error,
    >{
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_model_trackables::procedure_model_trackables::dsl::id
                    .eq(&self.destination)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_model_trackables::procedure_model_trackables::dsl::procedure_model_id
                            .eq(&self.id),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable,
            >(conn)
    }
    pub fn id<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel::table(),
                self.id,
            ),
            conn,
        )
    }
    pub fn source<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable::table(),
                self.source,
            ),
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn fractioning_procedure_models_source_id_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable,
        diesel::result::Error,
    >{
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_model_trackables::procedure_model_trackables::dsl::id
                    .eq(&self.source)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_model_trackables::procedure_model_trackables::dsl::procedure_model_id
                            .eq(&self.id),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable,
            >(conn)
    }
    pub fn weighed_with<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable::table(),
                self.weighed_with,
            ),
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn fractioning_procedure_models_weighed_with_id_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable,
        diesel::result::Error,
    >{
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_model_trackables::procedure_model_trackables::dsl::id
                    .eq(&self.weighed_with)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_model_trackables::procedure_model_trackables::dsl::procedure_model_id
                            .eq(&self.id),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable,
            >(conn)
    }
}
#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableFractioningProcedureModelBuilder {
    pub(crate) id: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
    pub(crate) kilograms: Option<f32>,
    pub(crate) tolerance_percentage: Option<f32>,
    pub(crate) weighed_with: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder,
    pub(crate) source: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder,
    pub(crate) destination: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder,
}
impl InsertableFractioningProcedureModelBuilder {
    pub fn kilograms<P>(
        mut self,
        kilograms: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableFractioningProcedureModelAttributes>,
    >
    where
        P: TryInto<f32>,
        <P as TryInto<f32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let kilograms = kilograms.try_into().map_err(|err: <P as TryInto<f32>>::Error| {
            Into::into(err).rename_field(InsertableFractioningProcedureModelAttributes::Kilograms)
        })?;
        pgrx_validation::must_be_strictly_positive_f32(kilograms).map_err(|e| {
            e.rename_field(InsertableFractioningProcedureModelAttributes::Kilograms)
        })?;
        self.kilograms = Some(kilograms);
        Ok(self)
    }
    pub fn tolerance_percentage<P>(
        mut self,
        tolerance_percentage: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableFractioningProcedureModelAttributes>,
    >
    where
        P: TryInto<f32>,
        <P as TryInto<f32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let tolerance_percentage =
            tolerance_percentage.try_into().map_err(|err: <P as TryInto<f32>>::Error| {
                Into::into(err).rename_field(
                    InsertableFractioningProcedureModelAttributes::TolerancePercentage,
                )
            })?;
        pgrx_validation::must_be_strictly_positive_f32(tolerance_percentage).map_err(|e| {
            e.rename_field(InsertableFractioningProcedureModelAttributes::TolerancePercentage)
        })?;
        pgrx_validation::must_be_smaller_than_f32(tolerance_percentage, 100f32).map_err(|e| {
            e.rename_field(InsertableFractioningProcedureModelAttributes::TolerancePercentage)
        })?;
        self.tolerance_percentage = Some(tolerance_percentage);
        Ok(self)
    }
    pub fn source(
        mut self,
        source: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes,
        >,
    >{
        if source.procedure_model_id.is_some() {
            return Err(
                web_common_traits::database::InsertError::BuilderError(
                    web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes::ProcedureModelId,
                    ),
                ),
            );
        }
        self.source = source;
        Ok(self)
    }
    pub fn destination(
        mut self,
        destination: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes,
        >,
    >{
        if destination.procedure_model_id.is_some() {
            return Err(
                web_common_traits::database::InsertError::BuilderError(
                    web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes::ProcedureModelId,
                    ),
                ),
            );
        }
        self.destination = destination;
        Ok(self)
    }
    pub fn weighed_with(
        mut self,
        weighed_with: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes,
        >,
    >{
        if weighed_with.procedure_model_id.is_some() {
            return Err(
                web_common_traits::database::InsertError::BuilderError(
                    web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes::ProcedureModelId,
                    ),
                ),
            );
        }
        self.weighed_with = weighed_with;
        Ok(self)
    }
    pub fn name<P>(
        mut self,
        name: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableFractioningProcedureModelAttributes>,
    >
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.name(name).map_err(|err| err.into_field_name())?;
        Ok(self)
    }
    pub fn description<P>(
        mut self,
        description: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableFractioningProcedureModelAttributes>,
    >
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.description(description).map_err(|err| err.into_field_name())?;
        Ok(self)
    }
    pub fn deprecated<P>(
        mut self,
        deprecated: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableFractioningProcedureModelAttributes>,
    >
    where
        P: TryInto<bool>,
        <P as TryInto<bool>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.deprecated(deprecated).map_err(|err| err.into_field_name())?;
        Ok(self)
    }
    pub fn photograph_id<P>(
        mut self,
        photograph_id: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableFractioningProcedureModelAttributes>,
    >
    where
        P: TryInto<Option<::rosetta_uuid::Uuid>>,
        <P as TryInto<Option<::rosetta_uuid::Uuid>>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.photograph_id(photograph_id).map_err(|err| err.into_field_name())?;
        Ok(self)
    }
    pub fn icon<P>(
        mut self,
        icon: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableFractioningProcedureModelAttributes>,
    >
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.icon(icon).map_err(|err| err.into_field_name())?;
        Ok(self)
    }
    pub fn created_by<P>(
        mut self,
        created_by: P,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableFractioningProcedureModelAttributes>,
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
        web_common_traits::database::InsertError<InsertableFractioningProcedureModelAttributes>,
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
        web_common_traits::database::InsertError<InsertableFractioningProcedureModelAttributes>,
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
        web_common_traits::database::InsertError<InsertableFractioningProcedureModelAttributes>,
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
impl InsertableFractioningProcedureModelBuilder {
    pub(crate) fn try_insert<C>(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<
        InsertableFractioningProcedureModel,
        web_common_traits::database::InsertError<
            InsertableFractioningProcedureModelAttributes,
        >,
    >
    where
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel,
            Error = web_common_traits::database::InsertError<
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelAttributes,
            >,
        >,
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable,
            Error = web_common_traits::database::InsertError<
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes,
            >,
        >,
    {
        use diesel::associations::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let kilograms =
            self.kilograms.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableFractioningProcedureModelAttributes::Kilograms,
            ))?;
        let tolerance_percentage = self.tolerance_percentage.ok_or(
            common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableFractioningProcedureModelAttributes::TolerancePercentage,
            ),
        )?;
        let id = self.id.insert(user_id, conn).map_err(|err| err.into_field_name())?.id();
        let source = self
            .source
            .procedure_model_id(id)
            .map_err(|err| err.into_field_name())?
            .insert(user_id, conn)
            .map_err(|err| err.into_field_name())?
            .id();
        let destination = self
            .destination
            .procedure_model_id(id)
            .map_err(|err| err.into_field_name())?
            .insert(user_id, conn)
            .map_err(|err| err.into_field_name())?
            .id();
        let weighed_with = self
            .weighed_with
            .procedure_model_id(id)
            .map_err(|err| err.into_field_name())?
            .insert(user_id, conn)
            .map_err(|err| err.into_field_name())?
            .id();
        Ok(InsertableFractioningProcedureModel {
            id,
            kilograms,
            tolerance_percentage,
            weighed_with,
            source,
            destination,
        })
    }
}
