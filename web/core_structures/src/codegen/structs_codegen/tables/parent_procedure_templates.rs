#[derive(Debug, Clone, PartialEq, Copy, Eq, PartialOrd, Ord, Hash)]
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
#[diesel(belongs_to(crate::User, foreign_key = created_by))]
#[diesel(primary_key(parent, child))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::parent_procedure_templates::parent_procedure_templates
)]
pub struct ParentProcedureTemplate {
    pub parent: i32,
    pub child: i32,
    pub created_by: i32,
    pub created_at: ::rosetta_timestamp::TimestampUTC,
}
impl web_common_traits::prelude::TableName for ParentProcedureTemplate {
    const TABLE_NAME: &'static str = "parent_procedure_templates";
}
impl web_common_traits::prelude::ExtensionTable<crate::ParentProcedureTemplate>
    for ParentProcedureTemplate
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a (i32, i32)>,
{
}
impl diesel::Identifiable for ParentProcedureTemplate {
    type Id = (i32, i32);
    fn id(self) -> Self::Id {
        (self.parent, self.child)
    }
}
impl ParentProcedureTemplate {
    pub fn child<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::ProcedureTemplate, diesel::result::Error>
    where
        crate::ProcedureTemplate: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::ProcedureTemplate::read(self.child, conn)
    }
    pub fn created_by<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::User, diesel::result::Error>
    where
        crate::User: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::User::read(self.created_by, conn)
    }
    pub fn parent<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::ProcedureTemplate, diesel::result::Error>
    where
        crate::ProcedureTemplate: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::ProcedureTemplate::read(self.parent, conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_child(
        child: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::parent_procedure_templates::parent_procedure_templates;
        Self::table()
            .filter(parent_procedure_templates::child.eq(child))
            .order_by((
                parent_procedure_templates::parent.asc(),
                parent_procedure_templates::child.asc(),
            ))
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_parent(
        parent: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::parent_procedure_templates::parent_procedure_templates;
        Self::table()
            .filter(parent_procedure_templates::parent.eq(parent))
            .order_by((
                parent_procedure_templates::parent.asc(),
                parent_procedure_templates::child.asc(),
            ))
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_created_at(
        created_at: &::rosetta_timestamp::TimestampUTC,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::parent_procedure_templates::parent_procedure_templates;
        Self::table()
            .filter(parent_procedure_templates::created_at.eq(created_at))
            .order_by((
                parent_procedure_templates::parent.asc(),
                parent_procedure_templates::child.asc(),
            ))
            .load::<Self>(conn)
    }
}
impl AsRef<ParentProcedureTemplate> for ParentProcedureTemplate {
    fn as_ref(&self) -> &ParentProcedureTemplate {
        self
    }
}
