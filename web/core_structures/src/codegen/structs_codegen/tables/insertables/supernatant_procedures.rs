#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SupernatantProcedureExtensionAttribute {
    Procedure(crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute),
}
impl core::fmt::Display for SupernatantProcedureExtensionAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Procedure(e) => write!(f, "{e}"),
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute>
    for SupernatantProcedureExtensionAttribute
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute,
    ) -> Self {
        Self::Procedure(attribute)
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SupernatantProcedureAttribute {
    Extension(SupernatantProcedureExtensionAttribute),
    Procedure,
    ProcedureTemplate,
    StratifiedSource,
    ProcedureTemplateStratifiedSourceModel,
    ProcedureStratifiedSource(
        crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute,
    ),
    SupernatantDestination,
    ProcedureTemplateSupernatantDestinationModel,
    ProcedureSupernatantDestination(
        crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute,
    ),
    TransferredWith,
    TransferredWithModel,
    ProcedureTemplateTransferredWithModel,
    ProcedureTransferredWith(
        crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute,
    ),
    PipetteTipModel,
    ProcedureTemplatePipetteTipModel,
    ProcedurePipetteTip(
        crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute,
    ),
}
impl core::str::FromStr for SupernatantProcedureAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ProcedureTemplate" => Ok(Self::ProcedureTemplate),
            "StratifiedSource" => Ok(Self::StratifiedSource),
            "ProcedureTemplateStratifiedSourceModel" => {
                Ok(Self::ProcedureTemplateStratifiedSourceModel)
            }
            "ProcedureStratifiedSource" => Ok(Self::ProcedureStratifiedSource(
                crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute::Id,
            )),
            "SupernatantDestination" => Ok(Self::SupernatantDestination),
            "ProcedureTemplateSupernatantDestinationModel" => {
                Ok(Self::ProcedureTemplateSupernatantDestinationModel)
            }
            "ProcedureSupernatantDestination" => Ok(Self::ProcedureSupernatantDestination(
                crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute::Id,
            )),
            "TransferredWith" => Ok(Self::TransferredWith),
            "TransferredWithModel" => Ok(Self::TransferredWithModel),
            "ProcedureTemplateTransferredWithModel" => {
                Ok(Self::ProcedureTemplateTransferredWithModel)
            }
            "ProcedureTransferredWith" => Ok(Self::ProcedureTransferredWith(
                crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute::Id,
            )),
            "PipetteTipModel" => Ok(Self::PipetteTipModel),
            "ProcedureTemplatePipetteTipModel" => Ok(Self::ProcedureTemplatePipetteTipModel),
            "ProcedurePipetteTip" => Ok(Self::ProcedurePipetteTip(
                crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute::Id,
            )),
            "procedure_template" => Ok(Self::ProcedureTemplate),
            "stratified_source" => Ok(Self::StratifiedSource),
            "procedure_template_stratified_source_model" => {
                Ok(Self::ProcedureTemplateStratifiedSourceModel)
            }
            "procedure_stratified_source" => Ok(Self::ProcedureStratifiedSource(
                crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute::Id,
            )),
            "supernatant_destination" => Ok(Self::SupernatantDestination),
            "procedure_template_supernatant_destination_model" => {
                Ok(Self::ProcedureTemplateSupernatantDestinationModel)
            }
            "procedure_supernatant_destination" => Ok(Self::ProcedureSupernatantDestination(
                crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute::Id,
            )),
            "transferred_with" => Ok(Self::TransferredWith),
            "transferred_with_model" => Ok(Self::TransferredWithModel),
            "procedure_template_transferred_with_model" => {
                Ok(Self::ProcedureTemplateTransferredWithModel)
            }
            "procedure_transferred_with" => Ok(Self::ProcedureTransferredWith(
                crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute::Id,
            )),
            "pipette_tip_model" => Ok(Self::PipetteTipModel),
            "procedure_template_pipette_tip_model" => Ok(Self::ProcedureTemplatePipetteTipModel),
            "procedure_pipette_tip" => Ok(Self::ProcedurePipetteTip(
                crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute::Id,
            )),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl core::fmt::Display for SupernatantProcedureAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::Procedure => write!(f, "supernatant_procedures.procedure"),
            Self::ProcedureTemplate => {
                write!(f, "supernatant_procedures.procedure_template")
            }
            Self::StratifiedSource => {
                write!(f, "supernatant_procedures.stratified_source")
            }
            Self::ProcedureTemplateStratifiedSourceModel => {
                write!(f, "supernatant_procedures.procedure_template_stratified_source_model")
            }
            Self::ProcedureStratifiedSource(e) => write!(f, "supernatant_procedures.{e}"),
            Self::SupernatantDestination => {
                write!(f, "supernatant_procedures.supernatant_destination")
            }
            Self::ProcedureTemplateSupernatantDestinationModel => {
                write!(f, "supernatant_procedures.procedure_template_supernatant_destination_model")
            }
            Self::ProcedureSupernatantDestination(e) => {
                write!(f, "supernatant_procedures.{e}")
            }
            Self::TransferredWith => write!(f, "supernatant_procedures.transferred_with"),
            Self::TransferredWithModel => {
                write!(f, "supernatant_procedures.transferred_with_model")
            }
            Self::ProcedureTemplateTransferredWithModel => {
                write!(f, "supernatant_procedures.procedure_template_transferred_with_model")
            }
            Self::ProcedureTransferredWith(e) => write!(f, "supernatant_procedures.{e}"),
            Self::PipetteTipModel => {
                write!(f, "supernatant_procedures.pipette_tip_model")
            }
            Self::ProcedureTemplatePipetteTipModel => {
                write!(f, "supernatant_procedures.procedure_template_pipette_tip_model")
            }
            Self::ProcedurePipetteTip(e) => write!(f, "supernatant_procedures.{e}"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::supernatant_procedures::supernatant_procedures
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableSupernatantProcedure {
    pub(crate) procedure: ::rosetta_uuid::Uuid,
    pub(crate) procedure_template: i32,
    pub(crate) stratified_source: ::rosetta_uuid::Uuid,
    pub(crate) procedure_template_stratified_source_model: i32,
    pub(crate) procedure_stratified_source: ::rosetta_uuid::Uuid,
    pub(crate) supernatant_destination: ::rosetta_uuid::Uuid,
    pub(crate) procedure_template_supernatant_destination_model: i32,
    pub(crate) procedure_supernatant_destination: ::rosetta_uuid::Uuid,
    pub(crate) transferred_with: ::rosetta_uuid::Uuid,
    pub(crate) transferred_with_model: i32,
    pub(crate) procedure_template_transferred_with_model: i32,
    pub(crate) procedure_transferred_with: ::rosetta_uuid::Uuid,
    pub(crate) pipette_tip_model: i32,
    pub(crate) procedure_template_pipette_tip_model: i32,
    pub(crate) procedure_pipette_tip: ::rosetta_uuid::Uuid,
}
impl InsertableSupernatantProcedure {
    pub fn pipette_tip_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel::read(
            self.pipette_tip_model,
            conn,
        )
    }
    pub fn procedure<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::codegen::structs_codegen::tables::procedures::Procedure, diesel::result::Error>
    where
        crate::codegen::structs_codegen::tables::procedures::Procedure:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::procedures::Procedure::read(self.procedure, conn)
    }
    pub fn procedure_pipette_tip<C: diesel::connection::LoadConnection>(
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
            self.procedure_pipette_tip,
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn supernatant_procedures_procedure_pipette_tip_pipette_tip_fkey1(
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
                    .eq(&self.procedure_pipette_tip)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset_model
                            .eq(&self.pipette_tip_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn supernatant_procedures_procedure_pipette_tip_pipette_tip_m_fkey(
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
                    .eq(&self.procedure_pipette_tip)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset_model
                            .eq(&self.pipette_tip_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn supernatant_procedures_procedure_pipette_tip_procedure_tem_fkey(
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
                    .eq(&self.procedure_pipette_tip)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::procedure_template_asset_model
                            .eq(&self.procedure_template_pipette_tip_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn supernatant_procedures_procedure_procedure_template_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::procedures::Procedure, diesel::result::Error>
    {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::procedures::Procedure::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedures::procedures::dsl::procedure
                    .eq(&self.procedure)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedures::procedures::dsl::procedure_template
                            .eq(&self.procedure_template),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedures::Procedure,
            >(conn)
    }
    pub fn procedure_stratified_source<C: diesel::connection::LoadConnection>(
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
            self.procedure_stratified_source,
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn supernatant_procedures_procedure_stratified_source_procedu_fkey(
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
                    .eq(&self.procedure_stratified_source)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::procedure_template_asset_model
                            .eq(&self.procedure_template_stratified_source_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn supernatant_procedures_procedure_stratified_source_stratif_fkey(
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
                    .eq(&self.procedure_stratified_source)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset
                            .eq(&self.stratified_source),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
    }
    pub fn procedure_supernatant_destination<C: diesel::connection::LoadConnection>(
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
            self.procedure_supernatant_destination,
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn supernatant_procedures_procedure_supernatant_destination_p_fkey(
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
                    .eq(&self.procedure_supernatant_destination)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::procedure_template_asset_model
                            .eq(&self.procedure_template_supernatant_destination_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn supernatant_procedures_procedure_supernatant_destination_s_fkey(
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
                    .eq(&self.procedure_supernatant_destination)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset
                            .eq(&self.supernatant_destination),
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
        crate::codegen::structs_codegen::tables::supernatant_procedure_templates::SupernatantProcedureTemplate,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::supernatant_procedure_templates::SupernatantProcedureTemplate: web_common_traits::database::Read<
            C,
        >,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::supernatant_procedure_templates::SupernatantProcedureTemplate::read(
            self.procedure_template,
            conn,
        )
    }
    pub fn procedure_template_pipette_tip_model<C: diesel::connection::LoadConnection>(
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
            self.procedure_template_pipette_tip_model,
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn supernatant_procedures_procedure_template_procedure_templ_fkey1(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::supernatant_procedure_templates::SupernatantProcedureTemplate,
        diesel::result::Error,
    >{
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::supernatant_procedure_templates::SupernatantProcedureTemplate::table()
            .filter(
                crate::codegen::diesel_codegen::tables::supernatant_procedure_templates::supernatant_procedure_templates::dsl::procedure_template
                    .eq(&self.procedure_template)
                    .and(
                        crate::codegen::diesel_codegen::tables::supernatant_procedure_templates::supernatant_procedure_templates::dsl::procedure_template_supernatant_destination_model
                            .eq(&self.procedure_template_supernatant_destination_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::supernatant_procedure_templates::SupernatantProcedureTemplate,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn supernatant_procedures_procedure_template_procedure_templ_fkey2(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::supernatant_procedure_templates::SupernatantProcedureTemplate,
        diesel::result::Error,
    >{
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::supernatant_procedure_templates::SupernatantProcedureTemplate::table()
            .filter(
                crate::codegen::diesel_codegen::tables::supernatant_procedure_templates::supernatant_procedure_templates::dsl::procedure_template
                    .eq(&self.procedure_template)
                    .and(
                        crate::codegen::diesel_codegen::tables::supernatant_procedure_templates::supernatant_procedure_templates::dsl::procedure_template_transferred_with_model
                            .eq(&self.procedure_template_transferred_with_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::supernatant_procedure_templates::SupernatantProcedureTemplate,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn supernatant_procedures_procedure_template_procedure_templ_fkey3(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::supernatant_procedure_templates::SupernatantProcedureTemplate,
        diesel::result::Error,
    >{
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::supernatant_procedure_templates::SupernatantProcedureTemplate::table()
            .filter(
                crate::codegen::diesel_codegen::tables::supernatant_procedure_templates::supernatant_procedure_templates::dsl::procedure_template
                    .eq(&self.procedure_template)
                    .and(
                        crate::codegen::diesel_codegen::tables::supernatant_procedure_templates::supernatant_procedure_templates::dsl::procedure_template_pipette_tip_model
                            .eq(&self.procedure_template_pipette_tip_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::supernatant_procedure_templates::SupernatantProcedureTemplate,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn supernatant_procedures_procedure_template_procedure_templa_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::supernatant_procedure_templates::SupernatantProcedureTemplate,
        diesel::result::Error,
    >{
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::supernatant_procedure_templates::SupernatantProcedureTemplate::table()
            .filter(
                crate::codegen::diesel_codegen::tables::supernatant_procedure_templates::supernatant_procedure_templates::dsl::procedure_template
                    .eq(&self.procedure_template)
                    .and(
                        crate::codegen::diesel_codegen::tables::supernatant_procedure_templates::supernatant_procedure_templates::dsl::procedure_template_stratified_source_model
                            .eq(&self.procedure_template_stratified_source_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::supernatant_procedure_templates::SupernatantProcedureTemplate,
            >(conn)
    }
    pub fn procedure_template_stratified_source_model<
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
            self.procedure_template_stratified_source_model,
            conn,
        )
    }
    pub fn procedure_template_supernatant_destination_model<
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
            self.procedure_template_supernatant_destination_model,
            conn,
        )
    }
    pub fn procedure_template_transferred_with_model<
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
            self.procedure_template_transferred_with_model,
            conn,
        )
    }
    pub fn procedure_transferred_with<C: diesel::connection::LoadConnection>(
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
            self.procedure_transferred_with,
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn supernatant_procedures_procedure_transferred_with_procedur_fkey(
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
                    .eq(&self.procedure_transferred_with)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::procedure_template_asset_model
                            .eq(&self.procedure_template_transferred_with_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn supernatant_procedures_procedure_transferred_with_transfe_fkey1(
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
                    .eq(&self.procedure_transferred_with)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset_model
                            .eq(&self.transferred_with_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn supernatant_procedures_procedure_transferred_with_transfer_fkey(
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
                    .eq(&self.procedure_transferred_with)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset
                            .eq(&self.transferred_with),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
    }
    pub fn stratified_source<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer::read(
            self.stratified_source,
            conn,
        )
    }
    pub fn supernatant_destination<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer::read(
            self.supernatant_destination,
            conn,
        )
    }
    pub fn transferred_with<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::codegen::structs_codegen::tables::pipettes::Pipette, diesel::result::Error>
    where
        crate::codegen::structs_codegen::tables::pipettes::Pipette:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::pipettes::Pipette::read(
            self.transferred_with,
            conn,
        )
    }
    pub fn transferred_with_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::pipette_models::PipetteModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::pipette_models::PipetteModel:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::pipette_models::PipetteModel::read(
            self.transferred_with_model,
            conn,
        )
    }
    pub fn supernatant_procedures_transferred_with_model_pipette_tip_fkey<
        C: diesel::connection::LoadConnection,
    >(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule::read(
            (self.transferred_with_model, self.pipette_tip_model),
            conn,
        )
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Hash, Ord, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableSupernatantProcedureBuilder<
    Procedure = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureBuilder,
> {
    pub(crate) procedure_template: Option<i32>,
    pub(crate) stratified_source: Option<::rosetta_uuid::Uuid>,
    pub(crate) procedure_template_stratified_source_model: Option<i32>,
    pub(crate) procedure_stratified_source: web_common_traits::database::IdOrBuilder<
        ::rosetta_uuid::Uuid,
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
    >,
    pub(crate) supernatant_destination: Option<::rosetta_uuid::Uuid>,
    pub(crate) procedure_template_supernatant_destination_model: Option<i32>,
    pub(crate) procedure_supernatant_destination: web_common_traits::database::IdOrBuilder<
        ::rosetta_uuid::Uuid,
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
    >,
    pub(crate) transferred_with: Option<::rosetta_uuid::Uuid>,
    pub(crate) transferred_with_model: Option<i32>,
    pub(crate) procedure_template_transferred_with_model: Option<i32>,
    pub(crate) procedure_transferred_with: web_common_traits::database::IdOrBuilder<
        ::rosetta_uuid::Uuid,
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
    >,
    pub(crate) pipette_tip_model: Option<i32>,
    pub(crate) procedure_template_pipette_tip_model: Option<i32>,
    pub(crate) procedure_pipette_tip: web_common_traits::database::IdOrBuilder<
        ::rosetta_uuid::Uuid,
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
    >,
    pub(crate) procedure: Procedure,
}
impl From<InsertableSupernatantProcedureBuilder>
    for web_common_traits::database::IdOrBuilder<
        ::rosetta_uuid::Uuid,
        InsertableSupernatantProcedureBuilder,
    >
{
    fn from(builder: InsertableSupernatantProcedureBuilder) -> Self {
        Self::Builder(builder)
    }
}
impl<Procedure> common_traits::builder::IsCompleteBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableSupernatantProcedureBuilder<
        Procedure,
    >
where
    Procedure: common_traits::builder::IsCompleteBuilder,
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder:
        common_traits::builder::IsCompleteBuilder,
{
    fn is_complete(&self) -> bool {
        self.procedure.is_complete()
            && self.procedure_template.is_some()
            && (self.stratified_source.is_some() || self.procedure_stratified_source.is_complete())
            && (self.procedure_template_stratified_source_model.is_some()
                || self.procedure_template.is_some()
                || self.procedure_stratified_source.is_complete())
            && self.procedure_stratified_source.is_complete()
            && (self.supernatant_destination.is_some()
                || self.procedure_supernatant_destination.is_complete())
            && (self.procedure_template_supernatant_destination_model.is_some()
                || self.procedure_template.is_some()
                || self.procedure_supernatant_destination.is_complete())
            && self.procedure_supernatant_destination.is_complete()
            && (self.transferred_with.is_some() || self.procedure_transferred_with.is_complete())
            && (self.transferred_with_model.is_some()
                || self.procedure_transferred_with.is_complete())
            && (self.procedure_template_transferred_with_model.is_some()
                || self.procedure_template.is_some()
                || self.procedure_transferred_with.is_complete())
            && self.procedure_transferred_with.is_complete()
            && (self.pipette_tip_model.is_some() || self.procedure_pipette_tip.is_complete())
            && (self.procedure_template_pipette_tip_model.is_some()
                || self.procedure_template.is_some()
                || self.procedure_pipette_tip.is_complete())
            && self.procedure_pipette_tip.is_complete()
    }
}
/// Trait defining setters for attributes of an instance of
/// `SupernatantProcedure` or descendant tables.
pub trait SupernatantProcedureSettable: Sized {
    /// Attributes required to build the insertable.
    type Attributes;
    /// Sets the value of the `public.supernatant_procedures.procedure_template`
    /// column.
    ///
    /// # Arguments
    /// * `procedure_template`: The value to set for the
    ///   `public.supernatant_procedures.procedure_template` column.
    ///
    /// # Implementation details
    /// This method accepts a reference to a generic value which can be
    /// converted to the required type for the column. This allows passing
    /// values of different types, as long as they can be converted to the
    /// required type using the `TryFrom` trait. The method, additionally,
    /// employs same-as and inferred same-as rules to ensure that the
    /// schema-defined ancestral tables and associated table values associated
    /// to the current column (if any) are also set appropriately.
    ///
    /// # Errors
    /// * If the provided value cannot be converted to the required type `i32`.
    /// * If the provided value does not pass schema-defined validation.
    fn procedure_template(
        self,
        procedure_template: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the `public.supernatant_procedures.stratified_source`
    /// column.
    ///
    /// # Arguments
    /// * `stratified_source`: The value to set for the
    ///   `public.supernatant_procedures.stratified_source` column.
    ///
    /// # Implementation details
    /// This method accepts a reference to a generic value which can be
    /// converted to the required type for the column. This allows passing
    /// values of different types, as long as they can be converted to the
    /// required type using the `TryFrom` trait. The method, additionally,
    /// employs same-as and inferred same-as rules to ensure that the
    /// schema-defined ancestral tables and associated table values associated
    /// to the current column (if any) are also set appropriately.
    ///
    /// # Errors
    /// * If the provided value cannot be converted to the required type
    ///   `::rosetta_uuid::Uuid`.
    /// * If the provided value does not pass schema-defined validation.
    fn stratified_source(
        self,
        stratified_source: ::rosetta_uuid::Uuid,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the
    /// `public.supernatant_procedures.
    /// procedure_template_stratified_source_model` column.
    ///
    /// # Arguments
    /// * `procedure_template_stratified_source_model`: The value to set for the
    ///   `public.supernatant_procedures.
    ///   procedure_template_stratified_source_model` column.
    ///
    /// # Implementation details
    /// This method accepts a reference to a generic value which can be
    /// converted to the required type for the column. This allows passing
    /// values of different types, as long as they can be converted to the
    /// required type using the `TryFrom` trait. The method, additionally,
    /// employs same-as and inferred same-as rules to ensure that the
    /// schema-defined ancestral tables and associated table values associated
    /// to the current column (if any) are also set appropriately.
    ///
    /// # Errors
    /// * If the provided value cannot be converted to the required type `i32`.
    /// * If the provided value does not pass schema-defined validation.
    fn procedure_template_stratified_source_model(
        self,
        procedure_template_stratified_source_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the
    /// `public.supernatant_procedures.procedure_stratified_source` column.
    ///
    /// # Arguments
    /// * `procedure_stratified_source`: The value to set for the
    ///   `public.supernatant_procedures.procedure_stratified_source` column.
    ///
    /// # Implementation details
    /// This method accepts a reference to a generic value which can be
    /// converted to the required type for the column. This allows passing
    /// values of different types, as long as they can be converted to the
    /// required type using the `TryFrom` trait. The method, additionally,
    /// employs same-as and inferred same-as rules to ensure that the
    /// schema-defined ancestral tables and associated table values associated
    /// to the current column (if any) are also set appropriately.
    ///
    /// # Errors
    /// * If the provided value cannot be converted to the required type
    ///   `::rosetta_uuid::Uuid`.
    /// * If the provided value does not pass schema-defined validation.
    fn procedure_stratified_source<PSS>(
        self,
        procedure_stratified_source: PSS,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PSS: Into<
            web_common_traits::database::IdOrBuilder<
                ::rosetta_uuid::Uuid,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
            >,
        >;
    /// Sets the value of the
    /// `public.supernatant_procedures.supernatant_destination` column.
    ///
    /// # Arguments
    /// * `supernatant_destination`: The value to set for the
    ///   `public.supernatant_procedures.supernatant_destination` column.
    ///
    /// # Implementation details
    /// This method accepts a reference to a generic value which can be
    /// converted to the required type for the column. This allows passing
    /// values of different types, as long as they can be converted to the
    /// required type using the `TryFrom` trait. The method, additionally,
    /// employs same-as and inferred same-as rules to ensure that the
    /// schema-defined ancestral tables and associated table values associated
    /// to the current column (if any) are also set appropriately.
    ///
    /// # Errors
    /// * If the provided value cannot be converted to the required type
    ///   `::rosetta_uuid::Uuid`.
    /// * If the provided value does not pass schema-defined validation.
    fn supernatant_destination(
        self,
        supernatant_destination: ::rosetta_uuid::Uuid,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the
    /// `public.supernatant_procedures.
    /// procedure_template_supernatant_destination_model` column.
    ///
    /// # Arguments
    /// * `procedure_template_supernatant_destination_model`: The value to set
    ///   for the `public.supernatant_procedures.
    ///   procedure_template_supernatant_destination_model` column.
    ///
    /// # Implementation details
    /// This method accepts a reference to a generic value which can be
    /// converted to the required type for the column. This allows passing
    /// values of different types, as long as they can be converted to the
    /// required type using the `TryFrom` trait. The method, additionally,
    /// employs same-as and inferred same-as rules to ensure that the
    /// schema-defined ancestral tables and associated table values associated
    /// to the current column (if any) are also set appropriately.
    ///
    /// # Errors
    /// * If the provided value cannot be converted to the required type `i32`.
    /// * If the provided value does not pass schema-defined validation.
    fn procedure_template_supernatant_destination_model(
        self,
        procedure_template_supernatant_destination_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the
    /// `public.supernatant_procedures.procedure_supernatant_destination`
    /// column.
    ///
    /// # Arguments
    /// * `procedure_supernatant_destination`: The value to set for the
    ///   `public.supernatant_procedures.procedure_supernatant_destination`
    ///   column.
    ///
    /// # Implementation details
    /// This method accepts a reference to a generic value which can be
    /// converted to the required type for the column. This allows passing
    /// values of different types, as long as they can be converted to the
    /// required type using the `TryFrom` trait. The method, additionally,
    /// employs same-as and inferred same-as rules to ensure that the
    /// schema-defined ancestral tables and associated table values associated
    /// to the current column (if any) are also set appropriately.
    ///
    /// # Errors
    /// * If the provided value cannot be converted to the required type
    ///   `::rosetta_uuid::Uuid`.
    /// * If the provided value does not pass schema-defined validation.
    fn procedure_supernatant_destination<PSD>(
        self,
        procedure_supernatant_destination: PSD,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PSD: Into<
            web_common_traits::database::IdOrBuilder<
                ::rosetta_uuid::Uuid,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
            >,
        >;
    /// Sets the value of the `public.supernatant_procedures.transferred_with`
    /// column.
    ///
    /// # Arguments
    /// * `transferred_with`: The value to set for the
    ///   `public.supernatant_procedures.transferred_with` column.
    ///
    /// # Implementation details
    /// This method accepts a reference to a generic value which can be
    /// converted to the required type for the column. This allows passing
    /// values of different types, as long as they can be converted to the
    /// required type using the `TryFrom` trait. The method, additionally,
    /// employs same-as and inferred same-as rules to ensure that the
    /// schema-defined ancestral tables and associated table values associated
    /// to the current column (if any) are also set appropriately.
    ///
    /// # Errors
    /// * If the provided value cannot be converted to the required type
    ///   `::rosetta_uuid::Uuid`.
    /// * If the provided value does not pass schema-defined validation.
    fn transferred_with(
        self,
        transferred_with: ::rosetta_uuid::Uuid,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the
    /// `public.supernatant_procedures.transferred_with_model` column.
    ///
    /// # Arguments
    /// * `transferred_with_model`: The value to set for the
    ///   `public.supernatant_procedures.transferred_with_model` column.
    ///
    /// # Implementation details
    /// This method accepts a reference to a generic value which can be
    /// converted to the required type for the column. This allows passing
    /// values of different types, as long as they can be converted to the
    /// required type using the `TryFrom` trait. The method, additionally,
    /// employs same-as and inferred same-as rules to ensure that the
    /// schema-defined ancestral tables and associated table values associated
    /// to the current column (if any) are also set appropriately.
    ///
    /// # Errors
    /// * If the provided value cannot be converted to the required type `i32`.
    /// * If the provided value does not pass schema-defined validation.
    fn transferred_with_model(
        self,
        transferred_with_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the
    /// `public.supernatant_procedures.
    /// procedure_template_transferred_with_model` column.
    ///
    /// # Arguments
    /// * `procedure_template_transferred_with_model`: The value to set for the
    ///   `public.supernatant_procedures.
    ///   procedure_template_transferred_with_model` column.
    ///
    /// # Implementation details
    /// This method accepts a reference to a generic value which can be
    /// converted to the required type for the column. This allows passing
    /// values of different types, as long as they can be converted to the
    /// required type using the `TryFrom` trait. The method, additionally,
    /// employs same-as and inferred same-as rules to ensure that the
    /// schema-defined ancestral tables and associated table values associated
    /// to the current column (if any) are also set appropriately.
    ///
    /// # Errors
    /// * If the provided value cannot be converted to the required type `i32`.
    /// * If the provided value does not pass schema-defined validation.
    fn procedure_template_transferred_with_model(
        self,
        procedure_template_transferred_with_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the
    /// `public.supernatant_procedures.procedure_transferred_with` column.
    ///
    /// # Arguments
    /// * `procedure_transferred_with`: The value to set for the
    ///   `public.supernatant_procedures.procedure_transferred_with` column.
    ///
    /// # Implementation details
    /// This method accepts a reference to a generic value which can be
    /// converted to the required type for the column. This allows passing
    /// values of different types, as long as they can be converted to the
    /// required type using the `TryFrom` trait. The method, additionally,
    /// employs same-as and inferred same-as rules to ensure that the
    /// schema-defined ancestral tables and associated table values associated
    /// to the current column (if any) are also set appropriately.
    ///
    /// # Errors
    /// * If the provided value cannot be converted to the required type
    ///   `::rosetta_uuid::Uuid`.
    /// * If the provided value does not pass schema-defined validation.
    fn procedure_transferred_with<PTW>(
        self,
        procedure_transferred_with: PTW,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PTW: Into<
            web_common_traits::database::IdOrBuilder<
                ::rosetta_uuid::Uuid,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
            >,
        >;
    /// Sets the value of the `public.supernatant_procedures.pipette_tip_model`
    /// column.
    ///
    /// # Arguments
    /// * `pipette_tip_model`: The value to set for the
    ///   `public.supernatant_procedures.pipette_tip_model` column.
    ///
    /// # Implementation details
    /// This method accepts a reference to a generic value which can be
    /// converted to the required type for the column. This allows passing
    /// values of different types, as long as they can be converted to the
    /// required type using the `TryFrom` trait. The method, additionally,
    /// employs same-as and inferred same-as rules to ensure that the
    /// schema-defined ancestral tables and associated table values associated
    /// to the current column (if any) are also set appropriately.
    ///
    /// # Errors
    /// * If the provided value cannot be converted to the required type `i32`.
    /// * If the provided value does not pass schema-defined validation.
    fn pipette_tip_model(
        self,
        pipette_tip_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the
    /// `public.supernatant_procedures.procedure_template_pipette_tip_model`
    /// column.
    ///
    /// # Arguments
    /// * `procedure_template_pipette_tip_model`: The value to set for the
    ///   `public.supernatant_procedures.procedure_template_pipette_tip_model`
    ///   column.
    ///
    /// # Implementation details
    /// This method accepts a reference to a generic value which can be
    /// converted to the required type for the column. This allows passing
    /// values of different types, as long as they can be converted to the
    /// required type using the `TryFrom` trait. The method, additionally,
    /// employs same-as and inferred same-as rules to ensure that the
    /// schema-defined ancestral tables and associated table values associated
    /// to the current column (if any) are also set appropriately.
    ///
    /// # Errors
    /// * If the provided value cannot be converted to the required type `i32`.
    /// * If the provided value does not pass schema-defined validation.
    fn procedure_template_pipette_tip_model(
        self,
        procedure_template_pipette_tip_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the
    /// `public.supernatant_procedures.procedure_pipette_tip` column.
    ///
    /// # Arguments
    /// * `procedure_pipette_tip`: The value to set for the
    ///   `public.supernatant_procedures.procedure_pipette_tip` column.
    ///
    /// # Implementation details
    /// This method accepts a reference to a generic value which can be
    /// converted to the required type for the column. This allows passing
    /// values of different types, as long as they can be converted to the
    /// required type using the `TryFrom` trait. The method, additionally,
    /// employs same-as and inferred same-as rules to ensure that the
    /// schema-defined ancestral tables and associated table values associated
    /// to the current column (if any) are also set appropriately.
    ///
    /// # Errors
    /// * If the provided value cannot be converted to the required type
    ///   `::rosetta_uuid::Uuid`.
    /// * If the provided value does not pass schema-defined validation.
    fn procedure_pipette_tip<PPT>(
        self,
        procedure_pipette_tip: PPT,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PPT: Into<
            web_common_traits::database::IdOrBuilder<
                ::rosetta_uuid::Uuid,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
            >,
        >;
}
impl<
    Procedure: crate::codegen::structs_codegen::tables::insertables::ProcedureSettable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute,
        >,
> SupernatantProcedureSettable for InsertableSupernatantProcedureBuilder<Procedure>
{
    type Attributes =
        crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureAttribute;
    /// Sets the value of the `public.supernatant_procedures.procedure_template`
    /// column.
    ///
    /// # Implementation notes
    /// This method also set the values of other columns, due to
    /// same-as relationships or inferred values.
    ///
    /// ## Mermaid illustration
    ///
    /// ```mermaid
    /// flowchart BT
    /// classDef column-of-interest stroke: #f0746c,fill: #f49f9a
    /// classDef directly-involved-column stroke: #6c74f0,fill: #9a9ff4
    /// classDef undirectly-involved-column stroke: #a7eff0,stroke-dasharray: 5, 5,fill: #d2f6f7
    /// subgraph v7 ["`procedure_assets`"]
    ///    v6@{shape: rounded, label: "procedure_template_asset_model"}
    /// class v6 undirectly-involved-column
    /// end
    /// subgraph v8 ["`procedures`"]
    ///    v0@{shape: rounded, label: "procedure_template"}
    /// class v0 directly-involved-column
    /// end
    /// subgraph v9 ["`supernatant_procedures`"]
    ///    v1@{shape: rounded, label: "procedure_template"}
    /// class v1 column-of-interest
    ///    v2@{shape: rounded, label: "procedure_template_pipette_tip_model"}
    /// class v2 directly-involved-column
    ///    v4@{shape: rounded, label: "procedure_template_supernatant_destination_model"}
    /// class v4 directly-involved-column
    ///    v5@{shape: rounded, label: "procedure_template_transferred_with_model"}
    /// class v5 directly-involved-column
    ///    v3@{shape: rounded, label: "procedure_template_stratified_source_model"}
    /// class v3 directly-involved-column
    /// end
    /// v1 --->|"`ancestral same as`"| v0
    /// v1 -.->|"`foreign defines`"| v2
    /// v1 -.->|"`foreign defines`"| v3
    /// v1 -.->|"`foreign defines`"| v4
    /// v1 -.->|"`foreign defines`"| v5
    /// v2 --->|"`associated same as`"| v6
    /// v3 --->|"`associated same as`"| v6
    /// v4 --->|"`associated same as`"| v6
    /// v5 --->|"`associated same as`"| v6
    /// v9 --->|"`extends`"| v8
    /// v9 ---o|"`associated with`"| v7
    /// ```
    fn procedure_template(
        mut self,
        procedure_template: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.procedure = <Procedure as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::procedure_template(
                self.procedure,
                procedure_template,
            )
            .map_err(|err| {
                err.into_field_name(|attribute| Self::Attributes::Extension(
                    attribute.into(),
                ))
            })?;
        self.procedure_template = Some(procedure_template);
        Ok(self)
    }
    /// Sets the value of the `public.supernatant_procedures.stratified_source`
    /// column.
    ///
    /// # Implementation notes
    /// This method also set the values of other columns, due to
    /// same-as relationships or inferred values.
    ///
    /// ## Mermaid illustration
    ///
    /// ```mermaid
    /// flowchart BT
    /// classDef column-of-interest stroke: #f0746c,fill: #f49f9a
    /// classDef directly-involved-column stroke: #6c74f0,fill: #9a9ff4
    /// classDef undirectly-involved-column stroke: #a7eff0,stroke-dasharray: 5, 5,fill: #d2f6f7
    /// subgraph v4 ["`procedure_assets`"]
    ///    v3@{shape: rounded, label: "id"}
    /// class v3 undirectly-involved-column
    ///    v0@{shape: rounded, label: "asset"}
    /// class v0 directly-involved-column
    /// end
    /// subgraph v5 ["`supernatant_procedures`"]
    ///    v2@{shape: rounded, label: "stratified_source"}
    /// class v2 column-of-interest
    ///    v1@{shape: rounded, label: "procedure_stratified_source"}
    /// class v1 directly-involved-column
    /// end
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 -.->|"`foreign defines`"| v2
    /// v2 --->|"`associated same as`"| v0
    /// v5 ---o|"`associated with`"| v4
    /// ```
    fn stratified_source(
        mut self,
        stratified_source: ::rosetta_uuid::Uuid,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let web_common_traits::database::IdOrBuilder::Builder(procedure_stratified_source) =
            self.procedure_stratified_source
        {
            self.procedure_stratified_source = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset(
                    procedure_stratified_source,
                    Some(stratified_source),
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedureStratifiedSource(attribute)
                    })
                })?
                .into();
        }
        self.stratified_source = Some(stratified_source);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.supernatant_procedures.
    /// procedure_template_stratified_source_model` column.
    ///
    /// # Implementation notes
    /// This method also set the values of other columns, due to
    /// same-as relationships or inferred values.
    ///
    /// ## Mermaid illustration
    ///
    /// ```mermaid
    /// flowchart BT
    /// classDef column-of-interest stroke: #f0746c,fill: #f49f9a
    /// classDef directly-involved-column stroke: #6c74f0,fill: #9a9ff4
    /// classDef undirectly-involved-column stroke: #a7eff0,stroke-dasharray: 5, 5,fill: #d2f6f7
    /// subgraph v4 ["`procedure_assets`"]
    ///    v0@{shape: rounded, label: "procedure_template_asset_model"}
    /// class v0 directly-involved-column
    ///    v3@{shape: rounded, label: "id"}
    /// class v3 undirectly-involved-column
    /// end
    /// subgraph v5 ["`supernatant_procedures`"]
    ///    v2@{shape: rounded, label: "procedure_template_stratified_source_model"}
    /// class v2 column-of-interest
    ///    v1@{shape: rounded, label: "procedure_stratified_source"}
    /// class v1 directly-involved-column
    /// end
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 -.->|"`foreign defines`"| v2
    /// v2 --->|"`associated same as`"| v0
    /// v5 ---o|"`associated with`"| v4
    /// ```
    fn procedure_template_stratified_source_model(
        mut self,
        procedure_template_stratified_source_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let web_common_traits::database::IdOrBuilder::Builder(procedure_stratified_source) =
            self.procedure_stratified_source
        {
            self.procedure_stratified_source = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure_template_asset_model(
                    procedure_stratified_source,
                    procedure_template_stratified_source_model,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedureStratifiedSource(attribute)
                    })
                })?
                .into();
        }
        self.procedure_template_stratified_source_model =
            Some(procedure_template_stratified_source_model);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.supernatant_procedures.procedure_stratified_source` column.
    ///
    /// # Implementation notes
    /// This method also set the values of other columns, due to
    /// same-as relationships or inferred values.
    ///
    /// ## Mermaid illustration
    ///
    /// ```mermaid
    /// flowchart BT
    /// classDef column-of-interest stroke: #f0746c,fill: #f49f9a
    /// classDef directly-involved-column stroke: #6c74f0,fill: #9a9ff4
    /// classDef undirectly-involved-column stroke: #a7eff0,stroke-dasharray: 5, 5,fill: #d2f6f7
    /// subgraph v6 ["`procedure_assets`"]
    ///    v0@{shape: rounded, label: "asset"}
    /// class v0 directly-involved-column
    ///    v5@{shape: rounded, label: "id"}
    /// class v5 undirectly-involved-column
    ///    v1@{shape: rounded, label: "procedure_template_asset_model"}
    /// class v1 directly-involved-column
    /// end
    /// subgraph v7 ["`supernatant_procedures`"]
    ///    v3@{shape: rounded, label: "procedure_template_stratified_source_model"}
    /// class v3 directly-involved-column
    ///    v4@{shape: rounded, label: "stratified_source"}
    /// class v4 directly-involved-column
    ///    v2@{shape: rounded, label: "procedure_stratified_source"}
    /// class v2 column-of-interest
    /// end
    /// v2 --->|"`associated same as`"| v5
    /// v2 --->|"`associated same as`"| v5
    /// v2 --->|"`associated same as`"| v5
    /// v2 -.->|"`foreign defines`"| v3
    /// v2 -.->|"`foreign defines`"| v4
    /// v3 --->|"`associated same as`"| v1
    /// v4 --->|"`associated same as`"| v0
    /// v7 ---o|"`associated with`"| v6
    /// ```
    fn procedure_stratified_source<PSS>(
        mut self,
        procedure_stratified_source: PSS,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PSS: Into<
            web_common_traits::database::IdOrBuilder<
                ::rosetta_uuid::Uuid,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
            >,
        >,
    {
        let mut procedure_stratified_source = procedure_stratified_source.into();
        if let web_common_traits::database::IdOrBuilder::Builder(builder) =
            procedure_stratified_source
        {
            procedure_stratified_source = if let (
                Some(procedure_template_stratified_source_model),
                Some(procedure_template_asset_model),
            ) = (
                self.procedure_template_stratified_source_model,
                builder.procedure_template_asset_model,
            ) {
                if procedure_template_stratified_source_model != procedure_template_asset_model {
                    return Err(web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            Self::Attributes::ProcedureTemplateStratifiedSourceModel,
                        ),
                    ));
                }
                builder.into()
            } else if let Some(procedure_template_asset_model) =
                builder.procedure_template_asset_model
            {
                self.procedure_template_stratified_source_model =
                    Some(procedure_template_asset_model);
                builder.into()
            } else if let Some(procedure_template_stratified_source_model) =
                self.procedure_template_stratified_source_model
            {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure_template_asset_model(
                        builder,
                        procedure_template_stratified_source_model,
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            Self::Attributes::ProcedureStratifiedSource(attribute)
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        if let web_common_traits::database::IdOrBuilder::Builder(builder) =
            procedure_stratified_source
        {
            procedure_stratified_source = if let (Some(stratified_source), Some(asset)) =
                (self.stratified_source, builder.asset)
            {
                if stratified_source != asset {
                    return Err(web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            Self::Attributes::StratifiedSource,
                        ),
                    ));
                }
                builder.into()
            } else if let Some(asset) = builder.asset {
                self.stratified_source = Some(asset);
                builder.into()
            } else if let Some(stratified_source) = self.stratified_source {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset(
                        builder,
                        Some(stratified_source),
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            Self::Attributes::ProcedureStratifiedSource(attribute)
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        self.procedure_stratified_source = procedure_stratified_source;
        Ok(self)
    }
    /// Sets the value of the
    /// `public.supernatant_procedures.supernatant_destination` column.
    ///
    /// # Implementation notes
    /// This method also set the values of other columns, due to
    /// same-as relationships or inferred values.
    ///
    /// ## Mermaid illustration
    ///
    /// ```mermaid
    /// flowchart BT
    /// classDef column-of-interest stroke: #f0746c,fill: #f49f9a
    /// classDef directly-involved-column stroke: #6c74f0,fill: #9a9ff4
    /// classDef undirectly-involved-column stroke: #a7eff0,stroke-dasharray: 5, 5,fill: #d2f6f7
    /// subgraph v4 ["`procedure_assets`"]
    ///    v0@{shape: rounded, label: "asset"}
    /// class v0 directly-involved-column
    ///    v3@{shape: rounded, label: "id"}
    /// class v3 undirectly-involved-column
    /// end
    /// subgraph v5 ["`supernatant_procedures`"]
    ///    v1@{shape: rounded, label: "procedure_supernatant_destination"}
    /// class v1 directly-involved-column
    ///    v2@{shape: rounded, label: "supernatant_destination"}
    /// class v2 column-of-interest
    /// end
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 -.->|"`foreign defines`"| v2
    /// v2 --->|"`associated same as`"| v0
    /// v5 ---o|"`associated with`"| v4
    /// ```
    fn supernatant_destination(
        mut self,
        supernatant_destination: ::rosetta_uuid::Uuid,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let web_common_traits::database::IdOrBuilder::Builder(
            procedure_supernatant_destination,
        ) = self.procedure_supernatant_destination
        {
            self.procedure_supernatant_destination = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset(
                    procedure_supernatant_destination,
                    Some(supernatant_destination),
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedureSupernatantDestination(attribute)
                    })
                })?
                .into();
        }
        self.supernatant_destination = Some(supernatant_destination);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.supernatant_procedures.
    /// procedure_template_supernatant_destination_model` column.
    ///
    /// # Implementation notes
    /// This method also set the values of other columns, due to
    /// same-as relationships or inferred values.
    ///
    /// ## Mermaid illustration
    ///
    /// ```mermaid
    /// flowchart BT
    /// classDef column-of-interest stroke: #f0746c,fill: #f49f9a
    /// classDef directly-involved-column stroke: #6c74f0,fill: #9a9ff4
    /// classDef undirectly-involved-column stroke: #a7eff0,stroke-dasharray: 5, 5,fill: #d2f6f7
    /// subgraph v4 ["`procedure_assets`"]
    ///    v0@{shape: rounded, label: "procedure_template_asset_model"}
    /// class v0 directly-involved-column
    ///    v3@{shape: rounded, label: "id"}
    /// class v3 undirectly-involved-column
    /// end
    /// subgraph v5 ["`supernatant_procedures`"]
    ///    v2@{shape: rounded, label: "procedure_template_supernatant_destination_model"}
    /// class v2 column-of-interest
    ///    v1@{shape: rounded, label: "procedure_supernatant_destination"}
    /// class v1 directly-involved-column
    /// end
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 -.->|"`foreign defines`"| v2
    /// v2 --->|"`associated same as`"| v0
    /// v5 ---o|"`associated with`"| v4
    /// ```
    fn procedure_template_supernatant_destination_model(
        mut self,
        procedure_template_supernatant_destination_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let web_common_traits::database::IdOrBuilder::Builder(
            procedure_supernatant_destination,
        ) = self.procedure_supernatant_destination
        {
            self.procedure_supernatant_destination = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure_template_asset_model(
                    procedure_supernatant_destination,
                    procedure_template_supernatant_destination_model,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedureSupernatantDestination(attribute)
                    })
                })?
                .into();
        }
        self.procedure_template_supernatant_destination_model =
            Some(procedure_template_supernatant_destination_model);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.supernatant_procedures.procedure_supernatant_destination`
    /// column.
    ///
    /// # Implementation notes
    /// This method also set the values of other columns, due to
    /// same-as relationships or inferred values.
    ///
    /// ## Mermaid illustration
    ///
    /// ```mermaid
    /// flowchart BT
    /// classDef column-of-interest stroke: #f0746c,fill: #f49f9a
    /// classDef directly-involved-column stroke: #6c74f0,fill: #9a9ff4
    /// classDef undirectly-involved-column stroke: #a7eff0,stroke-dasharray: 5, 5,fill: #d2f6f7
    /// subgraph v6 ["`procedure_assets`"]
    ///    v5@{shape: rounded, label: "id"}
    /// class v5 undirectly-involved-column
    ///    v0@{shape: rounded, label: "asset"}
    /// class v0 directly-involved-column
    ///    v1@{shape: rounded, label: "procedure_template_asset_model"}
    /// class v1 directly-involved-column
    /// end
    /// subgraph v7 ["`supernatant_procedures`"]
    ///    v3@{shape: rounded, label: "procedure_template_supernatant_destination_model"}
    /// class v3 directly-involved-column
    ///    v4@{shape: rounded, label: "supernatant_destination"}
    /// class v4 directly-involved-column
    ///    v2@{shape: rounded, label: "procedure_supernatant_destination"}
    /// class v2 column-of-interest
    /// end
    /// v2 --->|"`associated same as`"| v5
    /// v2 --->|"`associated same as`"| v5
    /// v2 --->|"`associated same as`"| v5
    /// v2 -.->|"`foreign defines`"| v3
    /// v2 -.->|"`foreign defines`"| v4
    /// v3 --->|"`associated same as`"| v1
    /// v4 --->|"`associated same as`"| v0
    /// v7 ---o|"`associated with`"| v6
    /// ```
    fn procedure_supernatant_destination<PSD>(
        mut self,
        procedure_supernatant_destination: PSD,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PSD: Into<
            web_common_traits::database::IdOrBuilder<
                ::rosetta_uuid::Uuid,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
            >,
        >,
    {
        let mut procedure_supernatant_destination = procedure_supernatant_destination.into();
        if let web_common_traits::database::IdOrBuilder::Builder(builder) =
            procedure_supernatant_destination
        {
            procedure_supernatant_destination = if let (
                Some(procedure_template_supernatant_destination_model),
                Some(procedure_template_asset_model),
            ) = (
                self.procedure_template_supernatant_destination_model,
                builder.procedure_template_asset_model,
            ) {
                if procedure_template_supernatant_destination_model
                    != procedure_template_asset_model
                {
                    return Err(web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            Self::Attributes::ProcedureTemplateSupernatantDestinationModel,
                        ),
                    ));
                }
                builder.into()
            } else if let Some(procedure_template_asset_model) =
                builder.procedure_template_asset_model
            {
                self.procedure_template_supernatant_destination_model =
                    Some(procedure_template_asset_model);
                builder.into()
            } else if let Some(procedure_template_supernatant_destination_model) =
                self.procedure_template_supernatant_destination_model
            {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure_template_asset_model(
                        builder,
                        procedure_template_supernatant_destination_model,
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            Self::Attributes::ProcedureSupernatantDestination(attribute)
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        if let web_common_traits::database::IdOrBuilder::Builder(builder) =
            procedure_supernatant_destination
        {
            procedure_supernatant_destination = if let (
                Some(supernatant_destination),
                Some(asset),
            ) = (self.supernatant_destination, builder.asset)
            {
                if supernatant_destination != asset {
                    return Err(web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            Self::Attributes::SupernatantDestination,
                        ),
                    ));
                }
                builder.into()
            } else if let Some(asset) = builder.asset {
                self.supernatant_destination = Some(asset);
                builder.into()
            } else if let Some(supernatant_destination) = self.supernatant_destination {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset(
                        builder,
                        Some(supernatant_destination),
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            Self::Attributes::ProcedureSupernatantDestination(attribute)
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        self.procedure_supernatant_destination = procedure_supernatant_destination;
        Ok(self)
    }
    /// Sets the value of the `public.supernatant_procedures.transferred_with`
    /// column.
    ///
    /// # Implementation notes
    /// This method also set the values of other columns, due to
    /// same-as relationships or inferred values.
    ///
    /// ## Mermaid illustration
    ///
    /// ```mermaid
    /// flowchart BT
    /// classDef column-of-interest stroke: #f0746c,fill: #f49f9a
    /// classDef directly-involved-column stroke: #6c74f0,fill: #9a9ff4
    /// classDef undirectly-involved-column stroke: #a7eff0,stroke-dasharray: 5, 5,fill: #d2f6f7
    /// subgraph v4 ["`procedure_assets`"]
    ///    v0@{shape: rounded, label: "asset"}
    /// class v0 directly-involved-column
    ///    v3@{shape: rounded, label: "id"}
    /// class v3 undirectly-involved-column
    /// end
    /// subgraph v5 ["`supernatant_procedures`"]
    ///    v2@{shape: rounded, label: "transferred_with"}
    /// class v2 column-of-interest
    ///    v1@{shape: rounded, label: "procedure_transferred_with"}
    /// class v1 directly-involved-column
    /// end
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 -.->|"`foreign defines`"| v2
    /// v2 --->|"`associated same as`"| v0
    /// v5 ---o|"`associated with`"| v4
    /// ```
    fn transferred_with(
        mut self,
        transferred_with: ::rosetta_uuid::Uuid,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let web_common_traits::database::IdOrBuilder::Builder(procedure_transferred_with) =
            self.procedure_transferred_with
        {
            self.procedure_transferred_with = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset(
                    procedure_transferred_with,
                    Some(transferred_with),
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedureTransferredWith(attribute)
                    })
                })?
                .into();
        }
        self.transferred_with = Some(transferred_with);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.supernatant_procedures.transferred_with_model` column.
    ///
    /// # Implementation notes
    /// This method also set the values of other columns, due to
    /// same-as relationships or inferred values.
    ///
    /// ## Mermaid illustration
    ///
    /// ```mermaid
    /// flowchart BT
    /// classDef column-of-interest stroke: #f0746c,fill: #f49f9a
    /// classDef directly-involved-column stroke: #6c74f0,fill: #9a9ff4
    /// classDef undirectly-involved-column stroke: #a7eff0,stroke-dasharray: 5, 5,fill: #d2f6f7
    /// subgraph v4 ["`procedure_assets`"]
    ///    v3@{shape: rounded, label: "id"}
    /// class v3 undirectly-involved-column
    ///    v0@{shape: rounded, label: "asset_model"}
    /// class v0 directly-involved-column
    /// end
    /// subgraph v5 ["`supernatant_procedures`"]
    ///    v2@{shape: rounded, label: "transferred_with_model"}
    /// class v2 column-of-interest
    ///    v1@{shape: rounded, label: "procedure_transferred_with"}
    /// class v1 directly-involved-column
    /// end
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 -.->|"`foreign defines`"| v2
    /// v2 --->|"`associated same as`"| v0
    /// v5 ---o|"`associated with`"| v4
    /// ```
    fn transferred_with_model(
        mut self,
        transferred_with_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let web_common_traits::database::IdOrBuilder::Builder(procedure_transferred_with) =
            self.procedure_transferred_with
        {
            self.procedure_transferred_with = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset_model(
                    procedure_transferred_with,
                    transferred_with_model,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedureTransferredWith(attribute)
                    })
                })?
                .into();
        }
        self.transferred_with_model = Some(transferred_with_model);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.supernatant_procedures.
    /// procedure_template_transferred_with_model` column.
    ///
    /// # Implementation notes
    /// This method also set the values of other columns, due to
    /// same-as relationships or inferred values.
    ///
    /// ## Mermaid illustration
    ///
    /// ```mermaid
    /// flowchart BT
    /// classDef column-of-interest stroke: #f0746c,fill: #f49f9a
    /// classDef directly-involved-column stroke: #6c74f0,fill: #9a9ff4
    /// classDef undirectly-involved-column stroke: #a7eff0,stroke-dasharray: 5, 5,fill: #d2f6f7
    /// subgraph v4 ["`procedure_assets`"]
    ///    v3@{shape: rounded, label: "id"}
    /// class v3 undirectly-involved-column
    ///    v0@{shape: rounded, label: "procedure_template_asset_model"}
    /// class v0 directly-involved-column
    /// end
    /// subgraph v5 ["`supernatant_procedures`"]
    ///    v1@{shape: rounded, label: "procedure_template_transferred_with_model"}
    /// class v1 column-of-interest
    ///    v2@{shape: rounded, label: "procedure_transferred_with"}
    /// class v2 directly-involved-column
    /// end
    /// v1 --->|"`associated same as`"| v0
    /// v2 --->|"`associated same as`"| v3
    /// v2 --->|"`associated same as`"| v3
    /// v2 --->|"`associated same as`"| v3
    /// v2 --->|"`associated same as`"| v3
    /// v2 -.->|"`foreign defines`"| v1
    /// v5 ---o|"`associated with`"| v4
    /// ```
    fn procedure_template_transferred_with_model(
        mut self,
        procedure_template_transferred_with_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let web_common_traits::database::IdOrBuilder::Builder(procedure_transferred_with) =
            self.procedure_transferred_with
        {
            self.procedure_transferred_with = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure_template_asset_model(
                    procedure_transferred_with,
                    procedure_template_transferred_with_model,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedureTransferredWith(attribute)
                    })
                })?
                .into();
        }
        self.procedure_template_transferred_with_model =
            Some(procedure_template_transferred_with_model);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.supernatant_procedures.procedure_transferred_with` column.
    ///
    /// # Implementation notes
    /// This method also set the values of other columns, due to
    /// same-as relationships or inferred values.
    ///
    /// ## Mermaid illustration
    ///
    /// ```mermaid
    /// flowchart BT
    /// classDef column-of-interest stroke: #f0746c,fill: #f49f9a
    /// classDef directly-involved-column stroke: #6c74f0,fill: #9a9ff4
    /// classDef undirectly-involved-column stroke: #a7eff0,stroke-dasharray: 5, 5,fill: #d2f6f7
    /// subgraph v8 ["`procedure_assets`"]
    ///    v1@{shape: rounded, label: "asset_model"}
    /// class v1 directly-involved-column
    ///    v2@{shape: rounded, label: "procedure_template_asset_model"}
    /// class v2 directly-involved-column
    ///    v0@{shape: rounded, label: "asset"}
    /// class v0 directly-involved-column
    ///    v7@{shape: rounded, label: "id"}
    /// class v7 undirectly-involved-column
    /// end
    /// subgraph v9 ["`supernatant_procedures`"]
    ///    v5@{shape: rounded, label: "transferred_with"}
    /// class v5 directly-involved-column
    ///    v6@{shape: rounded, label: "transferred_with_model"}
    /// class v6 directly-involved-column
    ///    v4@{shape: rounded, label: "procedure_transferred_with"}
    /// class v4 column-of-interest
    ///    v3@{shape: rounded, label: "procedure_template_transferred_with_model"}
    /// class v3 directly-involved-column
    /// end
    /// v0 -.->|"`foreign defines`"| v1
    /// v3 --->|"`associated same as`"| v2
    /// v4 --->|"`associated same as`"| v7
    /// v4 --->|"`associated same as`"| v7
    /// v4 --->|"`associated same as`"| v7
    /// v4 --->|"`associated same as`"| v7
    /// v4 -.->|"`foreign defines`"| v3
    /// v4 -.->|"`foreign defines`"| v5
    /// v4 -.->|"`foreign defines`"| v6
    /// v5 --->|"`associated same as`"| v0
    /// v6 --->|"`associated same as`"| v1
    /// v9 ---o|"`associated with`"| v8
    /// ```
    fn procedure_transferred_with<PTW>(
        mut self,
        procedure_transferred_with: PTW,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PTW: Into<
            web_common_traits::database::IdOrBuilder<
                ::rosetta_uuid::Uuid,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
            >,
        >,
    {
        let mut procedure_transferred_with = procedure_transferred_with.into();
        if let web_common_traits::database::IdOrBuilder::Builder(builder) =
            procedure_transferred_with
        {
            procedure_transferred_with = if let (
                Some(procedure_template_transferred_with_model),
                Some(procedure_template_asset_model),
            ) = (
                self.procedure_template_transferred_with_model,
                builder.procedure_template_asset_model,
            ) {
                if procedure_template_transferred_with_model != procedure_template_asset_model {
                    return Err(web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            Self::Attributes::ProcedureTemplateTransferredWithModel,
                        ),
                    ));
                }
                builder.into()
            } else if let Some(procedure_template_asset_model) =
                builder.procedure_template_asset_model
            {
                self.procedure_template_transferred_with_model =
                    Some(procedure_template_asset_model);
                builder.into()
            } else if let Some(procedure_template_transferred_with_model) =
                self.procedure_template_transferred_with_model
            {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure_template_asset_model(
                        builder,
                        procedure_template_transferred_with_model,
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            Self::Attributes::ProcedureTransferredWith(attribute)
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        if let web_common_traits::database::IdOrBuilder::Builder(builder) =
            procedure_transferred_with
        {
            procedure_transferred_with = if let (Some(transferred_with_model), Some(asset_model)) =
                (self.transferred_with_model, builder.asset_model)
            {
                if transferred_with_model != asset_model {
                    return Err(web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            Self::Attributes::TransferredWithModel,
                        ),
                    ));
                }
                builder.into()
            } else if let Some(asset_model) = builder.asset_model {
                self.transferred_with_model = Some(asset_model);
                builder.into()
            } else if let Some(transferred_with_model) = self.transferred_with_model {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset_model(
                        builder,
                        transferred_with_model,
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            Self::Attributes::ProcedureTransferredWith(attribute)
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        if let web_common_traits::database::IdOrBuilder::Builder(builder) =
            procedure_transferred_with
        {
            procedure_transferred_with = if let (Some(transferred_with), Some(asset)) =
                (self.transferred_with, builder.asset)
            {
                if transferred_with != asset {
                    return Err(web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            Self::Attributes::TransferredWith,
                        ),
                    ));
                }
                builder.into()
            } else if let Some(asset) = builder.asset {
                self.transferred_with = Some(asset);
                builder.into()
            } else if let Some(transferred_with) = self.transferred_with {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset(
                        builder,
                        Some(transferred_with),
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            Self::Attributes::ProcedureTransferredWith(attribute)
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        self.procedure_transferred_with = procedure_transferred_with;
        Ok(self)
    }
    /// Sets the value of the `public.supernatant_procedures.pipette_tip_model`
    /// column.
    ///
    /// # Implementation notes
    /// This method also set the values of other columns, due to
    /// same-as relationships or inferred values.
    ///
    /// ## Mermaid illustration
    ///
    /// ```mermaid
    /// flowchart BT
    /// classDef column-of-interest stroke: #f0746c,fill: #f49f9a
    /// classDef directly-involved-column stroke: #6c74f0,fill: #9a9ff4
    /// classDef undirectly-involved-column stroke: #a7eff0,stroke-dasharray: 5, 5,fill: #d2f6f7
    /// subgraph v4 ["`procedure_assets`"]
    ///    v0@{shape: rounded, label: "asset_model"}
    /// class v0 directly-involved-column
    ///    v3@{shape: rounded, label: "id"}
    /// class v3 undirectly-involved-column
    /// end
    /// subgraph v5 ["`supernatant_procedures`"]
    ///    v2@{shape: rounded, label: "procedure_pipette_tip"}
    /// class v2 directly-involved-column
    ///    v1@{shape: rounded, label: "pipette_tip_model"}
    /// class v1 column-of-interest
    /// end
    /// v1 --->|"`associated same as`"| v0
    /// v1 --->|"`associated same as`"| v0
    /// v2 --->|"`associated same as`"| v3
    /// v2 --->|"`associated same as`"| v3
    /// v2 --->|"`associated same as`"| v3
    /// v2 --->|"`associated same as`"| v3
    /// v2 -.->|"`foreign defines`"| v1
    /// v2 -.->|"`foreign defines`"| v1
    /// v5 ---o|"`associated with`"| v4
    /// ```
    fn pipette_tip_model(
        mut self,
        pipette_tip_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let web_common_traits::database::IdOrBuilder::Builder(procedure_pipette_tip) =
            self.procedure_pipette_tip
        {
            self.procedure_pipette_tip = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset_model(
                    procedure_pipette_tip,
                    pipette_tip_model,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedurePipetteTip(attribute)
                    })
                })?
                .into();
        }
        if let web_common_traits::database::IdOrBuilder::Builder(procedure_pipette_tip) =
            self.procedure_pipette_tip
        {
            self.procedure_pipette_tip = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset_model(
                    procedure_pipette_tip,
                    pipette_tip_model,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedurePipetteTip(attribute)
                    })
                })?
                .into();
        }
        self.pipette_tip_model = Some(pipette_tip_model);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.supernatant_procedures.procedure_template_pipette_tip_model`
    /// column.
    ///
    /// # Implementation notes
    /// This method also set the values of other columns, due to
    /// same-as relationships or inferred values.
    ///
    /// ## Mermaid illustration
    ///
    /// ```mermaid
    /// flowchart BT
    /// classDef column-of-interest stroke: #f0746c,fill: #f49f9a
    /// classDef directly-involved-column stroke: #6c74f0,fill: #9a9ff4
    /// classDef undirectly-involved-column stroke: #a7eff0,stroke-dasharray: 5, 5,fill: #d2f6f7
    /// subgraph v4 ["`procedure_assets`"]
    ///    v0@{shape: rounded, label: "procedure_template_asset_model"}
    /// class v0 directly-involved-column
    ///    v3@{shape: rounded, label: "id"}
    /// class v3 undirectly-involved-column
    /// end
    /// subgraph v5 ["`supernatant_procedures`"]
    ///    v2@{shape: rounded, label: "procedure_template_pipette_tip_model"}
    /// class v2 column-of-interest
    ///    v1@{shape: rounded, label: "procedure_pipette_tip"}
    /// class v1 directly-involved-column
    /// end
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 -.->|"`foreign defines`"| v2
    /// v2 --->|"`associated same as`"| v0
    /// v5 ---o|"`associated with`"| v4
    /// ```
    fn procedure_template_pipette_tip_model(
        mut self,
        procedure_template_pipette_tip_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let web_common_traits::database::IdOrBuilder::Builder(procedure_pipette_tip) =
            self.procedure_pipette_tip
        {
            self.procedure_pipette_tip = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure_template_asset_model(
                    procedure_pipette_tip,
                    procedure_template_pipette_tip_model,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedurePipetteTip(attribute)
                    })
                })?
                .into();
        }
        self.procedure_template_pipette_tip_model = Some(procedure_template_pipette_tip_model);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.supernatant_procedures.procedure_pipette_tip` column.
    ///
    /// # Implementation notes
    /// This method also set the values of other columns, due to
    /// same-as relationships or inferred values.
    ///
    /// ## Mermaid illustration
    ///
    /// ```mermaid
    /// flowchart BT
    /// classDef column-of-interest stroke: #f0746c,fill: #f49f9a
    /// classDef directly-involved-column stroke: #6c74f0,fill: #9a9ff4
    /// classDef undirectly-involved-column stroke: #a7eff0,stroke-dasharray: 5, 5,fill: #d2f6f7
    /// subgraph v6 ["`procedure_assets`"]
    ///    v1@{shape: rounded, label: "procedure_template_asset_model"}
    /// class v1 directly-involved-column
    ///    v0@{shape: rounded, label: "asset_model"}
    /// class v0 directly-involved-column
    ///    v5@{shape: rounded, label: "id"}
    /// class v5 undirectly-involved-column
    /// end
    /// subgraph v7 ["`supernatant_procedures`"]
    ///    v3@{shape: rounded, label: "procedure_pipette_tip"}
    /// class v3 column-of-interest
    ///    v4@{shape: rounded, label: "procedure_template_pipette_tip_model"}
    /// class v4 directly-involved-column
    ///    v2@{shape: rounded, label: "pipette_tip_model"}
    /// class v2 directly-involved-column
    /// end
    /// v2 --->|"`associated same as`"| v0
    /// v2 --->|"`associated same as`"| v0
    /// v3 --->|"`associated same as`"| v5
    /// v3 --->|"`associated same as`"| v5
    /// v3 --->|"`associated same as`"| v5
    /// v3 --->|"`associated same as`"| v5
    /// v3 -.->|"`foreign defines`"| v2
    /// v3 -.->|"`foreign defines`"| v2
    /// v3 -.->|"`foreign defines`"| v4
    /// v4 --->|"`associated same as`"| v1
    /// v7 ---o|"`associated with`"| v6
    /// ```
    fn procedure_pipette_tip<PPT>(
        mut self,
        procedure_pipette_tip: PPT,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PPT: Into<
            web_common_traits::database::IdOrBuilder<
                ::rosetta_uuid::Uuid,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
            >,
        >,
    {
        let mut procedure_pipette_tip = procedure_pipette_tip.into();
        if let web_common_traits::database::IdOrBuilder::Builder(builder) = procedure_pipette_tip {
            procedure_pipette_tip = if let (Some(pipette_tip_model), Some(asset_model)) =
                (self.pipette_tip_model, builder.asset_model)
            {
                if pipette_tip_model != asset_model {
                    return Err(web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            Self::Attributes::PipetteTipModel,
                        ),
                    ));
                }
                builder.into()
            } else if let Some(asset_model) = builder.asset_model {
                self.pipette_tip_model = Some(asset_model);
                builder.into()
            } else if let Some(pipette_tip_model) = self.pipette_tip_model {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset_model(
                        builder,
                        pipette_tip_model,
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            Self::Attributes::ProcedurePipetteTip(attribute)
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        if let web_common_traits::database::IdOrBuilder::Builder(builder) = procedure_pipette_tip {
            procedure_pipette_tip = if let (Some(pipette_tip_model), Some(asset_model)) =
                (self.pipette_tip_model, builder.asset_model)
            {
                if pipette_tip_model != asset_model {
                    return Err(web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            Self::Attributes::PipetteTipModel,
                        ),
                    ));
                }
                builder.into()
            } else if let Some(asset_model) = builder.asset_model {
                self.pipette_tip_model = Some(asset_model);
                builder.into()
            } else if let Some(pipette_tip_model) = self.pipette_tip_model {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset_model(
                        builder,
                        pipette_tip_model,
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            Self::Attributes::ProcedurePipetteTip(attribute)
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        if let web_common_traits::database::IdOrBuilder::Builder(builder) = procedure_pipette_tip {
            procedure_pipette_tip = if let (
                Some(procedure_template_pipette_tip_model),
                Some(procedure_template_asset_model),
            ) =
                (self.procedure_template_pipette_tip_model, builder.procedure_template_asset_model)
            {
                if procedure_template_pipette_tip_model != procedure_template_asset_model {
                    return Err(web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            Self::Attributes::ProcedureTemplatePipetteTipModel,
                        ),
                    ));
                }
                builder.into()
            } else if let Some(procedure_template_asset_model) =
                builder.procedure_template_asset_model
            {
                self.procedure_template_pipette_tip_model = Some(procedure_template_asset_model);
                builder.into()
            } else if let Some(procedure_template_pipette_tip_model) =
                self.procedure_template_pipette_tip_model
            {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure_template_asset_model(
                        builder,
                        procedure_template_pipette_tip_model,
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            Self::Attributes::ProcedurePipetteTip(attribute)
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        self.procedure_pipette_tip = procedure_pipette_tip;
        Ok(self)
    }
}
impl<
    Procedure: crate::codegen::structs_codegen::tables::insertables::ProcedureSettable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute,
        >,
> crate::codegen::structs_codegen::tables::insertables::ProcedureSettable
for InsertableSupernatantProcedureBuilder<Procedure>
where
    Self: crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureSettable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureAttribute,
    >,
{
    type Attributes = crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureAttribute;
    #[inline]
    ///Sets the value of the `public.procedures.procedure` column.
    fn procedure(
        mut self,
        procedure: ::rosetta_uuid::Uuid,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.procedure = <Procedure as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::procedure(
                self.procedure,
                procedure,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| Self::Attributes::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    ///Sets the value of the `public.procedures.procedure_template` column.
    ///
    ///# Implementation notes
    ///This method also set the values of other columns, due to
    ///same-as relationships or inferred values.
    ///
    ///## Mermaid illustration
    ///
    ///```mermaid
    ///flowchart BT
    ///classDef column-of-interest stroke: #f0746c,fill: #f49f9a
    ///classDef directly-involved-column stroke: #6c74f0,fill: #9a9ff4
    ///subgraph v2 ["`procedures`"]
    ///    v0@{shape: rounded, label: "procedure_template"}
    ///class v0 column-of-interest
    ///end
    ///subgraph v3 ["`supernatant_procedures`"]
    ///    v1@{shape: rounded, label: "procedure_template"}
    ///class v1 directly-involved-column
    ///end
    ///v1 --->|"`ancestral same as`"| v0
    ///v3 --->|"`extends`"| v2
    ///```
    fn procedure_template(
        self,
        procedure_template: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        <Self as SupernatantProcedureSettable>::procedure_template(
            self,
            procedure_template,
        )
    }
    #[inline]
    ///Sets the value of the `public.procedures.parent_procedure` column.
    fn parent_procedure(
        mut self,
        parent_procedure: Option<::rosetta_uuid::Uuid>,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.procedure = <Procedure as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::parent_procedure(
                self.procedure,
                parent_procedure,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| Self::Attributes::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    ///Sets the value of the `public.procedures.parent_procedure_template` column.
    fn parent_procedure_template(
        mut self,
        parent_procedure_template: Option<i32>,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.procedure = <Procedure as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::parent_procedure_template(
                self.procedure,
                parent_procedure_template,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| Self::Attributes::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    ///Sets the value of the `public.procedures.predecessor_procedure` column.
    fn predecessor_procedure(
        mut self,
        predecessor_procedure: Option<::rosetta_uuid::Uuid>,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.procedure = <Procedure as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::predecessor_procedure(
                self.procedure,
                predecessor_procedure,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| Self::Attributes::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    ///Sets the value of the `public.procedures.predecessor_procedure_template` column.
    fn predecessor_procedure_template(
        mut self,
        predecessor_procedure_template: Option<i32>,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.procedure = <Procedure as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::predecessor_procedure_template(
                self.procedure,
                predecessor_procedure_template,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| Self::Attributes::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    ///Sets the value of the `public.procedures.created_by` column.
    fn created_by(
        mut self,
        created_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.procedure = <Procedure as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::created_by(
                self.procedure,
                created_by,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| Self::Attributes::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    ///Sets the value of the `public.procedures.created_at` column.
    fn created_at<CA>(
        mut self,
        created_at: CA,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        CA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::SingleFieldError: From<
            <CA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error,
        >,
    {
        self.procedure = <Procedure as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::created_at(
                self.procedure,
                created_at,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| Self::Attributes::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    ///Sets the value of the `public.procedures.updated_by` column.
    fn updated_by(
        mut self,
        updated_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.procedure = <Procedure as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::updated_by(
                self.procedure,
                updated_by,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| Self::Attributes::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    ///Sets the value of the `public.procedures.updated_at` column.
    fn updated_at<UA>(
        mut self,
        updated_at: UA,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        UA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::SingleFieldError: From<
            <UA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error,
        >,
    {
        self.procedure = <Procedure as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::updated_at(
                self.procedure,
                updated_at,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| Self::Attributes::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    ///Sets the value of the `public.procedures.number_of_completed_subprocedures` column.
    fn number_of_completed_subprocedures<NOCS>(
        mut self,
        number_of_completed_subprocedures: NOCS,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        NOCS: TryInto<i16>,
        validation_errors::SingleFieldError: From<<NOCS as TryInto<i16>>::Error>,
    {
        self.procedure = <Procedure as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::number_of_completed_subprocedures(
                self.procedure,
                number_of_completed_subprocedures,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| Self::Attributes::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
}
impl<Procedure> web_common_traits::database::MostConcreteTable
    for InsertableSupernatantProcedureBuilder<Procedure>
where
    Procedure: web_common_traits::database::MostConcreteTable,
{
    fn set_most_concrete_table(&mut self, table_name: &str) {
        self.procedure.set_most_concrete_table(table_name);
    }
}
impl<Procedure> web_common_traits::prelude::SetPrimaryKey
    for InsertableSupernatantProcedureBuilder<Procedure>
where
    Procedure: web_common_traits::prelude::SetPrimaryKey<PrimaryKey = ::rosetta_uuid::Uuid>,
{
    type PrimaryKey = ::rosetta_uuid::Uuid;
    fn set_primary_key(mut self, primary_key: Self::PrimaryKey) -> Self {
        self.procedure = self.procedure.set_primary_key(primary_key);
        self
    }
}
impl<Procedure, C> web_common_traits::database::TryInsertGeneric<C>
for InsertableSupernatantProcedureBuilder<Procedure>
where
    Self: web_common_traits::database::InsertableVariant<
        C,
        UserId = i32,
        Row = crate::codegen::structs_codegen::tables::supernatant_procedures::SupernatantProcedure,
        Error = web_common_traits::database::InsertError<SupernatantProcedureAttribute>,
    >,
    Procedure: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = ::rosetta_uuid::Uuid,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder: web_common_traits::database::TryInsertGeneric<
        C,
    >,
{
    type Attributes = SupernatantProcedureAttribute;
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<
        Self::PrimaryKey,
        web_common_traits::database::InsertError<Self::Attributes>,
    > {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::supernatant_procedures::SupernatantProcedure = self
            .insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
