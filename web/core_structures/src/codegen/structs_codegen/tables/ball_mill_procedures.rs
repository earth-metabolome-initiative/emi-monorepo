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
#[diesel(belongs_to(crate::BeadModel, foreign_key = bead_model))]
#[diesel(belongs_to(crate::BallMillMachineModel, foreign_key = milled_with_model))]
#[diesel(belongs_to(crate::BallMillMachine, foreign_key = milled_with))]
#[diesel(belongs_to(crate::VolumetricContainer, foreign_key = milled_container))]
#[diesel(
    belongs_to(crate::VolumetricContainerModel, foreign_key = milled_container_model)
)]
#[diesel(primary_key(procedure))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::ball_mill_procedures::ball_mill_procedures
)]
pub struct BallMillProcedure {
    pub procedure: ::rosetta_uuid::Uuid,
    pub procedure_template: i32,
    pub bead_model: i32,
    pub procedure_template_bead_model: i32,
    pub procedure_bead: ::rosetta_uuid::Uuid,
    pub milled_with_model: i32,
    pub procedure_template_milled_with_model: i32,
    pub procedure_milled_with: ::rosetta_uuid::Uuid,
    pub milled_with: Option<::rosetta_uuid::Uuid>,
    pub milled_container: ::rosetta_uuid::Uuid,
    pub milled_container_model: i32,
    pub procedure_template_milled_container_model: i32,
    pub procedure_milled_container: ::rosetta_uuid::Uuid,
}
impl web_common_traits::prelude::TableName for BallMillProcedure {
    const TABLE_NAME: &'static str = "ball_mill_procedures";
}
impl web_common_traits::prelude::ExtensionTable<crate::Procedure> for BallMillProcedure where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a ::rosetta_uuid::Uuid>
{
}
impl web_common_traits::prelude::ExtensionTable<crate::BallMillProcedure> for BallMillProcedure where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a ::rosetta_uuid::Uuid>
{
}
impl diesel::Identifiable for BallMillProcedure {
    type Id = ::rosetta_uuid::Uuid;
    fn id(self) -> Self::Id {
        self.procedure
    }
}
impl BallMillProcedure {
    pub fn procedure<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::Procedure, diesel::result::Error>
    where
        crate::Procedure: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::Procedure::read(self.procedure, conn)
    }
    pub fn procedure_template<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::BallMillProcedureTemplate, diesel::result::Error>
    where
        crate::BallMillProcedureTemplate: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::BallMillProcedureTemplate::read(self.procedure_template, conn)
    }
    pub fn bead_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::BeadModel, diesel::result::Error>
    where
        crate::BeadModel: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::BeadModel::read(self.bead_model, conn)
    }
    pub fn procedure_template_bead_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::ProcedureTemplateAssetModel, diesel::result::Error>
    where
        crate::ProcedureTemplateAssetModel: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::ProcedureTemplateAssetModel::read(self.procedure_template_bead_model, conn)
    }
    pub fn procedure_bead<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::ProcedureAsset, diesel::result::Error>
    where
        crate::ProcedureAsset: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::ProcedureAsset::read(self.procedure_bead, conn)
    }
    pub fn milled_with_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::BallMillMachineModel, diesel::result::Error>
    where
        crate::BallMillMachineModel: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::BallMillMachineModel::read(self.milled_with_model, conn)
    }
    pub fn procedure_template_milled_with_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::ProcedureTemplateAssetModel, diesel::result::Error>
    where
        crate::ProcedureTemplateAssetModel: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::ProcedureTemplateAssetModel::read(self.procedure_template_milled_with_model, conn)
    }
    pub fn procedure_milled_with<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::ProcedureAsset, diesel::result::Error>
    where
        crate::ProcedureAsset: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::ProcedureAsset::read(self.procedure_milled_with, conn)
    }
    pub fn milled_with<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<Option<crate::BallMillMachine>, diesel::result::Error>
    where
        crate::BallMillMachine: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        let Some(milled_with) = self.milled_with else {
            return Ok(None);
        };
        crate::BallMillMachine::read(milled_with, conn).map(Some)
    }
    pub fn milled_container<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::VolumetricContainer, diesel::result::Error>
    where
        crate::VolumetricContainer: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::VolumetricContainer::read(self.milled_container, conn)
    }
    pub fn milled_container_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::VolumetricContainerModel, diesel::result::Error>
    where
        crate::VolumetricContainerModel: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::VolumetricContainerModel::read(self.milled_container_model, conn)
    }
    pub fn procedure_template_milled_container_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::ProcedureTemplateAssetModel, diesel::result::Error>
    where
        crate::ProcedureTemplateAssetModel: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::ProcedureTemplateAssetModel::read(
            self.procedure_template_milled_container_model,
            conn,
        )
    }
    pub fn procedure_milled_container<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::ProcedureAsset, diesel::result::Error>
    where
        crate::ProcedureAsset: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::ProcedureAsset::read(self.procedure_milled_container, conn)
    }
    #[cfg(feature = "postgres")]
    pub fn ball_mill_procedures_procedure_procedure_template_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::Procedure, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::Procedure::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedures::procedures::dsl::procedure
                    .eq(&self.procedure)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedures::procedures::dsl::procedure_template
                            .eq(&self.procedure_template),
                    ),
            )
            .first::<crate::Procedure>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn ball_mill_procedures_procedure_template_procedure_template_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::BallMillProcedureTemplate, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::BallMillProcedureTemplate::table()
            .filter(
                crate::codegen::diesel_codegen::tables::ball_mill_procedure_templates::ball_mill_procedure_templates::dsl::procedure_template
                    .eq(&self.procedure_template)
                    .and(
                        crate::codegen::diesel_codegen::tables::ball_mill_procedure_templates::ball_mill_procedure_templates::dsl::procedure_template_bead_model
                            .eq(&self.procedure_template_bead_model),
                    ),
            )
            .first::<crate::BallMillProcedureTemplate>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn ball_mill_procedures_procedure_template_procedure_templat_fkey1(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::BallMillProcedureTemplate, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::BallMillProcedureTemplate::table()
            .filter(
                crate::codegen::diesel_codegen::tables::ball_mill_procedure_templates::ball_mill_procedure_templates::dsl::procedure_template
                    .eq(&self.procedure_template)
                    .and(
                        crate::codegen::diesel_codegen::tables::ball_mill_procedure_templates::ball_mill_procedure_templates::dsl::procedure_template_milled_with_model
                            .eq(&self.procedure_template_milled_with_model),
                    ),
            )
            .first::<crate::BallMillProcedureTemplate>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn ball_mill_procedures_procedure_template_procedure_templat_fkey2(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::BallMillProcedureTemplate, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::BallMillProcedureTemplate::table()
            .filter(
                crate::codegen::diesel_codegen::tables::ball_mill_procedure_templates::ball_mill_procedure_templates::dsl::procedure_template
                    .eq(&self.procedure_template)
                    .and(
                        crate::codegen::diesel_codegen::tables::ball_mill_procedure_templates::ball_mill_procedure_templates::dsl::procedure_template_milled_container_model
                            .eq(&self.procedure_template_milled_container_model),
                    ),
            )
            .first::<crate::BallMillProcedureTemplate>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn ball_mill_procedures_procedure_bead_procedure_template_bea_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::ProcedureAsset, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_bead)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::procedure_template_asset_model
                            .eq(&self.procedure_template_bead_model),
                    ),
            )
            .first::<crate::ProcedureAsset>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn ball_mill_procedures_procedure_milled_with_procedure_templ_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::ProcedureAsset, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_milled_with)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::procedure_template_asset_model
                            .eq(&self.procedure_template_milled_with_model),
                    ),
            )
            .first::<crate::ProcedureAsset>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn ball_mill_procedures_procedure_milled_container_procedure_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::ProcedureAsset, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_milled_container)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::procedure_template_asset_model
                            .eq(&self.procedure_template_milled_container_model),
                    ),
            )
            .first::<crate::ProcedureAsset>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn ball_mill_procedures_procedure_milled_container_milled_con_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::ProcedureAsset, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_milled_container)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset_model
                            .eq(&self.milled_container_model),
                    ),
            )
            .first::<crate::ProcedureAsset>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn ball_mill_procedures_procedure_milled_with_milled_with_mod_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::ProcedureAsset, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_milled_with)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset_model
                            .eq(&self.milled_with_model),
                    ),
            )
            .first::<crate::ProcedureAsset>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn ball_mill_procedures_procedure_milled_with_milled_with_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<crate::ProcedureAsset>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        let Some(milled_with) = self.milled_with else {
            return Ok(None);
        };
        crate::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_milled_with)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset
                            .eq(milled_with),
                    ),
            )
            .first::<crate::ProcedureAsset>(conn)
            .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub fn ball_mill_procedures_procedure_bead_bead_model_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::ProcedureAsset, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_bead)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset_model
                            .eq(&self.bead_model),
                    ),
            )
            .first::<crate::ProcedureAsset>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn ball_mill_procedures_procedure_milled_container_milled_co_fkey1(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::ProcedureAsset, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_milled_container)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset
                            .eq(&self.milled_container),
                    ),
            )
            .first::<crate::ProcedureAsset>(conn)
    }
    pub fn ball_mill_procedures_milled_with_model_milled_container_mo_fkey<
        C: diesel::connection::LoadConnection,
    >(
        &self,
        conn: &mut C,
    ) -> Result<crate::AssetCompatibilityRule, diesel::result::Error>
    where
        crate::AssetCompatibilityRule: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::AssetCompatibilityRule::read(
            (self.milled_with_model, self.milled_container_model),
            conn,
        )
    }
    pub fn ball_mill_procedures_milled_with_model_bead_model_fkey<
        C: diesel::connection::LoadConnection,
    >(
        &self,
        conn: &mut C,
    ) -> Result<crate::AssetCompatibilityRule, diesel::result::Error>
    where
        crate::AssetCompatibilityRule: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::AssetCompatibilityRule::read((self.milled_with_model, self.bead_model), conn)
    }
    pub fn ball_mill_procedures_bead_model_milled_container_model_fkey<
        C: diesel::connection::LoadConnection,
    >(
        &self,
        conn: &mut C,
    ) -> Result<crate::AssetCompatibilityRule, diesel::result::Error>
    where
        crate::AssetCompatibilityRule: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::AssetCompatibilityRule::read((self.bead_model, self.milled_container_model), conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure(
        procedure: &::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::ball_mill_procedures::ball_mill_procedures;
        Self::table()
            .filter(ball_mill_procedures::procedure.eq(procedure))
            .order_by(ball_mill_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template(
        procedure_template: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::ball_mill_procedures::ball_mill_procedures;
        Self::table()
            .filter(ball_mill_procedures::procedure_template.eq(procedure_template))
            .order_by(ball_mill_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_bead_model(
        procedure_template_bead_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::ball_mill_procedures::ball_mill_procedures;
        Self::table()
            .filter(
                ball_mill_procedures::procedure_template_bead_model
                    .eq(procedure_template_bead_model),
            )
            .order_by(ball_mill_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_bead(
        procedure_bead: &::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::ball_mill_procedures::ball_mill_procedures;
        Self::table()
            .filter(ball_mill_procedures::procedure_bead.eq(procedure_bead))
            .order_by(ball_mill_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_milled_with_model(
        procedure_template_milled_with_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::ball_mill_procedures::ball_mill_procedures;
        Self::table()
            .filter(
                ball_mill_procedures::procedure_template_milled_with_model
                    .eq(procedure_template_milled_with_model),
            )
            .order_by(ball_mill_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_milled_with(
        procedure_milled_with: &::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::ball_mill_procedures::ball_mill_procedures;
        Self::table()
            .filter(ball_mill_procedures::procedure_milled_with.eq(procedure_milled_with))
            .order_by(ball_mill_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_milled_container_model(
        procedure_template_milled_container_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::ball_mill_procedures::ball_mill_procedures;
        Self::table()
            .filter(
                ball_mill_procedures::procedure_template_milled_container_model
                    .eq(procedure_template_milled_container_model),
            )
            .order_by(ball_mill_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_milled_container(
        procedure_milled_container: &::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::ball_mill_procedures::ball_mill_procedures;
        Self::table()
            .filter(ball_mill_procedures::procedure_milled_container.eq(procedure_milled_container))
            .order_by(ball_mill_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_and_procedure_template(
        procedure: &::rosetta_uuid::Uuid,
        procedure_template: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::ball_mill_procedures::ball_mill_procedures;
        Self::table()
            .filter(
                ball_mill_procedures::procedure
                    .eq(procedure)
                    .and(ball_mill_procedures::procedure_template.eq(procedure_template)),
            )
            .order_by(ball_mill_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_and_procedure_template_bead_model(
        procedure_template: &i32,
        procedure_template_bead_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::ball_mill_procedures::ball_mill_procedures;
        Self::table()
            .filter(
                ball_mill_procedures::procedure_template.eq(procedure_template).and(
                    ball_mill_procedures::procedure_template_bead_model
                        .eq(procedure_template_bead_model),
                ),
            )
            .order_by(ball_mill_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_and_procedure_template_milled_with_model(
        procedure_template: &i32,
        procedure_template_milled_with_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::ball_mill_procedures::ball_mill_procedures;
        Self::table()
            .filter(
                ball_mill_procedures::procedure_template.eq(procedure_template).and(
                    ball_mill_procedures::procedure_template_milled_with_model
                        .eq(procedure_template_milled_with_model),
                ),
            )
            .order_by(ball_mill_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_and_procedure_template_milled_container_model(
        procedure_template: &i32,
        procedure_template_milled_container_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::ball_mill_procedures::ball_mill_procedures;
        Self::table()
            .filter(
                ball_mill_procedures::procedure_template.eq(procedure_template).and(
                    ball_mill_procedures::procedure_template_milled_container_model
                        .eq(procedure_template_milled_container_model),
                ),
            )
            .order_by(ball_mill_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_bead_and_procedure_template_bead_model(
        procedure_bead: &::rosetta_uuid::Uuid,
        procedure_template_bead_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::ball_mill_procedures::ball_mill_procedures;
        Self::table()
            .filter(
                ball_mill_procedures::procedure_bead.eq(procedure_bead).and(
                    ball_mill_procedures::procedure_template_bead_model
                        .eq(procedure_template_bead_model),
                ),
            )
            .order_by(ball_mill_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_milled_with_and_procedure_template_milled_with_model(
        procedure_milled_with: &::rosetta_uuid::Uuid,
        procedure_template_milled_with_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::ball_mill_procedures::ball_mill_procedures;
        Self::table()
            .filter(
                ball_mill_procedures::procedure_milled_with.eq(procedure_milled_with).and(
                    ball_mill_procedures::procedure_template_milled_with_model
                        .eq(procedure_template_milled_with_model),
                ),
            )
            .order_by(ball_mill_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_milled_container_and_procedure_template_milled_container_model(
        procedure_milled_container: &::rosetta_uuid::Uuid,
        procedure_template_milled_container_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::ball_mill_procedures::ball_mill_procedures;
        Self::table()
            .filter(
                ball_mill_procedures::procedure_milled_container
                    .eq(procedure_milled_container)
                    .and(
                        ball_mill_procedures::procedure_template_milled_container_model
                            .eq(procedure_template_milled_container_model),
                    ),
            )
            .order_by(ball_mill_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_milled_container_and_milled_container_model(
        procedure_milled_container: &::rosetta_uuid::Uuid,
        milled_container_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::ball_mill_procedures::ball_mill_procedures;
        Self::table()
            .filter(
                ball_mill_procedures::procedure_milled_container
                    .eq(procedure_milled_container)
                    .and(ball_mill_procedures::milled_container_model.eq(milled_container_model)),
            )
            .order_by(ball_mill_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_milled_with_and_milled_with_model(
        procedure_milled_with: &::rosetta_uuid::Uuid,
        milled_with_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::ball_mill_procedures::ball_mill_procedures;
        Self::table()
            .filter(
                ball_mill_procedures::procedure_milled_with
                    .eq(procedure_milled_with)
                    .and(ball_mill_procedures::milled_with_model.eq(milled_with_model)),
            )
            .order_by(ball_mill_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_milled_with_and_milled_with(
        procedure_milled_with: &::rosetta_uuid::Uuid,
        milled_with: &::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::ball_mill_procedures::ball_mill_procedures;
        Self::table()
            .filter(
                ball_mill_procedures::procedure_milled_with
                    .eq(procedure_milled_with)
                    .and(ball_mill_procedures::milled_with.eq(milled_with)),
            )
            .order_by(ball_mill_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_bead_and_bead_model(
        procedure_bead: &::rosetta_uuid::Uuid,
        bead_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::ball_mill_procedures::ball_mill_procedures;
        Self::table()
            .filter(
                ball_mill_procedures::procedure_bead
                    .eq(procedure_bead)
                    .and(ball_mill_procedures::bead_model.eq(bead_model)),
            )
            .order_by(ball_mill_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_milled_container_and_milled_container(
        procedure_milled_container: &::rosetta_uuid::Uuid,
        milled_container: &::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::ball_mill_procedures::ball_mill_procedures;
        Self::table()
            .filter(
                ball_mill_procedures::procedure_milled_container
                    .eq(procedure_milled_container)
                    .and(ball_mill_procedures::milled_container.eq(milled_container)),
            )
            .order_by(ball_mill_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_milled_with_model_and_milled_container_model(
        milled_with_model: &i32,
        milled_container_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::ball_mill_procedures::ball_mill_procedures;
        Self::table()
            .filter(
                ball_mill_procedures::milled_with_model
                    .eq(milled_with_model)
                    .and(ball_mill_procedures::milled_container_model.eq(milled_container_model)),
            )
            .order_by(ball_mill_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_milled_with_model_and_bead_model(
        milled_with_model: &i32,
        bead_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::ball_mill_procedures::ball_mill_procedures;
        Self::table()
            .filter(
                ball_mill_procedures::milled_with_model
                    .eq(milled_with_model)
                    .and(ball_mill_procedures::bead_model.eq(bead_model)),
            )
            .order_by(ball_mill_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_bead_model_and_milled_container_model(
        bead_model: &i32,
        milled_container_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::ball_mill_procedures::ball_mill_procedures;
        Self::table()
            .filter(
                ball_mill_procedures::bead_model
                    .eq(bead_model)
                    .and(ball_mill_procedures::milled_container_model.eq(milled_container_model)),
            )
            .order_by(ball_mill_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_and_procedure(
        procedure_template: &i32,
        procedure: &::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl,
            SelectableHelper, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            ball_mill_procedures::ball_mill_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(ball_mill_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(
                procedures::procedure_template
                    .eq(procedure_template)
                    .and(procedures::procedure.eq(procedure)),
            )
            .order_by(ball_mill_procedures::procedure.asc())
            .select(Self::as_select())
            .first::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_parent_procedure(
        parent_procedure: &::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            ball_mill_procedures::ball_mill_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(ball_mill_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::parent_procedure.eq(parent_procedure))
            .order_by(ball_mill_procedures::procedure.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_parent_procedure_template(
        parent_procedure_template: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            ball_mill_procedures::ball_mill_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(ball_mill_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::parent_procedure_template.eq(parent_procedure_template))
            .order_by(ball_mill_procedures::procedure.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_most_concrete_table(
        most_concrete_table: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            ball_mill_procedures::ball_mill_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(ball_mill_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::most_concrete_table.eq(most_concrete_table))
            .order_by(ball_mill_procedures::procedure.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_created_by(
        created_by: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            ball_mill_procedures::ball_mill_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(ball_mill_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::created_by.eq(created_by))
            .order_by(ball_mill_procedures::procedure.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_created_at(
        created_at: &::rosetta_timestamp::TimestampUTC,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            ball_mill_procedures::ball_mill_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(ball_mill_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::created_at.eq(created_at))
            .order_by(ball_mill_procedures::procedure.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_updated_by(
        updated_by: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            ball_mill_procedures::ball_mill_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(ball_mill_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::updated_by.eq(updated_by))
            .order_by(ball_mill_procedures::procedure.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_updated_at(
        updated_at: &::rosetta_timestamp::TimestampUTC,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            ball_mill_procedures::ball_mill_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(ball_mill_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::updated_at.eq(updated_at))
            .order_by(ball_mill_procedures::procedure.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
}
impl AsRef<BallMillProcedure> for BallMillProcedure {
    fn as_ref(&self) -> &BallMillProcedure {
        self
    }
}
