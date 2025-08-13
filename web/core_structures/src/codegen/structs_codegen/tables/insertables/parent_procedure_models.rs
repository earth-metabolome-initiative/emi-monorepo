#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableParentProcedureModelAttributes {
    ParentProcedureModelId,
    ChildProcedureModelId,
    Snoozable,
    Copiable,
    Repeatable,
    Skippable,
    CreatedBy,
    CreatedAt,
}
impl core::str::FromStr for InsertableParentProcedureModelAttributes {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ParentProcedureModelId" => Ok(Self::ParentProcedureModelId),
            "ChildProcedureModelId" => Ok(Self::ChildProcedureModelId),
            "Snoozable" => Ok(Self::Snoozable),
            "Copiable" => Ok(Self::Copiable),
            "Repeatable" => Ok(Self::Repeatable),
            "Skippable" => Ok(Self::Skippable),
            "CreatedBy" => Ok(Self::CreatedBy),
            "CreatedAt" => Ok(Self::CreatedAt),
            "parent_procedure_model_id" => Ok(Self::ParentProcedureModelId),
            "child_procedure_model_id" => Ok(Self::ChildProcedureModelId),
            "snoozable" => Ok(Self::Snoozable),
            "copiable" => Ok(Self::Copiable),
            "repeatable" => Ok(Self::Repeatable),
            "skippable" => Ok(Self::Skippable),
            "created_by" => Ok(Self::CreatedBy),
            "created_at" => Ok(Self::CreatedAt),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl core::fmt::Display for InsertableParentProcedureModelAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::ParentProcedureModelId => write!(f, "parent_procedure_model_id"),
            Self::ChildProcedureModelId => write!(f, "child_procedure_model_id"),
            Self::Snoozable => write!(f, "snoozable"),
            Self::Copiable => write!(f, "copiable"),
            Self::Repeatable => write!(f, "repeatable"),
            Self::Skippable => write!(f, "skippable"),
            Self::CreatedBy => write!(f, "created_by"),
            Self::CreatedAt => write!(f, "created_at"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::parent_procedure_models::parent_procedure_models
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableParentProcedureModel {
    pub(crate) parent_procedure_model_id: i32,
    pub(crate) child_procedure_model_id: i32,
    pub(crate) snoozable: bool,
    pub(crate) copiable: bool,
    pub(crate) repeatable: bool,
    pub(crate) skippable: bool,
    pub(crate) created_by: i32,
    pub(crate) created_at: ::rosetta_timestamp::TimestampUTC,
}
impl InsertableParentProcedureModel {
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
}
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableParentProcedureModelBuilder {
    pub(crate) parent_procedure_model_id: Option<i32>,
    pub(crate) child_procedure_model_id: Option<i32>,
    pub(crate) snoozable: Option<bool>,
    pub(crate) copiable: Option<bool>,
    pub(crate) repeatable: Option<bool>,
    pub(crate) skippable: Option<bool>,
    pub(crate) created_by: Option<i32>,
    pub(crate) created_at: Option<::rosetta_timestamp::TimestampUTC>,
}
impl Default for InsertableParentProcedureModelBuilder {
    fn default() -> Self {
        Self {
            parent_procedure_model_id: Default::default(),
            child_procedure_model_id: Default::default(),
            snoozable: Some(false),
            copiable: Some(false),
            repeatable: Some(false),
            skippable: Some(false),
            created_by: Default::default(),
            created_at: Some(rosetta_timestamp::TimestampUTC::default()),
        }
    }
}
impl web_common_traits::database::ExtendableBuilder for InsertableParentProcedureModelBuilder {
    type Attributes = InsertableParentProcedureModelAttributes;
    fn extend_builder(
        mut self,
        other: Self,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        match (other.parent_procedure_model_id, other.child_procedure_model_id) {
            (Some(parent_procedure_model_id), Some(child_procedure_model_id)) => {
                self = self.parent_procedure_model_and_child_procedure_model(
                    parent_procedure_model_id,
                    child_procedure_model_id,
                )?;
            }
            (None, Some(child_procedure_model_id)) => {
                self = self.child_procedure_model(child_procedure_model_id)?;
            }
            (Some(parent_procedure_model_id), None) => {
                self = self.parent_procedure_model(parent_procedure_model_id)?;
            }
            (None, None) => {}
        }
        if let Some(snoozable) = other.snoozable {
            self = self.snoozable(snoozable)?;
        }
        if let Some(copiable) = other.copiable {
            self = self.copiable(copiable)?;
        }
        if let Some(repeatable) = other.repeatable {
            self = self.repeatable(repeatable)?;
        }
        if let Some(skippable) = other.skippable {
            self = self.skippable(skippable)?;
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
impl web_common_traits::prelude::SetPrimaryKey for InsertableParentProcedureModelBuilder {
    type PrimaryKey = (i32, i32);
    fn set_primary_key(self, _primary_key: Self::PrimaryKey) -> Self {
        self
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableParentProcedureModelBuilder {
    /// Sets the value of the `parent_procedure_models.child_procedure_model_id`
    /// column from table `parent_procedure_models`.
    pub fn child_procedure_model(
        mut self,
        child_procedure_model_id: i32,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableParentProcedureModelAttributes>,
    > {
        if let Some(parent_procedure_model_id) = self.parent_procedure_model_id {
            pgrx_validation::must_be_distinct_i32(
                    parent_procedure_model_id,
                    child_procedure_model_id,
                )
                .map_err(|e| {
                    e
                        .rename_fields(
                            crate::codegen::structs_codegen::tables::insertables::InsertableParentProcedureModelAttributes::ParentProcedureModelId,
                            crate::codegen::structs_codegen::tables::insertables::InsertableParentProcedureModelAttributes::ChildProcedureModelId,
                        )
                })?;
        }
        self.child_procedure_model_id = Some(child_procedure_model_id);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableParentProcedureModelBuilder {
    /// Sets the value of the `parent_procedure_models.copiable` column from
    /// table `parent_procedure_models`.
    pub fn copiable<Copiable>(
        mut self,
        copiable: Copiable,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableParentProcedureModelAttributes>,
    >
    where
        Copiable: TryInto<bool>,
        <Copiable as TryInto<bool>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let copiable = copiable.try_into().map_err(|err: <Copiable as TryInto<bool>>::Error| {
            Into::into(err).rename_field(InsertableParentProcedureModelAttributes::Copiable)
        })?;
        self.copiable = Some(copiable);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableParentProcedureModelBuilder {
    /// Sets the value of the `parent_procedure_models.created_at` column from
    /// table `parent_procedure_models`.
    pub fn created_at<CreatedAt>(
        mut self,
        created_at: CreatedAt,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableParentProcedureModelAttributes>,
    >
    where
        CreatedAt: TryInto<::rosetta_timestamp::TimestampUTC>,
        <CreatedAt as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let created_at = created_at.try_into().map_err(
            |err: <CreatedAt as TryInto<::rosetta_timestamp::TimestampUTC>>::Error| {
                Into::into(err).rename_field(InsertableParentProcedureModelAttributes::CreatedAt)
            },
        )?;
        self.created_at = Some(created_at);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableParentProcedureModelBuilder {
    /// Sets the value of the `parent_procedure_models.created_by` column from
    /// table `parent_procedure_models`.
    pub fn created_by(
        mut self,
        created_by: i32,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableParentProcedureModelAttributes>,
    > {
        self.created_by = Some(created_by);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableParentProcedureModelBuilder {
    /// Sets the value of the
    /// `parent_procedure_models.parent_procedure_model_id` column from table
    /// `parent_procedure_models`.
    pub fn parent_procedure_model(
        mut self,
        parent_procedure_model_id: i32,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableParentProcedureModelAttributes>,
    > {
        if let Some(child_procedure_model_id) = self.child_procedure_model_id {
            pgrx_validation::must_be_distinct_i32(
                    parent_procedure_model_id,
                    child_procedure_model_id,
                )
                .map_err(|e| {
                    e
                        .rename_fields(
                            crate::codegen::structs_codegen::tables::insertables::InsertableParentProcedureModelAttributes::ParentProcedureModelId,
                            crate::codegen::structs_codegen::tables::insertables::InsertableParentProcedureModelAttributes::ChildProcedureModelId,
                        )
                })?;
        }
        self.parent_procedure_model_id = Some(parent_procedure_model_id);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableParentProcedureModelBuilder {
    /// Sets the value of the
    /// `parent_procedure_models.parent_procedure_model_id`,
    /// `parent_procedure_models.child_procedure_model_id` columns from table
    /// `parent_procedure_models`.
    pub fn parent_procedure_model_and_child_procedure_model(
        mut self,
        parent_procedure_model_id: i32,
        child_procedure_model_id: i32,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableParentProcedureModelAttributes>,
    > {
        pgrx_validation::must_be_distinct_i32(
                parent_procedure_model_id,
                child_procedure_model_id,
            )
            .map_err(|e| {
                e
                    .rename_fields(
                        crate::codegen::structs_codegen::tables::insertables::InsertableParentProcedureModelAttributes::ParentProcedureModelId,
                        crate::codegen::structs_codegen::tables::insertables::InsertableParentProcedureModelAttributes::ChildProcedureModelId,
                    )
            })?;
        self.parent_procedure_model_id = Some(parent_procedure_model_id);
        self.child_procedure_model_id = Some(child_procedure_model_id);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableParentProcedureModelBuilder {
    /// Sets the value of the `parent_procedure_models.repeatable` column from
    /// table `parent_procedure_models`.
    pub fn repeatable<Repeatable>(
        mut self,
        repeatable: Repeatable,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableParentProcedureModelAttributes>,
    >
    where
        Repeatable: TryInto<bool>,
        <Repeatable as TryInto<bool>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let repeatable =
            repeatable.try_into().map_err(|err: <Repeatable as TryInto<bool>>::Error| {
                Into::into(err).rename_field(InsertableParentProcedureModelAttributes::Repeatable)
            })?;
        self.repeatable = Some(repeatable);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableParentProcedureModelBuilder {
    /// Sets the value of the `parent_procedure_models.skippable` column from
    /// table `parent_procedure_models`.
    pub fn skippable<Skippable>(
        mut self,
        skippable: Skippable,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableParentProcedureModelAttributes>,
    >
    where
        Skippable: TryInto<bool>,
        <Skippable as TryInto<bool>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let skippable =
            skippable.try_into().map_err(|err: <Skippable as TryInto<bool>>::Error| {
                Into::into(err).rename_field(InsertableParentProcedureModelAttributes::Skippable)
            })?;
        self.skippable = Some(skippable);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableParentProcedureModelBuilder {
    /// Sets the value of the `parent_procedure_models.snoozable` column from
    /// table `parent_procedure_models`.
    pub fn snoozable<Snoozable>(
        mut self,
        snoozable: Snoozable,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableParentProcedureModelAttributes>,
    >
    where
        Snoozable: TryInto<bool>,
        <Snoozable as TryInto<bool>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let snoozable =
            snoozable.try_into().map_err(|err: <Snoozable as TryInto<bool>>::Error| {
                Into::into(err).rename_field(InsertableParentProcedureModelAttributes::Snoozable)
            })?;
        self.snoozable = Some(snoozable);
        Ok(self)
    }
}
impl<C> web_common_traits::database::TryInsertGeneric<C>
for InsertableParentProcedureModelBuilder
where
    Self: web_common_traits::database::InsertableVariant<
        C,
        UserId = i32,
        Row = crate::codegen::structs_codegen::tables::parent_procedure_models::ParentProcedureModel,
        Error = web_common_traits::database::InsertError<
            InsertableParentProcedureModelAttributes,
        >,
    >,
{
    type Attributes = InsertableParentProcedureModelAttributes;
    fn is_complete(&self) -> bool {
        self.parent_procedure_model_id.is_some()
            && self.child_procedure_model_id.is_some() && self.snoozable.is_some()
            && self.copiable.is_some() && self.repeatable.is_some()
            && self.skippable.is_some() && self.created_by.is_some()
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
        let insertable: crate::codegen::structs_codegen::tables::parent_procedure_models::ParentProcedureModel = self
            .insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
