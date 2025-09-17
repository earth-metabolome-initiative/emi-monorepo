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
#[diesel(primary_key(parent, predecessor, successor))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::next_procedure_templates::next_procedure_templates
)]
pub struct NextProcedureTemplate {
    pub parent: i32,
    pub predecessor: i32,
    pub successor: i32,
    pub created_by: i32,
    pub created_at: ::rosetta_timestamp::TimestampUTC,
}
impl web_common_traits::prelude::TableName for NextProcedureTemplate {
    const TABLE_NAME: &'static str = "next_procedure_templates";
}
impl<'a> From<&'a NextProcedureTemplate>
for web_common_traits::database::IdOrBuilder<
    (i32, i32, i32),
    crate::codegen::structs_codegen::tables::insertables::InsertableNextProcedureTemplateBuilder,
> {
    fn from(value: &'a NextProcedureTemplate) -> Self {
        web_common_traits::database::IdOrBuilder::Id((
            value.parent,
            value.predecessor,
            value.successor,
        ))
    }
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::next_procedure_templates::NextProcedureTemplate,
    > for NextProcedureTemplate
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a (i32, i32, i32)>,
{
}
impl diesel::Identifiable for NextProcedureTemplate {
    type Id = (i32, i32, i32);
    fn id(self) -> Self::Id {
        (self.parent, self.predecessor, self.successor)
    }
}
impl web_common_traits::database::PrimaryKeyLike for NextProcedureTemplate {
    type PrimaryKey = (i32, i32, i32);
    fn primary_key(&self) -> Self::PrimaryKey {
        (self.parent, self.predecessor, self.successor)
    }
}
impl NextProcedureTemplate {
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
    pub fn next_procedure_templates_parent_predecessor_fkey<
        C: diesel::connection::LoadConnection,
    >(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate: web_common_traits::database::Read<
            C,
        >,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate::read(
            (self.parent, self.predecessor),
            conn,
        )
    }
    pub fn next_procedure_templates_parent_successor_fkey<
        C: diesel::connection::LoadConnection,
    >(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate: web_common_traits::database::Read<
            C,
        >,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate::read(
            (self.parent, self.successor),
            conn,
        )
    }
    pub fn predecessor<C: diesel::connection::LoadConnection>(
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
            self.predecessor,
            conn,
        )
    }
    pub fn successor<C: diesel::connection::LoadConnection>(
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
            self.successor,
            conn,
        )
    }
    pub fn from_parent<C>(
        parent: i32,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::next_procedure_templates::next_procedure_templates::parent as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::next_procedure_templates::next_procedure_templates::parent as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            (
                diesel::helper_types::Asc<
                    crate::codegen::diesel_codegen::tables::next_procedure_templates::next_procedure_templates::parent,
                >,
                diesel::helper_types::Asc<
                    crate::codegen::diesel_codegen::tables::next_procedure_templates::next_procedure_templates::predecessor,
                >,
                diesel::helper_types::Asc<
                    crate::codegen::diesel_codegen::tables::next_procedure_templates::next_procedure_templates::successor,
                >,
            ),
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::next_procedure_templates::next_procedure_templates::parent as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            (
                diesel::helper_types::Asc<
                    crate::codegen::diesel_codegen::tables::next_procedure_templates::next_procedure_templates::parent,
                >,
                diesel::helper_types::Asc<
                    crate::codegen::diesel_codegen::tables::next_procedure_templates::next_procedure_templates::predecessor,
                >,
                diesel::helper_types::Asc<
                    crate::codegen::diesel_codegen::tables::next_procedure_templates::next_procedure_templates::successor,
                >,
            ),
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::next_procedure_templates::next_procedure_templates;
        Self::table()
            .filter(next_procedure_templates::parent.eq(parent))
            .order_by((
                next_procedure_templates::parent.asc(),
                next_procedure_templates::predecessor.asc(),
                next_procedure_templates::successor.asc(),
            ))
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_parent_and_predecessor(
        parent: i32,
        predecessor: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::next_procedure_templates::next_procedure_templates;
        Self::table()
            .filter(
                next_procedure_templates::parent
                    .eq(parent)
                    .and(next_procedure_templates::predecessor.eq(predecessor)),
            )
            .order_by((
                next_procedure_templates::parent.asc(),
                next_procedure_templates::predecessor.asc(),
                next_procedure_templates::successor.asc(),
            ))
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_parent_and_successor(
        parent: i32,
        successor: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::next_procedure_templates::next_procedure_templates;
        Self::table()
            .filter(
                next_procedure_templates::parent
                    .eq(parent)
                    .and(next_procedure_templates::successor.eq(successor)),
            )
            .order_by((
                next_procedure_templates::parent.asc(),
                next_procedure_templates::predecessor.asc(),
                next_procedure_templates::successor.asc(),
            ))
            .load::<Self>(conn)
    }
    pub fn from_predecessor<C>(
        predecessor: i32,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::next_procedure_templates::next_procedure_templates::predecessor as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::next_procedure_templates::next_procedure_templates::predecessor as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            (
                diesel::helper_types::Asc<
                    crate::codegen::diesel_codegen::tables::next_procedure_templates::next_procedure_templates::parent,
                >,
                diesel::helper_types::Asc<
                    crate::codegen::diesel_codegen::tables::next_procedure_templates::next_procedure_templates::predecessor,
                >,
                diesel::helper_types::Asc<
                    crate::codegen::diesel_codegen::tables::next_procedure_templates::next_procedure_templates::successor,
                >,
            ),
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::next_procedure_templates::next_procedure_templates::predecessor as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            (
                diesel::helper_types::Asc<
                    crate::codegen::diesel_codegen::tables::next_procedure_templates::next_procedure_templates::parent,
                >,
                diesel::helper_types::Asc<
                    crate::codegen::diesel_codegen::tables::next_procedure_templates::next_procedure_templates::predecessor,
                >,
                diesel::helper_types::Asc<
                    crate::codegen::diesel_codegen::tables::next_procedure_templates::next_procedure_templates::successor,
                >,
            ),
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::next_procedure_templates::next_procedure_templates;
        Self::table()
            .filter(next_procedure_templates::predecessor.eq(predecessor))
            .order_by((
                next_procedure_templates::parent.asc(),
                next_procedure_templates::predecessor.asc(),
                next_procedure_templates::successor.asc(),
            ))
            .load::<Self>(conn)
    }
    pub fn from_successor<C>(
        successor: i32,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::next_procedure_templates::next_procedure_templates::successor as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::next_procedure_templates::next_procedure_templates::successor as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            (
                diesel::helper_types::Asc<
                    crate::codegen::diesel_codegen::tables::next_procedure_templates::next_procedure_templates::parent,
                >,
                diesel::helper_types::Asc<
                    crate::codegen::diesel_codegen::tables::next_procedure_templates::next_procedure_templates::predecessor,
                >,
                diesel::helper_types::Asc<
                    crate::codegen::diesel_codegen::tables::next_procedure_templates::next_procedure_templates::successor,
                >,
            ),
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::next_procedure_templates::next_procedure_templates::successor as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            (
                diesel::helper_types::Asc<
                    crate::codegen::diesel_codegen::tables::next_procedure_templates::next_procedure_templates::parent,
                >,
                diesel::helper_types::Asc<
                    crate::codegen::diesel_codegen::tables::next_procedure_templates::next_procedure_templates::predecessor,
                >,
                diesel::helper_types::Asc<
                    crate::codegen::diesel_codegen::tables::next_procedure_templates::next_procedure_templates::successor,
                >,
            ),
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::next_procedure_templates::next_procedure_templates;
        Self::table()
            .filter(next_procedure_templates::successor.eq(successor))
            .order_by((
                next_procedure_templates::parent.asc(),
                next_procedure_templates::predecessor.asc(),
                next_procedure_templates::successor.asc(),
            ))
            .load::<Self>(conn)
    }
}
impl AsRef<NextProcedureTemplate> for NextProcedureTemplate {
    fn as_ref(&self) -> &NextProcedureTemplate {
        self
    }
}
