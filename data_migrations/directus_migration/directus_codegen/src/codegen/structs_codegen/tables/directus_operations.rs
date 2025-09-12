#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    diesel::Selectable,
    diesel::Insertable,
    diesel::AsChangeset,
    diesel::Queryable,
    diesel::Identifiable,
    diesel::Associations,
)]
#[diesel(
    belongs_to(
        crate::codegen::structs_codegen::tables::directus_flows::DirectusFlow,
        foreign_key = flow
    )
)]
#[diesel(
    belongs_to(
        crate::codegen::structs_codegen::tables::directus_users::DirectusUser,
        foreign_key = user_created
    )
)]
#[diesel(primary_key(id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::directus_operations::directus_operations
)]
pub struct DirectusOperation {
    pub id: ::rosetta_uuid::Uuid,
    pub name: Option<String>,
    pub key: String,
    pub r#type: String,
    pub position_x: i32,
    pub position_y: i32,
    pub options: Option<::serde_json::Value>,
    pub resolve: Option<::rosetta_uuid::Uuid>,
    pub reject: Option<::rosetta_uuid::Uuid>,
    pub flow: ::rosetta_uuid::Uuid,
    pub date_created: Option<::rosetta_timestamp::TimestampUTC>,
    pub user_created: Option<::rosetta_uuid::Uuid>,
}
impl web_common_traits::prelude::TableName for DirectusOperation {
    const TABLE_NAME: &'static str = "directus_operations";
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::directus_operations::DirectusOperation,
    > for DirectusOperation
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a ::rosetta_uuid::Uuid>,
{
}
impl diesel::Identifiable for DirectusOperation {
    type Id = ::rosetta_uuid::Uuid;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl web_common_traits::database::PrimaryKeyLike for DirectusOperation {
    type PrimaryKey = ::rosetta_uuid::Uuid;
    fn primary_key(&self) -> Self::PrimaryKey {
        self.id
    }
}
impl DirectusOperation {
    pub fn flow<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::directus_flows::DirectusFlow,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::directus_flows::DirectusFlow:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::directus_flows::DirectusFlow::read(self.flow, conn)
    }
    pub fn reject<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::directus_operations::DirectusOperation>,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::directus_operations::DirectusOperation:
            web_common_traits::database::Read<C>,
    {
        use diesel::OptionalExtension;
        use web_common_traits::database::Read;
        let Some(reject) = self.reject else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_operations::DirectusOperation::read(
            reject, conn,
        )
        .optional()
    }
    pub fn resolve<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::directus_operations::DirectusOperation>,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::directus_operations::DirectusOperation:
            web_common_traits::database::Read<C>,
    {
        use diesel::OptionalExtension;
        use web_common_traits::database::Read;
        let Some(resolve) = self.resolve else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_operations::DirectusOperation::read(
            resolve, conn,
        )
        .optional()
    }
    pub fn user_created<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::directus_users::DirectusUser>,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::directus_users::DirectusUser:
            web_common_traits::database::Read<C>,
    {
        use diesel::OptionalExtension;
        use web_common_traits::database::Read;
        let Some(user_created) = self.user_created else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_users::DirectusUser::read(
            user_created,
            conn,
        )
        .optional()
    }
    #[cfg(feature = "postgres")]
    pub fn from_reject(
        reject: ::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_operations::directus_operations;
        Self::table()
            .filter(directus_operations::reject.eq(reject))
            .order_by(directus_operations::id.asc())
            .first::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_resolve(
        resolve: ::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_operations::directus_operations;
        Self::table()
            .filter(directus_operations::resolve.eq(resolve))
            .order_by(directus_operations::id.asc())
            .first::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_name(
        name: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_operations::directus_operations;
        Self::table()
            .filter(directus_operations::name.eq(name))
            .order_by(directus_operations::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_key(
        key: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_operations::directus_operations;
        Self::table()
            .filter(directus_operations::key.eq(key))
            .order_by(directus_operations::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_type(
        r#type: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_operations::directus_operations;
        Self::table()
            .filter(directus_operations::r#type.eq(r#type))
            .order_by(directus_operations::id.asc())
            .load::<Self>(conn)
    }
    pub fn from_position_x<C>(
        position_x: i32,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::directus_operations::directus_operations::position_x as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::directus_operations::directus_operations::position_x as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::directus_operations::directus_operations::id,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::directus_operations::directus_operations::position_x as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::directus_operations::directus_operations::id,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_operations::directus_operations;
        Self::table()
            .filter(directus_operations::position_x.eq(position_x))
            .order_by(directus_operations::id.asc())
            .load::<Self>(conn)
    }
    pub fn from_position_y<C>(
        position_y: i32,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::directus_operations::directus_operations::position_y as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::directus_operations::directus_operations::position_y as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::directus_operations::directus_operations::id,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::directus_operations::directus_operations::position_y as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::directus_operations::directus_operations::id,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_operations::directus_operations;
        Self::table()
            .filter(directus_operations::position_y.eq(position_y))
            .order_by(directus_operations::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_date_created(
        date_created: ::rosetta_timestamp::TimestampUTC,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_operations::directus_operations;
        Self::table()
            .filter(directus_operations::date_created.eq(date_created))
            .order_by(directus_operations::id.asc())
            .load::<Self>(conn)
    }
}
impl AsRef<DirectusOperation> for DirectusOperation {
    fn as_ref(&self) -> &DirectusOperation {
        self
    }
}
