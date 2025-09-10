#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    diesel::Selectable,
    diesel::Insertable,
    diesel::AsChangeset,
    diesel::Queryable,
    diesel::Identifiable,
)]
#[cfg_attr(feature = "yew", derive(yew::prelude::Properties))]
#[diesel(primary_key(procedure))]
#[diesel(table_name = crate::codegen::diesel_codegen::tables::procedures::procedures)]
pub struct Procedure {
    pub procedure: ::rosetta_uuid::Uuid,
    pub procedure_template: i32,
    pub parent_procedure: Option<::rosetta_uuid::Uuid>,
    pub parent_procedure_template: Option<i32>,
    pub predecessor_procedure: Option<::rosetta_uuid::Uuid>,
    pub predecessor_procedure_template: Option<i32>,
    pub most_concrete_table: String,
    pub created_by: i32,
    pub created_at: ::rosetta_timestamp::TimestampUTC,
    pub updated_by: i32,
    pub updated_at: ::rosetta_timestamp::TimestampUTC,
    pub number_of_completed_subprocedures: i16,
}
impl web_common_traits::prelude::TableName for Procedure {
    const TABLE_NAME: &'static str = "procedures";
}
impl<'a> From<&'a Procedure>
    for web_common_traits::database::IdOrBuilder<
        ::rosetta_uuid::Uuid,
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureBuilder,
    >
{
    fn from(value: &'a Procedure) -> Self {
        web_common_traits::database::IdOrBuilder::Id(value.procedure)
    }
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::procedures::Procedure,
    > for Procedure
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a ::rosetta_uuid::Uuid>,
{
}
impl diesel::Identifiable for Procedure {
    type Id = ::rosetta_uuid::Uuid;
    fn id(self) -> Self::Id {
        self.procedure
    }
}
impl Procedure {
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
    pub fn parent_procedure<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::procedures::Procedure>,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedures::Procedure:
            web_common_traits::database::Read<C>,
    {
        use diesel::OptionalExtension;
        use web_common_traits::database::Read;
        let Some(parent_procedure) = self.parent_procedure else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::procedures::Procedure::read(parent_procedure, conn)
            .optional()
    }
    #[cfg(feature = "postgres")]
    pub fn procedures_parent_procedure_parent_procedure_template_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::procedures::Procedure>,
        diesel::result::Error,
    > {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl,
            associations::HasTable,
        };
        let Some(parent_procedure) = self.parent_procedure else {
            return Ok(None);
        };
        let Some(parent_procedure_template) = self.parent_procedure_template else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::procedures::Procedure::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedures::procedures::dsl::procedure
                    .eq(parent_procedure)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedures::procedures::dsl::procedure_template
                            .eq(parent_procedure_template),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedures::Procedure,
            >(conn)
            .optional()
    }
    pub fn parent_procedure_template<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate>,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate:
            web_common_traits::database::Read<C>,
    {
        use diesel::OptionalExtension;
        use web_common_traits::database::Read;
        let Some(parent_procedure_template) = self.parent_procedure_template else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate::read(
            parent_procedure_template,
            conn,
        )
        .optional()
    }
    pub fn procedures_parent_procedure_template_predecessor_procedure_fkey<
        C: diesel::connection::LoadConnection,
    >(
        &self,
        conn: &mut C,
    ) -> Result<
        Option<
            crate::codegen::structs_codegen::tables::next_procedure_templates::NextProcedureTemplate,
        >,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::next_procedure_templates::NextProcedureTemplate: web_common_traits::database::Read<
            C,
        >,
    {
        use diesel::OptionalExtension;
        use web_common_traits::database::Read;
        let Some(parent_procedure_template) = self.parent_procedure_template else {
            return Ok(None);
        };
        let Some(predecessor_procedure_template) = self.predecessor_procedure_template else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::next_procedure_templates::NextProcedureTemplate::read(
                (
                    parent_procedure_template,
                    predecessor_procedure_template,
                    self.procedure_template,
                ),
                conn,
            )
            .optional()
    }
    pub fn procedures_parent_procedure_template_procedure_template_fkey<
        C: diesel::connection::LoadConnection,
    >(
        &self,
        conn: &mut C,
    ) -> Result<
        Option<
            crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate,
        >,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate: web_common_traits::database::Read<
            C,
        >,
    {
        use diesel::OptionalExtension;
        use web_common_traits::database::Read;
        let Some(parent_procedure_template) = self.parent_procedure_template else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate::read(
                (parent_procedure_template, self.procedure_template),
                conn,
            )
            .optional()
    }
    pub fn predecessor_procedure<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::procedures::Procedure>,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedures::Procedure:
            web_common_traits::database::Read<C>,
    {
        use diesel::OptionalExtension;
        use web_common_traits::database::Read;
        let Some(predecessor_procedure) = self.predecessor_procedure else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::procedures::Procedure::read(
            predecessor_procedure,
            conn,
        )
        .optional()
    }
    #[cfg(feature = "postgres")]
    pub fn procedures_predecessor_procedure_predecessor_procedure_tem_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::procedures::Procedure>,
        diesel::result::Error,
    > {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl,
            associations::HasTable,
        };
        let Some(predecessor_procedure) = self.predecessor_procedure else {
            return Ok(None);
        };
        let Some(predecessor_procedure_template) = self.predecessor_procedure_template else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::procedures::Procedure::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedures::procedures::dsl::procedure
                    .eq(predecessor_procedure)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedures::procedures::dsl::procedure_template
                            .eq(predecessor_procedure_template),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedures::Procedure,
            >(conn)
            .optional()
    }
    pub fn predecessor_procedure_template<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate>,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate:
            web_common_traits::database::Read<C>,
    {
        use diesel::OptionalExtension;
        use web_common_traits::database::Read;
        let Some(predecessor_procedure_template) = self.predecessor_procedure_template else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate::read(
            predecessor_procedure_template,
            conn,
        )
        .optional()
    }
    pub fn procedure_template<C: diesel::connection::LoadConnection>(
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
            self.procedure_template,
            conn,
        )
    }
    pub fn updated_by<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::codegen::structs_codegen::tables::users::User, diesel::result::Error>
    where
        crate::codegen::structs_codegen::tables::users::User: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::users::User::read(self.updated_by, conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_and_procedure(
        procedure_template: i32,
        procedure: ::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::procedures::procedures;
        Self::table()
            .filter(
                procedures::procedure_template
                    .eq(procedure_template)
                    .and(procedures::procedure.eq(procedure)),
            )
            .order_by(procedures::procedure.asc())
            .first::<Self>(conn)
    }
    pub fn from_created_by<C>(
        created_by: i32,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::procedures::procedures::created_by as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::procedures::procedures::created_by as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::procedures::procedures::procedure,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::procedures::procedures::created_by as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::procedures::procedures::procedure,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::procedures::procedures;
        Self::table()
            .filter(procedures::created_by.eq(created_by))
            .order_by(procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_parent_procedure(
        parent_procedure: ::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::procedures::procedures;
        Self::table()
            .filter(procedures::parent_procedure.eq(parent_procedure))
            .order_by(procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_parent_procedure_and_parent_procedure_template(
        parent_procedure: ::rosetta_uuid::Uuid,
        parent_procedure_template: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::procedures::procedures;
        Self::table()
            .filter(
                procedures::parent_procedure
                    .eq(parent_procedure)
                    .and(procedures::parent_procedure_template.eq(parent_procedure_template)),
            )
            .order_by(procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_parent_procedure_template(
        parent_procedure_template: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::procedures::procedures;
        Self::table()
            .filter(procedures::parent_procedure_template.eq(parent_procedure_template))
            .order_by(procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_parent_procedure_template_and_predecessor_procedure_template_and_procedure_template(
        parent_procedure_template: i32,
        predecessor_procedure_template: i32,
        procedure_template: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::procedures::procedures;
        Self::table()
            .filter(
                procedures::parent_procedure_template
                    .eq(parent_procedure_template)
                    .and(
                        procedures::predecessor_procedure_template
                            .eq(predecessor_procedure_template),
                    )
                    .and(procedures::procedure_template.eq(procedure_template)),
            )
            .order_by(procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_parent_procedure_template_and_procedure_template(
        parent_procedure_template: i32,
        procedure_template: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::procedures::procedures;
        Self::table()
            .filter(
                procedures::parent_procedure_template
                    .eq(parent_procedure_template)
                    .and(procedures::procedure_template.eq(procedure_template)),
            )
            .order_by(procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_predecessor_procedure(
        predecessor_procedure: ::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::procedures::procedures;
        Self::table()
            .filter(procedures::predecessor_procedure.eq(predecessor_procedure))
            .order_by(procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_predecessor_procedure_and_predecessor_procedure_template(
        predecessor_procedure: ::rosetta_uuid::Uuid,
        predecessor_procedure_template: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::procedures::procedures;
        Self::table()
            .filter(
                procedures::predecessor_procedure.eq(predecessor_procedure).and(
                    procedures::predecessor_procedure_template.eq(predecessor_procedure_template),
                ),
            )
            .order_by(procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_predecessor_procedure_template(
        predecessor_procedure_template: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::procedures::procedures;
        Self::table()
            .filter(procedures::predecessor_procedure_template.eq(predecessor_procedure_template))
            .order_by(procedures::procedure.asc())
            .load::<Self>(conn)
    }
    pub fn from_procedure_template<C>(
        procedure_template: i32,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::procedures::procedures::procedure_template as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::procedures::procedures::procedure_template as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::procedures::procedures::procedure,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::procedures::procedures::procedure_template as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::procedures::procedures::procedure,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::procedures::procedures;
        Self::table()
            .filter(procedures::procedure_template.eq(procedure_template))
            .order_by(procedures::procedure.asc())
            .load::<Self>(conn)
    }
    pub fn from_updated_by<C>(
        updated_by: i32,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::procedures::procedures::updated_by as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::procedures::procedures::updated_by as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::procedures::procedures::procedure,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::procedures::procedures::updated_by as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::procedures::procedures::procedure,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::procedures::procedures;
        Self::table()
            .filter(procedures::updated_by.eq(updated_by))
            .order_by(procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_most_concrete_table(
        most_concrete_table: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::procedures::procedures;
        Self::table()
            .filter(procedures::most_concrete_table.eq(most_concrete_table))
            .order_by(procedures::procedure.asc())
            .load::<Self>(conn)
    }
    pub fn from_created_at<C>(
        created_at: ::rosetta_timestamp::TimestampUTC,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::procedures::procedures::created_at as diesel::expression_methods::EqAll<
                ::rosetta_timestamp::TimestampUTC,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::procedures::procedures::created_at as diesel::expression_methods::EqAll<
                ::rosetta_timestamp::TimestampUTC,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::procedures::procedures::procedure,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::procedures::procedures::created_at as diesel::expression_methods::EqAll<
                ::rosetta_timestamp::TimestampUTC,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::procedures::procedures::procedure,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::procedures::procedures;
        Self::table()
            .filter(procedures::created_at.eq(created_at))
            .order_by(procedures::procedure.asc())
            .load::<Self>(conn)
    }
    pub fn from_updated_at<C>(
        updated_at: ::rosetta_timestamp::TimestampUTC,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::procedures::procedures::updated_at as diesel::expression_methods::EqAll<
                ::rosetta_timestamp::TimestampUTC,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::procedures::procedures::updated_at as diesel::expression_methods::EqAll<
                ::rosetta_timestamp::TimestampUTC,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::procedures::procedures::procedure,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::procedures::procedures::updated_at as diesel::expression_methods::EqAll<
                ::rosetta_timestamp::TimestampUTC,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::procedures::procedures::procedure,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::procedures::procedures;
        Self::table()
            .filter(procedures::updated_at.eq(updated_at))
            .order_by(procedures::procedure.asc())
            .load::<Self>(conn)
    }
    pub fn from_number_of_completed_subprocedures<C>(
        number_of_completed_subprocedures: i16,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::procedures::procedures::number_of_completed_subprocedures as diesel::expression_methods::EqAll<
                i16,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::procedures::procedures::number_of_completed_subprocedures as diesel::expression_methods::EqAll<
                i16,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::procedures::procedures::procedure,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::procedures::procedures::number_of_completed_subprocedures as diesel::expression_methods::EqAll<
                i16,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::procedures::procedures::procedure,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::procedures::procedures;
        Self::table()
            .filter(
                procedures::number_of_completed_subprocedures.eq(number_of_completed_subprocedures),
            )
            .order_by(procedures::procedure.asc())
            .load::<Self>(conn)
    }
}
impl AsRef<Procedure> for Procedure {
    fn as_ref(&self) -> &Procedure {
        self
    }
}
