#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableUserAttributes {
    Id,
    FirstName,
    LastName,
    CreatedAt,
    UpdatedAt,
}
impl core::fmt::Display for InsertableUserAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Id => write!(f, "id"),
            Self::FirstName => write!(f, "first_name"),
            Self::LastName => write!(f, "last_name"),
            Self::CreatedAt => write!(f, "created_at"),
            Self::UpdatedAt => write!(f, "updated_at"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(table_name = crate::codegen::diesel_codegen::tables::users::users)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableUser {
    pub(crate) first_name: String,
    pub(crate) last_name: String,
    pub(crate) created_at: ::rosetta_timestamp::TimestampUTC,
    pub(crate) updated_at: ::rosetta_timestamp::TimestampUTC,
}
impl InsertableUser {}
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableUserBuilder {
    pub(crate) first_name: Option<String>,
    pub(crate) last_name: Option<String>,
    pub(crate) created_at: Option<::rosetta_timestamp::TimestampUTC>,
    pub(crate) updated_at: Option<::rosetta_timestamp::TimestampUTC>,
}
impl Default for InsertableUserBuilder {
    fn default() -> Self {
        Self {
            first_name: Default::default(),
            last_name: Default::default(),
            created_at: Some(rosetta_timestamp::TimestampUTC::default()),
            updated_at: Some(rosetta_timestamp::TimestampUTC::default()),
        }
    }
}
impl web_common_traits::database::ExtendableBuilder for InsertableUserBuilder {
    type Attributes = InsertableUserAttributes;
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
        if let Some(first_name) = other.first_name {
            self = self.first_name(first_name)?;
        }
        if let Some(last_name) = other.last_name {
            self = self.last_name(last_name)?;
        }
        Ok(self)
    }
}
impl web_common_traits::prelude::SetPrimaryKey for InsertableUserBuilder {
    type PrimaryKey = i32;
    fn set_primary_key(self, _primary_key: Self::PrimaryKey) -> Self {
        self
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableUserBuilder {
    /// Sets the value of the `users.created_at` column from table `users`.
    pub fn created_at<CreatedAt>(
        mut self,
        created_at: CreatedAt,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableUserAttributes>>
    where
        CreatedAt: TryInto<::rosetta_timestamp::TimestampUTC>,
        <CreatedAt as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let created_at = created_at.try_into().map_err(
            |err: <CreatedAt as TryInto<::rosetta_timestamp::TimestampUTC>>::Error| {
                Into::into(err).rename_field(InsertableUserAttributes::CreatedAt)
            },
        )?;
        if let Some(updated_at) = self.updated_at {
            pgrx_validation::must_be_smaller_than_utc(created_at, updated_at)
                .map_err(|e| {
                    e
                        .rename_fields(
                            crate::codegen::structs_codegen::tables::insertables::InsertableUserAttributes::CreatedAt,
                            crate::codegen::structs_codegen::tables::insertables::InsertableUserAttributes::UpdatedAt,
                        )
                })?;
        }
        self.created_at = Some(created_at);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableUserBuilder {
    /// Sets the value of the `users.created_at`, `users.updated_at` columns
    /// from table `users`.
    pub fn created_at_and_updated_at<CreatedAt, UpdatedAt>(
        mut self,
        created_at: CreatedAt,
        updated_at: UpdatedAt,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableUserAttributes>>
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
                Into::into(err).rename_field(InsertableUserAttributes::CreatedAt)
            },
        )?;
        let updated_at = updated_at.try_into().map_err(
            |err: <UpdatedAt as TryInto<::rosetta_timestamp::TimestampUTC>>::Error| {
                Into::into(err).rename_field(InsertableUserAttributes::UpdatedAt)
            },
        )?;
        pgrx_validation::must_be_smaller_than_utc(created_at, updated_at)
            .map_err(|e| {
                e
                    .rename_fields(
                        crate::codegen::structs_codegen::tables::insertables::InsertableUserAttributes::CreatedAt,
                        crate::codegen::structs_codegen::tables::insertables::InsertableUserAttributes::UpdatedAt,
                    )
            })?;
        self.created_at = Some(created_at);
        self.updated_at = Some(updated_at);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableUserBuilder {
    /// Sets the value of the `users.first_name` column from table `users`.
    pub fn first_name<FirstName>(
        mut self,
        first_name: FirstName,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableUserAttributes>>
    where
        FirstName: TryInto<String>,
        <FirstName as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let first_name =
            first_name.try_into().map_err(|err: <FirstName as TryInto<String>>::Error| {
                Into::into(err).rename_field(InsertableUserAttributes::FirstName)
            })?;
        pgrx_validation::must_be_paragraph(first_name.as_ref())
            .map_err(|e| {
                e
                    .rename_field(
                        crate::codegen::structs_codegen::tables::insertables::InsertableUserAttributes::FirstName,
                    )
            })?;
        self.first_name = Some(first_name);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableUserBuilder {
    /// Sets the value of the `users.last_name` column from table `users`.
    pub fn last_name<LastName>(
        mut self,
        last_name: LastName,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableUserAttributes>>
    where
        LastName: TryInto<String>,
        <LastName as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let last_name =
            last_name.try_into().map_err(|err: <LastName as TryInto<String>>::Error| {
                Into::into(err).rename_field(InsertableUserAttributes::LastName)
            })?;
        pgrx_validation::must_be_paragraph(last_name.as_ref())
            .map_err(|e| {
                e
                    .rename_field(
                        crate::codegen::structs_codegen::tables::insertables::InsertableUserAttributes::LastName,
                    )
            })?;
        self.last_name = Some(last_name);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableUserBuilder {
    /// Sets the value of the `users.updated_at` column from table `users`.
    pub fn updated_at<UpdatedAt>(
        mut self,
        updated_at: UpdatedAt,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableUserAttributes>>
    where
        UpdatedAt: TryInto<::rosetta_timestamp::TimestampUTC>,
        <UpdatedAt as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let updated_at = updated_at.try_into().map_err(
            |err: <UpdatedAt as TryInto<::rosetta_timestamp::TimestampUTC>>::Error| {
                Into::into(err).rename_field(InsertableUserAttributes::UpdatedAt)
            },
        )?;
        if let Some(created_at) = self.created_at {
            pgrx_validation::must_be_smaller_than_utc(created_at, updated_at)
                .map_err(|e| {
                    e
                        .rename_fields(
                            crate::codegen::structs_codegen::tables::insertables::InsertableUserAttributes::CreatedAt,
                            crate::codegen::structs_codegen::tables::insertables::InsertableUserAttributes::UpdatedAt,
                        )
                })?;
        }
        self.updated_at = Some(updated_at);
        Ok(self)
    }
}
impl<C> web_common_traits::database::TryInsertGeneric<C> for InsertableUserBuilder
where
    Self: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::codegen::structs_codegen::tables::users::User,
            Error = web_common_traits::database::InsertError<InsertableUserAttributes>,
        >,
{
    type Attributes = InsertableUserAttributes;
    fn is_complete(&self) -> bool {
        self.first_name.is_some()
            && self.last_name.is_some()
            && self.created_at.is_some()
            && self.updated_at.is_some()
    }
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, web_common_traits::database::InsertError<Self::Attributes>> {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::users::User =
            self.insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
