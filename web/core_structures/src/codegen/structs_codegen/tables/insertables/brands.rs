#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableBrandAttributes {
    Name,
    CreatedBy,
    CreatedAt,
    UpdatedBy,
    UpdatedAt,
}
impl core::fmt::Display for InsertableBrandAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableBrandAttributes::Name => write!(f, "name"),
            InsertableBrandAttributes::CreatedBy => write!(f, "created_by"),
            InsertableBrandAttributes::CreatedAt => write!(f, "created_at"),
            InsertableBrandAttributes::UpdatedBy => write!(f, "updated_by"),
            InsertableBrandAttributes::UpdatedAt => write!(f, "updated_at"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(table_name = crate::codegen::diesel_codegen::tables::brands::brands)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableBrand {
    name: String,
    created_by: i32,
    created_at: ::rosetta_timestamp::TimestampUTC,
    updated_by: i32,
    updated_at: ::rosetta_timestamp::TimestampUTC,
}
impl InsertableBrand {
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
pub struct InsertableBrandBuilder {
    pub(crate) name: Option<String>,
    pub(crate) created_by: Option<i32>,
    pub(crate) created_at: Option<::rosetta_timestamp::TimestampUTC>,
    pub(crate) updated_by: Option<i32>,
    pub(crate) updated_at: Option<::rosetta_timestamp::TimestampUTC>,
}
impl Default for InsertableBrandBuilder {
    fn default() -> Self {
        Self {
            name: None,
            created_by: None,
            created_at: Some(rosetta_timestamp::TimestampUTC::default()),
            updated_by: None,
            updated_at: Some(rosetta_timestamp::TimestampUTC::default()),
        }
    }
}
impl InsertableBrandBuilder {
    pub fn name<P>(
        mut self,
        name: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableBrandAttributes>>
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let name = name.try_into().map_err(|err: <P as TryInto<String>>::Error| {
            Into::into(err).rename_field(InsertableBrandAttributes::Name)
        })?;
        pgrx_validation::must_be_paragraph(name.as_ref())
            .map_err(|e| e.rename_field(InsertableBrandAttributes::Name))?;
        self.name = Some(name);
        Ok(self)
    }
    pub fn created_by<P>(
        mut self,
        created_by: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableBrandAttributes>>
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let created_by = created_by.try_into().map_err(|err: <P as TryInto<i32>>::Error| {
            Into::into(err).rename_field(InsertableBrandAttributes::CreatedBy)
        })?;
        self.created_by = Some(created_by);
        self = self.updated_by(created_by)?;
        Ok(self)
    }
    pub fn created_at<P>(
        mut self,
        created_at: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableBrandAttributes>>
    where
        P: TryInto<::rosetta_timestamp::TimestampUTC>,
        <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let created_at = created_at.try_into().map_err(
            |err: <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error| {
                Into::into(err).rename_field(InsertableBrandAttributes::CreatedAt)
            },
        )?;
        self.created_at = Some(created_at);
        Ok(self)
    }
    pub fn updated_by<P>(
        mut self,
        updated_by: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableBrandAttributes>>
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let updated_by = updated_by.try_into().map_err(|err: <P as TryInto<i32>>::Error| {
            Into::into(err).rename_field(InsertableBrandAttributes::UpdatedBy)
        })?;
        self.updated_by = Some(updated_by);
        Ok(self)
    }
    pub fn updated_at<P>(
        mut self,
        updated_at: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableBrandAttributes>>
    where
        P: TryInto<::rosetta_timestamp::TimestampUTC>,
        <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let updated_at = updated_at.try_into().map_err(
            |err: <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error| {
                Into::into(err).rename_field(InsertableBrandAttributes::UpdatedAt)
            },
        )?;
        self.updated_at = Some(updated_at);
        Ok(self)
    }
}
impl TryFrom<InsertableBrandBuilder> for InsertableBrand {
    type Error = common_traits::prelude::BuilderError<InsertableBrandAttributes>;
    fn try_from(builder: InsertableBrandBuilder) -> Result<InsertableBrand, Self::Error> {
        Ok(Self {
            name: builder.name.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableBrandAttributes::Name,
            ))?,
            created_by: builder.created_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableBrandAttributes::CreatedBy,
                ),
            )?,
            created_at: builder.created_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableBrandAttributes::CreatedAt,
                ),
            )?,
            updated_by: builder.updated_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableBrandAttributes::UpdatedBy,
                ),
            )?,
            updated_at: builder.updated_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableBrandAttributes::UpdatedAt,
                ),
            )?,
        })
    }
}
