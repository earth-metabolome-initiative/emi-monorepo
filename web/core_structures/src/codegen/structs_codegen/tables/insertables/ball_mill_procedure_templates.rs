#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableBallMillProcedureTemplateExtensionAttributes {
    ProcedureTemplate(
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAttributes,
    ),
}
impl core::fmt::Display for InsertableBallMillProcedureTemplateExtensionAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::ProcedureTemplate(e) => write!(f, "{e}"),
        }
    }
}
impl
    From<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAttributes,
    > for InsertableBallMillProcedureTemplateExtensionAttributes
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAttributes,
    ) -> Self {
        Self::ProcedureTemplate(attribute)
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableBallMillProcedureTemplateAttributes {
    Extension(InsertableBallMillProcedureTemplateExtensionAttributes),
    ProcedureTemplate,
    Kelvin,
    KelvinTolerancePercentage,
    Seconds,
    Hertz,
    BeadModel,
    ProcedureTemplateBeadModel(
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelAttributes,
    ),
    NumberOfBeads,
    MilledWithModel,
    ProcedureTemplateMilledWithModel(
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelAttributes,
    ),
    MilledContainerModel,
    ForeignProcedureTemplate,
    ProcedureTemplateMilledContainerModel,
}
impl core::str::FromStr for InsertableBallMillProcedureTemplateAttributes {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Kelvin" => Ok(Self::Kelvin),
            "KelvinTolerancePercentage" => Ok(Self::KelvinTolerancePercentage),
            "Seconds" => Ok(Self::Seconds),
            "Hertz" => Ok(Self::Hertz),
            "BeadModel" => Ok(Self::BeadModel),
            "ProcedureTemplateBeadModel" => {
                Ok(
                    Self::ProcedureTemplateBeadModel(
                        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelAttributes::Id,
                    ),
                )
            }
            "NumberOfBeads" => Ok(Self::NumberOfBeads),
            "MilledWithModel" => Ok(Self::MilledWithModel),
            "ProcedureTemplateMilledWithModel" => {
                Ok(
                    Self::ProcedureTemplateMilledWithModel(
                        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelAttributes::Id,
                    ),
                )
            }
            "MilledContainerModel" => Ok(Self::MilledContainerModel),
            "ForeignProcedureTemplate" => Ok(Self::ForeignProcedureTemplate),
            "ProcedureTemplateMilledContainerModel" => {
                Ok(Self::ProcedureTemplateMilledContainerModel)
            }
            "kelvin" => Ok(Self::Kelvin),
            "kelvin_tolerance_percentage" => Ok(Self::KelvinTolerancePercentage),
            "seconds" => Ok(Self::Seconds),
            "hertz" => Ok(Self::Hertz),
            "bead_model" => Ok(Self::BeadModel),
            "procedure_template_bead_model" => {
                Ok(
                    Self::ProcedureTemplateBeadModel(
                        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelAttributes::Id,
                    ),
                )
            }
            "number_of_beads" => Ok(Self::NumberOfBeads),
            "milled_with_model" => Ok(Self::MilledWithModel),
            "procedure_template_milled_with_model" => {
                Ok(
                    Self::ProcedureTemplateMilledWithModel(
                        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelAttributes::Id,
                    ),
                )
            }
            "milled_container_model" => Ok(Self::MilledContainerModel),
            "foreign_procedure_template" => Ok(Self::ForeignProcedureTemplate),
            "procedure_template_milled_container_model" => {
                Ok(Self::ProcedureTemplateMilledContainerModel)
            }
            _ => {
                Err(
                    web_common_traits::database::InsertError::UnknownAttribute(
                        s.to_owned(),
                    ),
                )
            }
        }
    }
}
impl core::fmt::Display for InsertableBallMillProcedureTemplateAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::ProcedureTemplate => write!(f, "procedure_template"),
            Self::Kelvin => write!(f, "kelvin"),
            Self::KelvinTolerancePercentage => write!(f, "kelvin_tolerance_percentage"),
            Self::Seconds => write!(f, "seconds"),
            Self::Hertz => write!(f, "hertz"),
            Self::BeadModel => write!(f, "bead_model"),
            Self::ProcedureTemplateBeadModel(e) => write!(f, "{e}"),
            Self::NumberOfBeads => write!(f, "number_of_beads"),
            Self::MilledWithModel => write!(f, "milled_with_model"),
            Self::ProcedureTemplateMilledWithModel(e) => write!(f, "{e}"),
            Self::MilledContainerModel => write!(f, "milled_container_model"),
            Self::ForeignProcedureTemplate => write!(f, "foreign_procedure_template"),
            Self::ProcedureTemplateMilledContainerModel => {
                write!(f, "procedure_template_milled_container_model")
            }
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::ball_mill_procedure_templates::ball_mill_procedure_templates
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableBallMillProcedureTemplate {
    pub(crate) procedure_template: i32,
    pub(crate) kelvin: f32,
    pub(crate) kelvin_tolerance_percentage: f32,
    pub(crate) seconds: f32,
    pub(crate) hertz: f32,
    pub(crate) bead_model: i32,
    pub(crate) procedure_template_bead_model: i32,
    pub(crate) number_of_beads: i16,
    pub(crate) milled_with_model: i32,
    pub(crate) procedure_template_milled_with_model: i32,
    pub(crate) milled_container_model: i32,
    pub(crate) foreign_procedure_template: i32,
    pub(crate) procedure_template_milled_container_model: i32,
}
impl InsertableBallMillProcedureTemplate {
    pub fn procedure_template<C: diesel::connection::LoadConnection>(
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
                self.procedure_template,
            ),
            conn,
        )
    }
    pub fn bead_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::beads_models::BeadsModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::beads_models::BeadsModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::beads_models::BeadsModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::beads_models::BeadsModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::beads_models::BeadsModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::beads_models::BeadsModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::beads_models::BeadsModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::beads_models::BeadsModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::beads_models::BeadsModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::beads_models::BeadsModel::table(),
                self.bead_model,
            ),
            conn,
        )
    }
    pub fn procedure_template_bead_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel::table(),
                self.procedure_template_bead_model,
            ),
            conn,
        )
    }
    pub fn milled_with_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel::table(),
                self.milled_with_model,
            ),
            conn,
        )
    }
    pub fn procedure_template_milled_with_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel::table(),
                self.procedure_template_milled_with_model,
            ),
            conn,
        )
    }
    pub fn milled_container_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel::table(),
                self.milled_container_model,
            ),
            conn,
        )
    }
    pub fn foreign_procedure_template<C: diesel::connection::LoadConnection>(
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
                self.foreign_procedure_template,
            ),
            conn,
        )
    }
    pub fn procedure_template_milled_container_model<
        C: diesel::connection::LoadConnection,
    >(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel::table(),
                self.procedure_template_milled_container_model,
            ),
            conn,
        )
    }
    pub fn ball_mill_pm_container_compatibility_rules<
        C: diesel::connection::LoadConnection,
    >(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule::table(),
                (self.milled_with_model, self.milled_container_model),
            ),
            conn,
        )
    }
    pub fn ball_mill_pm_beads_compatibility_rules<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule::table(),
                (self.milled_with_model, self.bead_model),
            ),
            conn,
        )
    }
    pub fn ball_mill_pm_beads_container_compatibility_rules<
        C: diesel::connection::LoadConnection,
    >(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule::table(),
                (self.bead_model, self.milled_container_model),
            ),
            conn,
        )
    }
}
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableBallMillProcedureTemplateBuilder<
    ProcedureTemplate
        = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateBuilder,
