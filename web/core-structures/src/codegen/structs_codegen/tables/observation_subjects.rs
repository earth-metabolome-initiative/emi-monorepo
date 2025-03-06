#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "diesel", derive(diesel::Selectable, diesel::Queryable, diesel::Identifiable))]
#[cfg_attr(feature = "diesel", diesel(primary_key(id)))]
# [cfg_attr (feature = "diesel" , diesel (table_name = crate :: codegen :: diesel_codegen :: tables :: observation_subjects :: observation_subjects))]
pub struct ObservationSubject {
    pub name: String,
    pub description: String,
    pub icon: String,
    pub color: String,
    pub id: i16,
}
impl ObservationSubject {
    #[cfg(feature = "postgres")]
    pub async fn from_name(
        name: &str,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{associations::HasTable, OptionalExtension, QueryDsl};
        use diesel_async::RunQueryDsl;
        Self :: table () . filter (diesel :: ExpressionMethods :: eq (crate :: codegen :: diesel_codegen :: tables :: observation_subjects :: observation_subjects :: name , name)) . first :: < Self > (conn) . await . optional ()
    }
    #[cfg(feature = "postgres")]
    pub async fn from_description(
        description: &str,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{associations::HasTable, OptionalExtension, QueryDsl};
        use diesel_async::RunQueryDsl;
        Self :: table () . filter (diesel :: ExpressionMethods :: eq (crate :: codegen :: diesel_codegen :: tables :: observation_subjects :: observation_subjects :: description , description)) . first :: < Self > (conn) . await . optional ()
    }
    #[cfg(feature = "postgres")]
    pub async fn from_icon(
        icon: &str,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{associations::HasTable, OptionalExtension, QueryDsl};
        use diesel_async::RunQueryDsl;
        Self :: table () . filter (diesel :: ExpressionMethods :: eq (crate :: codegen :: diesel_codegen :: tables :: observation_subjects :: observation_subjects :: icon , icon)) . first :: < Self > (conn) . await . optional ()
    }
}
