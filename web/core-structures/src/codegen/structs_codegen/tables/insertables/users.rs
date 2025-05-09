#[derive(Clone, core::fmt::Debug)]
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
    created_at: rosetta_timestamp::TimestampUTC,
    updated_at: rosetta_timestamp::TimestampUTC,
}
impl InsertableUser {}
pub struct InsertableUserBuilder {
    first_name: Option<String>,
    last_name: Option<String>,
    created_at: Option<rosetta_timestamp::TimestampUTC>,
    updated_at: Option<rosetta_timestamp::TimestampUTC>,
}
impl Default for InsertableUserBuilder {
    fn default() -> Self {
        Self {
            first_name: None,
            last_name: None,
            created_at: Some(rosetta_timestamp::TimestampUTC::default()),
            updated_at: Some(rosetta_timestamp::TimestampUTC::default()),
        }
    }
}
impl InsertableUserBuilder {
    pub fn first_name<P: Into<String>>(
        mut self,
        first_name: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let first_name = first_name.into();
        pgrx_validation::must_be_paragraph(first_name.as_ref())
            .map_err(|e| e.rename_field(InsertableUserAttributes::FirstName))?;
        self.first_name = Some(first_name);
        Ok(self)
    }
    pub fn last_name<P: Into<String>>(
        mut self,
        last_name: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let last_name = last_name.into();
        pgrx_validation::must_be_paragraph(last_name.as_ref())
            .map_err(|e| e.rename_field(InsertableUserAttributes::LastName))?;
        self.last_name = Some(last_name);
        Ok(self)
    }
    pub fn created_at<P: Into<rosetta_timestamp::TimestampUTC>>(
        mut self,
        created_at: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let created_at = created_at.into();
        self.created_at = Some(created_at);
        Ok(self)
    }
    pub fn updated_at<P: Into<rosetta_timestamp::TimestampUTC>>(
        mut self,
        updated_at: P,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        let updated_at = updated_at.into();
        self.updated_at = Some(updated_at);
        Ok(self)
    }
}
impl common_traits::prelude::Builder for InsertableUserBuilder {
    type Error = web_common_traits::database::InsertError<InsertableUserAttributes>;
    type Object = InsertableUser;
    type Attribute = InsertableUserAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            first_name: self.first_name.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableUserAttributes::FirstName,
                ),
            )?,
            last_name: self.last_name.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableUserAttributes::LastName,
                ),
            )?,
            created_at: self.created_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableUserAttributes::CreatedAt,
                ),
            )?,
            updated_at: self.updated_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableUserAttributes::UpdatedAt,
                ),
            )?,
        })
    }
}
impl TryFrom<InsertableUser> for InsertableUserBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableUser) -> Result<Self, Self::Error> {
        Self::default()
            .first_name(insertable_variant.first_name)?
            .last_name(insertable_variant.last_name)?
            .created_at(insertable_variant.created_at)?
            .updated_at(insertable_variant.updated_at)
    }
}
