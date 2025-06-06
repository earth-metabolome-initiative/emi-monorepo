#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    diesel::Selectable,
    diesel::Insertable,
    diesel::AsChangeset,
    diesel::Queryable,
    diesel::Identifiable,
)]
#[diesel(primary_key(id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::directus_notifications::directus_notifications
)]
pub struct DirectusNotification {
    pub id: i32,
    pub timestamp: Option<::rosetta_timestamp::TimestampUTC>,
    pub status: Option<String>,
    pub recipient: ::rosetta_uuid::Uuid,
    pub sender: Option<::rosetta_uuid::Uuid>,
    pub subject: String,
    pub message: Option<String>,
    pub collection: Option<String>,
    pub item: Option<String>,
}
impl web_common_traits::prelude::TableName for DirectusNotification {
    const TABLE_NAME: &'static str = "directus_notifications";
}
impl diesel::Identifiable for DirectusNotification {
    type Id = i32;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl DirectusNotification {
    pub fn recipient<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::directus_users::DirectusUser,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::directus_users::DirectusUser: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::directus_users::DirectusUser as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::directus_users::DirectusUser as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::directus_users::DirectusUser as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::directus_users::DirectusUser as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::directus_users::DirectusUser as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::directus_users::DirectusUser as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::directus_users::DirectusUser,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::directus_users::DirectusUser::table(),
                self.recipient,
            ),
            conn,
        )
    }
    pub fn sender<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::directus_users::DirectusUser>,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::directus_users::DirectusUser: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::directus_users::DirectusUser as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::directus_users::DirectusUser as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::directus_users::DirectusUser as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::directus_users::DirectusUser as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::directus_users::DirectusUser as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::directus_users::DirectusUser as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::directus_users::DirectusUser,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        let Some(sender) = self.sender else {
            return Ok(None);
        };
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::directus_users::DirectusUser::table(),
                sender,
            ),
            conn,
        )
        .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub fn from_timestamp(
        timestamp: &::rosetta_timestamp::TimestampUTC,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_notifications::directus_notifications;
        Self::table()
            .filter(directus_notifications::timestamp.eq(timestamp))
            .order_by(directus_notifications::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_status(
        status: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_notifications::directus_notifications;
        Self::table()
            .filter(directus_notifications::status.eq(status))
            .order_by(directus_notifications::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_recipient(
        recipient: &::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_notifications::directus_notifications;
        Self::table()
            .filter(directus_notifications::recipient.eq(recipient))
            .order_by(directus_notifications::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_sender(
        sender: &::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_notifications::directus_notifications;
        Self::table()
            .filter(directus_notifications::sender.eq(sender))
            .order_by(directus_notifications::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_subject(
        subject: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_notifications::directus_notifications;
        Self::table()
            .filter(directus_notifications::subject.eq(subject))
            .order_by(directus_notifications::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_message(
        message: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_notifications::directus_notifications;
        Self::table()
            .filter(directus_notifications::message.eq(message))
            .order_by(directus_notifications::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_collection(
        collection: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_notifications::directus_notifications;
        Self::table()
            .filter(directus_notifications::collection.eq(collection))
            .order_by(directus_notifications::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_item(
        item: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_notifications::directus_notifications;
        Self::table()
            .filter(directus_notifications::item.eq(item))
            .order_by(directus_notifications::id.asc())
            .load::<Self>(conn)
    }
}
impl AsRef<DirectusNotification> for DirectusNotification {
    fn as_ref(&self) -> &DirectusNotification {
        self
    }
}
