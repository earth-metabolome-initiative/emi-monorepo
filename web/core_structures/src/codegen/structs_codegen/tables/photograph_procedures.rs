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
        crate::codegen::structs_codegen::tables::photographs::Photograph,
        foreign_key = photograph
    )
)]
#[diesel(
    belongs_to(
        crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset,
        foreign_key = photographed_asset
    )
)]
#[diesel(
    belongs_to(
        crate::codegen::structs_codegen::tables::cameras::Camera,
        foreign_key = photographed_with
    )
)]
#[diesel(primary_key(procedure))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::photograph_procedures::photograph_procedures
)]
pub struct PhotographProcedure {
    pub procedure: ::rosetta_uuid::Uuid,
    pub procedure_template: i32,
    pub photographed_asset: Option<::rosetta_uuid::Uuid>,
    pub procedure_template_photographed_asset_model: i32,
    pub procedure_photographed_asset: ::rosetta_uuid::Uuid,
    pub photographed_with: Option<::rosetta_uuid::Uuid>,
    pub procedure_template_photographed_with_model: i32,
    pub procedure_photographed_with: ::rosetta_uuid::Uuid,
    pub photograph: ::rosetta_uuid::Uuid,
    pub procedure_template_photograph_model: i32,
    pub procedure_photograph: ::rosetta_uuid::Uuid,
}
impl web_common_traits::prelude::TableName for PhotographProcedure {
    const TABLE_NAME: &'static str = "photograph_procedures";
}
impl<'a> From<&'a PhotographProcedure>
    for web_common_traits::database::IdOrBuilder<
        ::rosetta_uuid::Uuid,
        crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureBuilder,
    >
{
    fn from(value: &'a PhotographProcedure) -> Self {
        web_common_traits::database::IdOrBuilder::Id(value.procedure)
    }
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::procedures::Procedure,
    > for PhotographProcedure
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a ::rosetta_uuid::Uuid>,
{
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::photograph_procedures::PhotographProcedure,
    > for PhotographProcedure
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a ::rosetta_uuid::Uuid>,
{
}
impl diesel::Identifiable for PhotographProcedure {
    type Id = ::rosetta_uuid::Uuid;
    fn id(self) -> Self::Id {
        self.procedure
    }
}
impl web_common_traits::database::PrimaryKeyLike for PhotographProcedure {
    type PrimaryKey = ::rosetta_uuid::Uuid;
    fn primary_key(&self) -> Self::PrimaryKey {
        self.procedure
    }
}
impl PhotographProcedure {
    pub fn photograph<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::photographs::Photograph,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::photographs::Photograph:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::photographs::Photograph::read(
            self.photograph,
            conn,
        )
    }
    pub fn photographed_asset<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset>,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset:
            web_common_traits::database::Read<C>,
    {
        use diesel::OptionalExtension;
        use web_common_traits::database::Read;
        let Some(photographed_asset) = self.photographed_asset else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset::read(
            photographed_asset,
            conn,
        )
        .optional()
    }
    pub fn photographed_with<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::cameras::Camera>,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::cameras::Camera:
            web_common_traits::database::Read<C>,
    {
        use diesel::OptionalExtension;
        use web_common_traits::database::Read;
        let Some(photographed_with) = self.photographed_with else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::cameras::Camera::read(photographed_with, conn)
            .optional()
    }
    pub fn procedure_photograph<C: diesel::connection::LoadConnection>(
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
            self.procedure_photograph,
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn photograph_procedures_procedure_photograph_photograph_fkey(
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
                    .eq(&self.procedure_photograph)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset
                            .eq(&self.photograph),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn photograph_procedures_procedure_photograph_procedure_templ_fkey(
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
                    .eq(&self.procedure_photograph)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::procedure_template_asset_model
                            .eq(&self.procedure_template_photograph_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
    }
    pub fn procedure_photographed_asset<C: diesel::connection::LoadConnection>(
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
            self.procedure_photographed_asset,
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn photograph_procedures_procedure_photographed_asset_photogr_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset>,
        diesel::result::Error,
    > {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl,
            associations::HasTable,
        };
        let Some(photographed_asset) = self.photographed_asset else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_photographed_asset)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset
                            .eq(photographed_asset),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
            .optional()
    }
    #[cfg(feature = "postgres")]
    pub fn photograph_procedures_procedure_photographed_asset_procedu_fkey(
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
                    .eq(&self.procedure_photographed_asset)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::procedure_template_asset_model
                            .eq(&self.procedure_template_photographed_asset_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
    }
    pub fn procedure_photographed_with<C: diesel::connection::LoadConnection>(
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
            self.procedure_photographed_with,
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn photograph_procedures_procedure_photographed_with_photogra_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset>,
        diesel::result::Error,
    > {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl,
            associations::HasTable,
        };
        let Some(photographed_with) = self.photographed_with else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_photographed_with)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset
                            .eq(photographed_with),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
            .optional()
    }
    #[cfg(feature = "postgres")]
    pub fn photograph_procedures_procedure_photographed_with_procedur_fkey(
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
                    .eq(&self.procedure_photographed_with)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::procedure_template_asset_model
                            .eq(&self.procedure_template_photographed_with_model),
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
        crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate: web_common_traits::database::Read<
            C,
        >,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate::read(
            self.procedure_template,
            conn,
        )
    }
    pub fn procedure_template_photograph_model<C: diesel::connection::LoadConnection>(
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
            self.procedure_template_photograph_model,
            conn,
        )
    }
    pub fn procedure_template_photographed_asset_model<
        C: diesel::connection::LoadConnection,
    >(
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
            self.procedure_template_photographed_asset_model,
            conn,
        )
    }
    pub fn procedure_template_photographed_with_model<
        C: diesel::connection::LoadConnection,
    >(
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
            self.procedure_template_photographed_with_model,
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn photograph_procedures_procedure_template_procedure_templa_fkey1(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate,
        diesel::result::Error,
    >{
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate::table()
            .filter(
                crate::codegen::diesel_codegen::tables::photograph_procedure_templates::photograph_procedure_templates::dsl::procedure_template
                    .eq(&self.procedure_template)
                    .and(
                        crate::codegen::diesel_codegen::tables::photograph_procedure_templates::photograph_procedure_templates::dsl::procedure_template_photographed_asset_model
                            .eq(&self.procedure_template_photographed_asset_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn photograph_procedures_procedure_template_procedure_templa_fkey2(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate,
        diesel::result::Error,
    >{
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate::table()
            .filter(
                crate::codegen::diesel_codegen::tables::photograph_procedure_templates::photograph_procedure_templates::dsl::procedure_template
                    .eq(&self.procedure_template)
                    .and(
                        crate::codegen::diesel_codegen::tables::photograph_procedure_templates::photograph_procedure_templates::dsl::procedure_template_photograph_model
                            .eq(&self.procedure_template_photograph_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn photograph_procedures_procedure_template_procedure_templat_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate,
        diesel::result::Error,
    >{
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate::table()
            .filter(
                crate::codegen::diesel_codegen::tables::photograph_procedure_templates::photograph_procedure_templates::dsl::procedure_template
                    .eq(&self.procedure_template)
                    .and(
                        crate::codegen::diesel_codegen::tables::photograph_procedure_templates::photograph_procedure_templates::dsl::procedure_template_photographed_with_model
                            .eq(&self.procedure_template_photographed_with_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate,
            >(conn)
    }
    pub fn from_procedure_photograph<C>(
        procedure_photograph: ::rosetta_uuid::Uuid,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::photograph_procedures::photograph_procedures::procedure_photograph as diesel::expression_methods::EqAll<
                ::rosetta_uuid::Uuid,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::photograph_procedures::photograph_procedures::procedure_photograph as diesel::expression_methods::EqAll<
                ::rosetta_uuid::Uuid,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::photograph_procedures::photograph_procedures::procedure,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::photograph_procedures::photograph_procedures::procedure_photograph as diesel::expression_methods::EqAll<
                ::rosetta_uuid::Uuid,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::photograph_procedures::photograph_procedures::procedure,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::photograph_procedures::photograph_procedures;
        Self::table()
            .filter(photograph_procedures::procedure_photograph.eq(procedure_photograph))
            .order_by(photograph_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_photograph_and_photograph(
        procedure_photograph: ::rosetta_uuid::Uuid,
        photograph: ::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::photograph_procedures::photograph_procedures;
        Self::table()
            .filter(
                photograph_procedures::procedure_photograph
                    .eq(procedure_photograph)
                    .and(photograph_procedures::photograph.eq(photograph)),
            )
            .order_by(photograph_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_photograph_and_procedure_template_photograph_model(
        procedure_photograph: ::rosetta_uuid::Uuid,
        procedure_template_photograph_model: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::photograph_procedures::photograph_procedures;
        Self::table()
            .filter(
                photograph_procedures::procedure_photograph.eq(procedure_photograph).and(
                    photograph_procedures::procedure_template_photograph_model
                        .eq(procedure_template_photograph_model),
                ),
            )
            .order_by(photograph_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    pub fn from_procedure_photographed_asset<C>(
        procedure_photographed_asset: ::rosetta_uuid::Uuid,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::photograph_procedures::photograph_procedures::procedure_photographed_asset as diesel::expression_methods::EqAll<
                ::rosetta_uuid::Uuid,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::photograph_procedures::photograph_procedures::procedure_photographed_asset as diesel::expression_methods::EqAll<
                ::rosetta_uuid::Uuid,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::photograph_procedures::photograph_procedures::procedure,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::photograph_procedures::photograph_procedures::procedure_photographed_asset as diesel::expression_methods::EqAll<
                ::rosetta_uuid::Uuid,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::photograph_procedures::photograph_procedures::procedure,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::photograph_procedures::photograph_procedures;
        Self::table()
            .filter(
                photograph_procedures::procedure_photographed_asset
                    .eq(procedure_photographed_asset),
            )
            .order_by(photograph_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_photographed_asset_and_photographed_asset(
        procedure_photographed_asset: ::rosetta_uuid::Uuid,
        photographed_asset: ::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::photograph_procedures::photograph_procedures;
        Self::table()
            .filter(
                photograph_procedures::procedure_photographed_asset
                    .eq(procedure_photographed_asset)
                    .and(photograph_procedures::photographed_asset.eq(photographed_asset)),
            )
            .order_by(photograph_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_photographed_asset_and_procedure_template_photographed_asset_model(
        procedure_photographed_asset: ::rosetta_uuid::Uuid,
        procedure_template_photographed_asset_model: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::photograph_procedures::photograph_procedures;
        Self::table()
            .filter(
                photograph_procedures::procedure_photographed_asset
                    .eq(procedure_photographed_asset)
                    .and(
                        photograph_procedures::procedure_template_photographed_asset_model
                            .eq(procedure_template_photographed_asset_model),
                    ),
            )
            .order_by(photograph_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    pub fn from_procedure_photographed_with<C>(
        procedure_photographed_with: ::rosetta_uuid::Uuid,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::photograph_procedures::photograph_procedures::procedure_photographed_with as diesel::expression_methods::EqAll<
                ::rosetta_uuid::Uuid,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::photograph_procedures::photograph_procedures::procedure_photographed_with as diesel::expression_methods::EqAll<
                ::rosetta_uuid::Uuid,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::photograph_procedures::photograph_procedures::procedure,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::photograph_procedures::photograph_procedures::procedure_photographed_with as diesel::expression_methods::EqAll<
                ::rosetta_uuid::Uuid,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::photograph_procedures::photograph_procedures::procedure,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::photograph_procedures::photograph_procedures;
        Self::table()
            .filter(
                photograph_procedures::procedure_photographed_with.eq(procedure_photographed_with),
            )
            .order_by(photograph_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_photographed_with_and_photographed_with(
        procedure_photographed_with: ::rosetta_uuid::Uuid,
        photographed_with: ::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::photograph_procedures::photograph_procedures;
        Self::table()
            .filter(
                photograph_procedures::procedure_photographed_with
                    .eq(procedure_photographed_with)
                    .and(photograph_procedures::photographed_with.eq(photographed_with)),
            )
            .order_by(photograph_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_photographed_with_and_procedure_template_photographed_with_model(
        procedure_photographed_with: ::rosetta_uuid::Uuid,
        procedure_template_photographed_with_model: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::photograph_procedures::photograph_procedures;
        Self::table()
            .filter(
                photograph_procedures::procedure_photographed_with
                    .eq(procedure_photographed_with)
                    .and(
                        photograph_procedures::procedure_template_photographed_with_model
                            .eq(procedure_template_photographed_with_model),
                    ),
            )
            .order_by(photograph_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    pub fn from_procedure_template<C>(
        procedure_template: i32,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::photograph_procedures::photograph_procedures::procedure_template as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::photograph_procedures::photograph_procedures::procedure_template as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::photograph_procedures::photograph_procedures::procedure,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::photograph_procedures::photograph_procedures::procedure_template as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::photograph_procedures::photograph_procedures::procedure,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::photograph_procedures::photograph_procedures;
        Self::table()
            .filter(photograph_procedures::procedure_template.eq(procedure_template))
            .order_by(photograph_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    pub fn from_procedure_template_photograph_model<C>(
        procedure_template_photograph_model: i32,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::photograph_procedures::photograph_procedures::procedure_template_photograph_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::photograph_procedures::photograph_procedures::procedure_template_photograph_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::photograph_procedures::photograph_procedures::procedure,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::photograph_procedures::photograph_procedures::procedure_template_photograph_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::photograph_procedures::photograph_procedures::procedure,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::photograph_procedures::photograph_procedures;
        Self::table()
            .filter(
                photograph_procedures::procedure_template_photograph_model
                    .eq(procedure_template_photograph_model),
            )
            .order_by(photograph_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    pub fn from_procedure_template_photographed_asset_model<C>(
        procedure_template_photographed_asset_model: i32,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::photograph_procedures::photograph_procedures::procedure_template_photographed_asset_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::photograph_procedures::photograph_procedures::procedure_template_photographed_asset_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::photograph_procedures::photograph_procedures::procedure,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::photograph_procedures::photograph_procedures::procedure_template_photographed_asset_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::photograph_procedures::photograph_procedures::procedure,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::photograph_procedures::photograph_procedures;
        Self::table()
            .filter(
                photograph_procedures::procedure_template_photographed_asset_model
                    .eq(procedure_template_photographed_asset_model),
            )
            .order_by(photograph_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    pub fn from_procedure_template_photographed_with_model<C>(
        procedure_template_photographed_with_model: i32,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::photograph_procedures::photograph_procedures::procedure_template_photographed_with_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::photograph_procedures::photograph_procedures::procedure_template_photographed_with_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::photograph_procedures::photograph_procedures::procedure,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::photograph_procedures::photograph_procedures::procedure_template_photographed_with_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::photograph_procedures::photograph_procedures::procedure,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::photograph_procedures::photograph_procedures;
        Self::table()
            .filter(
                photograph_procedures::procedure_template_photographed_with_model
                    .eq(procedure_template_photographed_with_model),
            )
            .order_by(photograph_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_and_procedure_template_photographed_asset_model(
        procedure_template: i32,
        procedure_template_photographed_asset_model: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::photograph_procedures::photograph_procedures;
        Self::table()
            .filter(
                photograph_procedures::procedure_template.eq(procedure_template).and(
                    photograph_procedures::procedure_template_photographed_asset_model
                        .eq(procedure_template_photographed_asset_model),
                ),
            )
            .order_by(photograph_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_and_procedure_template_photograph_model(
        procedure_template: i32,
        procedure_template_photograph_model: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::photograph_procedures::photograph_procedures;
        Self::table()
            .filter(
                photograph_procedures::procedure_template.eq(procedure_template).and(
                    photograph_procedures::procedure_template_photograph_model
                        .eq(procedure_template_photograph_model),
                ),
            )
            .order_by(photograph_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_and_procedure_template_photographed_with_model(
        procedure_template: i32,
        procedure_template_photographed_with_model: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::photograph_procedures::photograph_procedures;
        Self::table()
            .filter(
                photograph_procedures::procedure_template.eq(procedure_template).and(
                    photograph_procedures::procedure_template_photographed_with_model
                        .eq(procedure_template_photographed_with_model),
                ),
            )
            .order_by(photograph_procedures::procedure.asc())
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
            photograph_procedures::photograph_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(photograph_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::parent_procedure.eq(parent_procedure))
            .order_by(photograph_procedures::procedure.asc())
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
            photograph_procedures::photograph_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(photograph_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::parent_procedure_template.eq(parent_procedure_template))
            .order_by(photograph_procedures::procedure.asc())
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
            photograph_procedures::photograph_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(photograph_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::predecessor_procedure.eq(predecessor_procedure))
            .order_by(photograph_procedures::procedure.asc())
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
            photograph_procedures::photograph_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(photograph_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::predecessor_procedure_template.eq(predecessor_procedure_template))
            .order_by(photograph_procedures::procedure.asc())
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
            photograph_procedures::photograph_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(photograph_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::created_by.eq(created_by))
            .order_by(photograph_procedures::procedure.asc())
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
            photograph_procedures::photograph_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(photograph_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::updated_by.eq(updated_by))
            .order_by(photograph_procedures::procedure.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
}
impl AsRef<PhotographProcedure> for PhotographProcedure {
    fn as_ref(&self) -> &PhotographProcedure {
        self
    }
}
