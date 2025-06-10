#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableProcedureModelAttributes {
    Name,
    Description,
    Deprecated,
    PhotographId,
    Icon,
    CreatedBy,
    CreatedAt,
    UpdatedBy,
    UpdatedAt,
}
impl core::fmt::Display for InsertableProcedureModelAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableProcedureModelAttributes::Name => write!(f, "name"),
            InsertableProcedureModelAttributes::Description => write!(f, "description"),
            InsertableProcedureModelAttributes::Deprecated => write!(f, "deprecated"),
            InsertableProcedureModelAttributes::PhotographId => {
                write!(f, "photograph_id")
            }
            InsertableProcedureModelAttributes::Icon => write!(f, "icon"),
            InsertableProcedureModelAttributes::CreatedBy => write!(f, "created_by"),
            InsertableProcedureModelAttributes::CreatedAt => write!(f, "created_at"),
            InsertableProcedureModelAttributes::UpdatedBy => write!(f, "updated_by"),
            InsertableProcedureModelAttributes::UpdatedAt => write!(f, "updated_at"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::procedure_models::procedure_models
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableProcedureModel {
    name: String,
    description: String,
    deprecated: bool,
    photograph_id: Option<::rosetta_uuid::Uuid>,
    icon: String,
    created_by: i32,
    created_at: ::rosetta_timestamp::TimestampUTC,
    updated_by: i32,
    updated_at: ::rosetta_timestamp::TimestampUTC,
}
impl InsertableProcedureModel {
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
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, RunQueryDsl};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::users::User::table(),
                self.created_by,
            ),
            conn,
        )
    }
    pub fn photograph<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::documents::Document>,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::documents::Document: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::documents::Document as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::documents::Document as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::documents::Document as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::documents::Document as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::documents::Document as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::documents::Document as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::documents::Document,
        >,
    {
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, RunQueryDsl};
        let Some(photograph_id) = self.photograph_id else {
            return Ok(None);
        };
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::documents::Document::table(),
                photograph_id,
            ),
            conn,
        )
        .map(Some)
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
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, RunQueryDsl};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::users::User::table(),
                self.updated_by,
            ),
            conn,
        )
    }
}
pub struct InsertableProcedureModelBuilder {
    pub(crate) name: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) deprecated: Option<bool>,
    pub(crate) photograph_id: Option<::rosetta_uuid::Uuid>,
    pub(crate) icon: Option<String>,
    pub(crate) created_by: Option<i32>,
    pub(crate) created_at: Option<::rosetta_timestamp::TimestampUTC>,
    pub(crate) updated_by: Option<i32>,
    pub(crate) updated_at: Option<::rosetta_timestamp::TimestampUTC>,
}
impl Default for InsertableProcedureModelBuilder {
    fn default() -> Self {
        Self {
            name: Default::default(),
            description: Default::default(),
            deprecated: Some(false),
            photograph_id: Default::default(),
            icon: Some("book".to_owned()),
            created_by: Default::default(),
            created_at: Some(rosetta_timestamp::TimestampUTC::default()),
            updated_by: Default::default(),
            updated_at: Some(rosetta_timestamp::TimestampUTC::default()),
        }
    }
}
impl InsertableProcedureModelBuilder {
    pub fn name<P>(
        mut self,
        name: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableProcedureModelAttributes>>
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let name = name.try_into().map_err(|err: <P as TryInto<String>>::Error| {
            Into::into(err).rename_field(InsertableProcedureModelAttributes::Name)
        })?;
        if let Some(description) = self.description.as_ref() {
            pgrx_validation::must_be_distinct(name.as_ref(), description).map_err(|e| {
                e.rename_fields(
                    InsertableProcedureModelAttributes::Name,
                    InsertableProcedureModelAttributes::Description,
                )
            })?;
        }
        pgrx_validation::must_be_paragraph(name.as_ref())
            .map_err(|e| e.rename_field(InsertableProcedureModelAttributes::Name))?;
        self.name = Some(name);
        Ok(self)
    }
    pub fn description<P>(
        mut self,
        description: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableProcedureModelAttributes>>
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let description =
            description.try_into().map_err(|err: <P as TryInto<String>>::Error| {
                Into::into(err).rename_field(InsertableProcedureModelAttributes::Description)
            })?;
        if let Some(name) = self.name.as_ref() {
            pgrx_validation::must_be_distinct(name, description.as_ref()).map_err(|e| {
                e.rename_fields(
                    InsertableProcedureModelAttributes::Name,
                    InsertableProcedureModelAttributes::Description,
                )
            })?;
        }
        pgrx_validation::must_be_paragraph(description.as_ref())
            .map_err(|e| e.rename_field(InsertableProcedureModelAttributes::Description))?;
        self.description = Some(description);
        Ok(self)
    }
    pub fn deprecated<P>(
        mut self,
        deprecated: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableProcedureModelAttributes>>
    where
        P: TryInto<bool>,
        <P as TryInto<bool>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let deprecated = deprecated.try_into().map_err(|err: <P as TryInto<bool>>::Error| {
            Into::into(err).rename_field(InsertableProcedureModelAttributes::Deprecated)
        })?;
        self.deprecated = Some(deprecated);
        Ok(self)
    }
    pub fn photograph_id<P>(
        mut self,
        photograph_id: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableProcedureModelAttributes>>
    where
        P: TryInto<Option<::rosetta_uuid::Uuid>>,
        <P as TryInto<Option<::rosetta_uuid::Uuid>>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let photograph_id = photograph_id.try_into().map_err(
            |err: <P as TryInto<Option<::rosetta_uuid::Uuid>>>::Error| {
                Into::into(err).rename_field(InsertableProcedureModelAttributes::PhotographId)
            },
        )?;
        self.photograph_id = photograph_id;
        Ok(self)
    }
    pub fn icon<P>(
        mut self,
        icon: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableProcedureModelAttributes>>
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let icon = icon.try_into().map_err(|err: <P as TryInto<String>>::Error| {
            Into::into(err).rename_field(InsertableProcedureModelAttributes::Icon)
        })?;
        pgrx_validation::must_be_font_awesome_class(icon.as_ref())
            .map_err(|e| e.rename_field(InsertableProcedureModelAttributes::Icon))?;
        self.icon = Some(icon);
        Ok(self)
    }
    pub fn created_by<P>(
        mut self,
        created_by: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableProcedureModelAttributes>>
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let created_by = created_by.try_into().map_err(|err: <P as TryInto<i32>>::Error| {
            Into::into(err).rename_field(InsertableProcedureModelAttributes::CreatedBy)
        })?;
        self.created_by = Some(created_by);
        self = self.updated_by(created_by)?;
        Ok(self)
    }
    pub fn created_at<P>(
        mut self,
        created_at: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableProcedureModelAttributes>>
    where
        P: TryInto<::rosetta_timestamp::TimestampUTC>,
        <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let created_at = created_at.try_into().map_err(
            |err: <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error| {
                Into::into(err).rename_field(InsertableProcedureModelAttributes::CreatedAt)
            },
        )?;
        if let Some(updated_at) = self.updated_at {
            pgrx_validation::must_be_smaller_than_utc(created_at, updated_at).map_err(|e| {
                e.rename_fields(
                    InsertableProcedureModelAttributes::CreatedAt,
                    InsertableProcedureModelAttributes::UpdatedAt,
                )
            })?;
        }
        self.created_at = Some(created_at);
        Ok(self)
    }
    pub fn updated_by<P>(
        mut self,
        updated_by: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableProcedureModelAttributes>>
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let updated_by = updated_by.try_into().map_err(|err: <P as TryInto<i32>>::Error| {
            Into::into(err).rename_field(InsertableProcedureModelAttributes::UpdatedBy)
        })?;
        self.updated_by = Some(updated_by);
        Ok(self)
    }
    pub fn updated_at<P>(
        mut self,
        updated_at: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableProcedureModelAttributes>>
    where
        P: TryInto<::rosetta_timestamp::TimestampUTC>,
        <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let updated_at = updated_at.try_into().map_err(
            |err: <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error| {
                Into::into(err).rename_field(InsertableProcedureModelAttributes::UpdatedAt)
            },
        )?;
        if let Some(created_at) = self.created_at {
            pgrx_validation::must_be_smaller_than_utc(created_at, updated_at).map_err(|e| {
                e.rename_fields(
                    InsertableProcedureModelAttributes::CreatedAt,
                    InsertableProcedureModelAttributes::UpdatedAt,
                )
            })?;
        }
        self.updated_at = Some(updated_at);
        Ok(self)
    }
}
impl TryFrom<InsertableProcedureModelBuilder> for InsertableProcedureModel {
    type Error = common_traits::prelude::BuilderError<InsertableProcedureModelAttributes>;
    fn try_from(
        builder: InsertableProcedureModelBuilder,
    ) -> Result<InsertableProcedureModel, Self::Error> {
        Ok(Self {
            name: builder.name.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableProcedureModelAttributes::Name,
            ))?,
            description: builder.description.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableProcedureModelAttributes::Description,
                ),
            )?,
            deprecated: builder.deprecated.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableProcedureModelAttributes::Deprecated,
                ),
            )?,
            photograph_id: builder.photograph_id,
            icon: builder.icon.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableProcedureModelAttributes::Icon,
            ))?,
            created_by: builder.created_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableProcedureModelAttributes::CreatedBy,
                ),
            )?,
            created_at: builder.created_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableProcedureModelAttributes::CreatedAt,
                ),
            )?,
            updated_by: builder.updated_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableProcedureModelAttributes::UpdatedBy,
                ),
            )?,
            updated_at: builder.updated_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableProcedureModelAttributes::UpdatedAt,
                ),
            )?,
        })
    }
}
