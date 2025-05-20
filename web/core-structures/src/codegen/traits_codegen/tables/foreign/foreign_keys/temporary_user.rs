#[derive(Debug, Clone, PartialEq, Default, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TemporaryUserForeignKeys {
    pub login_provider:
        Option<crate::codegen::structs_codegen::tables::login_providers::LoginProvider>,
}
impl web_common_traits::prelude::HasForeignKeys
    for crate::codegen::structs_codegen::tables::temporary_user::TemporaryUser
{
    type ForeignKeys = TemporaryUserForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::LoginProvider(
                self.login_provider_id,
            ),
        ));
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.login_provider.is_some()
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
                if login_providers.id == self.login_provider_id {
                    foreign_keys.login_provider = Some(login_providers);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::LoginProvider(login_providers),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if login_providers.id == self.login_provider_id {
                    foreign_keys.login_provider = None;
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
impl web_common_traits::prelude::ForeignKeys for TemporaryUserForeignKeys {}
