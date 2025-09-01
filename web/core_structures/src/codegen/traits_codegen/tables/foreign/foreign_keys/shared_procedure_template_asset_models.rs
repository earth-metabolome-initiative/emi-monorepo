#[derive(Debug, Clone, PartialEq, Default, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SharedProcedureTemplateAssetModelForeignKeys {
    pub parent: Option<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
    >,
    pub parent_asset_model: Option<
        crate::codegen::structs_codegen::tables::asset_models::AssetModel,
    >,
    pub parent_procedure_template: Option<
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate,
    >,
    pub child: Option<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
    >,
    pub child_asset_model: Option<
        crate::codegen::structs_codegen::tables::asset_models::AssetModel,
    >,
    pub child_procedure_template: Option<
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate,
    >,
    pub created_by: Option<crate::codegen::structs_codegen::tables::users::User>,
    pub shared_procedure_template_ass_parent_procedure_template_ch_fkey: Option<
        crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate,
    >,
    pub shared_procedure_template_ass_parent_asset_model_child_ass_fkey: Option<
        crate::codegen::structs_codegen::tables::asset_model_ancestors::AssetModelAncestor,
    >,
}
impl web_common_traits::prelude::HasForeignKeys
for crate::codegen::structs_codegen::tables::shared_procedure_template_asset_models::SharedProcedureTemplateAssetModel {
    type ForeignKeys = SharedProcedureTemplateAssetModelForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplateAssetModel(
                        self.parent,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::AssetModel(
                        self.parent_asset_model,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplate(
                        self.parent_procedure_template,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplateAssetModel(
                        self.child_id,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::AssetModel(
                        self.child_asset_model,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplate(
                        self.child_procedure_template,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::User(
                        self.created_by,
                    ),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::ParentProcedureTemplate((
                        self.parent_procedure_template,
                        self.child_procedure_template,
                    )),
                ),
            );
        connector
            .send(
                web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                    crate::codegen::tables::table_primary_keys::TablePrimaryKey::AssetModelAncestor((
                        self.parent_asset_model,
                        self.child_asset_model,
                    )),
                ),
            );
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.parent.is_some() && foreign_keys.parent_asset_model.is_some()
            && foreign_keys.parent_procedure_template.is_some()
            && foreign_keys.child.is_some() && foreign_keys.child_asset_model.is_some()
            && foreign_keys.child_procedure_template.is_some()
            && foreign_keys.created_by.is_some()
            && foreign_keys
                .shared_procedure_template_ass_parent_procedure_template_ch_fkey
                .is_some()
            && foreign_keys
                .shared_procedure_template_ass_parent_asset_model_child_ass_fkey
                .is_some()
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
                crate::codegen::tables::row::Row::ProcedureTemplate(procedure_templates),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.parent_procedure_template
                    == procedure_templates.procedure_template
                {
                    foreign_keys.parent_procedure_template = Some(
                        procedure_templates.clone(),
                    );
                    updated = true;
                }
                if self.child_procedure_template
                    == procedure_templates.procedure_template
                {
                    foreign_keys.child_procedure_template = Some(
                        procedure_templates.clone(),
                    );
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ProcedureTemplate(procedure_templates),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.parent_procedure_template
                    == procedure_templates.procedure_template
                {
                    foreign_keys.parent_procedure_template = None;
                    updated = true;
                }
                if self.child_procedure_template
                    == procedure_templates.procedure_template
                {
                    foreign_keys.child_procedure_template = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::AssetModelAncestor(
                    asset_model_ancestors,
                ),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.parent_asset_model == asset_model_ancestors.descendant_model
                    && self.child_asset_model == asset_model_ancestors.ancestor_model
                {
                    foreign_keys
                        .shared_procedure_template_ass_parent_asset_model_child_ass_fkey = Some(
                        asset_model_ancestors,
                    );
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::AssetModelAncestor(
                    asset_model_ancestors,
                ),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.parent_asset_model == asset_model_ancestors.descendant_model
                    && self.child_asset_model == asset_model_ancestors.ancestor_model
                {
                    foreign_keys
                        .shared_procedure_template_ass_parent_asset_model_child_ass_fkey = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::AssetModel(asset_models),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.parent_asset_model == asset_models.id {
                    foreign_keys.parent_asset_model = Some(asset_models.clone());
                    updated = true;
                }
                if self.child_asset_model == asset_models.id {
                    foreign_keys.child_asset_model = Some(asset_models.clone());
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::AssetModel(asset_models),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.parent_asset_model == asset_models.id {
                    foreign_keys.parent_asset_model = None;
                    updated = true;
                }
                if self.child_asset_model == asset_models.id {
                    foreign_keys.child_asset_model = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ParentProcedureTemplate(
                    parent_procedure_templates,
                ),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.parent_procedure_template
                    == parent_procedure_templates.parent_procedure_template
                    && self.child_procedure_template
                        == parent_procedure_templates.child_procedure_template
                {
                    foreign_keys
                        .shared_procedure_template_ass_parent_procedure_template_ch_fkey = Some(
                        parent_procedure_templates,
                    );
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ParentProcedureTemplate(
                    parent_procedure_templates,
                ),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.parent_procedure_template
                    == parent_procedure_templates.parent_procedure_template
                    && self.child_procedure_template
                        == parent_procedure_templates.child_procedure_template
                {
                    foreign_keys
                        .shared_procedure_template_ass_parent_procedure_template_ch_fkey = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ProcedureTemplateAssetModel(
                    procedure_template_asset_models,
                ),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.parent == procedure_template_asset_models.id {
                    foreign_keys.parent = Some(procedure_template_asset_models.clone());
                    updated = true;
                }
                if self.child_id == procedure_template_asset_models.id {
                    foreign_keys.child = Some(procedure_template_asset_models.clone());
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ProcedureTemplateAssetModel(
                    procedure_template_asset_models,
                ),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.parent == procedure_template_asset_models.id {
                    foreign_keys.parent = None;
                    updated = true;
                }
                if self.child_id == procedure_template_asset_models.id {
                    foreign_keys.child = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::User(users),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.created_by == users.id {
                    foreign_keys.created_by = Some(users);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::User(users),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.created_by == users.id {
                    foreign_keys.created_by = None;
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
impl web_common_traits::prelude::ForeignKeys for SharedProcedureTemplateAssetModelForeignKeys {}
