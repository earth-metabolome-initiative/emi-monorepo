#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableUserAttributes {
    FirstName,
    LastName,
    CreatedAt,
    UpdatedAt,
}
impl core::fmt::Display for InsertableUserAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableUserAttributes::FirstName => write!(f, "first_name"),
            InsertableUserAttributes::LastName => write!(f, "last_name"),
            InsertableUserAttributes::CreatedAt => write!(f, "created_at"),
            InsertableUserAttributes::UpdatedAt => write!(f, "updated_at"),
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
    first_name: String,
    last_name: String,
    created_at: ::rosetta_timestamp::TimestampUTC,
    updated_at: ::rosetta_timestamp::TimestampUTC,
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
impl InsertableUserBuilder {
    pub fn first_name<P>(
        mut self,
        first_name: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableUserAttributes>>
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let first_name = first_name.try_into().map_err(|err: <P as TryInto<String>>::Error| {
            Into::into(err).rename_field(InsertableUserAttributes::FirstName)
        })?;
        pgrx_validation::must_be_paragraph(first_name.as_ref())
            .map_err(|e| e.rename_field(InsertableUserAttributes::FirstName))?;
        self.first_name = Some(first_name);
        Ok(self)
    }
    pub fn last_name<P>(
        mut self,
        last_name: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableUserAttributes>>
    where
        P: TryInto<String>,
        <P as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let last_name = last_name.try_into().map_err(|err: <P as TryInto<String>>::Error| {
            Into::into(err).rename_field(InsertableUserAttributes::LastName)
        })?;
        pgrx_validation::must_be_paragraph(last_name.as_ref())
            .map_err(|e| e.rename_field(InsertableUserAttributes::LastName))?;
        self.last_name = Some(last_name);
        Ok(self)
    }
    pub fn created_at<P>(
        mut self,
        created_at: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableUserAttributes>>
    where
        P: TryInto<::rosetta_timestamp::TimestampUTC>,
        <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let created_at = created_at.try_into().map_err(
            |err: <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error| {
                Into::into(err).rename_field(InsertableUserAttributes::CreatedAt)
            },
        )?;
        if let Some(updated_at) = self.updated_at {
            pgrx_validation::must_be_smaller_than_utc(created_at, updated_at).map_err(|e| {
                e.rename_fields(
                    InsertableUserAttributes::CreatedAt,
                    InsertableUserAttributes::UpdatedAt,
                )
            })?;
        }
        self.created_at = Some(created_at);
        Ok(self)
    }
    pub fn updated_at<P>(
        mut self,
        updated_at: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableUserAttributes>>
    where
        P: TryInto<::rosetta_timestamp::TimestampUTC>,
        <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let updated_at = updated_at.try_into().map_err(
            |err: <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error| {
                Into::into(err).rename_field(InsertableUserAttributes::UpdatedAt)
            },
        )?;
        if let Some(created_at) = self.created_at {
            pgrx_validation::must_be_smaller_than_utc(created_at, updated_at).map_err(|e| {
                e.rename_fields(
                    InsertableUserAttributes::CreatedAt,
                    InsertableUserAttributes::UpdatedAt,
                )
            })?;
        }
        self.updated_at = Some(updated_at);
        Ok(self)
    }
}
impl TryFrom<InsertableUserBuilder> for InsertableUser {
    type Error = common_traits::prelude::BuilderError<InsertableUserAttributes>;
    fn try_from(builder: InsertableUserBuilder) -> Result<InsertableUser, Self::Error> {
        Ok(Self {
            first_name: builder.first_name.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableUserAttributes::FirstName,
                ),
            )?,
            last_name: builder.last_name.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableUserAttributes::LastName,
                ),
            )?,
            created_at: builder.created_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableUserAttributes::CreatedAt,
                ),
            )?,
            updated_at: builder.updated_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableUserAttributes::UpdatedAt,
                ),
            )?,
        })
    }
}
