#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableSupernatantProcedureTemplateExtensionAttributes {
    ProcedureTemplate(
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAttributes,
    ),
}
impl core::fmt::Display for InsertableSupernatantProcedureTemplateExtensionAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::ProcedureTemplate(e) => write!(f, "{e}"),
        }
    }
}
impl
    From<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAttributes,
    > for InsertableSupernatantProcedureTemplateExtensionAttributes
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAttributes,
    ) -> Self {
        Self::ProcedureTemplate(attribute)
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableSupernatantProcedureTemplateAttributes {
    Extension(InsertableSupernatantProcedureTemplateExtensionAttributes),
    ProcedureTemplate,
    Liters,
    StratifiedSourceModel,
    ForeignProcedureTemplate,
    ProcedureTemplateStratifiedSourceModel,
    SupernatantDestinationModel,
    ProcedureTemplateSupernatantDestinationModel(
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelAttributes,
    ),
    TransferredWithModel,
    ProcedureTemplateTransferredWithModel(
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelAttributes,
    ),
    PipetteTipModel,
    ProcedureTemplatePipetteTipModel(
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelAttributes,
    ),
}
impl core::str::FromStr for InsertableSupernatantProcedureTemplateAttributes {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Liters" => Ok(Self::Liters),
            "StratifiedSourceModel" => Ok(Self::StratifiedSourceModel),
            "ForeignProcedureTemplate" => Ok(Self::ForeignProcedureTemplate),
            "ProcedureTemplateStratifiedSourceModel" => {
                Ok(Self::ProcedureTemplateStratifiedSourceModel)
            }
            "SupernatantDestinationModel" => Ok(Self::SupernatantDestinationModel),
            "ProcedureTemplateSupernatantDestinationModel" => {
                Ok(
                    Self::ProcedureTemplateSupernatantDestinationModel(
                        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelAttributes::Id,
                    ),
                )
            }
            "TransferredWithModel" => Ok(Self::TransferredWithModel),
            "ProcedureTemplateTransferredWithModel" => {
                Ok(
                    Self::ProcedureTemplateTransferredWithModel(
                        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelAttributes::Id,
                    ),
                )
            }
            "PipetteTipModel" => Ok(Self::PipetteTipModel),
            "ProcedureTemplatePipetteTipModel" => {
                Ok(
                    Self::ProcedureTemplatePipetteTipModel(
                        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelAttributes::Id,
                    ),
                )
            }
            "liters" => Ok(Self::Liters),
            "stratified_source_model" => Ok(Self::StratifiedSourceModel),
            "foreign_procedure_template" => Ok(Self::ForeignProcedureTemplate),
            "procedure_template_stratified_source_model" => {
                Ok(Self::ProcedureTemplateStratifiedSourceModel)
            }
            "supernatant_destination_model" => Ok(Self::SupernatantDestinationModel),
            "procedure_template_supernatant_destination_model" => {
                Ok(
                    Self::ProcedureTemplateSupernatantDestinationModel(
                        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelAttributes::Id,
                    ),
                )
            }
            "transferred_with_model" => Ok(Self::TransferredWithModel),
            "procedure_template_transferred_with_model" => {
                Ok(
                    Self::ProcedureTemplateTransferredWithModel(
                        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelAttributes::Id,
                    ),
                )
            }
            "pipette_tip_model" => Ok(Self::PipetteTipModel),
            "procedure_template_pipette_tip_model" => {
                Ok(
                    Self::ProcedureTemplatePipetteTipModel(
                        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelAttributes::Id,
                    ),
                )
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
impl core::fmt::Display for InsertableSupernatantProcedureTemplateAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::ProcedureTemplate => write!(f, "procedure_template"),
            Self::Liters => write!(f, "liters"),
            Self::StratifiedSourceModel => write!(f, "stratified_source_model"),
            Self::ForeignProcedureTemplate => write!(f, "foreign_procedure_template"),
            Self::ProcedureTemplateStratifiedSourceModel => {
                write!(f, "procedure_template_stratified_source_model")
            }
            Self::SupernatantDestinationModel => {
                write!(f, "supernatant_destination_model")
            }
            Self::ProcedureTemplateSupernatantDestinationModel(e) => write!(f, "{e}"),
            Self::TransferredWithModel => write!(f, "transferred_with_model"),
            Self::ProcedureTemplateTransferredWithModel(e) => write!(f, "{e}"),
            Self::PipetteTipModel => write!(f, "pipette_tip_model"),
            Self::ProcedureTemplatePipetteTipModel(e) => write!(f, "{e}"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::supernatant_procedure_templates::supernatant_procedure_templates
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableSupernatantProcedureTemplate {
    pub(crate) procedure_template: i32,
    pub(crate) liters: f32,
    pub(crate) stratified_source_model: i32,
    pub(crate) foreign_procedure_template: i32,
    pub(crate) procedure_template_stratified_source_model: i32,
    pub(crate) supernatant_destination_model: i32,
    pub(crate) procedure_template_supernatant_destination_model: i32,
    pub(crate) transferred_with_model: i32,
    pub(crate) procedure_template_transferred_with_model: i32,
    pub(crate) pipette_tip_model: i32,
    pub(crate) procedure_template_pipette_tip_model: i32,
}
impl InsertableSupernatantProcedureTemplate {
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
    pub fn stratified_source_model<C: diesel::connection::LoadConnection>(
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
                self.stratified_source_model,
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
                self.procedure_template_stratified_source_model,
            ),
            conn,
        )
    }
    pub fn supernatant_destination_model<C: diesel::connection::LoadConnection>(
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
                self.supernatant_destination_model,
            ),
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
                self.procedure_template_supernatant_destination_model,
            ),
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
        crate::codegen::structs_codegen::tables::pipette_models::PipetteModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::pipette_models::PipetteModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::pipette_models::PipetteModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::pipette_models::PipetteModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::pipette_models::PipetteModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::pipette_models::PipetteModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::pipette_models::PipetteModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::pipette_models::PipetteModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::pipette_models::PipetteModel::table(),
                self.transferred_with_model,
            ),
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
                self.procedure_template_transferred_with_model,
            ),
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
        crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel::table(
                ),
                self.pipette_tip_model,
            ),
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
                self.procedure_template_pipette_tip_model,
            ),
            conn,
        )
    }
    pub fn supernatant_pm_compatibility_rules<C: diesel::connection::LoadConnection>(
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
                (self.transferred_with_model, self.pipette_tip_model),
            ),
            conn,
        )
    }
}
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableSupernatantProcedureTemplateBuilder<
    ProcedureTemplate
        = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateBuilder,
