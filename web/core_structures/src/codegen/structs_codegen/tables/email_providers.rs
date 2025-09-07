#[derive(Debug, Clone, PartialEq, Copy, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    diesel::Selectable,
    diesel::Insertable,
    diesel::Queryable,
    diesel::Identifiable,
    diesel::Associations,
)]
#[cfg_attr(feature = "yew", derive(yew::prelude::Properties))]
#[diesel(belongs_to(crate::UserEmail, foreign_key = email_id))]
#[diesel(belongs_to(crate::LoginProvider, foreign_key = login_provider_id))]
#[diesel(primary_key(email_id, login_provider_id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::email_providers::email_providers
)]
pub struct EmailProvider {
    pub email_id: i32,
    pub login_provider_id: i16,
}
impl web_common_traits::prelude::TableName for EmailProvider {
    const TABLE_NAME: &'static str = "email_providers";
}
impl web_common_traits::prelude::ExtensionTable<crate::EmailProvider> for EmailProvider where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a (i32, i16)>
{
}
impl diesel::Identifiable for EmailProvider {
    type Id = (i32, i16);
    fn id(self) -> Self::Id {
        (self.email_id, self.login_provider_id)
    }
}
impl EmailProvider {
    pub fn email<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::UserEmail, diesel::result::Error>
    where
        crate::UserEmail: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::UserEmail::read(self.email_id, conn)
    }
    pub fn login_provider<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::LoginProvider, diesel::result::Error>
    where
        crate::LoginProvider: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::LoginProvider::read(self.login_provider_id, conn)
    }
}
impl AsRef<EmailProvider> for EmailProvider {
    fn as_ref(&self) -> &EmailProvider {
        self
    }
}
