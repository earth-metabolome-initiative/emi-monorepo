#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableSharedProcedureModelTrackableAttributes {
    ParentId,
    ParentTrackableId,
    ParentProcedureModelId,
    ChildId,
    ChildTrackableId,
    ChildProcedureModelId,
    CreatedBy,
    CreatedAt,
}
impl core::str::FromStr for InsertableSharedProcedureModelTrackableAttributes {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ParentId" => Ok(Self::ParentId),
            "ParentTrackableId" => Ok(Self::ParentTrackableId),
            "ParentProcedureModelId" => Ok(Self::ParentProcedureModelId),
            "ChildId" => Ok(Self::ChildId),
            "ChildTrackableId" => Ok(Self::ChildTrackableId),
            "ChildProcedureModelId" => Ok(Self::ChildProcedureModelId),
            "CreatedBy" => Ok(Self::CreatedBy),
            "CreatedAt" => Ok(Self::CreatedAt),
            "parent_id" => Ok(Self::ParentId),
            "parent_trackable_id" => Ok(Self::ParentTrackableId),
            "parent_procedure_model_id" => Ok(Self::ParentProcedureModelId),
            "child_id" => Ok(Self::ChildId),
            "child_trackable_id" => Ok(Self::ChildTrackableId),
            "child_procedure_model_id" => Ok(Self::ChildProcedureModelId),
            "created_by" => Ok(Self::CreatedBy),
            "created_at" => Ok(Self::CreatedAt),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl core::fmt::Display for InsertableSharedProcedureModelTrackableAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::ParentId => write!(f, "parent_id"),
            Self::ParentTrackableId => write!(f, "parent_trackable_id"),
            Self::ParentProcedureModelId => write!(f, "parent_procedure_model_id"),
            Self::ChildId => write!(f, "child_id"),
            Self::ChildTrackableId => write!(f, "child_trackable_id"),
            Self::ChildProcedureModelId => write!(f, "child_procedure_model_id"),
            Self::CreatedBy => write!(f, "created_by"),
            Self::CreatedAt => write!(f, "created_at"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::shared_procedure_model_trackables::shared_procedure_model_trackables
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableSharedProcedureModelTrackable {
    pub(crate) parent_id: i32,
    pub(crate) parent_trackable_id: ::rosetta_uuid::Uuid,
    pub(crate) parent_procedure_model_id: i32,
    pub(crate) child_id: i32,
    pub(crate) child_trackable_id: ::rosetta_uuid::Uuid,
    pub(crate) child_procedure_model_id: i32,
    pub(crate) created_by: i32,
    pub(crate) created_at: ::rosetta_timestamp::TimestampUTC,
}
impl InsertableSharedProcedureModelTrackable {
    pub fn parent<C: diesel::connection::LoadConnection>(
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
                self.parent_id,
            ),
            conn,
        )
    }
    pub fn parent_trackable<C: diesel::connection::LoadConnection>(
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
                self.parent_trackable_id,
            ),
            conn,
        )
    }
    pub fn parent_procedure_model<C: diesel::connection::LoadConnection>(
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
                self.parent_procedure_model_id,
            ),
            conn,
        )
    }
    pub fn child<C: diesel::connection::LoadConnection>(
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
                self.child_id,
            ),
            conn,
        )
    }
    pub fn child_trackable<C: diesel::connection::LoadConnection>(
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
                self.child_trackable_id,
            ),
            conn,
        )
    }
    pub fn child_procedure_model<C: diesel::connection::LoadConnection>(
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
                self.child_procedure_model_id,
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
    pub fn shared_procedure_model_tracka_parent_procedure_model_id_ch_fkey<
        C: diesel::connection::LoadConnection,
    >(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::parent_procedure_models::ParentProcedureModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::parent_procedure_models::ParentProcedureModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::parent_procedure_models::ParentProcedureModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::parent_procedure_models::ParentProcedureModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::parent_procedure_models::ParentProcedureModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::parent_procedure_models::ParentProcedureModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::parent_procedure_models::ParentProcedureModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::parent_procedure_models::ParentProcedureModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::parent_procedure_models::ParentProcedureModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::parent_procedure_models::ParentProcedureModel::table(),
                (self.parent_procedure_model_id, self.child_procedure_model_id),
            ),
            conn,
        )
    }
}
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableSharedProcedureModelTrackableBuilder {
    pub(crate) parent_id: Option<i32>,
    pub(crate) parent_trackable_id: Option<::rosetta_uuid::Uuid>,
    pub(crate) parent_procedure_model_id: Option<i32>,
    pub(crate) child_id: Option<i32>,
    pub(crate) child_trackable_id: Option<::rosetta_uuid::Uuid>,
    pub(crate) child_procedure_model_id: Option<i32>,
    pub(crate) created_by: Option<i32>,
    pub(crate) created_at: Option<::rosetta_timestamp::TimestampUTC>,
}
impl Default for InsertableSharedProcedureModelTrackableBuilder {
    fn default() -> Self {
        Self {
            parent_id: Default::default(),
            parent_trackable_id: Default::default(),
            parent_procedure_model_id: Default::default(),
            child_id: Default::default(),
            child_trackable_id: Default::default(),
            child_procedure_model_id: Default::default(),
            created_by: Default::default(),
            created_at: Some(rosetta_timestamp::TimestampUTC::default()),
        }
    }
}
impl web_common_traits::database::ExtendableBuilder
    for InsertableSharedProcedureModelTrackableBuilder
{
    type Attributes = InsertableSharedProcedureModelTrackableAttributes;
    fn extend_builder(
        mut self,
        other: Self,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        match (other.parent_id, other.child_id) {
            (Some(parent_id), Some(child_id)) => {
                self = self.parent_and_child(parent_id, child_id)?;
            }
            (None, Some(child_id)) => {
                self = self.child(child_id)?;
            }
            (Some(parent_id), None) => {
                self = self.parent(parent_id)?;
            }
            (None, None) => {}
        }
        if let Some(parent_trackable_id) = other.parent_trackable_id {
            self = self.parent_trackable(parent_trackable_id)?;
        }
        if let Some(parent_procedure_model_id) = other.parent_procedure_model_id {
            self = self.parent_procedure_model(parent_procedure_model_id)?;
        }
        if let Some(child_trackable_id) = other.child_trackable_id {
            self = self.child_trackable(child_trackable_id)?;
        }
        if let Some(child_procedure_model_id) = other.child_procedure_model_id {
            self = self.child_procedure_model(child_procedure_model_id)?;
        }
        if let Some(created_by) = other.created_by {
            self = self.created_by(created_by)?;
        }
        if let Some(created_at) = other.created_at {
            self = self.created_at(created_at)?;
        }
        Ok(self)
    }
}
impl web_common_traits::prelude::SetPrimaryKey for InsertableSharedProcedureModelTrackableBuilder {
    type PrimaryKey = (i32, i32);
    fn set_primary_key(self, _primary_key: Self::PrimaryKey) -> Self {
        self
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableSharedProcedureModelTrackableBuilder {
    ///Sets the value of the `shared_procedure_model_trackables.child_id` column from table `shared_procedure_model_trackables`.
    pub fn child(
        mut self,
        child_id: i32,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<
            InsertableSharedProcedureModelTrackableAttributes,
        >,
    > {
        if let Some(parent_id) = self.parent_id {
            pgrx_validation::must_be_distinct_i32(parent_id, child_id)
                .map_err(|e| {
                    e
                        .rename_fields(
                            crate::codegen::structs_codegen::tables::insertables::InsertableSharedProcedureModelTrackableAttributes::ParentId,
                            crate::codegen::structs_codegen::tables::insertables::InsertableSharedProcedureModelTrackableAttributes::ChildId,
                        )
                })?;
        }
        self.child_id = Some(child_id);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableSharedProcedureModelTrackableBuilder {
    ///Sets the value of the `shared_procedure_model_trackables.child_procedure_model_id` column from table `shared_procedure_model_trackables`.
    pub fn child_procedure_model(
        mut self,
        child_procedure_model_id: i32,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<
            InsertableSharedProcedureModelTrackableAttributes,
        >,
    > {
        self.child_procedure_model_id = Some(child_procedure_model_id);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableSharedProcedureModelTrackableBuilder {
    ///Sets the value of the `shared_procedure_model_trackables.child_trackable_id` column from table `shared_procedure_model_trackables`.
    pub fn child_trackable(
        mut self,
        child_trackable_id: ::rosetta_uuid::Uuid,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<
            InsertableSharedProcedureModelTrackableAttributes,
        >,
    > {
        self.child_trackable_id = Some(child_trackable_id);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableSharedProcedureModelTrackableBuilder {
    ///Sets the value of the `shared_procedure_model_trackables.created_at` column from table `shared_procedure_model_trackables`.
    pub fn created_at<CreatedAt>(
        mut self,
        created_at: CreatedAt,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<
            InsertableSharedProcedureModelTrackableAttributes,
        >,
    >
    where
        CreatedAt: TryInto<::rosetta_timestamp::TimestampUTC>,
        <CreatedAt as TryInto<
            ::rosetta_timestamp::TimestampUTC,
        >>::Error: Into<validation_errors::SingleFieldError>,
    {
        let created_at = created_at
            .try_into()
            .map_err(|
                err: <CreatedAt as TryInto<::rosetta_timestamp::TimestampUTC>>::Error|
            {
                Into::into(err)
                    .rename_field(
                        InsertableSharedProcedureModelTrackableAttributes::CreatedAt,
                    )
            })?;
        self.created_at = Some(created_at);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableSharedProcedureModelTrackableBuilder {
    ///Sets the value of the `shared_procedure_model_trackables.created_by` column from table `shared_procedure_model_trackables`.
    pub fn created_by(
        mut self,
        created_by: i32,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<
            InsertableSharedProcedureModelTrackableAttributes,
        >,
    > {
        self.created_by = Some(created_by);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableSharedProcedureModelTrackableBuilder {
    ///Sets the value of the `shared_procedure_model_trackables.parent_id` column from table `shared_procedure_model_trackables`.
    pub fn parent(
        mut self,
        parent_id: i32,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<
            InsertableSharedProcedureModelTrackableAttributes,
        >,
    > {
        if let Some(child_id) = self.child_id {
            pgrx_validation::must_be_distinct_i32(parent_id, child_id)
                .map_err(|e| {
                    e
                        .rename_fields(
                            crate::codegen::structs_codegen::tables::insertables::InsertableSharedProcedureModelTrackableAttributes::ParentId,
                            crate::codegen::structs_codegen::tables::insertables::InsertableSharedProcedureModelTrackableAttributes::ChildId,
                        )
                })?;
        }
        self.parent_id = Some(parent_id);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableSharedProcedureModelTrackableBuilder {
    ///Sets the value of the `shared_procedure_model_trackables.parent_id`, `shared_procedure_model_trackables.child_id` columns from table `shared_procedure_model_trackables`.
    pub fn parent_and_child(
        mut self,
        parent_id: i32,
        child_id: i32,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<
            InsertableSharedProcedureModelTrackableAttributes,
        >,
    > {
        pgrx_validation::must_be_distinct_i32(parent_id, child_id)
            .map_err(|e| {
                e
                    .rename_fields(
                        crate::codegen::structs_codegen::tables::insertables::InsertableSharedProcedureModelTrackableAttributes::ParentId,
                        crate::codegen::structs_codegen::tables::insertables::InsertableSharedProcedureModelTrackableAttributes::ChildId,
                    )
            })?;
        self.parent_id = Some(parent_id);
        self.child_id = Some(child_id);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableSharedProcedureModelTrackableBuilder {
    ///Sets the value of the `shared_procedure_model_trackables.parent_procedure_model_id` column from table `shared_procedure_model_trackables`.
    pub fn parent_procedure_model(
        mut self,
        parent_procedure_model_id: i32,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<
            InsertableSharedProcedureModelTrackableAttributes,
        >,
    > {
        self.parent_procedure_model_id = Some(parent_procedure_model_id);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableSharedProcedureModelTrackableBuilder {
    ///Sets the value of the `shared_procedure_model_trackables.parent_trackable_id` column from table `shared_procedure_model_trackables`.
    pub fn parent_trackable(
        mut self,
        parent_trackable_id: ::rosetta_uuid::Uuid,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<
            InsertableSharedProcedureModelTrackableAttributes,
        >,
    > {
        self.parent_trackable_id = Some(parent_trackable_id);
        Ok(self)
    }
}
impl<C> web_common_traits::database::TryInsertGeneric<C>
for InsertableSharedProcedureModelTrackableBuilder
where
    Self: web_common_traits::database::InsertableVariant<
        C,
        UserId = i32,
        Row = crate::codegen::structs_codegen::tables::shared_procedure_model_trackables::SharedProcedureModelTrackable,
        Error = web_common_traits::database::InsertError<
            InsertableSharedProcedureModelTrackableAttributes,
        >,
    >,
{
    type Attributes = InsertableSharedProcedureModelTrackableAttributes;
    fn is_complete(&self) -> bool {
        self.parent_id.is_some() && self.parent_trackable_id.is_some()
            && self.parent_procedure_model_id.is_some() && self.child_id.is_some()
            && self.child_trackable_id.is_some()
            && self.child_procedure_model_id.is_some() && self.created_by.is_some()
            && self.created_at.is_some()
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
        let insertable: crate::codegen::structs_codegen::tables::shared_procedure_model_trackables::SharedProcedureModelTrackable = self
            .insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
