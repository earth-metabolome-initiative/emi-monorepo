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
#[diesel(
    belongs_to(
        crate::codegen::structs_codegen::tables::users::User,
        foreign_key = created_by
    )
)]
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
impl<'a> From<&'a ParentProcedureTemplate>
for web_common_traits::database::IdOrBuilder<
    (i32, i32),
    crate::codegen::structs_codegen::tables::insertables::InsertableParentProcedureTemplateBuilder,
> {
    fn from(value: &'a ParentProcedureTemplate) -> Self {
        web_common_traits::database::IdOrBuilder::Id((value.parent, value.child))
    }
}
impl web_common_traits::prelude::ExtensionTable<
    crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate,
> for ParentProcedureTemplate
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a (i32, i32)>,
{}
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
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate::read(
            self.child, conn,
        )
    }
    pub fn created_by<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::codegen::structs_codegen::tables::users::User, diesel::result::Error>
    where
        crate::codegen::structs_codegen::tables::users::User: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::users::User::read(self.created_by, conn)
    }
    pub fn parent<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate::read(
            self.parent,
            conn,
        )
    }
    pub fn from_child<C>(
        child: i32,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::parent_procedure_templates::parent_procedure_templates::child as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::parent_procedure_templates::parent_procedure_templates::child as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            (
                diesel::helper_types::Asc<
                    crate::codegen::diesel_codegen::tables::parent_procedure_templates::parent_procedure_templates::parent,
                >,
                diesel::helper_types::Asc<
                    crate::codegen::diesel_codegen::tables::parent_procedure_templates::parent_procedure_templates::child,
                >,
            ),
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::parent_procedure_templates::parent_procedure_templates::child as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            (
                diesel::helper_types::Asc<
                    crate::codegen::diesel_codegen::tables::parent_procedure_templates::parent_procedure_templates::parent,
                >,
                diesel::helper_types::Asc<
                    crate::codegen::diesel_codegen::tables::parent_procedure_templates::parent_procedure_templates::child,
                >,
            ),
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
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
    pub fn from_parent<C>(
        parent: i32,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::parent_procedure_templates::parent_procedure_templates::parent as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::parent_procedure_templates::parent_procedure_templates::parent as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            (
                diesel::helper_types::Asc<
                    crate::codegen::diesel_codegen::tables::parent_procedure_templates::parent_procedure_templates::parent,
                >,
                diesel::helper_types::Asc<
                    crate::codegen::diesel_codegen::tables::parent_procedure_templates::parent_procedure_templates::child,
                >,
            ),
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::parent_procedure_templates::parent_procedure_templates::parent as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            (
                diesel::helper_types::Asc<
                    crate::codegen::diesel_codegen::tables::parent_procedure_templates::parent_procedure_templates::parent,
                >,
                diesel::helper_types::Asc<
                    crate::codegen::diesel_codegen::tables::parent_procedure_templates::parent_procedure_templates::child,
                >,
            ),
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
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
    pub fn from_created_at<C>(
        created_at: ::rosetta_timestamp::TimestampUTC,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::parent_procedure_templates::parent_procedure_templates::created_at as diesel::expression_methods::EqAll<
                ::rosetta_timestamp::TimestampUTC,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::parent_procedure_templates::parent_procedure_templates::created_at as diesel::expression_methods::EqAll<
                ::rosetta_timestamp::TimestampUTC,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            (
                diesel::helper_types::Asc<
                    crate::codegen::diesel_codegen::tables::parent_procedure_templates::parent_procedure_templates::parent,
                >,
                diesel::helper_types::Asc<
                    crate::codegen::diesel_codegen::tables::parent_procedure_templates::parent_procedure_templates::child,
                >,
            ),
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::parent_procedure_templates::parent_procedure_templates::created_at as diesel::expression_methods::EqAll<
                ::rosetta_timestamp::TimestampUTC,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            (
                diesel::helper_types::Asc<
                    crate::codegen::diesel_codegen::tables::parent_procedure_templates::parent_procedure_templates::parent,
                >,
                diesel::helper_types::Asc<
                    crate::codegen::diesel_codegen::tables::parent_procedure_templates::parent_procedure_templates::child,
                >,
            ),
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
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
