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
        crate::codegen::structs_codegen::tables::samples::Sample,
        foreign_key = sample
    )
)]
#[diesel(
    belongs_to(
        crate::codegen::structs_codegen::tables::sample_sources::SampleSource,
        foreign_key = sample_source
    )
)]
#[diesel(primary_key(procedure))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::harvesting_procedures::harvesting_procedures
)]
pub struct HarvestingProcedure {
    pub procedure: ::rosetta_uuid::Uuid,
    pub procedure_template: i32,
    pub sample_source: ::rosetta_uuid::Uuid,
    pub procedure_template_sample_source_model: i32,
    pub procedure_sample_source: ::rosetta_uuid::Uuid,
    pub sample: ::rosetta_uuid::Uuid,
    pub procedure_template_sample_model: i32,
    pub procedure_sample: ::rosetta_uuid::Uuid,
}
impl web_common_traits::prelude::TableName for HarvestingProcedure {
    const TABLE_NAME: &'static str = "harvesting_procedures";
}
impl<'a> From<&'a HarvestingProcedure>
    for web_common_traits::database::IdOrBuilder<
        ::rosetta_uuid::Uuid,
        crate::codegen::structs_codegen::tables::insertables::InsertableHarvestingProcedureBuilder,
    >
{
    fn from(value: &'a HarvestingProcedure) -> Self {
        web_common_traits::database::IdOrBuilder::Id(value.procedure)
    }
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::procedures::Procedure,
    > for HarvestingProcedure
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a ::rosetta_uuid::Uuid>,
{
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::harvesting_procedures::HarvestingProcedure,
    > for HarvestingProcedure
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a ::rosetta_uuid::Uuid>,
{
}
impl diesel::Identifiable for HarvestingProcedure {
    type Id = ::rosetta_uuid::Uuid;
    fn id(self) -> Self::Id {
        self.procedure
    }
}
impl web_common_traits::database::PrimaryKeyLike for HarvestingProcedure {
    type PrimaryKey = ::rosetta_uuid::Uuid;
    fn primary_key(&self) -> Self::PrimaryKey {
        self.procedure
    }
}
impl HarvestingProcedure {
    pub fn procedure_sample<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::read(
            self.procedure_sample,
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn harvesting_procedures_procedure_sample_procedure_template_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
        diesel::result::Error,
    > {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_sample)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::procedure_template_asset_model
                            .eq(&self.procedure_template_sample_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn harvesting_procedures_procedure_sample_sample_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
        diesel::result::Error,
    > {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_sample)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset
                            .eq(&self.sample),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
    }
    pub fn procedure_sample_source<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::read(
            self.procedure_sample_source,
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn harvesting_procedures_procedure_sample_source_procedure_te_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
        diesel::result::Error,
    > {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_sample_source)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::procedure_template_asset_model
                            .eq(&self.procedure_template_sample_source_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn harvesting_procedures_procedure_sample_source_sample_sourc_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
        diesel::result::Error,
    > {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_sample_source)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset
                            .eq(&self.sample_source),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
    }
    pub fn procedure_template<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::harvesting_procedure_templates::HarvestingProcedureTemplate,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::harvesting_procedure_templates::HarvestingProcedureTemplate: web_common_traits::database::Read<
            C,
        >,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::harvesting_procedure_templates::HarvestingProcedureTemplate::read(
            self.procedure_template,
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn harvesting_procedures_procedure_template_procedure_templa_fkey1(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::harvesting_procedure_templates::HarvestingProcedureTemplate,
        diesel::result::Error,
    >{
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::harvesting_procedure_templates::HarvestingProcedureTemplate::table()
            .filter(
                crate::codegen::diesel_codegen::tables::harvesting_procedure_templates::harvesting_procedure_templates::dsl::procedure_template
                    .eq(&self.procedure_template_id)
                    .and(
                        crate::codegen::diesel_codegen::tables::harvesting_procedure_templates::harvesting_procedure_templates::dsl::procedure_template_sample_model
                            .eq(&self.procedure_template_sample_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::harvesting_procedure_templates::HarvestingProcedureTemplate,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn harvesting_procedures_procedure_template_procedure_templat_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::harvesting_procedure_templates::HarvestingProcedureTemplate,
        diesel::result::Error,
    >{
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::harvesting_procedure_templates::HarvestingProcedureTemplate::table()
            .filter(
                crate::codegen::diesel_codegen::tables::harvesting_procedure_templates::harvesting_procedure_templates::dsl::procedure_template
                    .eq(&self.procedure_template_id)
                    .and(
                        crate::codegen::diesel_codegen::tables::harvesting_procedure_templates::harvesting_procedure_templates::dsl::procedure_template_sample_source_model
                            .eq(&self.procedure_template_sample_source_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::harvesting_procedure_templates::HarvestingProcedureTemplate,
            >(conn)
    }
    pub fn procedure_template_sample_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel: web_common_traits::database::Read<
            C,
        >,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel::read(
            self.procedure_template_sample_model,
            conn,
        )
    }
    pub fn procedure_template_sample_source_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel: web_common_traits::database::Read<
            C,
        >,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel::read(
            self.procedure_template_sample_source_model,
            conn,
        )
    }
    pub fn sample<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::codegen::structs_codegen::tables::samples::Sample, diesel::result::Error>
    where
        crate::codegen::structs_codegen::tables::samples::Sample:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::samples::Sample::read(self.sample, conn)
    }
    pub fn sample_source<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::sample_sources::SampleSource,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::sample_sources::SampleSource:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::sample_sources::SampleSource::read(
            self.sample_source,
            conn,
        )
    }
    pub fn from_procedure_sample<C>(
        procedure_sample: ::rosetta_uuid::Uuid,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::harvesting_procedures::harvesting_procedures::procedure_sample as diesel::expression_methods::EqAll<
                ::rosetta_uuid::Uuid,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::harvesting_procedures::harvesting_procedures::procedure_sample as diesel::expression_methods::EqAll<
                ::rosetta_uuid::Uuid,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::harvesting_procedures::harvesting_procedures::procedure,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::harvesting_procedures::harvesting_procedures::procedure_sample as diesel::expression_methods::EqAll<
                ::rosetta_uuid::Uuid,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::harvesting_procedures::harvesting_procedures::procedure,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::harvesting_procedures::harvesting_procedures;
        Self::table()
            .filter(harvesting_procedures::procedure_sample.eq(procedure_sample))
            .order_by(harvesting_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_sample_and_procedure_template_sample_model(
        procedure_sample: ::rosetta_uuid::Uuid,
        procedure_template_sample_model: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::harvesting_procedures::harvesting_procedures;
        Self::table()
            .filter(
                harvesting_procedures::procedure_sample.eq(procedure_sample).and(
                    harvesting_procedures::procedure_template_sample_model
                        .eq(procedure_template_sample_model),
                ),
            )
            .order_by(harvesting_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_sample_and_sample(
        procedure_sample: ::rosetta_uuid::Uuid,
        sample: ::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::harvesting_procedures::harvesting_procedures;
        Self::table()
            .filter(
                harvesting_procedures::procedure_sample
                    .eq(procedure_sample)
                    .and(harvesting_procedures::sample.eq(sample)),
            )
            .order_by(harvesting_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    pub fn from_procedure_sample_source<C>(
        procedure_sample_source: ::rosetta_uuid::Uuid,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::harvesting_procedures::harvesting_procedures::procedure_sample_source as diesel::expression_methods::EqAll<
                ::rosetta_uuid::Uuid,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::harvesting_procedures::harvesting_procedures::procedure_sample_source as diesel::expression_methods::EqAll<
                ::rosetta_uuid::Uuid,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::harvesting_procedures::harvesting_procedures::procedure,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::harvesting_procedures::harvesting_procedures::procedure_sample_source as diesel::expression_methods::EqAll<
                ::rosetta_uuid::Uuid,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::harvesting_procedures::harvesting_procedures::procedure,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::harvesting_procedures::harvesting_procedures;
        Self::table()
            .filter(harvesting_procedures::procedure_sample_source.eq(procedure_sample_source))
            .order_by(harvesting_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_sample_source_and_procedure_template_sample_source_model(
        procedure_sample_source: ::rosetta_uuid::Uuid,
        procedure_template_sample_source_model: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::harvesting_procedures::harvesting_procedures;
        Self::table()
            .filter(
                harvesting_procedures::procedure_sample_source.eq(procedure_sample_source).and(
                    harvesting_procedures::procedure_template_sample_source_model
                        .eq(procedure_template_sample_source_model),
                ),
            )
            .order_by(harvesting_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_sample_source_and_sample_source(
        procedure_sample_source: ::rosetta_uuid::Uuid,
        sample_source: ::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::harvesting_procedures::harvesting_procedures;
        Self::table()
            .filter(
                harvesting_procedures::procedure_sample_source
                    .eq(procedure_sample_source)
                    .and(harvesting_procedures::sample_source.eq(sample_source)),
            )
            .order_by(harvesting_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    pub fn from_procedure_template<C>(
        procedure_template: i32,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::harvesting_procedures::harvesting_procedures::procedure_template as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::harvesting_procedures::harvesting_procedures::procedure_template as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::harvesting_procedures::harvesting_procedures::procedure,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::harvesting_procedures::harvesting_procedures::procedure_template as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::harvesting_procedures::harvesting_procedures::procedure,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::harvesting_procedures::harvesting_procedures;
        Self::table()
            .filter(harvesting_procedures::procedure_template.eq(procedure_template_id))
            .order_by(harvesting_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_and_procedure_template_sample_model(
        procedure_template: i32,
        procedure_template_sample_model: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::harvesting_procedures::harvesting_procedures;
        Self::table()
            .filter(
                harvesting_procedures::procedure_template.eq(procedure_template_id).and(
                    harvesting_procedures::procedure_template_sample_model
                        .eq(procedure_template_sample_model),
                ),
            )
            .order_by(harvesting_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_and_procedure_template_sample_source_model(
        procedure_template: i32,
        procedure_template_sample_source_model: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::harvesting_procedures::harvesting_procedures;
        Self::table()
            .filter(
                harvesting_procedures::procedure_template.eq(procedure_template_id).and(
                    harvesting_procedures::procedure_template_sample_source_model
                        .eq(procedure_template_sample_source_model),
                ),
            )
            .order_by(harvesting_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    pub fn from_procedure_template_sample_model<C>(
        procedure_template_sample_model: i32,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::harvesting_procedures::harvesting_procedures::procedure_template_sample_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::harvesting_procedures::harvesting_procedures::procedure_template_sample_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::harvesting_procedures::harvesting_procedures::procedure,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::harvesting_procedures::harvesting_procedures::procedure_template_sample_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::harvesting_procedures::harvesting_procedures::procedure,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::harvesting_procedures::harvesting_procedures;
        Self::table()
            .filter(
                harvesting_procedures::procedure_template_sample_model
                    .eq(procedure_template_sample_model),
            )
            .order_by(harvesting_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    pub fn from_procedure_template_sample_source_model<C>(
        procedure_template_sample_source_model: i32,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::harvesting_procedures::harvesting_procedures::procedure_template_sample_source_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::harvesting_procedures::harvesting_procedures::procedure_template_sample_source_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::harvesting_procedures::harvesting_procedures::procedure,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::harvesting_procedures::harvesting_procedures::procedure_template_sample_source_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::harvesting_procedures::harvesting_procedures::procedure,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::harvesting_procedures::harvesting_procedures;
        Self::table()
            .filter(
                harvesting_procedures::procedure_template_sample_source_model
                    .eq(procedure_template_sample_source_model),
            )
            .order_by(harvesting_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_parent_procedure(
        parent_procedure: ::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            harvesting_procedures::harvesting_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(harvesting_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::parent_procedure.eq(parent_procedure))
            .order_by(harvesting_procedures::procedure.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_parent_procedure_template(
        parent_procedure_template: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            harvesting_procedures::harvesting_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(harvesting_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::parent_procedure_template.eq(parent_procedure_template_id))
            .order_by(harvesting_procedures::procedure.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_predecessor_procedure(
        predecessor_procedure: ::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            harvesting_procedures::harvesting_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(harvesting_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::predecessor_procedure.eq(predecessor_procedure))
            .order_by(harvesting_procedures::procedure.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_predecessor_procedure_template(
        predecessor_procedure_template: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            harvesting_procedures::harvesting_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(harvesting_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(
                procedures::predecessor_procedure_template.eq(predecessor_procedure_template_id),
            )
            .order_by(harvesting_procedures::procedure.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_created_by(
        created_by: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            harvesting_procedures::harvesting_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(harvesting_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::created_by.eq(created_by))
            .order_by(harvesting_procedures::procedure.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_updated_by(
        updated_by: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            harvesting_procedures::harvesting_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(harvesting_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::updated_by.eq(updated_by))
            .order_by(harvesting_procedures::procedure.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
}
impl AsRef<HarvestingProcedure> for HarvestingProcedure {
    fn as_ref(&self) -> &HarvestingProcedure {
        self
    }
}
