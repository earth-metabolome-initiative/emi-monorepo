#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    diesel::Selectable,
    diesel::Insertable,
    diesel::AsChangeset,
    diesel::Queryable,
    diesel::Identifiable,
    diesel::Associations,
)]
#[cfg_attr(feature = "yew", derive(yew::prelude::Properties))]
#[diesel(
    belongs_to(
        crate::codegen::structs_codegen::tables::login_providers::LoginProvider,
        foreign_key = login_provider_id
    )
)]
#[diesel(primary_key(id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::temporary_user::temporary_user
)]
pub struct TemporaryUser {
    pub id: i32,
    pub email: String,
    pub login_provider_id: i16,
}
impl web_common_traits::prelude::TableName for TemporaryUser {
    const TABLE_NAME: &'static str = "temporary_user";
}
impl<'a> From<&'a TemporaryUser>
    for web_common_traits::database::IdOrBuilder<
        i32,
        crate::codegen::structs_codegen::tables::insertables::InsertableTemporaryUserBuilder,
    >
{
    fn from(value: &'a TemporaryUser) -> Self {
        web_common_traits::database::IdOrBuilder::Id(value.id)
    }
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::temporary_user::TemporaryUser,
    > for TemporaryUser
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{
}
impl diesel::Identifiable for TemporaryUser {
    type Id = i32;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl web_common_traits::database::PrimaryKeyLike for TemporaryUser {
    type PrimaryKey = i32;
    fn primary_key(&self) -> Self::PrimaryKey {
        self.id
    }
}
impl TemporaryUser {
    pub fn login_provider<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::login_providers::LoginProvider,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::login_providers::LoginProvider:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::login_providers::LoginProvider::read(
            self.login_provider_id,
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn from_email_and_login_provider_id(
        email: &str,
        login_provider_id: i16,
        conn: &mut diesel::PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::temporary_user::temporary_user;
        Self::table()
            .filter(
                temporary_user::email
                    .eq(email)
                    .and(temporary_user::login_provider_id.eq(login_provider_id)),
            )
            .order_by(temporary_user::id.asc())
            .first::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_email(
        email: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::temporary_user::temporary_user;
        Self::table()
            .filter(temporary_user::email.eq(email))
            .order_by(temporary_user::id.asc())
            .load::<Self>(conn)
    }
}
impl AsRef<TemporaryUser> for TemporaryUser {
    fn as_ref(&self) -> &TemporaryUser {
        self
    }
}