> {
    pub(crate) liters: Option<f32>,
    pub(crate) stratified_source_model: Option<i32>,
    pub(crate) foreign_procedure_template: Option<i32>,
    pub(crate) procedure_template_stratified_source_model: Option<i32>,
    pub(crate) supernatant_destination_model: Option<i32>,
    pub(crate) procedure_template_supernatant_destination_model: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
    pub(crate) transferred_with_model: Option<i32>,
    pub(crate) procedure_template_transferred_with_model: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
    pub(crate) pipette_tip_model: Option<i32>,
    pub(crate) procedure_template_pipette_tip_model: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
    pub(crate) procedure_template: ProcedureTemplate,
}
/// Trait defining setters for attributes of an instance of
/// `SupernatantProcedureTemplate` or descendant tables.
pub trait SupernatantProcedureTemplateBuildable: Sized {
    /// Attributes required to build the insertable.
    type Attributes;
    /// Sets the value of the `public.supernatant_procedure_templates.liters`
    /// column.
    ///
    /// # Arguments
    /// * `liters`: The value to set for the
    ///   `public.supernatant_procedure_templates.liters` column.
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
    fn liters<L>(
        self,
        liters: L,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        L: TryInto<f32>,
        validation_errors::SingleFieldError: From<<L as TryInto<f32>>::Error>;
    /// Sets the value of the
    /// `public.supernatant_procedure_templates.stratified_source_model` column.
    ///
    /// # Arguments
    /// * `stratified_source_model`: The value to set for the
    ///   `public.supernatant_procedure_templates.stratified_source_model`
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
    fn stratified_source_model(
        self,
        stratified_source_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the
    /// `public.supernatant_procedure_templates.foreign_procedure_template`
    /// column.
    ///
    /// # Arguments
    /// * `foreign_procedure_template`: The value to set for the
    ///   `public.supernatant_procedure_templates.foreign_procedure_template`
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
    /// `public.supernatant_procedure_templates.
    /// procedure_template_stratified_source_model` column.
    ///
    /// # Arguments
    /// * `procedure_template_stratified_source_model`: The value to set for the
    ///   `public.supernatant_procedure_templates.
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
    /// `public.supernatant_procedure_templates.supernatant_destination_model`
    /// column.
    ///
    /// # Arguments
    /// * `supernatant_destination_model`: The value to set for the
    ///   `public.supernatant_procedure_templates.supernatant_destination_model`
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
    fn supernatant_destination_model(
        self,
        supernatant_destination_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the
    /// `public.supernatant_procedure_templates.
    /// procedure_template_supernatant_destination_model` column.
    ///
    /// # Arguments
    /// * `procedure_template_supernatant_destination_model`: The value to set
    ///   for the `public.supernatant_procedure_templates.
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
        procedure_template_supernatant_destination_model: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the
    /// `public.supernatant_procedure_templates.transferred_with_model` column.
    ///
    /// # Arguments
    /// * `transferred_with_model`: The value to set for the
    ///   `public.supernatant_procedure_templates.transferred_with_model`
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
    fn transferred_with_model(
        self,
        transferred_with_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the
    /// `public.supernatant_procedure_templates.
    /// procedure_template_transferred_with_model` column.
    ///
    /// # Arguments
    /// * `procedure_template_transferred_with_model`: The value to set for the
    ///   `public.supernatant_procedure_templates.
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
        procedure_template_transferred_with_model: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the
    /// `public.supernatant_procedure_templates.pipette_tip_model` column.
    ///
    /// # Arguments
    /// * `pipette_tip_model`: The value to set for the
    ///   `public.supernatant_procedure_templates.pipette_tip_model` column.
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
    /// `public.supernatant_procedure_templates.
    /// procedure_template_pipette_tip_model` column.
    ///
    /// # Arguments
    /// * `procedure_template_pipette_tip_model`: The value to set for the
    ///   `public.supernatant_procedure_templates.
    ///   procedure_template_pipette_tip_model` column.
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
        procedure_template_pipette_tip_model: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
}
impl<ProcedureTemplate> SupernatantProcedureTemplateBuildable
    for InsertableSupernatantProcedureTemplateBuilder<ProcedureTemplate>
{
    type Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableSupernatantProcedureTemplateAttributes;
    /// Sets the value of the `public.supernatant_procedure_templates.liters`
    /// column.
    fn liters<L>(
        mut self,
        liters: L,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        L: TryInto<f32>,
        validation_errors::SingleFieldError: From<<L as TryInto<f32>>::Error>,
    {
        let liters = liters.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(InsertableSupernatantProcedureTemplateAttributes::Liters)
        })?;
        pgrx_validation::must_be_strictly_positive_f32(liters)
            .map_err(|e| {
                e
                    .rename_field(
                        crate::codegen::structs_codegen::tables::insertables::InsertableSupernatantProcedureTemplateAttributes::Liters,
                    )
            })?;
        self.liters = Some(liters);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.supernatant_procedure_templates.stratified_source_model` column.
    fn stratified_source_model(
        mut self,
        stratified_source_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.stratified_source_model = Some(stratified_source_model);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.supernatant_procedure_templates.foreign_procedure_template`
    /// column.
    fn foreign_procedure_template(
        mut self,
        foreign_procedure_template: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.foreign_procedure_template = Some(foreign_procedure_template);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.supernatant_procedure_templates.
    /// procedure_template_stratified_source_model` column.
    fn procedure_template_stratified_source_model(
        mut self,
        procedure_template_stratified_source_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.procedure_template_stratified_source_model =
            Some(procedure_template_stratified_source_model);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.supernatant_procedure_templates.supernatant_destination_model`
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
    /// classDef undirectly-involved-column stroke: #a7eff0,stroke-dasharray: 5, 5,fill: #d2f6f7
    /// subgraph v4 ["`procedure_template_asset_models`"]
    ///    v3@{shape: rounded, label: "id"}
    /// class v3 undirectly-involved-column
    ///    v0@{shape: rounded, label: "asset_model"}
    /// class v0 directly-involved-column
    /// end
    /// subgraph v5 ["`supernatant_procedure_templates`"]
    ///    v2@{shape: rounded, label: "supernatant_destination_model"}
    /// class v2 column-of-interest
    ///    v1@{shape: rounded, label: "procedure_template_supernatant_destination_model"}
    /// class v1 directly-involved-column
    /// end
    /// v2 --->|"`associated same as`"| v0
    /// v1 --->|"`associated same as`"| v3
    /// v5 ---o|"`associated with`"| v4
    /// ```
    fn supernatant_destination_model(
        mut self,
        supernatant_destination_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.procedure_template_supernatant_destination_model = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelBuildable>::asset_model(
                self.procedure_template_supernatant_destination_model,
                supernatant_destination_model,
            )
            .map_err(|e| {
                e.into_field_name(|attribute| {
                    Self::Attributes::ProcedureTemplateSupernatantDestinationModel(
                        attribute,
                    )
                })
            })?;
        self.supernatant_destination_model = Some(supernatant_destination_model);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.supernatant_procedure_templates.
    /// procedure_template_supernatant_destination_model` column.
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
    /// classDef undirectly-involved-column stroke: #a7eff0,stroke-dasharray: 5, 5,fill: #d2f6f7
    /// subgraph v4 ["`procedure_template_asset_models`"]
    ///    v0@{shape: rounded, label: "asset_model"}
    /// class v0 directly-involved-column
    ///    v3@{shape: rounded, label: "id"}
    /// class v3 undirectly-involved-column
    /// end
    /// subgraph v5 ["`supernatant_procedure_templates`"]
    ///    v1@{shape: rounded, label: "procedure_template_supernatant_destination_model"}
    /// class v1 column-of-interest
    ///    v2@{shape: rounded, label: "supernatant_destination_model"}
    /// class v2 directly-involved-column
    /// end
    /// v1 --->|"`associated same as`"| v3
    /// v2 --->|"`associated same as`"| v0
    /// v5 ---o|"`associated with`"| v4
    /// ```
    fn procedure_template_supernatant_destination_model(
        mut self,
        mut procedure_template_supernatant_destination_model: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let (Some(local), Some(foreign)) = (
            self.supernatant_destination_model,
            procedure_template_supernatant_destination_model.asset_model,
        ) {
            if local != foreign {
                return Err(web_common_traits::database::InsertError::BuilderError(
                    web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                        Self::Attributes::SupernatantDestinationModel,
                    ),
                ));
            }
        } else if let Some(asset_model) =
            procedure_template_supernatant_destination_model.asset_model
        {
            self.supernatant_destination_model = Some(asset_model);
        } else if let Some(local) = self.supernatant_destination_model {
            procedure_template_supernatant_destination_model = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelBuildable>::asset_model(
                    procedure_template_supernatant_destination_model,
                    local,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedureTemplateSupernatantDestinationModel(
                            attribute,
                        )
                    })
                })?;
        }
        self.procedure_template_supernatant_destination_model =
            procedure_template_supernatant_destination_model;
        Ok(self)
    }
    /// Sets the value of the
    /// `public.supernatant_procedure_templates.transferred_with_model` column.
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
    /// classDef undirectly-involved-column stroke: #a7eff0,stroke-dasharray: 5, 5,fill: #d2f6f7
    /// subgraph v4 ["`procedure_template_asset_models`"]
    ///    v3@{shape: rounded, label: "id"}
    /// class v3 undirectly-involved-column
    ///    v0@{shape: rounded, label: "asset_model"}
    /// class v0 directly-involved-column
    /// end
    /// subgraph v5 ["`supernatant_procedure_templates`"]
    ///    v1@{shape: rounded, label: "procedure_template_transferred_with_model"}
    /// class v1 directly-involved-column
    ///    v2@{shape: rounded, label: "transferred_with_model"}
    /// class v2 column-of-interest
    /// end
    /// v1 --->|"`associated same as`"| v3
    /// v2 --->|"`associated same as`"| v0
    /// v5 ---o|"`associated with`"| v4
    /// ```
    fn transferred_with_model(
        mut self,
        transferred_with_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.procedure_template_transferred_with_model = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelBuildable>::asset_model(
                self.procedure_template_transferred_with_model,
                transferred_with_model,
            )
            .map_err(|e| {
                e.into_field_name(|attribute| {
                    Self::Attributes::ProcedureTemplateTransferredWithModel(attribute)
                })
            })?;
        self.transferred_with_model = Some(transferred_with_model);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.supernatant_procedure_templates.
    /// procedure_template_transferred_with_model` column.
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
    /// classDef undirectly-involved-column stroke: #a7eff0,stroke-dasharray: 5, 5,fill: #d2f6f7
    /// subgraph v4 ["`procedure_template_asset_models`"]
    ///    v0@{shape: rounded, label: "asset_model"}
    /// class v0 directly-involved-column
    ///    v3@{shape: rounded, label: "id"}
    /// class v3 undirectly-involved-column
    /// end
    /// subgraph v5 ["`supernatant_procedure_templates`"]
    ///    v1@{shape: rounded, label: "procedure_template_transferred_with_model"}
    /// class v1 column-of-interest
    ///    v2@{shape: rounded, label: "transferred_with_model"}
    /// class v2 directly-involved-column
    /// end
    /// v1 --->|"`associated same as`"| v3
    /// v2 --->|"`associated same as`"| v0
    /// v5 ---o|"`associated with`"| v4
    /// ```
    fn procedure_template_transferred_with_model(
        mut self,
        mut procedure_template_transferred_with_model: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let (Some(local), Some(foreign)) =
            (self.transferred_with_model, procedure_template_transferred_with_model.asset_model)
        {
            if local != foreign {
                return Err(web_common_traits::database::InsertError::BuilderError(
                    web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                        Self::Attributes::TransferredWithModel,
                    ),
                ));
            }
        } else if let Some(asset_model) = procedure_template_transferred_with_model.asset_model {
            self.transferred_with_model = Some(asset_model);
        } else if let Some(local) = self.transferred_with_model {
            procedure_template_transferred_with_model = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelBuildable>::asset_model(
                    procedure_template_transferred_with_model,
                    local,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedureTemplateTransferredWithModel(
                            attribute,
                        )
                    })
                })?;
        }
        self.procedure_template_transferred_with_model = procedure_template_transferred_with_model;
        Ok(self)
    }
    /// Sets the value of the
    /// `public.supernatant_procedure_templates.pipette_tip_model` column.
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
    /// classDef undirectly-involved-column stroke: #a7eff0,stroke-dasharray: 5, 5,fill: #d2f6f7
    /// subgraph v4 ["`procedure_template_asset_models`"]
    ///    v3@{shape: rounded, label: "id"}
    /// class v3 undirectly-involved-column
    ///    v0@{shape: rounded, label: "asset_model"}
    /// class v0 directly-involved-column
    /// end
    /// subgraph v5 ["`supernatant_procedure_templates`"]
    ///    v1@{shape: rounded, label: "pipette_tip_model"}
    /// class v1 column-of-interest
    ///    v2@{shape: rounded, label: "procedure_template_pipette_tip_model"}
    /// class v2 directly-involved-column
    /// end
    /// v1 --->|"`associated same as`"| v0
    /// v2 --->|"`associated same as`"| v3
    /// v5 ---o|"`associated with`"| v4
    /// ```
    fn pipette_tip_model(
        mut self,
        pipette_tip_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.procedure_template_pipette_tip_model = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelBuildable>::asset_model(
                self.procedure_template_pipette_tip_model,
                pipette_tip_model,
            )
            .map_err(|e| {
                e.into_field_name(|attribute| {
                    Self::Attributes::ProcedureTemplatePipetteTipModel(attribute)
                })
            })?;
        self.pipette_tip_model = Some(pipette_tip_model);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.supernatant_procedure_templates.
    /// procedure_template_pipette_tip_model` column.
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
    /// classDef undirectly-involved-column stroke: #a7eff0,stroke-dasharray: 5, 5,fill: #d2f6f7
    /// subgraph v4 ["`procedure_template_asset_models`"]
    ///    v0@{shape: rounded, label: "asset_model"}
    /// class v0 directly-involved-column
    ///    v3@{shape: rounded, label: "id"}
    /// class v3 undirectly-involved-column
    /// end
    /// subgraph v5 ["`supernatant_procedure_templates`"]
    ///    v2@{shape: rounded, label: "procedure_template_pipette_tip_model"}
    /// class v2 column-of-interest
    ///    v1@{shape: rounded, label: "pipette_tip_model"}
    /// class v1 directly-involved-column
    /// end
    /// v2 --->|"`associated same as`"| v3
    /// v1 --->|"`associated same as`"| v0
    /// v5 ---o|"`associated with`"| v4
    /// ```
    fn procedure_template_pipette_tip_model(
        mut self,
        mut procedure_template_pipette_tip_model: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let (Some(local), Some(foreign)) =
            (self.pipette_tip_model, procedure_template_pipette_tip_model.asset_model)
        {
            if local != foreign {
                return Err(web_common_traits::database::InsertError::BuilderError(
                    web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                        Self::Attributes::PipetteTipModel,
                    ),
                ));
            }
        } else if let Some(asset_model) = procedure_template_pipette_tip_model.asset_model {
            self.pipette_tip_model = Some(asset_model);
        } else if let Some(local) = self.pipette_tip_model {
            procedure_template_pipette_tip_model = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelBuildable>::asset_model(
                    procedure_template_pipette_tip_model,
                    local,
                )
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        Self::Attributes::ProcedureTemplatePipetteTipModel(attribute)
                    })
                })?;
        }
        self.procedure_template_pipette_tip_model = procedure_template_pipette_tip_model;
        Ok(self)
    }
}
impl<
    ProcedureTemplate: crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateBuildable<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAttributes,
        >,
> crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateBuildable
for InsertableSupernatantProcedureTemplateBuilder<ProcedureTemplate> {
    type Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableSupernatantProcedureTemplateAttributes;
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
    for InsertableSupernatantProcedureTemplateBuilder<ProcedureTemplate>
where
    ProcedureTemplate: web_common_traits::database::MostConcreteTable,
{
    fn set_most_concrete_table(&mut self, table_name: &str) {
        self.procedure_template.set_most_concrete_table(table_name);
    }
}
impl<ProcedureTemplate> web_common_traits::prelude::SetPrimaryKey
    for InsertableSupernatantProcedureTemplateBuilder<ProcedureTemplate>
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
for InsertableSupernatantProcedureTemplateBuilder<ProcedureTemplate>
where
    Self: web_common_traits::database::InsertableVariant<
        C,
        UserId = i32,
        Row = crate::codegen::structs_codegen::tables::supernatant_procedure_templates::SupernatantProcedureTemplate,
        Error = web_common_traits::database::InsertError<
            InsertableSupernatantProcedureTemplateAttributes,
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
    type Attributes = InsertableSupernatantProcedureTemplateAttributes;
    fn is_complete(&self) -> bool {
        self.procedure_template.is_complete() && self.liters.is_some()
            && self.stratified_source_model.is_some()
            && self.foreign_procedure_template.is_some()
            && self.procedure_template_stratified_source_model.is_some()
            && self.supernatant_destination_model.is_some()
            && self.procedure_template_supernatant_destination_model.is_complete()
            && self.transferred_with_model.is_some()
            && self.procedure_template_transferred_with_model.is_complete()
            && self.pipette_tip_model.is_some()
            && self.procedure_template_pipette_tip_model.is_complete()
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
        let insertable: crate::codegen::structs_codegen::tables::supernatant_procedure_templates::SupernatantProcedureTemplate = self
            .insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
