#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableProcedureModelTrackableAttributes {
    Id,
    Name,
    ProcedureModelId,
    TrackableId,
    CreatedBy,
    CreatedAt,
    UpdatedBy,
    UpdatedAt,
}
impl core::fmt::Display for InsertableProcedureModelTrackableAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Id => write!(f, "id"),
            Self::Name => write!(f, "name"),
            Self::ProcedureModelId => write!(f, "procedure_model_id"),
            Self::TrackableId => write!(f, "trackable_id"),
            Self::CreatedBy => write!(f, "created_by"),
            Self::CreatedAt => write!(f, "created_at"),
            Self::UpdatedBy => write!(f, "updated_by"),
            Self::UpdatedAt => write!(f, "updated_at"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::procedure_model_trackables::procedure_model_trackables
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableProcedureModelTrackable {
    pub(crate) name: String,
    pub(crate) procedure_model_id: i32,
    pub(crate) trackable_id: ::rosetta_uuid::Uuid,
    pub(crate) created_by: i32,
    pub(crate) created_at: ::rosetta_timestamp::TimestampUTC,
    pub(crate) updated_by: i32,
    pub(crate) updated_at: ::rosetta_timestamp::TimestampUTC,
}
impl InsertableProcedureModelTrackable {
    pub fn procedure_model<C: diesel::connection::LoadConnection>(
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
                self.procedure_model_id,
            ),
            conn,
        )
    }
    pub fn trackable<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::trackables::Trackable,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::trackables::Trackable: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::trackables::Trackable as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::trackables::Trackable as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::trackables::Trackable as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::trackables::Trackable as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::trackables::Trackable as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::trackables::Trackable as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::trackables::Trackable,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::trackables::Trackable::table(),
                self.trackable_id,
            ),
            conn,
        )
    }
    pub fn created_by<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::users::User,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::users::User: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::users::User as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::users::User as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::users::User as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::users::User as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::users::User as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::users::User as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::users::User,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::users::User::table(),
                self.created_by,
            ),
            conn,
        )
    }
    pub fn updated_by<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::users::User,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::users::User: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::users::User as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::users::User as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::users::User as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::users::User as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::users::User as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::users::User as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::users::User,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::users::User::table(),
                self.updated_by,
            ),
            conn,
        )
    }
}
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableProcedureModelTrackableBuilder {
    pub(crate) name: Option<String>,
    pub(crate) procedure_model_id: Option<i32>,
    pub(crate) trackable_id: Option<::rosetta_uuid::Uuid>,
    pub(crate) created_by: Option<i32>,
    pub(crate) created_at: Option<::rosetta_timestamp::TimestampUTC>,
    pub(crate) updated_by: Option<i32>,
    pub(crate) updated_at: Option<::rosetta_timestamp::TimestampUTC>,
}
impl Default for InsertableProcedureModelTrackableBuilder {
    fn default() -> Self {
        Self {
            name: Default::default(),
            procedure_model_id: Default::default(),
            trackable_id: Default::default(),
            created_by: Default::default(),
            created_at: Some(rosetta_timestamp::TimestampUTC::default()),
            updated_by: Default::default(),
            updated_at: Some(rosetta_timestamp::TimestampUTC::default()),
        }
    }
}
impl web_common_traits::database::ExtendableBuilder for InsertableProcedureModelTrackableBuilder {
    type Attributes = InsertableProcedureModelTrackableAttributes;
    fn extend_builder(
        mut self,
        other: Self,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        match (other.created_at, other.updated_at) {
            (Some(created_at), Some(updated_at)) => {
                self = self.created_at_and_updated_at(created_at, updated_at)?;
            }
            (None, Some(updated_at)) => {
                self = self.updated_at(updated_at)?;
            }
            (Some(created_at), None) => {
                self = self.created_at(created_at)?;
            }
            (None, None) => {}
        }
        if let Some(name) = other.name {
            self = self.name(name)?;
        }
        if let Some(procedure_model_id) = other.procedure_model_id {
            self = self.procedure_model(procedure_model_id)?;
        }
        if let Some(trackable_id) = other.trackable_id {
            self = self.trackable(trackable_id)?;
        }
        if let Some(created_by) = other.created_by {
            self = self.created_by(created_by)?;
        }
        if let Some(updated_by) = other.updated_by {
            self = self.updated_by(updated_by)?;
        }
        Ok(self)
    }
}
impl web_common_traits::prelude::SetPrimaryKey for InsertableProcedureModelTrackableBuilder {
    type PrimaryKey = i32;
    fn set_primary_key(self, _primary_key: Self::PrimaryKey) -> Self {
        self
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder
{
    /// Sets the value of the `procedure_model_trackables.created_at` column
    /// from table `procedure_model_trackables`.
    pub fn created_at<CreatedAt>(
        mut self,
        created_at: CreatedAt,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableProcedureModelTrackableAttributes>,
    >
    where
        CreatedAt: TryInto<::rosetta_timestamp::TimestampUTC>,
        <CreatedAt as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let created_at = created_at.try_into().map_err(
            |err: <CreatedAt as TryInto<::rosetta_timestamp::TimestampUTC>>::Error| {
                Into::into(err).rename_field(InsertableProcedureModelTrackableAttributes::CreatedAt)
            },
        )?;
        if let Some(updated_at) = self.updated_at {
            pgrx_validation::must_be_smaller_than_utc(created_at, updated_at)
                .map_err(|e| {
                    e
                        .rename_fields(
                            crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes::CreatedAt,
                            crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes::UpdatedAt,
                        )
                })?;
        }
        self.created_at = Some(created_at);
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder
{
    /// Sets the value of the `procedure_model_trackables.created_at`,
    /// `procedure_model_trackables.updated_at` columns from table
    /// `procedure_model_trackables`.
    pub fn created_at_and_updated_at<CreatedAt, UpdatedAt>(
        mut self,
        created_at: CreatedAt,
        updated_at: UpdatedAt,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableProcedureModelTrackableAttributes>,
    >
    where
        CreatedAt: TryInto<::rosetta_timestamp::TimestampUTC>,
        <CreatedAt as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
        UpdatedAt: TryInto<::rosetta_timestamp::TimestampUTC>,
        <UpdatedAt as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let created_at = created_at.try_into().map_err(
            |err: <CreatedAt as TryInto<::rosetta_timestamp::TimestampUTC>>::Error| {
                Into::into(err).rename_field(InsertableProcedureModelTrackableAttributes::CreatedAt)
            },
        )?;
        let updated_at = updated_at.try_into().map_err(
            |err: <UpdatedAt as TryInto<::rosetta_timestamp::TimestampUTC>>::Error| {
                Into::into(err).rename_field(InsertableProcedureModelTrackableAttributes::UpdatedAt)
            },
        )?;
        pgrx_validation::must_be_smaller_than_utc(created_at, updated_at)
            .map_err(|e| {
                e
                    .rename_fields(
                        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes::CreatedAt,
                        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes::UpdatedAt,
                    )
            })?;
        self.created_at = Some(created_at);
        self.updated_at = Some(updated_at);
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder
{
    /// Sets the value of the `procedure_model_trackables.created_by` column
    /// from table `procedure_model_trackables`.
    pub fn created_by(
        mut self,
        created_by: i32,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableProcedureModelTrackableAttributes>,
    > {
        self.created_by = Some(created_by);
        self = self.updated_by(created_by)?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder
{
    /// Sets the value of the `procedure_model_trackables.name` column from
    /// table `procedure_model_trackables`.
    pub fn name<Name>(
        mut self,
        name: Name,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableProcedureModelTrackableAttributes>,
    >
    where
        Name: TryInto<String>,
        <Name as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let name = name.try_into().map_err(|err: <Name as TryInto<String>>::Error| {
            Into::into(err).rename_field(InsertableProcedureModelTrackableAttributes::Name)
        })?;
        pgrx_validation::must_be_paragraph(name.as_ref())
            .map_err(|e| {
                e
                    .rename_field(
                        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes::Name,
                    )
            })?;
        self.name = Some(name);
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder
{
    /// Sets the value of the `procedure_model_trackables.procedure_model_id`
    /// column from table `procedure_model_trackables`.
    pub fn procedure_model(
        mut self,
        procedure_model_id: i32,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableProcedureModelTrackableAttributes>,
    > {
        self.procedure_model_id = Some(procedure_model_id);
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder
{
    /// Sets the value of the `procedure_model_trackables.trackable_id` column
    /// from table `procedure_model_trackables`.
    pub fn trackable(
        mut self,
        trackable_id: ::rosetta_uuid::Uuid,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableProcedureModelTrackableAttributes>,
    > {
        self.trackable_id = Some(trackable_id);
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder
{
    /// Sets the value of the `procedure_model_trackables.updated_at` column
    /// from table `procedure_model_trackables`.
    pub fn updated_at<UpdatedAt>(
        mut self,
        updated_at: UpdatedAt,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableProcedureModelTrackableAttributes>,
    >
    where
        UpdatedAt: TryInto<::rosetta_timestamp::TimestampUTC>,
        <UpdatedAt as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let updated_at = updated_at.try_into().map_err(
            |err: <UpdatedAt as TryInto<::rosetta_timestamp::TimestampUTC>>::Error| {
                Into::into(err).rename_field(InsertableProcedureModelTrackableAttributes::UpdatedAt)
            },
        )?;
        if let Some(created_at) = self.created_at {
            pgrx_validation::must_be_smaller_than_utc(created_at, updated_at)
                .map_err(|e| {
                    e
                        .rename_fields(
                            crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes::CreatedAt,
                            crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes::UpdatedAt,
                        )
                })?;
        }
        self.updated_at = Some(updated_at);
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder
{
    /// Sets the value of the `procedure_model_trackables.updated_by` column
    /// from table `procedure_model_trackables`.
    pub fn updated_by(
        mut self,
        updated_by: i32,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableProcedureModelTrackableAttributes>,
    > {
        self.updated_by = Some(updated_by);
        Ok(self)
    }
}
impl<C> web_common_traits::database::TryInsertGeneric<C>
for InsertableProcedureModelTrackableBuilder
where
    Self: web_common_traits::database::InsertableVariant<
        C,
        UserId = i32,
        Row = crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable,
        Error = web_common_traits::database::InsertError<
            InsertableProcedureModelTrackableAttributes,
        >,
    >,
{
    type Attributes = InsertableProcedureModelTrackableAttributes;
    fn is_complete(&self) -> bool {
        self.name.is_some() && self.procedure_model_id.is_some()
            && self.trackable_id.is_some() && self.created_by.is_some()
            && self.created_at.is_some() && self.updated_by.is_some()
            && self.updated_at.is_some()
    }
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<
        Self::PrimaryKey,
        web_common_traits::database::InsertError<Self::Attributes>,
    > {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable = self
            .insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
