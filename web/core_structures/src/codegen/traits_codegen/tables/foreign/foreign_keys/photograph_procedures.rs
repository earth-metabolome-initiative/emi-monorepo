#[derive(Debug, Clone, PartialEq, Default, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PhotographProcedureForeignKeys {
    pub procedure: Option<
        crate::codegen::structs_codegen::tables::procedures::Procedure,
    >,
    pub procedure_template: Option<
        crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate,
    >,
    pub foreign_procedure_template: Option<
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate,
    >,
    pub foreign_procedure: Option<
        crate::codegen::structs_codegen::tables::procedures::Procedure,
    >,
    pub photographed_asset: Option<
        crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset,
    >,
    pub photographed_with: Option<
        crate::codegen::structs_codegen::tables::cameras::Camera,
    >,
    pub photographed_with_model: Option<
        crate::codegen::structs_codegen::tables::camera_models::CameraModel,
    >,
    pub photograph_procedures_procedure_photographed_with_model_fkey: Option<
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
    >,
}
impl web_common_traits::prelude::HasForeignKeys
    for crate::codegen::structs_codegen::tables::photograph_procedures::PhotographProcedure
{
    type ForeignKeys = PhotographProcedureForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Procedure(self.procedure),
        ));
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::PhotographProcedureTemplate(
                        self.procedure_template,
                    ),
                ),
            );
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplate(
                self.foreign_procedure_template,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Procedure(
                self.foreign_procedure,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::PhysicalAsset(
                self.photographed_asset,
            ),
        ));
        if let Some(photographed_with) = self.photographed_with {
            connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Camera(
                    photographed_with,
                ),
            ));
        }
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::CameraModel(
                self.photographed_with_model,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureAsset((
                self.procedure,
                self.photographed_with_model,
            )),
        ));
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.procedure.is_some()
            && foreign_keys.procedure_template.is_some()
            && foreign_keys.foreign_procedure_template.is_some()
            && foreign_keys.foreign_procedure.is_some()
            && foreign_keys.photographed_asset.is_some()
            && (foreign_keys.photographed_with.is_some() || self.photographed_with.is_some())
            && foreign_keys.photographed_with_model.is_some()
            && foreign_keys.photograph_procedures_procedure_photographed_with_model_fkey.is_some()
    }
    fn update(
        &self,
        foreign_keys: &mut Self::ForeignKeys,
        row: Self::Row,
        crud: web_common_traits::crud::CRUD,
    ) -> bool {
        let mut updated = false;
        match (row, crud) {
            (
                crate::codegen::tables::row::Row::CameraModel(camera_models),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.photographed_with_model == camera_models.id {
                    foreign_keys.photographed_with_model = Some(camera_models);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::CameraModel(camera_models),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.photographed_with_model == camera_models.id {
                    foreign_keys.photographed_with_model = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Camera(cameras),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self
                    .photographed_with
                    .is_some_and(|photographed_with| photographed_with == cameras.id)
                {
                    foreign_keys.photographed_with = Some(cameras);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Camera(cameras),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self
                    .photographed_with
                    .is_some_and(|photographed_with| photographed_with == cameras.id)
                {
                    foreign_keys.photographed_with = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::PhotographProcedureTemplate(
                    photograph_procedure_templates,
                ),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.procedure_template == photograph_procedure_templates.procedure_template {
                    foreign_keys.procedure_template = Some(photograph_procedure_templates);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::PhotographProcedureTemplate(
                    photograph_procedure_templates,
                ),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.procedure_template == photograph_procedure_templates.procedure_template {
                    foreign_keys.procedure_template = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::PhysicalAsset(physical_assets),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.photographed_asset == physical_assets.id {
                    foreign_keys.photographed_asset = Some(physical_assets);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::PhysicalAsset(physical_assets),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.photographed_asset == physical_assets.id {
                    foreign_keys.photographed_asset = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ProcedureAsset(procedure_assets),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.procedure == procedure_assets.procedure
                    && self.photographed_with_model == procedure_assets.asset_model
                {
                    foreign_keys.photograph_procedures_procedure_photographed_with_model_fkey =
                        Some(procedure_assets);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ProcedureAsset(procedure_assets),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.procedure == procedure_assets.procedure
                    && self.photographed_with_model == procedure_assets.asset_model
                {
                    foreign_keys.photograph_procedures_procedure_photographed_with_model_fkey =
                        None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ProcedureTemplate(procedure_templates),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.foreign_procedure_template == procedure_templates.procedure_template {
                    foreign_keys.foreign_procedure_template = Some(procedure_templates);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ProcedureTemplate(procedure_templates),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.foreign_procedure_template == procedure_templates.procedure_template {
                    foreign_keys.foreign_procedure_template = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Procedure(procedures),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.procedure == procedures.procedure {
                    foreign_keys.procedure = Some(procedures.clone());
                    updated = true;
                }
                if self.foreign_procedure == procedures.procedure {
                    foreign_keys.foreign_procedure = Some(procedures.clone());
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Procedure(procedures),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.procedure == procedures.procedure {
                    foreign_keys.procedure = None;
                    updated = true;
                }
                if self.foreign_procedure == procedures.procedure {
                    foreign_keys.foreign_procedure = None;
                    updated = true;
                }
            }
            (_, crud) => {
                unreachable!("Unexpected row type with operation {crud}");
            }
        }
        updated
    }
}
impl web_common_traits::prelude::ForeignKeys for PhotographProcedureForeignKeys {}