> {
    pub(crate) kelvin: Option<f32>,
    pub(crate) kelvin_tolerance_percentage: Option<f32>,
    pub(crate) seconds: Option<f32>,
    pub(crate) hertz: Option<f32>,
    pub(crate) bead_model: Option<i32>,
    pub(crate) procedure_template_bead_model: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
    pub(crate) number_of_beads: Option<i16>,
    pub(crate) milled_with_model: Option<i32>,
    pub(crate) procedure_template_milled_with_model: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
    pub(crate) milled_container_model: Option<i32>,
    pub(crate) foreign_procedure_template: Option<i32>,
    pub(crate) procedure_template_milled_container_model: Option<i32>,
    pub(crate) procedure_template: ProcedureTemplate,
}
impl<ProcedureTemplate> Default for InsertableBallMillProcedureTemplateBuilder<ProcedureTemplate>
where
    ProcedureTemplate: Default,
{
    fn default() -> Self {
        Self {
            procedure_template: Default::default(),
            kelvin: Some(293.15f32),
            kelvin_tolerance_percentage: Some(1f32),
            seconds: Some(150f32),
            hertz: Some(25f32),
            bead_model: Default::default(),
            procedure_template_bead_model: Default::default(),
            number_of_beads: Some(3i16),
            milled_with_model: Default::default(),
            procedure_template_milled_with_model: Default::default(),
            milled_container_model: Default::default(),
            foreign_procedure_template: Default::default(),
            procedure_template_milled_container_model: Default::default(),
        }
    }
}
/// Trait defining setters for attributes of an instance of
/// `BallMillProcedureTemplate` or descendant tables.
pub trait BallMillProcedureTemplateBuildable: Sized {
    /// Attributes required to build the insertable.
    type Attributes;
    /// Sets the value of the `public.ball_mill_procedure_templates.kelvin`
    /// column.
    ///
    /// # Arguments
    /// * `kelvin`: The value to set for the
    ///   `public.ball_mill_procedure_templates.kelvin` column.
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
    /// * If the provided value cannot be converted to the required type `f32`.
    /// * If the provided value does not pass schema-defined validation.
    fn kelvin<K>(
        self,
        kelvin: K,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        K: TryInto<f32>,
        validation_errors::SingleFieldError: From<<K as TryInto<f32>>::Error>;
    /// Sets the value of the
    /// `public.ball_mill_procedure_templates.kelvin_tolerance_percentage`
    /// column.
    ///
    /// # Arguments
    /// * `kelvin_tolerance_percentage`: The value to set for the
    ///   `public.ball_mill_procedure_templates.kelvin_tolerance_percentage`
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
    /// * If the provided value cannot be converted to the required type `f32`.
    /// * If the provided value does not pass schema-defined validation.
    fn kelvin_tolerance_percentage<KTP>(
        self,
        kelvin_tolerance_percentage: KTP,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        KTP: TryInto<f32>,
        validation_errors::SingleFieldError: From<<KTP as TryInto<f32>>::Error>;
    /// Sets the value of the `public.ball_mill_procedure_templates.seconds`
    /// column.
    ///
    /// # Arguments
    /// * `seconds`: The value to set for the
    ///   `public.ball_mill_procedure_templates.seconds` column.
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
    /// * If the provided value cannot be converted to the required type `f32`.
    /// * If the provided value does not pass schema-defined validation.
    fn seconds<S>(
        self,
        seconds: S,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        S: TryInto<f32>,
        validation_errors::SingleFieldError: From<<S as TryInto<f32>>::Error>;
    /// Sets the value of the `public.ball_mill_procedure_templates.hertz`
    /// column.
    ///
    /// # Arguments
    /// * `hertz`: The value to set for the
    ///   `public.ball_mill_procedure_templates.hertz` column.
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
    /// * If the provided value cannot be converted to the required type `f32`.
    /// * If the provided value does not pass schema-defined validation.
    fn hertz<H>(
        self,
        hertz: H,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        H: TryInto<f32>,
        validation_errors::SingleFieldError: From<<H as TryInto<f32>>::Error>;
    /// Sets the value of the `public.ball_mill_procedure_templates.bead_model`
    /// column.
    ///
    /// # Arguments
    /// * `bead_model`: The value to set for the
    ///   `public.ball_mill_procedure_templates.bead_model` column.
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
    fn bead_model(
        self,
        bead_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the
    /// `public.ball_mill_procedure_templates.procedure_template_bead_model`
    /// column.
    ///
    /// # Arguments
    /// * `procedure_template_bead_model`: The value to set for the
    ///   `public.ball_mill_procedure_templates.procedure_template_bead_model`
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
    fn procedure_template_bead_model(
        self,
        procedure_template_bead_model: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the
    /// `public.ball_mill_procedure_templates.number_of_beads` column.
    ///
    /// # Arguments
    /// * `number_of_beads`: The value to set for the
    ///   `public.ball_mill_procedure_templates.number_of_beads` column.
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
    /// * If the provided value cannot be converted to the required type `i16`.
    /// * If the provided value does not pass schema-defined validation.
    fn number_of_beads<NOB>(
        self,
        number_of_beads: NOB,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        NOB: TryInto<i16>,
        validation_errors::SingleFieldError: From<<NOB as TryInto<i16>>::Error>;
    /// Sets the value of the
    /// `public.ball_mill_procedure_templates.milled_with_model` column.
    ///
    /// # Arguments
    /// * `milled_with_model`: The value to set for the
    ///   `public.ball_mill_procedure_templates.milled_with_model` column.
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
    fn milled_with_model(
        self,
        milled_with_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the
    /// `public.ball_mill_procedure_templates.
    /// procedure_template_milled_with_model` column.
    ///
    /// # Arguments
    /// * `procedure_template_milled_with_model`: The value to set for the
    ///   `public.ball_mill_procedure_templates.
    ///   procedure_template_milled_with_model` column.
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
    fn procedure_template_milled_with_model(
        self,
        procedure_template_milled_with_model: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the
    /// `public.ball_mill_procedure_templates.milled_container_model` column.
    ///
    /// # Arguments
    /// * `milled_container_model`: The value to set for the
    ///   `public.ball_mill_procedure_templates.milled_container_model` column.
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
    fn milled_container_model(
        self,
        milled_container_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the
    /// `public.ball_mill_procedure_templates.foreign_procedure_template`
    /// column.
    ///
    /// # Arguments
    /// * `foreign_procedure_template`: The value to set for the
    ///   `public.ball_mill_procedure_templates.foreign_procedure_template`
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
    fn foreign_procedure_template(
        self,
        foreign_procedure_template: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the
    /// `public.ball_mill_procedure_templates.
    /// procedure_template_milled_container_model` column.
    ///
    /// # Arguments
    /// * `procedure_template_milled_container_model`: The value to set for the
    ///   `public.ball_mill_procedure_templates.
    ///   procedure_template_milled_container_model` column.
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
    fn procedure_template_milled_container_model(
        self,
        procedure_template_milled_container_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
}
impl<ProcedureTemplate> BallMillProcedureTemplateBuildable
    for InsertableBallMillProcedureTemplateBuilder<ProcedureTemplate>
{
    type Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureTemplateAttributes;
    /// Sets the value of the `public.ball_mill_procedure_templates.kelvin`
    /// column.
    fn kelvin<K>(
        mut self,
        kelvin: K,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        K: TryInto<f32>,
        validation_errors::SingleFieldError: From<<K as TryInto<f32>>::Error>,
    {
        let kelvin = kelvin.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(InsertableBallMillProcedureTemplateAttributes::Kelvin)
        })?;
        pgrx_validation::must_be_strictly_positive_f32(kelvin)
            .map_err(|e| {
                e
                    .rename_field(
                        crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureTemplateAttributes::Kelvin,
                    )
            })?;
        self.kelvin = Some(kelvin);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.ball_mill_procedure_templates.kelvin_tolerance_percentage`
    /// column.
    fn kelvin_tolerance_percentage<KTP>(
        mut self,
        kelvin_tolerance_percentage: KTP,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        KTP: TryInto<f32>,
        validation_errors::SingleFieldError: From<<KTP as TryInto<f32>>::Error>,
    {
        let kelvin_tolerance_percentage =
            kelvin_tolerance_percentage.try_into().map_err(|err| {
                validation_errors::SingleFieldError::from(err).rename_field(
                    InsertableBallMillProcedureTemplateAttributes::KelvinTolerancePercentage,
                )
            })?;
        pgrx_validation::must_be_strictly_positive_f32(kelvin_tolerance_percentage)
            .map_err(|e| {
                e
                    .rename_field(
                        crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureTemplateAttributes::KelvinTolerancePercentage,
                    )
            })
            .and_then(|_| {
                pgrx_validation::must_be_smaller_than_f32(
                        kelvin_tolerance_percentage,
                        100f32,
                    )
                    .map_err(|e| {
                        e
                            .rename_field(
                                crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureTemplateAttributes::KelvinTolerancePercentage,
                            )
                    })
            })?;
        self.kelvin_tolerance_percentage = Some(kelvin_tolerance_percentage);
        Ok(self)
    }
    /// Sets the value of the `public.ball_mill_procedure_templates.seconds`
    /// column.
    fn seconds<S>(
        mut self,
        seconds: S,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        S: TryInto<f32>,
        validation_errors::SingleFieldError: From<<S as TryInto<f32>>::Error>,
    {
        let seconds = seconds.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(InsertableBallMillProcedureTemplateAttributes::Seconds)
        })?;
        pgrx_validation::must_be_strictly_smaller_than_f32(seconds, 900f32)
            .map_err(|e| {
                e
                    .rename_field(
                        crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureTemplateAttributes::Seconds,
                    )
            })
            .and_then(|_| {
                pgrx_validation::must_be_strictly_greater_than_f32(seconds, 30f32)
                    .map_err(|e| {
                        e
                            .rename_field(
                                crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureTemplateAttributes::Seconds,
                            )
                    })
            })?;
        self.seconds = Some(seconds);
        Ok(self)
    }
    /// Sets the value of the `public.ball_mill_procedure_templates.hertz`
    /// column.
    fn hertz<H>(
        mut self,
        hertz: H,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        H: TryInto<f32>,
        validation_errors::SingleFieldError: From<<H as TryInto<f32>>::Error>,
    {
        let hertz = hertz.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(InsertableBallMillProcedureTemplateAttributes::Hertz)
        })?;
        pgrx_validation::must_be_strictly_smaller_than_f32(hertz, 50f32)
            .map_err(|e| {
                e
                    .rename_field(
                        crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureTemplateAttributes::Hertz,
                    )
            })
            .and_then(|_| {
                pgrx_validation::must_be_strictly_greater_than_f32(hertz, 15f32)
                    .map_err(|e| {
                        e
                            .rename_field(
                                crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureTemplateAttributes::Hertz,
                            )
                    })
            })?;
        self.hertz = Some(hertz);
        Ok(self)
    }
    /// Sets the value of the `public.ball_mill_procedure_templates.bead_model`
    /// column.
    ///
    /// # Implementation notes
    /// This method also set the values of other columns, due to
    /// same-as relationships or inferred values.
    ///
    /// ## Mermaid illustration
    ///
    /// ```mermaid
    /// flowchart LR
    /// classDef column-of-interest stroke: #f0746c,fill: #f49f9a
    /// classDef directly-involved-column stroke: #6c74f0,fill: #9a9ff4
    /// subgraph v3 ["`ball_mill_procedure_templates`"]
    ///    v1@{shape: rounded, label: "procedure_template_bead_model"}
    /// class v1 directly-involved-column
    ///    v0@{shape: rounded, label: "bead_model"}
    /// class v0 column-of-interest
    /// end
    /// subgraph v4 ["`procedure_template_asset_models`"]
    ///    v2@{shape: rounded, label: "asset_model"}
    /// class v2 directly-involved-column
    /// end
    /// v0 --->|"`associated same as`"| v2
    /// v3 ---o|"`associated with`"| v4
    /// ```
    fn bead_model(
        mut self,
        bead_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        let bead_model = bead_model.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(InsertableBallMillProcedureTemplateAttributes::BeadModel)
        })?;
        self.procedure_template_bead_model = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelBuildable>::asset_model(
                self.procedure_template_bead_model,
                bead_model,
            )
            .map_err(|e| {
                e.into_field_name(|attribute| {
                    Self::Attributes::ProcedureTemplateBeadModel(attribute)
                })
            })?;
        self.bead_model = Some(bead_model);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.ball_mill_procedure_templates.procedure_template_bead_model`
    /// column.
    ///
    /// # Implementation notes
    /// This method also set the values of other columns, due to
    /// same-as relationships or inferred values.
    ///
    /// ## Mermaid illustration
    ///
    /// ```mermaid
    /// flowchart LR
    /// classDef column-of-interest stroke: #f0746c,fill: #f49f9a
    /// classDef directly-involved-column stroke: #6c74f0,fill: #9a9ff4
    /// subgraph v3 ["`ball_mill_procedure_templates`"]
    ///    v1@{shape: rounded, label: "procedure_template_bead_model"}
    /// class v1 column-of-interest
    ///    v0@{shape: rounded, label: "bead_model"}
    /// class v0 directly-involved-column
    /// end
    /// subgraph v4 ["`procedure_template_asset_models`"]
    ///    v2@{shape: rounded, label: "asset_model"}
    /// class v2 directly-involved-column
    /// end
    /// v0 --->|"`associated same as`"| v2
    /// v3 ---o|"`associated with`"| v4
    /// ```
    fn procedure_template_bead_model(
        mut self,
        mut procedure_template_bead_model: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let (Some(local), Some(foreign)) =
            (self.bead_model, procedure_template_bead_model.asset_model)
        {
            if local != foreign {
                return Err(web_common_traits::database::InsertError::BuilderError(
                    web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                        Self::Attributes::BeadModel,
                    ),
                ));
            }
        } else if let Some(asset_model) = procedure_template_bead_model.asset_model {
            self.bead_model = Some(asset_model);
        } else if let Some(local) = self.bead_model {
            procedure_template_bead_model = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelBuildable>::asset_model(
                    procedure_template_bead_model,
                    local,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedureTemplateBeadModel(attribute)
                    })
                })?;
        }
        self.procedure_template_bead_model = procedure_template_bead_model;
        Ok(self)
    }
    /// Sets the value of the
    /// `public.ball_mill_procedure_templates.number_of_beads` column.
    fn number_of_beads<NOB>(
        mut self,
        number_of_beads: NOB,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        NOB: TryInto<i16>,
        validation_errors::SingleFieldError: From<<NOB as TryInto<i16>>::Error>,
    {
        let number_of_beads = number_of_beads.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(InsertableBallMillProcedureTemplateAttributes::NumberOfBeads)
        })?;
        pgrx_validation::must_be_strictly_positive_i16(number_of_beads)
            .map_err(|e| {
                e
                    .rename_field(
                        crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureTemplateAttributes::NumberOfBeads,
                    )
            })?;
        self.number_of_beads = Some(number_of_beads);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.ball_mill_procedure_templates.milled_with_model` column.
    ///
    /// # Implementation notes
    /// This method also set the values of other columns, due to
    /// same-as relationships or inferred values.
    ///
    /// ## Mermaid illustration
    ///
    /// ```mermaid
    /// flowchart LR
    /// classDef column-of-interest stroke: #f0746c,fill: #f49f9a
    /// classDef directly-involved-column stroke: #6c74f0,fill: #9a9ff4
    /// subgraph v3 ["`ball_mill_procedure_templates`"]
    ///    v0@{shape: rounded, label: "milled_with_model"}
    /// class v0 column-of-interest
    ///    v1@{shape: rounded, label: "procedure_template_milled_with_model"}
    /// class v1 directly-involved-column
    /// end
    /// subgraph v4 ["`procedure_template_asset_models`"]
    ///    v2@{shape: rounded, label: "asset_model"}
    /// class v2 directly-involved-column
    /// end
    /// v0 --->|"`associated same as`"| v2
    /// v3 ---o|"`associated with`"| v4
    /// ```
    fn milled_with_model(
        mut self,
        milled_with_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        let milled_with_model = milled_with_model.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(InsertableBallMillProcedureTemplateAttributes::MilledWithModel)
        })?;
        self.procedure_template_milled_with_model = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelBuildable>::asset_model(
                self.procedure_template_milled_with_model,
                milled_with_model,
            )
            .map_err(|e| {
                e.into_field_name(|attribute| {
                    Self::Attributes::ProcedureTemplateMilledWithModel(attribute)
                })
            })?;
        self.milled_with_model = Some(milled_with_model);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.ball_mill_procedure_templates.
    /// procedure_template_milled_with_model` column.
    ///
    /// # Implementation notes
    /// This method also set the values of other columns, due to
    /// same-as relationships or inferred values.
    ///
    /// ## Mermaid illustration
    ///
    /// ```mermaid
    /// flowchart LR
    /// classDef column-of-interest stroke: #f0746c,fill: #f49f9a
    /// classDef directly-involved-column stroke: #6c74f0,fill: #9a9ff4
    /// subgraph v3 ["`ball_mill_procedure_templates`"]
    ///    v0@{shape: rounded, label: "milled_with_model"}
    /// class v0 directly-involved-column
    ///    v1@{shape: rounded, label: "procedure_template_milled_with_model"}
    /// class v1 column-of-interest
    /// end
    /// subgraph v4 ["`procedure_template_asset_models`"]
    ///    v2@{shape: rounded, label: "asset_model"}
    /// class v2 directly-involved-column
    /// end
    /// v0 --->|"`associated same as`"| v2
    /// v3 ---o|"`associated with`"| v4
    /// ```
    fn procedure_template_milled_with_model(
        mut self,
        mut procedure_template_milled_with_model: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let (Some(local), Some(foreign)) =
            (self.milled_with_model, procedure_template_milled_with_model.asset_model)
        {
            if local != foreign {
                return Err(web_common_traits::database::InsertError::BuilderError(
                    web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                        Self::Attributes::MilledWithModel,
                    ),
                ));
            }
        } else if let Some(asset_model) = procedure_template_milled_with_model.asset_model {
            self.milled_with_model = Some(asset_model);
        } else if let Some(local) = self.milled_with_model {
            procedure_template_milled_with_model = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelBuildable>::asset_model(
                    procedure_template_milled_with_model,
                    local,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedureTemplateMilledWithModel(attribute)
                    })
                })?;
        }
        self.procedure_template_milled_with_model = procedure_template_milled_with_model;
        Ok(self)
    }
    /// Sets the value of the
    /// `public.ball_mill_procedure_templates.milled_container_model` column.
    fn milled_container_model(
        mut self,
        milled_container_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        let milled_container_model = milled_container_model.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(InsertableBallMillProcedureTemplateAttributes::MilledContainerModel)
        })?;
        self.milled_container_model = Some(milled_container_model);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.ball_mill_procedure_templates.foreign_procedure_template`
    /// column.
    fn foreign_procedure_template(
        mut self,
        foreign_procedure_template: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        let foreign_procedure_template = foreign_procedure_template.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err).rename_field(
                InsertableBallMillProcedureTemplateAttributes::ForeignProcedureTemplate,
            )
        })?;
        self.foreign_procedure_template = Some(foreign_procedure_template);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.ball_mill_procedure_templates.
    /// procedure_template_milled_container_model` column.
    fn procedure_template_milled_container_model(
        mut self,
        procedure_template_milled_container_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        let procedure_template_milled_container_model = procedure_template_milled_container_model
            .try_into()
            .map_err(|err| {
                validation_errors::SingleFieldError::from(err)
                    .rename_field(
                        InsertableBallMillProcedureTemplateAttributes::ProcedureTemplateMilledContainerModel,
                    )
            })?;
        self.procedure_template_milled_container_model =
            Some(procedure_template_milled_container_model);
        Ok(self)
    }
}
impl<
    ProcedureTemplate: crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAttributes,
        >,
> crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateBuildable
for InsertableBallMillProcedureTemplateBuilder<ProcedureTemplate> {
    type Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureTemplateAttributes;
    #[inline]
    ///Sets the value of the `public.procedure_templates.name` column.
    fn name<N>(
        mut self,
        name: N,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        N: TryInto<String>,
        validation_errors::SingleFieldError: From<<N as TryInto<String>>::Error>,
    {
        self.procedure_template = <ProcedureTemplate as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateBuildable>::name(
                self.procedure_template,
                name,
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
    ///Sets the value of the `public.procedure_templates.description` column.
    fn description<D>(
        mut self,
        description: D,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        D: TryInto<String>,
        validation_errors::SingleFieldError: From<<D as TryInto<String>>::Error>,
    {
        self.procedure_template = <ProcedureTemplate as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateBuildable>::description(
                self.procedure_template,
                description,
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
    ///Sets the value of the `public.procedure_templates.deprecated` column.
    fn deprecated<D>(
        mut self,
        deprecated: D,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        D: TryInto<bool>,
        validation_errors::SingleFieldError: From<<D as TryInto<bool>>::Error>,
    {
        self.procedure_template = <ProcedureTemplate as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateBuildable>::deprecated(
                self.procedure_template,
                deprecated,
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
    ///Sets the value of the `public.procedure_templates.icon` column.
    fn icon<I>(
        mut self,
        icon: I,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        I: TryInto<String>,
        validation_errors::SingleFieldError: From<<I as TryInto<String>>::Error>,
    {
        self.procedure_template = <ProcedureTemplate as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateBuildable>::icon(
                self.procedure_template,
                icon,
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
    ///Sets the value of the `public.procedure_templates.created_by` column.
    fn created_by(
        mut self,
        created_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.procedure_template = <ProcedureTemplate as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateBuildable>::created_by(
                self.procedure_template,
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
    ///Sets the value of the `public.procedure_templates.created_at` column.
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
        self.procedure_template = <ProcedureTemplate as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateBuildable>::created_at(
                self.procedure_template,
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
    ///Sets the value of the `public.procedure_templates.updated_by` column.
    fn updated_by(
        mut self,
        updated_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.procedure_template = <ProcedureTemplate as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateBuildable>::updated_by(
                self.procedure_template,
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
    ///Sets the value of the `public.procedure_templates.updated_at` column.
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
        self.procedure_template = <ProcedureTemplate as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateBuildable>::updated_at(
                self.procedure_template,
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
}
impl<ProcedureTemplate> web_common_traits::database::MostConcreteTable
    for InsertableBallMillProcedureTemplateBuilder<ProcedureTemplate>
where
    ProcedureTemplate: web_common_traits::database::MostConcreteTable,
{
    fn set_most_concrete_table(&mut self, table_name: &str) {
        self.procedure_template.set_most_concrete_table(table_name);
    }
}
impl<ProcedureTemplate> web_common_traits::prelude::SetPrimaryKey
    for InsertableBallMillProcedureTemplateBuilder<ProcedureTemplate>
where
    ProcedureTemplate: web_common_traits::prelude::SetPrimaryKey<PrimaryKey = i32>,
{
    type PrimaryKey = i32;
    fn set_primary_key(mut self, primary_key: Self::PrimaryKey) -> Self {
        self.procedure_template = self.procedure_template.set_primary_key(primary_key);
        self
    }
}
impl<ProcedureTemplate, C> web_common_traits::database::TryInsertGeneric<C>
for InsertableBallMillProcedureTemplateBuilder<ProcedureTemplate>
where
    Self: web_common_traits::database::InsertableVariant<
        C,
        UserId = i32,
        Row = crate::codegen::structs_codegen::tables::ball_mill_procedure_templates::BallMillProcedureTemplate,
        Error = web_common_traits::database::InsertError<
            InsertableBallMillProcedureTemplateAttributes,
        >,
    >,
    ProcedureTemplate: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder: web_common_traits::database::TryInsertGeneric<
        C,
    >,
{
    type Attributes = InsertableBallMillProcedureTemplateAttributes;
    fn is_complete(&self) -> bool {
        self.procedure_template.is_complete() && self.kelvin.is_some()
            && self.kelvin_tolerance_percentage.is_some() && self.seconds.is_some()
            && self.hertz.is_some() && self.bead_model.is_some()
            && self.procedure_template_bead_model.is_complete()
            && self.number_of_beads.is_some() && self.milled_with_model.is_some()
            && self.procedure_template_milled_with_model.is_complete()
            && self.milled_container_model.is_some()
            && self.foreign_procedure_template.is_some()
            && self.procedure_template_milled_container_model.is_some()
    }
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
        let insertable: crate::codegen::structs_codegen::tables::ball_mill_procedure_templates::BallMillProcedureTemplate = self
            .insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
