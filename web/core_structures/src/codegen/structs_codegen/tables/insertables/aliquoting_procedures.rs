#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AliquotingProcedureExtensionAttribute {
    Procedure(crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute),
}
impl core::fmt::Display for AliquotingProcedureExtensionAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Procedure(e) => write!(f, "{e}"),
        }
    }
}
impl From<crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute>
    for AliquotingProcedureExtensionAttribute
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute,
    ) -> Self {
        Self::Procedure(attribute)
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AliquotingProcedureAttribute {
    Extension(AliquotingProcedureExtensionAttribute),
    Procedure,
    ProcedureTemplate,
    AliquotedWith,
    AliquotedWithModel,
    ProcedureTemplateAliquotedWithModel,
    ProcedureAliquotedWith(
        crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute,
    ),
    PipetteTipModel,
    ProcedureTemplatePipetteTipModel,
    ProcedurePipetteTip(
        crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute,
    ),
    AliquotedFrom,
    ProcedureTemplateAliquotedFromModel,
    ProcedureAliquotedFrom(
        crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute,
    ),
    AliquotedInto,
    ProcedureTemplateAliquotedIntoModel,
    ProcedureAliquotedInto(
        crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute,
    ),
}
impl core::str::FromStr for AliquotingProcedureAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ProcedureTemplate" => Ok(Self::ProcedureTemplate),
            "AliquotedWith" => Ok(Self::AliquotedWith),
            "AliquotedWithModel" => Ok(Self::AliquotedWithModel),
            "ProcedureTemplateAliquotedWithModel" => Ok(Self::ProcedureTemplateAliquotedWithModel),
            "ProcedureAliquotedWith" => Ok(Self::ProcedureAliquotedWith(
                crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute::Id,
            )),
            "PipetteTipModel" => Ok(Self::PipetteTipModel),
            "ProcedureTemplatePipetteTipModel" => Ok(Self::ProcedureTemplatePipetteTipModel),
            "ProcedurePipetteTip" => Ok(Self::ProcedurePipetteTip(
                crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute::Id,
            )),
            "AliquotedFrom" => Ok(Self::AliquotedFrom),
            "ProcedureTemplateAliquotedFromModel" => Ok(Self::ProcedureTemplateAliquotedFromModel),
            "ProcedureAliquotedFrom" => Ok(Self::ProcedureAliquotedFrom(
                crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute::Id,
            )),
            "AliquotedInto" => Ok(Self::AliquotedInto),
            "ProcedureTemplateAliquotedIntoModel" => Ok(Self::ProcedureTemplateAliquotedIntoModel),
            "ProcedureAliquotedInto" => Ok(Self::ProcedureAliquotedInto(
                crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute::Id,
            )),
            "procedure_template" => Ok(Self::ProcedureTemplate),
            "aliquoted_with" => Ok(Self::AliquotedWith),
            "aliquoted_with_model" => Ok(Self::AliquotedWithModel),
            "procedure_template_aliquoted_with_model" => {
                Ok(Self::ProcedureTemplateAliquotedWithModel)
            }
            "procedure_aliquoted_with" => Ok(Self::ProcedureAliquotedWith(
                crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute::Id,
            )),
            "pipette_tip_model" => Ok(Self::PipetteTipModel),
            "procedure_template_pipette_tip_model" => Ok(Self::ProcedureTemplatePipetteTipModel),
            "procedure_pipette_tip" => Ok(Self::ProcedurePipetteTip(
                crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute::Id,
            )),
            "aliquoted_from" => Ok(Self::AliquotedFrom),
            "procedure_template_aliquoted_from_model" => {
                Ok(Self::ProcedureTemplateAliquotedFromModel)
            }
            "procedure_aliquoted_from" => Ok(Self::ProcedureAliquotedFrom(
                crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute::Id,
            )),
            "aliquoted_into" => Ok(Self::AliquotedInto),
            "procedure_template_aliquoted_into_model" => {
                Ok(Self::ProcedureTemplateAliquotedIntoModel)
            }
            "procedure_aliquoted_into" => Ok(Self::ProcedureAliquotedInto(
                crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute::Id,
            )),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl
    web_common_traits::database::DefaultExtensionAttribute<
        crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute,
    > for AliquotingProcedureAttribute
{
    /// Returns the default value for the target attribute.
    fn target_default() -> Self {
        Self::Extension(
            crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute::Procedure
                .into(),
        )
    }
}
impl
    web_common_traits::database::FromExtensionAttribute<
        crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute,
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureBuilder,
    > for AliquotingProcedureAttribute
{
    type EffectiveExtensionAttribute =
        crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute;
    fn from_extension_attribute(extension_attribute: Self::EffectiveExtensionAttribute) -> Self {
        Self::Extension(extension_attribute.into())
    }
}
impl core::fmt::Display for AliquotingProcedureAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::Procedure => write!(f, "aliquoting_procedures.procedure"),
            Self::ProcedureTemplate => {
                write!(f, "aliquoting_procedures.procedure_template")
            }
            Self::AliquotedWith => write!(f, "aliquoting_procedures.aliquoted_with"),
            Self::AliquotedWithModel => {
                write!(f, "aliquoting_procedures.aliquoted_with_model")
            }
            Self::ProcedureTemplateAliquotedWithModel => {
                write!(f, "aliquoting_procedures.procedure_template_aliquoted_with_model")
            }
            Self::ProcedureAliquotedWith(e) => write!(f, "aliquoting_procedures.{e}"),
            Self::PipetteTipModel => write!(f, "aliquoting_procedures.pipette_tip_model"),
            Self::ProcedureTemplatePipetteTipModel => {
                write!(f, "aliquoting_procedures.procedure_template_pipette_tip_model")
            }
            Self::ProcedurePipetteTip(e) => write!(f, "aliquoting_procedures.{e}"),
            Self::AliquotedFrom => write!(f, "aliquoting_procedures.aliquoted_from"),
            Self::ProcedureTemplateAliquotedFromModel => {
                write!(f, "aliquoting_procedures.procedure_template_aliquoted_from_model")
            }
            Self::ProcedureAliquotedFrom(e) => write!(f, "aliquoting_procedures.{e}"),
            Self::AliquotedInto => write!(f, "aliquoting_procedures.aliquoted_into"),
            Self::ProcedureTemplateAliquotedIntoModel => {
                write!(f, "aliquoting_procedures.procedure_template_aliquoted_into_model")
            }
            Self::ProcedureAliquotedInto(e) => write!(f, "aliquoting_procedures.{e}"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::aliquoting_procedures::aliquoting_procedures
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableAliquotingProcedure {
    pub(crate) procedure: ::rosetta_uuid::Uuid,
    pub(crate) procedure_template: i32,
    pub(crate) aliquoted_with: Option<::rosetta_uuid::Uuid>,
    pub(crate) aliquoted_with_model: i32,
    pub(crate) procedure_template_aliquoted_with_model: i32,
    pub(crate) procedure_aliquoted_with: ::rosetta_uuid::Uuid,
    pub(crate) pipette_tip_model: i32,
    pub(crate) procedure_template_pipette_tip_model: i32,
    pub(crate) procedure_pipette_tip: ::rosetta_uuid::Uuid,
    pub(crate) aliquoted_from: ::rosetta_uuid::Uuid,
    pub(crate) procedure_template_aliquoted_from_model: i32,
    pub(crate) procedure_aliquoted_from: ::rosetta_uuid::Uuid,
    pub(crate) aliquoted_into: ::rosetta_uuid::Uuid,
    pub(crate) procedure_template_aliquoted_into_model: i32,
    pub(crate) procedure_aliquoted_into: ::rosetta_uuid::Uuid,
}
impl InsertableAliquotingProcedure {
    pub fn aliquoted_from<C: diesel::connection::LoadConnection>(
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
            self.aliquoted_from,
            conn,
        )
    }
    pub fn aliquoted_into<C: diesel::connection::LoadConnection>(
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
            self.aliquoted_into,
            conn,
        )
    }
    pub fn aliquoted_with<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::pipettes::Pipette>,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::pipettes::Pipette:
            web_common_traits::database::Read<C>,
    {
        use diesel::OptionalExtension;
        use web_common_traits::database::Read;
        let Some(aliquoted_with) = self.aliquoted_with else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::pipettes::Pipette::read(aliquoted_with, conn)
            .optional()
    }
    pub fn aliquoted_with_model<C: diesel::connection::LoadConnection>(
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
            self.aliquoted_with_model,
            conn,
        )
    }
    pub fn aliquoting_procedures_aliquoted_with_model_pipette_tip_mod_fkey<
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
            (self.aliquoted_with_model, self.pipette_tip_model),
            conn,
        )
    }
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
    #[cfg(feature = "postgres")]
    pub fn aliquoting_procedures_procedure_aliquoted_from_aliquoted_f_fkey(
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
                    .eq(&self.procedure_aliquoted_from)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset
                            .eq(&self.aliquoted_from),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
    }
    pub fn procedure_aliquoted_from<C: diesel::connection::LoadConnection>(
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
            self.procedure_aliquoted_from,
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn aliquoting_procedures_procedure_aliquoted_from_procedure_t_fkey(
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
                    .eq(&self.procedure_aliquoted_from)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::procedure_template_asset_model
                            .eq(&self.procedure_template_aliquoted_from_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn aliquoting_procedures_procedure_aliquoted_into_aliquoted_i_fkey(
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
                    .eq(&self.procedure_aliquoted_into)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset
                            .eq(&self.aliquoted_into),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
    }
    pub fn procedure_aliquoted_into<C: diesel::connection::LoadConnection>(
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
            self.procedure_aliquoted_into,
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn aliquoting_procedures_procedure_aliquoted_into_procedure_t_fkey(
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
                    .eq(&self.procedure_aliquoted_into)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::procedure_template_asset_model
                            .eq(&self.procedure_template_aliquoted_into_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn aliquoting_procedures_procedure_aliquoted_with_aliquoted_fkey1(
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
        let Some(aliquoted_with) = self.aliquoted_with else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::id
                    .eq(&self.procedure_aliquoted_with)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset
                            .eq(aliquoted_with),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
            .optional()
    }
    #[cfg(feature = "postgres")]
    pub fn aliquoting_procedures_procedure_aliquoted_with_aliquoted_w_fkey(
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
                    .eq(&self.procedure_aliquoted_with)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset_model
                            .eq(&self.aliquoted_with_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
    }
    pub fn procedure_aliquoted_with<C: diesel::connection::LoadConnection>(
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
            self.procedure_aliquoted_with,
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn aliquoting_procedures_procedure_aliquoted_with_procedure_t_fkey(
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
                    .eq(&self.procedure_aliquoted_with)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::procedure_template_asset_model
                            .eq(&self.procedure_template_aliquoted_with_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
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
    pub fn aliquoting_procedures_procedure_pipette_tip_pipette_tip_mo_fkey(
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
    pub fn aliquoting_procedures_procedure_pipette_tip_procedure_temp_fkey(
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
    pub fn aliquoting_procedures_procedure_procedure_template_fkey(
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
    pub fn procedure_template_aliquoted_from_model<
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
            self.procedure_template_aliquoted_from_model,
            conn,
        )
    }
    pub fn procedure_template_aliquoted_into_model<
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
            self.procedure_template_aliquoted_into_model,
            conn,
        )
    }
    pub fn procedure_template_aliquoted_with_model<
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
            self.procedure_template_aliquoted_with_model,
            conn,
        )
    }
    pub fn procedure_template<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate: web_common_traits::database::Read<
            C,
        >,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate::read(
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
    pub fn aliquoting_procedures_procedure_template_procedure_templa_fkey1(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate,
        diesel::result::Error,
    >{
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate::table()
            .filter(
                crate::codegen::diesel_codegen::tables::aliquoting_procedure_templates::aliquoting_procedure_templates::dsl::procedure_template
                    .eq(&self.procedure_template)
                    .and(
                        crate::codegen::diesel_codegen::tables::aliquoting_procedure_templates::aliquoting_procedure_templates::dsl::procedure_template_pipette_tip_model
                            .eq(&self.procedure_template_pipette_tip_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn aliquoting_procedures_procedure_template_procedure_templa_fkey2(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate,
        diesel::result::Error,
    >{
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate::table()
            .filter(
                crate::codegen::diesel_codegen::tables::aliquoting_procedure_templates::aliquoting_procedure_templates::dsl::procedure_template
                    .eq(&self.procedure_template)
                    .and(
                        crate::codegen::diesel_codegen::tables::aliquoting_procedure_templates::aliquoting_procedure_templates::dsl::procedure_template_aliquoted_from_model
                            .eq(&self.procedure_template_aliquoted_from_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn aliquoting_procedures_procedure_template_procedure_templa_fkey3(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate,
        diesel::result::Error,
    >{
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate::table()
            .filter(
                crate::codegen::diesel_codegen::tables::aliquoting_procedure_templates::aliquoting_procedure_templates::dsl::procedure_template
                    .eq(&self.procedure_template)
                    .and(
                        crate::codegen::diesel_codegen::tables::aliquoting_procedure_templates::aliquoting_procedure_templates::dsl::procedure_template_aliquoted_into_model
                            .eq(&self.procedure_template_aliquoted_into_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn aliquoting_procedures_procedure_template_procedure_templat_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate,
        diesel::result::Error,
    >{
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate::table()
            .filter(
                crate::codegen::diesel_codegen::tables::aliquoting_procedure_templates::aliquoting_procedure_templates::dsl::procedure_template
                    .eq(&self.procedure_template)
                    .and(
                        crate::codegen::diesel_codegen::tables::aliquoting_procedure_templates::aliquoting_procedure_templates::dsl::procedure_template_aliquoted_with_model
                            .eq(&self.procedure_template_aliquoted_with_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate,
            >(conn)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Hash, Ord, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Builder for creating and inserting a new
/// [`AliquotingProcedure`](crate::codegen::structs_codegen::tables::aliquoting_procedures::AliquotingProcedure).
///
///
/// # Implementation details
/// While this builder implements several methods, a reasonably complete
/// **basic** usage example (*which may not apply to your own specific use case,
/// please adapt accordingly*) is as follows:
///
/// ```rust,ignore
/// use core_structures::AliquotingProcedure;
/// use core_structures::tables::insertables::AliquotingProcedureSettable;
/// use core_structures::tables::insertables::ProcedureSettable;
/// use web_common_traits::database::Insertable;
/// use web_common_traits::database::InsertableVariant;
///
/// let aliquoting_procedure = AliquotingProcedure::new()
///    // Set mandatory fields
///    .procedure_aliquoted_from(procedure_aliquoted_from)?
///    .procedure_aliquoted_into(procedure_aliquoted_into)?
///    .procedure_aliquoted_with(procedure_aliquoted_with)?
///    .procedure_pipette_tip(procedure_pipette_tip)?
///    .procedure_template(procedure_template)?
///    .created_by(created_by)?
///    .most_concrete_table(most_concrete_table)?
///    .updated_by(updated_by)?
///    // Optionally set fields with default values
///    .created_at(created_at)?
///    .number_of_completed_subprocedures(number_of_completed_subprocedures)?
///    .procedure(procedure)?
///    .updated_at(updated_at)?
///    // Optionally set optional fields
///    .parent_procedure(parent_procedure)?
///    .predecessor_procedure(predecessor_procedure)?
///    // Finally, insert the new record in the database
///    .insert(user.id, conn)?;
/// ```
pub struct InsertableAliquotingProcedureBuilder<
    Procedure = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureBuilder,
> {
    pub(crate) procedure_template: Option<i32>,
    pub(crate) aliquoted_with: Option<::rosetta_uuid::Uuid>,
    pub(crate) aliquoted_with_model: Option<i32>,
    pub(crate) procedure_template_aliquoted_with_model: Option<i32>,
    pub(crate) procedure_aliquoted_with: web_common_traits::database::IdOrBuilder<
        ::rosetta_uuid::Uuid,
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
    >,
    pub(crate) pipette_tip_model: Option<i32>,
    pub(crate) procedure_template_pipette_tip_model: Option<i32>,
    pub(crate) procedure_pipette_tip: web_common_traits::database::IdOrBuilder<
        ::rosetta_uuid::Uuid,
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
    >,
    pub(crate) aliquoted_from: Option<::rosetta_uuid::Uuid>,
    pub(crate) procedure_template_aliquoted_from_model: Option<i32>,
    pub(crate) procedure_aliquoted_from: web_common_traits::database::IdOrBuilder<
        ::rosetta_uuid::Uuid,
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
    >,
    pub(crate) aliquoted_into: Option<::rosetta_uuid::Uuid>,
    pub(crate) procedure_template_aliquoted_into_model: Option<i32>,
    pub(crate) procedure_aliquoted_into: web_common_traits::database::IdOrBuilder<
        ::rosetta_uuid::Uuid,
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
    >,
    pub(crate) procedure: Procedure,
}
impl From<InsertableAliquotingProcedureBuilder>
    for web_common_traits::database::IdOrBuilder<
        ::rosetta_uuid::Uuid,
        InsertableAliquotingProcedureBuilder,
    >
{
    fn from(builder: InsertableAliquotingProcedureBuilder) -> Self {
        Self::Builder(builder)
    }
}
impl<Procedure> common_traits::builder::IsCompleteBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureBuilder<
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
            && (self.aliquoted_with_model.is_some() || self.procedure_aliquoted_with.is_complete())
            && (self.procedure_template_aliquoted_with_model.is_some()
                || self.procedure_template.is_some()
                || self.procedure_aliquoted_with.is_complete())
            && self.procedure_aliquoted_with.is_complete()
            && (self.pipette_tip_model.is_some() || self.procedure_pipette_tip.is_complete())
            && (self.procedure_template_pipette_tip_model.is_some()
                || self.procedure_template.is_some()
                || self.procedure_pipette_tip.is_complete())
            && self.procedure_pipette_tip.is_complete()
            && (self.aliquoted_from.is_some() || self.procedure_aliquoted_from.is_complete())
            && (self.procedure_template_aliquoted_from_model.is_some()
                || self.procedure_template.is_some()
                || self.procedure_aliquoted_from.is_complete())
            && self.procedure_aliquoted_from.is_complete()
            && (self.aliquoted_into.is_some() || self.procedure_aliquoted_into.is_complete())
            && (self.procedure_template_aliquoted_into_model.is_some()
                || self.procedure_template.is_some()
                || self.procedure_aliquoted_into.is_complete())
            && self.procedure_aliquoted_into.is_complete()
    }
}
/// Trait defining setters for attributes of an instance of
/// `AliquotingProcedure` or descendant tables.
pub trait AliquotingProcedureSettable: Sized {
    /// Attributes required to build the insertable.
    type Attributes;
    /// Sets the value of the `public.aliquoting_procedures.procedure_template`
    /// column.
    ///
    /// # Arguments
    /// * `procedure_template`: The value to set for the
    ///   `public.aliquoting_procedures.procedure_template` column.
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
    fn procedure_template<PT>(
        self,
        procedure_template: PT,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PT: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
    /// Sets the value of the `public.aliquoting_procedures.aliquoted_with`
    /// column.
    ///
    /// # Arguments
    /// * `aliquoted_with`: The value to set for the
    ///   `public.aliquoting_procedures.aliquoted_with` column.
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
    fn aliquoted_with<AW>(
        self,
        aliquoted_with: AW,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        AW: web_common_traits::database::MaybePrimaryKeyLike<PrimaryKey = ::rosetta_uuid::Uuid>;
    /// Sets the value of the
    /// `public.aliquoting_procedures.aliquoted_with_model` column.
    ///
    /// # Arguments
    /// * `aliquoted_with_model`: The value to set for the
    ///   `public.aliquoting_procedures.aliquoted_with_model` column.
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
    fn aliquoted_with_model<AWM>(
        self,
        aliquoted_with_model: AWM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        AWM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
    /// Sets the value of the
    /// `public.aliquoting_procedures.procedure_template_aliquoted_with_model`
    /// column.
    ///
    /// # Arguments
    /// * `procedure_template_aliquoted_with_model`: The value to set for the
    ///   `public.aliquoting_procedures.procedure_template_aliquoted_with_model`
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
    fn procedure_template_aliquoted_with_model<PTAWM>(
        self,
        procedure_template_aliquoted_with_model: PTAWM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PTAWM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
    /// Sets the value of the
    /// `public.aliquoting_procedures.procedure_aliquoted_with` column.
    ///
    /// # Arguments
    /// * `procedure_aliquoted_with`: The value to set for the
    ///   `public.aliquoting_procedures.procedure_aliquoted_with` column.
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
    fn procedure_aliquoted_with<PAW>(
        self,
        procedure_aliquoted_with: PAW,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PAW: Into<
            web_common_traits::database::IdOrBuilder<
                ::rosetta_uuid::Uuid,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
            >,
        >;
    /// Sets the value of the `public.aliquoting_procedures.pipette_tip_model`
    /// column.
    ///
    /// # Arguments
    /// * `pipette_tip_model`: The value to set for the
    ///   `public.aliquoting_procedures.pipette_tip_model` column.
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
    fn pipette_tip_model<PTM>(
        self,
        pipette_tip_model: PTM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PTM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
    /// Sets the value of the
    /// `public.aliquoting_procedures.procedure_template_pipette_tip_model`
    /// column.
    ///
    /// # Arguments
    /// * `procedure_template_pipette_tip_model`: The value to set for the
    ///   `public.aliquoting_procedures.procedure_template_pipette_tip_model`
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
    fn procedure_template_pipette_tip_model<PTPTM>(
        self,
        procedure_template_pipette_tip_model: PTPTM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PTPTM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
    /// Sets the value of the
    /// `public.aliquoting_procedures.procedure_pipette_tip` column.
    ///
    /// # Arguments
    /// * `procedure_pipette_tip`: The value to set for the
    ///   `public.aliquoting_procedures.procedure_pipette_tip` column.
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
    /// Sets the value of the `public.aliquoting_procedures.aliquoted_from`
    /// column.
    ///
    /// # Arguments
    /// * `aliquoted_from`: The value to set for the
    ///   `public.aliquoting_procedures.aliquoted_from` column.
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
    fn aliquoted_from<AF>(
        self,
        aliquoted_from: AF,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        AF: web_common_traits::database::PrimaryKeyLike<PrimaryKey = ::rosetta_uuid::Uuid>;
    /// Sets the value of the
    /// `public.aliquoting_procedures.procedure_template_aliquoted_from_model`
    /// column.
    ///
    /// # Arguments
    /// * `procedure_template_aliquoted_from_model`: The value to set for the
    ///   `public.aliquoting_procedures.procedure_template_aliquoted_from_model`
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
    fn procedure_template_aliquoted_from_model<PTAFM>(
        self,
        procedure_template_aliquoted_from_model: PTAFM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PTAFM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
    /// Sets the value of the
    /// `public.aliquoting_procedures.procedure_aliquoted_from` column.
    ///
    /// # Arguments
    /// * `procedure_aliquoted_from`: The value to set for the
    ///   `public.aliquoting_procedures.procedure_aliquoted_from` column.
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
    fn procedure_aliquoted_from<PAF>(
        self,
        procedure_aliquoted_from: PAF,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PAF: Into<
            web_common_traits::database::IdOrBuilder<
                ::rosetta_uuid::Uuid,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
            >,
        >;
    /// Sets the value of the `public.aliquoting_procedures.aliquoted_into`
    /// column.
    ///
    /// # Arguments
    /// * `aliquoted_into`: The value to set for the
    ///   `public.aliquoting_procedures.aliquoted_into` column.
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
    fn aliquoted_into<AI>(
        self,
        aliquoted_into: AI,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        AI: web_common_traits::database::PrimaryKeyLike<PrimaryKey = ::rosetta_uuid::Uuid>;
    /// Sets the value of the
    /// `public.aliquoting_procedures.procedure_template_aliquoted_into_model`
    /// column.
    ///
    /// # Arguments
    /// * `procedure_template_aliquoted_into_model`: The value to set for the
    ///   `public.aliquoting_procedures.procedure_template_aliquoted_into_model`
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
    fn procedure_template_aliquoted_into_model<PTAIM>(
        self,
        procedure_template_aliquoted_into_model: PTAIM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PTAIM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
    /// Sets the value of the
    /// `public.aliquoting_procedures.procedure_aliquoted_into` column.
    ///
    /// # Arguments
    /// * `procedure_aliquoted_into`: The value to set for the
    ///   `public.aliquoting_procedures.procedure_aliquoted_into` column.
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
    fn procedure_aliquoted_into<PAI>(
        self,
        procedure_aliquoted_into: PAI,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PAI: Into<
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
> AliquotingProcedureSettable for InsertableAliquotingProcedureBuilder<Procedure>
{
    type Attributes =
        crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureAttribute;
    /// Sets the value of the `public.aliquoting_procedures.procedure_template`
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
    /// subgraph v7 ["`aliquoting_procedures`"]
    ///    v0@{shape: rounded, label: "procedure_template"}
    /// class v0 column-of-interest
    ///    v1@{shape: rounded, label: "procedure_template_aliquoted_from_model"}
    /// class v1 directly-involved-column
    ///    v2@{shape: rounded, label: "procedure_template_aliquoted_into_model"}
    /// class v2 directly-involved-column
    ///    v3@{shape: rounded, label: "procedure_template_aliquoted_with_model"}
    /// class v3 directly-involved-column
    ///    v4@{shape: rounded, label: "procedure_template_pipette_tip_model"}
    /// class v4 directly-involved-column
    /// end
    /// subgraph v8 ["`procedure_assets`"]
    ///    v6@{shape: rounded, label: "procedure_template_asset_model"}
    /// class v6 undirectly-involved-column
    /// end
    /// subgraph v9 ["`procedures`"]
    ///    v5@{shape: rounded, label: "procedure_template"}
    /// class v5 directly-involved-column
    /// end
    /// v0 --->|"`ancestral same as`"| v5
    /// v0 -.->|"`foreign defines`"| v1
    /// v0 -.->|"`foreign defines`"| v2
    /// v0 -.->|"`foreign defines`"| v3
    /// v0 -.->|"`foreign defines`"| v4
    /// v1 --->|"`associated same as`"| v6
    /// v2 --->|"`associated same as`"| v6
    /// v3 --->|"`associated same as`"| v6
    /// v4 --->|"`associated same as`"| v6
    /// v7 --->|"`extends`"| v9
    /// v7 ---o|"`associated with`"| v8
    /// ```
    fn procedure_template<PT>(
        mut self,
        procedure_template: PT,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PT: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let procedure_template =
            <PT as web_common_traits::database::PrimaryKeyLike>::primary_key(&procedure_template);
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
    /// Sets the value of the `public.aliquoting_procedures.aliquoted_with`
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
    /// subgraph v4 ["`aliquoting_procedures`"]
    ///    v0@{shape: rounded, label: "aliquoted_with"}
    /// class v0 column-of-interest
    ///    v1@{shape: rounded, label: "procedure_aliquoted_with"}
    /// class v1 directly-involved-column
    /// end
    /// subgraph v5 ["`procedure_assets`"]
    ///    v2@{shape: rounded, label: "asset"}
    /// class v2 directly-involved-column
    ///    v3@{shape: rounded, label: "id"}
    /// class v3 undirectly-involved-column
    /// end
    /// v0 --->|"`associated same as`"| v2
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 -.->|"`foreign defines`"| v0
    /// v4 ---o|"`associated with`"| v5
    /// ```
    fn aliquoted_with<AW>(
        mut self,
        aliquoted_with: AW,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        AW: web_common_traits::database::MaybePrimaryKeyLike<PrimaryKey = ::rosetta_uuid::Uuid>,
    {
        let aliquoted_with =
            <AW as web_common_traits::database::MaybePrimaryKeyLike>::maybe_primary_key(
                &aliquoted_with,
            );
        if let web_common_traits::database::IdOrBuilder::Builder(procedure_aliquoted_with) =
            self.procedure_aliquoted_with
        {
            self.procedure_aliquoted_with = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset(
                    procedure_aliquoted_with,
                    aliquoted_with,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedureAliquotedWith(attribute)
                    })
                })?
                .into();
        }
        self.aliquoted_with = aliquoted_with;
        Ok(self)
    }
    /// Sets the value of the
    /// `public.aliquoting_procedures.aliquoted_with_model` column.
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
    /// subgraph v4 ["`aliquoting_procedures`"]
    ///    v0@{shape: rounded, label: "aliquoted_with_model"}
    /// class v0 column-of-interest
    ///    v1@{shape: rounded, label: "procedure_aliquoted_with"}
    /// class v1 directly-involved-column
    /// end
    /// subgraph v5 ["`procedure_assets`"]
    ///    v2@{shape: rounded, label: "asset_model"}
    /// class v2 directly-involved-column
    ///    v3@{shape: rounded, label: "id"}
    /// class v3 undirectly-involved-column
    /// end
    /// v0 --->|"`associated same as`"| v2
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 -.->|"`foreign defines`"| v0
    /// v4 ---o|"`associated with`"| v5
    /// ```
    fn aliquoted_with_model<AWM>(
        mut self,
        aliquoted_with_model: AWM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        AWM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let aliquoted_with_model =
            <AWM as web_common_traits::database::PrimaryKeyLike>::primary_key(
                &aliquoted_with_model,
            );
        if let web_common_traits::database::IdOrBuilder::Builder(procedure_aliquoted_with) =
            self.procedure_aliquoted_with
        {
            self.procedure_aliquoted_with = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset_model(
                    procedure_aliquoted_with,
                    aliquoted_with_model,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedureAliquotedWith(attribute)
                    })
                })?
                .into();
        }
        self.aliquoted_with_model = Some(aliquoted_with_model);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.aliquoting_procedures.procedure_template_aliquoted_with_model`
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
    /// subgraph v4 ["`aliquoting_procedures`"]
    ///    v0@{shape: rounded, label: "procedure_aliquoted_with"}
    /// class v0 directly-involved-column
    ///    v1@{shape: rounded, label: "procedure_template_aliquoted_with_model"}
    /// class v1 column-of-interest
    /// end
    /// subgraph v5 ["`procedure_assets`"]
    ///    v2@{shape: rounded, label: "procedure_template_asset_model"}
    /// class v2 directly-involved-column
    ///    v3@{shape: rounded, label: "id"}
    /// class v3 undirectly-involved-column
    /// end
    /// v0 --->|"`associated same as`"| v3
    /// v0 --->|"`associated same as`"| v3
    /// v0 --->|"`associated same as`"| v3
    /// v0 --->|"`associated same as`"| v3
    /// v0 -.->|"`foreign defines`"| v1
    /// v1 --->|"`associated same as`"| v2
    /// v4 ---o|"`associated with`"| v5
    /// ```
    fn procedure_template_aliquoted_with_model<PTAWM>(
        mut self,
        procedure_template_aliquoted_with_model: PTAWM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PTAWM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let procedure_template_aliquoted_with_model =
            <PTAWM as web_common_traits::database::PrimaryKeyLike>::primary_key(
                &procedure_template_aliquoted_with_model,
            );
        if let web_common_traits::database::IdOrBuilder::Builder(procedure_aliquoted_with) =
            self.procedure_aliquoted_with
        {
            self.procedure_aliquoted_with = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure_template_asset_model(
                    procedure_aliquoted_with,
                    procedure_template_aliquoted_with_model,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedureAliquotedWith(attribute)
                    })
                })?
                .into();
        }
        self.procedure_template_aliquoted_with_model =
            Some(procedure_template_aliquoted_with_model);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.aliquoting_procedures.procedure_aliquoted_with` column.
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
    /// subgraph v8 ["`aliquoting_procedures`"]
    ///    v0@{shape: rounded, label: "aliquoted_with"}
    /// class v0 directly-involved-column
    ///    v1@{shape: rounded, label: "aliquoted_with_model"}
    /// class v1 directly-involved-column
    ///    v2@{shape: rounded, label: "procedure_aliquoted_with"}
    /// class v2 column-of-interest
    ///    v3@{shape: rounded, label: "procedure_template_aliquoted_with_model"}
    /// class v3 directly-involved-column
    /// end
    /// subgraph v9 ["`procedure_assets`"]
    ///    v4@{shape: rounded, label: "asset"}
    /// class v4 directly-involved-column
    ///    v5@{shape: rounded, label: "asset_model"}
    /// class v5 directly-involved-column
    ///    v6@{shape: rounded, label: "procedure_template_asset_model"}
    /// class v6 directly-involved-column
    ///    v7@{shape: rounded, label: "id"}
    /// class v7 undirectly-involved-column
    /// end
    /// v0 --->|"`associated same as`"| v4
    /// v1 --->|"`associated same as`"| v5
    /// v2 --->|"`associated same as`"| v7
    /// v2 --->|"`associated same as`"| v7
    /// v2 --->|"`associated same as`"| v7
    /// v2 --->|"`associated same as`"| v7
    /// v2 -.->|"`foreign defines`"| v0
    /// v2 -.->|"`foreign defines`"| v1
    /// v2 -.->|"`foreign defines`"| v3
    /// v3 --->|"`associated same as`"| v6
    /// v4 -.->|"`foreign defines`"| v5
    /// v8 ---o|"`associated with`"| v9
    /// ```
    fn procedure_aliquoted_with<PAW>(
        mut self,
        procedure_aliquoted_with: PAW,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PAW: Into<
            web_common_traits::database::IdOrBuilder<
                ::rosetta_uuid::Uuid,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
            >,
        >,
    {
        let mut procedure_aliquoted_with = procedure_aliquoted_with.into();
        if let web_common_traits::database::IdOrBuilder::Builder(builder) = procedure_aliquoted_with
        {
            procedure_aliquoted_with = if let (Some(aliquoted_with), Some(asset)) =
                (self.aliquoted_with, builder.asset)
            {
                if aliquoted_with != asset {
                    return Err(web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            Self::Attributes::AliquotedWith,
                        ),
                    ));
                }
                builder.into()
            } else if let Some(asset) = builder.asset {
                self.aliquoted_with = Some(asset);
                builder.into()
            } else if let Some(aliquoted_with) = self.aliquoted_with {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset(
                        builder,
                        aliquoted_with,
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            Self::Attributes::ProcedureAliquotedWith(attribute)
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        if let web_common_traits::database::IdOrBuilder::Builder(builder) = procedure_aliquoted_with
        {
            procedure_aliquoted_with = if let (Some(aliquoted_with_model), Some(asset_model)) =
                (self.aliquoted_with_model, builder.asset_model)
            {
                if aliquoted_with_model != asset_model {
                    return Err(web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            Self::Attributes::AliquotedWithModel,
                        ),
                    ));
                }
                builder.into()
            } else if let Some(asset_model) = builder.asset_model {
                self.aliquoted_with_model = Some(asset_model);
                builder.into()
            } else if let Some(aliquoted_with_model) = self.aliquoted_with_model {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset_model(
                        builder,
                        aliquoted_with_model,
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            Self::Attributes::ProcedureAliquotedWith(attribute)
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        if let web_common_traits::database::IdOrBuilder::Builder(builder) = procedure_aliquoted_with
        {
            procedure_aliquoted_with = if let (
                Some(procedure_template_aliquoted_with_model),
                Some(procedure_template_asset_model),
            ) = (
                self.procedure_template_aliquoted_with_model,
                builder.procedure_template_asset_model,
            ) {
                if procedure_template_aliquoted_with_model != procedure_template_asset_model {
                    return Err(web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            Self::Attributes::ProcedureTemplateAliquotedWithModel,
                        ),
                    ));
                }
                builder.into()
            } else if let Some(procedure_template_asset_model) =
                builder.procedure_template_asset_model
            {
                self.procedure_template_aliquoted_with_model = Some(procedure_template_asset_model);
                builder.into()
            } else if let Some(procedure_template_aliquoted_with_model) =
                self.procedure_template_aliquoted_with_model
            {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure_template_asset_model(
                        builder,
                        procedure_template_aliquoted_with_model,
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            Self::Attributes::ProcedureAliquotedWith(attribute)
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        self.procedure_aliquoted_with = procedure_aliquoted_with;
        Ok(self)
    }
    /// Sets the value of the `public.aliquoting_procedures.pipette_tip_model`
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
    /// subgraph v4 ["`aliquoting_procedures`"]
    ///    v0@{shape: rounded, label: "pipette_tip_model"}
    /// class v0 column-of-interest
    ///    v1@{shape: rounded, label: "procedure_pipette_tip"}
    /// class v1 directly-involved-column
    /// end
    /// subgraph v5 ["`procedure_assets`"]
    ///    v2@{shape: rounded, label: "asset_model"}
    /// class v2 directly-involved-column
    ///    v3@{shape: rounded, label: "id"}
    /// class v3 undirectly-involved-column
    /// end
    /// v0 --->|"`associated same as`"| v2
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 -.->|"`foreign defines`"| v0
    /// v4 ---o|"`associated with`"| v5
    /// ```
    fn pipette_tip_model<PTM>(
        mut self,
        pipette_tip_model: PTM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PTM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let pipette_tip_model =
            <PTM as web_common_traits::database::PrimaryKeyLike>::primary_key(&pipette_tip_model);
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
    /// `public.aliquoting_procedures.procedure_template_pipette_tip_model`
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
    /// subgraph v4 ["`aliquoting_procedures`"]
    ///    v0@{shape: rounded, label: "procedure_pipette_tip"}
    /// class v0 directly-involved-column
    ///    v1@{shape: rounded, label: "procedure_template_pipette_tip_model"}
    /// class v1 column-of-interest
    /// end
    /// subgraph v5 ["`procedure_assets`"]
    ///    v2@{shape: rounded, label: "procedure_template_asset_model"}
    /// class v2 directly-involved-column
    ///    v3@{shape: rounded, label: "id"}
    /// class v3 undirectly-involved-column
    /// end
    /// v0 --->|"`associated same as`"| v3
    /// v0 --->|"`associated same as`"| v3
    /// v0 --->|"`associated same as`"| v3
    /// v0 -.->|"`foreign defines`"| v1
    /// v1 --->|"`associated same as`"| v2
    /// v4 ---o|"`associated with`"| v5
    /// ```
    fn procedure_template_pipette_tip_model<PTPTM>(
        mut self,
        procedure_template_pipette_tip_model: PTPTM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PTPTM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let procedure_template_pipette_tip_model =
            <PTPTM as web_common_traits::database::PrimaryKeyLike>::primary_key(
                &procedure_template_pipette_tip_model,
            );
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
    /// `public.aliquoting_procedures.procedure_pipette_tip` column.
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
    /// subgraph v6 ["`aliquoting_procedures`"]
    ///    v0@{shape: rounded, label: "pipette_tip_model"}
    /// class v0 directly-involved-column
    ///    v1@{shape: rounded, label: "procedure_pipette_tip"}
    /// class v1 column-of-interest
    ///    v2@{shape: rounded, label: "procedure_template_pipette_tip_model"}
    /// class v2 directly-involved-column
    /// end
    /// subgraph v7 ["`procedure_assets`"]
    ///    v3@{shape: rounded, label: "asset_model"}
    /// class v3 directly-involved-column
    ///    v4@{shape: rounded, label: "procedure_template_asset_model"}
    /// class v4 directly-involved-column
    ///    v5@{shape: rounded, label: "id"}
    /// class v5 undirectly-involved-column
    /// end
    /// v0 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v5
    /// v1 --->|"`associated same as`"| v5
    /// v1 --->|"`associated same as`"| v5
    /// v1 -.->|"`foreign defines`"| v0
    /// v1 -.->|"`foreign defines`"| v2
    /// v2 --->|"`associated same as`"| v4
    /// v6 ---o|"`associated with`"| v7
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
    /// Sets the value of the `public.aliquoting_procedures.aliquoted_from`
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
    /// subgraph v4 ["`aliquoting_procedures`"]
    ///    v0@{shape: rounded, label: "aliquoted_from"}
    /// class v0 column-of-interest
    ///    v1@{shape: rounded, label: "procedure_aliquoted_from"}
    /// class v1 directly-involved-column
    /// end
    /// subgraph v5 ["`procedure_assets`"]
    ///    v2@{shape: rounded, label: "asset"}
    /// class v2 directly-involved-column
    ///    v3@{shape: rounded, label: "id"}
    /// class v3 undirectly-involved-column
    /// end
    /// v0 --->|"`associated same as`"| v2
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 -.->|"`foreign defines`"| v0
    /// v4 ---o|"`associated with`"| v5
    /// ```
    fn aliquoted_from<AF>(
        mut self,
        aliquoted_from: AF,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        AF: web_common_traits::database::PrimaryKeyLike<PrimaryKey = ::rosetta_uuid::Uuid>,
    {
        let aliquoted_from =
            <AF as web_common_traits::database::PrimaryKeyLike>::primary_key(&aliquoted_from);
        if let web_common_traits::database::IdOrBuilder::Builder(procedure_aliquoted_from) =
            self.procedure_aliquoted_from
        {
            self.procedure_aliquoted_from = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset(
                    procedure_aliquoted_from,
                    aliquoted_from,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedureAliquotedFrom(attribute)
                    })
                })?
                .into();
        }
        self.aliquoted_from = Some(aliquoted_from);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.aliquoting_procedures.procedure_template_aliquoted_from_model`
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
    /// subgraph v4 ["`aliquoting_procedures`"]
    ///    v0@{shape: rounded, label: "procedure_aliquoted_from"}
    /// class v0 directly-involved-column
    ///    v1@{shape: rounded, label: "procedure_template_aliquoted_from_model"}
    /// class v1 column-of-interest
    /// end
    /// subgraph v5 ["`procedure_assets`"]
    ///    v2@{shape: rounded, label: "procedure_template_asset_model"}
    /// class v2 directly-involved-column
    ///    v3@{shape: rounded, label: "id"}
    /// class v3 undirectly-involved-column
    /// end
    /// v0 --->|"`associated same as`"| v3
    /// v0 --->|"`associated same as`"| v3
    /// v0 --->|"`associated same as`"| v3
    /// v0 -.->|"`foreign defines`"| v1
    /// v1 --->|"`associated same as`"| v2
    /// v4 ---o|"`associated with`"| v5
    /// ```
    fn procedure_template_aliquoted_from_model<PTAFM>(
        mut self,
        procedure_template_aliquoted_from_model: PTAFM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PTAFM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let procedure_template_aliquoted_from_model =
            <PTAFM as web_common_traits::database::PrimaryKeyLike>::primary_key(
                &procedure_template_aliquoted_from_model,
            );
        if let web_common_traits::database::IdOrBuilder::Builder(procedure_aliquoted_from) =
            self.procedure_aliquoted_from
        {
            self.procedure_aliquoted_from = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure_template_asset_model(
                    procedure_aliquoted_from,
                    procedure_template_aliquoted_from_model,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedureAliquotedFrom(attribute)
                    })
                })?
                .into();
        }
        self.procedure_template_aliquoted_from_model =
            Some(procedure_template_aliquoted_from_model);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.aliquoting_procedures.procedure_aliquoted_from` column.
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
    /// subgraph v6 ["`aliquoting_procedures`"]
    ///    v0@{shape: rounded, label: "aliquoted_from"}
    /// class v0 directly-involved-column
    ///    v1@{shape: rounded, label: "procedure_aliquoted_from"}
    /// class v1 column-of-interest
    ///    v2@{shape: rounded, label: "procedure_template_aliquoted_from_model"}
    /// class v2 directly-involved-column
    /// end
    /// subgraph v7 ["`procedure_assets`"]
    ///    v3@{shape: rounded, label: "asset"}
    /// class v3 directly-involved-column
    ///    v4@{shape: rounded, label: "procedure_template_asset_model"}
    /// class v4 directly-involved-column
    ///    v5@{shape: rounded, label: "id"}
    /// class v5 undirectly-involved-column
    /// end
    /// v0 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v5
    /// v1 --->|"`associated same as`"| v5
    /// v1 --->|"`associated same as`"| v5
    /// v1 -.->|"`foreign defines`"| v0
    /// v1 -.->|"`foreign defines`"| v2
    /// v2 --->|"`associated same as`"| v4
    /// v6 ---o|"`associated with`"| v7
    /// ```
    fn procedure_aliquoted_from<PAF>(
        mut self,
        procedure_aliquoted_from: PAF,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PAF: Into<
            web_common_traits::database::IdOrBuilder<
                ::rosetta_uuid::Uuid,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
            >,
        >,
    {
        let mut procedure_aliquoted_from = procedure_aliquoted_from.into();
        if let web_common_traits::database::IdOrBuilder::Builder(builder) = procedure_aliquoted_from
        {
            procedure_aliquoted_from = if let (Some(aliquoted_from), Some(asset)) =
                (self.aliquoted_from, builder.asset)
            {
                if aliquoted_from != asset {
                    return Err(web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            Self::Attributes::AliquotedFrom,
                        ),
                    ));
                }
                builder.into()
            } else if let Some(asset) = builder.asset {
                self.aliquoted_from = Some(asset);
                builder.into()
            } else if let Some(aliquoted_from) = self.aliquoted_from {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset(
                        builder,
                        aliquoted_from,
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            Self::Attributes::ProcedureAliquotedFrom(attribute)
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        if let web_common_traits::database::IdOrBuilder::Builder(builder) = procedure_aliquoted_from
        {
            procedure_aliquoted_from = if let (
                Some(procedure_template_aliquoted_from_model),
                Some(procedure_template_asset_model),
            ) = (
                self.procedure_template_aliquoted_from_model,
                builder.procedure_template_asset_model,
            ) {
                if procedure_template_aliquoted_from_model != procedure_template_asset_model {
                    return Err(web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            Self::Attributes::ProcedureTemplateAliquotedFromModel,
                        ),
                    ));
                }
                builder.into()
            } else if let Some(procedure_template_asset_model) =
                builder.procedure_template_asset_model
            {
                self.procedure_template_aliquoted_from_model = Some(procedure_template_asset_model);
                builder.into()
            } else if let Some(procedure_template_aliquoted_from_model) =
                self.procedure_template_aliquoted_from_model
            {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure_template_asset_model(
                        builder,
                        procedure_template_aliquoted_from_model,
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            Self::Attributes::ProcedureAliquotedFrom(attribute)
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        self.procedure_aliquoted_from = procedure_aliquoted_from;
        Ok(self)
    }
    /// Sets the value of the `public.aliquoting_procedures.aliquoted_into`
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
    /// subgraph v4 ["`aliquoting_procedures`"]
    ///    v0@{shape: rounded, label: "aliquoted_into"}
    /// class v0 column-of-interest
    ///    v1@{shape: rounded, label: "procedure_aliquoted_into"}
    /// class v1 directly-involved-column
    /// end
    /// subgraph v5 ["`procedure_assets`"]
    ///    v2@{shape: rounded, label: "asset"}
    /// class v2 directly-involved-column
    ///    v3@{shape: rounded, label: "id"}
    /// class v3 undirectly-involved-column
    /// end
    /// v0 --->|"`associated same as`"| v2
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v3
    /// v1 -.->|"`foreign defines`"| v0
    /// v4 ---o|"`associated with`"| v5
    /// ```
    fn aliquoted_into<AI>(
        mut self,
        aliquoted_into: AI,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        AI: web_common_traits::database::PrimaryKeyLike<PrimaryKey = ::rosetta_uuid::Uuid>,
    {
        let aliquoted_into =
            <AI as web_common_traits::database::PrimaryKeyLike>::primary_key(&aliquoted_into);
        if let web_common_traits::database::IdOrBuilder::Builder(procedure_aliquoted_into) =
            self.procedure_aliquoted_into
        {
            self.procedure_aliquoted_into = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset(
                    procedure_aliquoted_into,
                    aliquoted_into,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedureAliquotedInto(attribute)
                    })
                })?
                .into();
        }
        self.aliquoted_into = Some(aliquoted_into);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.aliquoting_procedures.procedure_template_aliquoted_into_model`
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
    /// subgraph v4 ["`aliquoting_procedures`"]
    ///    v0@{shape: rounded, label: "procedure_aliquoted_into"}
    /// class v0 directly-involved-column
    ///    v1@{shape: rounded, label: "procedure_template_aliquoted_into_model"}
    /// class v1 column-of-interest
    /// end
    /// subgraph v5 ["`procedure_assets`"]
    ///    v2@{shape: rounded, label: "procedure_template_asset_model"}
    /// class v2 directly-involved-column
    ///    v3@{shape: rounded, label: "id"}
    /// class v3 undirectly-involved-column
    /// end
    /// v0 --->|"`associated same as`"| v3
    /// v0 --->|"`associated same as`"| v3
    /// v0 --->|"`associated same as`"| v3
    /// v0 -.->|"`foreign defines`"| v1
    /// v1 --->|"`associated same as`"| v2
    /// v4 ---o|"`associated with`"| v5
    /// ```
    fn procedure_template_aliquoted_into_model<PTAIM>(
        mut self,
        procedure_template_aliquoted_into_model: PTAIM,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PTAIM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let procedure_template_aliquoted_into_model =
            <PTAIM as web_common_traits::database::PrimaryKeyLike>::primary_key(
                &procedure_template_aliquoted_into_model,
            );
        if let web_common_traits::database::IdOrBuilder::Builder(procedure_aliquoted_into) =
            self.procedure_aliquoted_into
        {
            self.procedure_aliquoted_into = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure_template_asset_model(
                    procedure_aliquoted_into,
                    procedure_template_aliquoted_into_model,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedureAliquotedInto(attribute)
                    })
                })?
                .into();
        }
        self.procedure_template_aliquoted_into_model =
            Some(procedure_template_aliquoted_into_model);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.aliquoting_procedures.procedure_aliquoted_into` column.
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
    /// subgraph v6 ["`aliquoting_procedures`"]
    ///    v0@{shape: rounded, label: "aliquoted_into"}
    /// class v0 directly-involved-column
    ///    v1@{shape: rounded, label: "procedure_aliquoted_into"}
    /// class v1 column-of-interest
    ///    v2@{shape: rounded, label: "procedure_template_aliquoted_into_model"}
    /// class v2 directly-involved-column
    /// end
    /// subgraph v7 ["`procedure_assets`"]
    ///    v3@{shape: rounded, label: "asset"}
    /// class v3 directly-involved-column
    ///    v4@{shape: rounded, label: "procedure_template_asset_model"}
    /// class v4 directly-involved-column
    ///    v5@{shape: rounded, label: "id"}
    /// class v5 undirectly-involved-column
    /// end
    /// v0 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v5
    /// v1 --->|"`associated same as`"| v5
    /// v1 --->|"`associated same as`"| v5
    /// v1 -.->|"`foreign defines`"| v0
    /// v1 -.->|"`foreign defines`"| v2
    /// v2 --->|"`associated same as`"| v4
    /// v6 ---o|"`associated with`"| v7
    /// ```
    fn procedure_aliquoted_into<PAI>(
        mut self,
        procedure_aliquoted_into: PAI,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PAI: Into<
            web_common_traits::database::IdOrBuilder<
                ::rosetta_uuid::Uuid,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder,
            >,
        >,
    {
        let mut procedure_aliquoted_into = procedure_aliquoted_into.into();
        if let web_common_traits::database::IdOrBuilder::Builder(builder) = procedure_aliquoted_into
        {
            procedure_aliquoted_into = if let (Some(aliquoted_into), Some(asset)) =
                (self.aliquoted_into, builder.asset)
            {
                if aliquoted_into != asset {
                    return Err(web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            Self::Attributes::AliquotedInto,
                        ),
                    ));
                }
                builder.into()
            } else if let Some(asset) = builder.asset {
                self.aliquoted_into = Some(asset);
                builder.into()
            } else if let Some(aliquoted_into) = self.aliquoted_into {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::asset(
                        builder,
                        aliquoted_into,
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            Self::Attributes::ProcedureAliquotedInto(attribute)
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        if let web_common_traits::database::IdOrBuilder::Builder(builder) = procedure_aliquoted_into
        {
            procedure_aliquoted_into = if let (
                Some(procedure_template_aliquoted_into_model),
                Some(procedure_template_asset_model),
            ) = (
                self.procedure_template_aliquoted_into_model,
                builder.procedure_template_asset_model,
            ) {
                if procedure_template_aliquoted_into_model != procedure_template_asset_model {
                    return Err(web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            Self::Attributes::ProcedureTemplateAliquotedIntoModel,
                        ),
                    ));
                }
                builder.into()
            } else if let Some(procedure_template_asset_model) =
                builder.procedure_template_asset_model
            {
                self.procedure_template_aliquoted_into_model = Some(procedure_template_asset_model);
                builder.into()
            } else if let Some(procedure_template_aliquoted_into_model) =
                self.procedure_template_aliquoted_into_model
            {
                <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure_template_asset_model(
                        builder,
                        procedure_template_aliquoted_into_model,
                    )
                    .map_err(|e| {
                        e.into_field_name(|attribute| {
                            Self::Attributes::ProcedureAliquotedInto(attribute)
                        })
                    })?
                    .into()
            } else {
                builder.into()
            };
        }
        self.procedure_aliquoted_into = procedure_aliquoted_into;
        Ok(self)
    }
}
impl<
    Procedure: crate::codegen::structs_codegen::tables::insertables::ProcedureSettable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute,
        >,
> crate::codegen::structs_codegen::tables::insertables::ProcedureSettable
for InsertableAliquotingProcedureBuilder<Procedure>
where
    Self: crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureSettable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureAttribute,
    >,
{
    type Attributes = crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureAttribute;
    #[inline]
    ///Sets the value of the `public.procedures.procedure` column.
    fn procedure<P>(
        mut self,
        procedure: P,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        P: web_common_traits::database::PrimaryKeyLike<
            PrimaryKey = ::rosetta_uuid::Uuid,
        >,
    {
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
    ///subgraph v2 ["`aliquoting_procedures`"]
    ///    v0@{shape: rounded, label: "procedure_template"}
    ///class v0 directly-involved-column
    ///end
    ///subgraph v3 ["`procedures`"]
    ///    v1@{shape: rounded, label: "procedure_template"}
    ///class v1 column-of-interest
    ///end
    ///v0 --->|"`ancestral same as`"| v1
    ///v2 --->|"`extends`"| v3
    ///```
    fn procedure_template<PT>(
        self,
        procedure_template: PT,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PT: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        <Self as AliquotingProcedureSettable>::procedure_template(
            self,
            procedure_template,
        )
    }
    #[inline]
    ///Sets the value of the `public.procedures.parent_procedure` column.
    fn parent_procedure<PP>(
        mut self,
        parent_procedure: PP,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PP: web_common_traits::database::MaybePrimaryKeyLike<
            PrimaryKey = ::rosetta_uuid::Uuid,
        >,
    {
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
    fn parent_procedure_template<PPT>(
        mut self,
        parent_procedure_template: PPT,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PPT: web_common_traits::database::MaybePrimaryKeyLike<PrimaryKey = i32>,
    {
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
    fn predecessor_procedure<PP>(
        mut self,
        predecessor_procedure: PP,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PP: web_common_traits::database::MaybePrimaryKeyLike<
            PrimaryKey = ::rosetta_uuid::Uuid,
        >,
    {
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
    fn predecessor_procedure_template<PPT>(
        mut self,
        predecessor_procedure_template: PPT,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        PPT: web_common_traits::database::MaybePrimaryKeyLike<PrimaryKey = i32>,
    {
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
    fn created_by<CB>(
        mut self,
        created_by: CB,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        CB: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
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
    fn updated_by<UB>(
        mut self,
        updated_by: UB,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        UB: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
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
    for InsertableAliquotingProcedureBuilder<Procedure>
where
    Procedure: web_common_traits::database::MostConcreteTable,
{
    fn set_most_concrete_table(&mut self, table_name: &str) {
        self.procedure.set_most_concrete_table(table_name);
    }
}
impl<Procedure> web_common_traits::prelude::SetPrimaryKey
    for InsertableAliquotingProcedureBuilder<Procedure>
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
for InsertableAliquotingProcedureBuilder<Procedure>
where
    Self: web_common_traits::database::InsertableVariant<
        C,
        UserId = i32,
        Row = crate::codegen::structs_codegen::tables::aliquoting_procedures::AliquotingProcedure,
        Error = web_common_traits::database::InsertError<AliquotingProcedureAttribute>,
    >,
    Procedure: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = ::rosetta_uuid::Uuid,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder: web_common_traits::database::TryInsertGeneric<
        C,
    >,
{
    type Attribute = AliquotingProcedureAttribute;
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<
        Self::PrimaryKey,
        web_common_traits::database::InsertError<Self::Attribute>,
    > {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::aliquoting_procedures::AliquotingProcedure = self
            .insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
