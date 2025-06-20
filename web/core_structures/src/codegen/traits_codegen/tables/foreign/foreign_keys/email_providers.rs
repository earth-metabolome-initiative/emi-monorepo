#[derive(Debug, Clone, PartialEq, Default, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EmailProviderForeignKeys {
    pub email: Option<crate::codegen::structs_codegen::tables::user_emails::UserEmail>,
    pub login_provider:
        Option<crate::codegen::structs_codegen::tables::login_providers::LoginProvider>,
}
impl web_common_traits::prelude::HasForeignKeys
    for crate::codegen::structs_codegen::tables::email_providers::EmailProvider
{
    type ForeignKeys = EmailProviderForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::UserEmail(self.email_id),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::LoginProvider(
                self.login_provider_id,
            ),
        ));
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.email.is_some() && foreign_keys.login_provider.is_some()
    }
    fn update(
        &self,
        foreign_keys: &mut Self::ForeignKeys,
        row: Self::Row,
        crud: web_common_traits::crud::CRUD,
    ) -> bool {
        let mut updated = false;
        match (row, crud) {
            (
                crate::codegen::tables::row::Row::LoginProvider(login_providers),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.login_provider_id == login_providers.id {
                    foreign_keys.login_provider = Some(login_providers);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::LoginProvider(login_providers),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.login_provider_id == login_providers.id {
                    foreign_keys.login_provider = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::UserEmail(user_emails),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.email_id == user_emails.id {
                    foreign_keys.email = Some(user_emails);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::UserEmail(user_emails),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.email_id == user_emails.id {
                    foreign_keys.email = None;
                    updated = true;
                }
            }
            (_, crud) => {
                unreachable!("Unexpected row type with operation {crud}");
            }
        }
        updated
    }
}
impl web_common_traits::prelude::ForeignKeys for EmailProviderForeignKeys {}
