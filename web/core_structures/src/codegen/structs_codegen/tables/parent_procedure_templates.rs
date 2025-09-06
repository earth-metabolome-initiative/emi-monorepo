#[derive(Debug, Clone, PartialEq, Copy, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    diesel::Selectable,
    diesel::Insertable,
    diesel::AsChangeset,
    diesel::Queryable,
    diesel::Identifiable,
)]
#[cfg_attr(feature = "yew", derive(yew::prelude::Properties))]
#[diesel(primary_key(parent_procedure_template, child_procedure_template))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::parent_procedure_templates::parent_procedure_templates
)]
pub struct ParentProcedureTemplate {
    pub parent_procedure_template: i32,
    pub child_procedure_template: i32,
    pub snoozable: bool,
    pub copiable: bool,
    pub repeatable: bool,
    pub skippable: bool,
    pub created_by: i32,
    pub created_at: ::rosetta_timestamp::TimestampUTC,
}
impl web_common_traits::prelude::TableName for ParentProcedureTemplate {
    const TABLE_NAME: &'static str = "parent_procedure_templates";
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
        (self.parent_procedure_template, self.child_procedure_template)
    }
}
impl ParentProcedureTemplate {
    pub fn child_procedure_template<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate::table(),
                self.child_procedure_template,
            ),
            conn,
        )
    }
    pub fn created_by<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::users::User,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::users::User: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::users::User as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::users::User as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::users::User as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::users::User as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::users::User as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::users::User as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::users::User,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::users::User::table(),
                self.created_by,
            ),
            conn,
        )
    }
    pub fn parent_procedure_template<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate::table(),
                self.parent_procedure_template,
            ),
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn from_parent_procedure_template(
        parent_procedure_template: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::parent_procedure_templates::parent_procedure_templates;
        Self::table()
            .filter(
                parent_procedure_templates::parent_procedure_template.eq(parent_procedure_template),
            )
            .order_by((
                parent_procedure_templates::parent_procedure_template.asc(),
                parent_procedure_templates::child_procedure_template.asc(),
            ))
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_child_procedure_template(
        child_procedure_template: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::parent_procedure_templates::parent_procedure_templates;
        Self::table()
            .filter(
                parent_procedure_templates::child_procedure_template.eq(child_procedure_template),
            )
            .order_by((
                parent_procedure_templates::parent_procedure_template.asc(),
                parent_procedure_templates::child_procedure_template.asc(),
            ))
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_snoozable(
        snoozable: &bool,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::parent_procedure_templates::parent_procedure_templates;
        Self::table()
            .filter(parent_procedure_templates::snoozable.eq(snoozable))
            .order_by((
                parent_procedure_templates::parent_procedure_template.asc(),
                parent_procedure_templates::child_procedure_template.asc(),
            ))
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_copiable(
        copiable: &bool,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::parent_procedure_templates::parent_procedure_templates;
        Self::table()
            .filter(parent_procedure_templates::copiable.eq(copiable))
            .order_by((
                parent_procedure_templates::parent_procedure_template.asc(),
                parent_procedure_templates::child_procedure_template.asc(),
            ))
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_repeatable(
        repeatable: &bool,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::parent_procedure_templates::parent_procedure_templates;
        Self::table()
            .filter(parent_procedure_templates::repeatable.eq(repeatable))
            .order_by((
                parent_procedure_templates::parent_procedure_template.asc(),
                parent_procedure_templates::child_procedure_template.asc(),
            ))
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_skippable(
        skippable: &bool,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::parent_procedure_templates::parent_procedure_templates;
        Self::table()
            .filter(parent_procedure_templates::skippable.eq(skippable))
            .order_by((
                parent_procedure_templates::parent_procedure_template.asc(),
                parent_procedure_templates::child_procedure_template.asc(),
            ))
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_created_by(
        created_by: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::parent_procedure_templates::parent_procedure_templates;
        Self::table()
            .filter(parent_procedure_templates::created_by.eq(created_by))
            .order_by((
                parent_procedure_templates::parent_procedure_template.asc(),
                parent_procedure_templates::child_procedure_template.asc(),
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
                parent_procedure_templates::parent_procedure_template.asc(),
                parent_procedure_templates::child_procedure_template.asc(),
            ))
            .load::<Self>(conn)
    }
}
impl AsRef<ParentProcedureTemplate> for ParentProcedureTemplate {
    fn as_ref(&self) -> &ParentProcedureTemplate {
        self
    }
}
