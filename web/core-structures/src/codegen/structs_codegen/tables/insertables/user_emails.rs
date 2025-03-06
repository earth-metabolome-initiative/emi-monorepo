#[derive(Clone, core :: fmt :: Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableUserEmailAttributes {
    Id,
    Email,
    CreatedBy,
    LoginProviderId,
    PrimaryEmail,
}
impl core::fmt::Display for InsertableUserEmailAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableUserEmailAttributes::Id => write!(f, "id"),
            InsertableUserEmailAttributes::Email => write!(f, "email"),
            InsertableUserEmailAttributes::CreatedBy => write!(f, "created_by"),
            InsertableUserEmailAttributes::LoginProviderId => write!(f, "login_provider_id"),
            InsertableUserEmailAttributes::PrimaryEmail => write!(f, "primary_email"),
        }
    }
}
#[cfg_attr(feature = "diesel", derive(diesel::Insertable))]
# [cfg_attr (feature = "diesel" , diesel (table_name = crate :: codegen :: diesel_codegen :: tables :: user_emails :: user_emails))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableUserEmail {
    id: i32,
    email: String,
    created_by: i32,
    login_provider_id: i16,
    primary_email: bool,
}
#[derive(Default)]
pub struct InsertableUserEmailBuilder {
    id: Option<i32>,
    email: Option<String>,
    created_by: Option<i32>,
    login_provider_id: Option<i16>,
    primary_email: Option<bool>,
}
impl InsertableUserEmailBuilder {
    pub fn id(mut self, id: i32) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.id = Some(id);
        Ok(self)
    }
    pub fn email(
        mut self,
        email: String,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.email = Some(email);
        Ok(self)
    }
    pub fn created_by(
        mut self,
        created_by: i32,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.created_by = Some(created_by);
        Ok(self)
    }
    pub fn login_provider_id(
        mut self,
        login_provider_id: i16,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.login_provider_id = Some(login_provider_id);
        Ok(self)
    }
    pub fn primary_email(
        mut self,
        primary_email: bool,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.primary_email = Some(primary_email);
        Ok(self)
    }
}
impl common_traits::prelude::Builder for InsertableUserEmailBuilder {
    type Error = web_common_traits::database::InsertError<InsertableUserEmailAttributes>;
    type Object = InsertableUserEmail;
    type Attribute = InsertableUserEmailAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            id: self.id.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableUserEmailAttributes::Id,
                )
            })?,
            email: self.email.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableUserEmailAttributes::Email,
                )
            })?,
            created_by: self.created_by.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableUserEmailAttributes::CreatedBy,
                )
            })?,
            login_provider_id: self.login_provider_id.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableUserEmailAttributes::LoginProviderId,
                )
            })?,
            primary_email: self.primary_email.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableUserEmailAttributes::PrimaryEmail,
                )
            })?,
        })
    }
}
impl TryFrom<InsertableUserEmail> for InsertableUserEmailBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableUserEmail) -> Result<Self, Self::Error> {
        Ok(Self::default()
            .id(insertable_variant.id)?
            .email(insertable_variant.email)?
            .created_by(insertable_variant.created_by)?
            .login_provider_id(insertable_variant.login_provider_id)?
            .primary_email(insertable_variant.primary_email)?)
    }
}
