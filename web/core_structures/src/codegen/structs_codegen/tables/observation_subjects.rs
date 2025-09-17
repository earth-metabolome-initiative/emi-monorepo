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
        crate::codegen::structs_codegen::tables::colors::Color,
        foreign_key = color_id
    )
)]
#[diesel(primary_key(id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::observation_subjects::observation_subjects
)]
pub struct ObservationSubject {
    pub name: String,
    pub description: String,
    pub icon: String,
    pub color_id: i16,
    pub id: i16,
}
impl web_common_traits::prelude::TableName for ObservationSubject {
    const TABLE_NAME: &'static str = "observation_subjects";
}
impl<'a> From<&'a ObservationSubject>
    for web_common_traits::database::IdOrBuilder<
        i16,
        crate::codegen::structs_codegen::tables::insertables::InsertableObservationSubjectBuilder,
    >
{
    fn from(value: &'a ObservationSubject) -> Self {
        web_common_traits::database::IdOrBuilder::Id(value.id)
    }
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::observation_subjects::ObservationSubject,
    > for ObservationSubject
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i16>,
{
}
impl diesel::Identifiable for ObservationSubject {
    type Id = i16;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl web_common_traits::database::PrimaryKeyLike for ObservationSubject {
    type PrimaryKey = i16;
    fn primary_key(&self) -> Self::PrimaryKey {
        self.id
    }
}
impl ObservationSubject {
    pub fn color<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::codegen::structs_codegen::tables::colors::Color, diesel::result::Error>
    where
        crate::codegen::structs_codegen::tables::colors::Color:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::colors::Color::read(self.color_id, conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_name(
        name: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::observation_subjects::observation_subjects;
        Self::table()
            .filter(observation_subjects::name.eq(name))
            .order_by(observation_subjects::id.asc())
            .first::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_description(
        description: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::observation_subjects::observation_subjects;
        Self::table()
            .filter(observation_subjects::description.eq(description))
            .order_by(observation_subjects::id.asc())
            .first::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_icon(
        icon: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::observation_subjects::observation_subjects;
        Self::table()
            .filter(observation_subjects::icon.eq(icon))
            .order_by(observation_subjects::id.asc())
            .first::<Self>(conn)
    }
}
impl AsRef<ObservationSubject> for ObservationSubject {
    fn as_ref(&self) -> &ObservationSubject {
        self
    }
}
