#[derive(Debug, Clone, PartialEq, Default, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProcedureForeignKeys {
    pub procedure_template: Option<
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate,
    >,
    pub parent_procedure: Option<
        crate::codegen::structs_codegen::tables::procedures::Procedure,
    >,
    pub parent_procedure_template: Option<
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate,
    >,
    pub created_by: Option<crate::codegen::structs_codegen::tables::users::User>,
    pub updated_by: Option<crate::codegen::structs_codegen::tables::users::User>,
    pub procedures_parent_procedure_template_procedure_template_fkey: Option<
        crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate,
    >,
}
impl web_common_traits::prelude::HasForeignKeys
    for crate::codegen::structs_codegen::tables::procedures::Procedure
{
    type ForeignKeys = ProcedureForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplate(
                self.procedure_template,
            ),
        ));
        if let Some(parent_procedure) = self.parent_procedure {
            connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::Procedure(
                    parent_procedure,
                ),
            ));
        }
        if let Some(parent_procedure_template) = self.parent_procedure_template {
            connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplate(
                    parent_procedure_template,
                ),
            ));
        }
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::User(self.created_by),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::User(self.updated_by),
        ));
        if let Some(parent_procedure_template) = self.parent_procedure_template {
            connector
                .send(
                    web_common_traits::crud::CrudPrimaryKeyOperation::Read(
                        crate::codegen::tables::table_primary_keys::TablePrimaryKey::ParentProcedureTemplate((
                            parent_procedure_template,
                            self.procedure_template,
                        )),
                    ),
                );
        }
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.procedure_template.is_some()
            && (foreign_keys.parent_procedure.is_some() || self.parent_procedure.is_some())
            && (foreign_keys.parent_procedure_template.is_some()
                || self.parent_procedure_template.is_some())
            && foreign_keys.created_by.is_some()
            && foreign_keys.updated_by.is_some()
            && (foreign_keys.procedures_parent_procedure_template_procedure_template_fkey.is_some()
                || self.parent_procedure_template.is_some())
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
                crate::codegen::tables::row::Row::ParentProcedureTemplate(
                    parent_procedure_templates,
                ),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.parent_procedure_template.is_some_and(|parent_procedure_template| {
                    parent_procedure_template == parent_procedure_templates.parent
                }) && self.procedure_template == parent_procedure_templates.child
                {
                    foreign_keys.procedures_parent_procedure_template_procedure_template_fkey =
                        Some(parent_procedure_templates);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ParentProcedureTemplate(
                    parent_procedure_templates,
                ),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.parent_procedure_template.is_some_and(|parent_procedure_template| {
                    parent_procedure_template == parent_procedure_templates.parent
                }) && self.procedure_template == parent_procedure_templates.child
                {
                    foreign_keys.procedures_parent_procedure_template_procedure_template_fkey =
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
                if self.procedure_template == procedure_templates.procedure_template {
                    foreign_keys.procedure_template = Some(procedure_templates.clone());
                    updated = true;
                }
                if self.parent_procedure_template.is_some_and(|parent_procedure_template| {
                    parent_procedure_template == procedure_templates.procedure_template
                }) {
                    foreign_keys.parent_procedure_template = Some(procedure_templates.clone());
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ProcedureTemplate(procedure_templates),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.procedure_template == procedure_templates.procedure_template {
                    foreign_keys.procedure_template = None;
                    updated = true;
                }
                if self.parent_procedure_template.is_some_and(|parent_procedure_template| {
                    parent_procedure_template == procedure_templates.procedure_template
                }) {
                    foreign_keys.parent_procedure_template = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Procedure(procedures),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self
                    .parent_procedure
                    .is_some_and(|parent_procedure| parent_procedure == procedures.procedure)
                {
                    foreign_keys.parent_procedure = Some(procedures);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Procedure(procedures),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self
                    .parent_procedure
                    .is_some_and(|parent_procedure| parent_procedure == procedures.procedure)
                {
                    foreign_keys.parent_procedure = None;
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
                    foreign_keys.created_by = Some(users.clone());
                    updated = true;
                }
                if self.updated_by == users.id {
                    foreign_keys.updated_by = Some(users.clone());
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
                if self.updated_by == users.id {
                    foreign_keys.updated_by = None;
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
impl web_common_traits::prelude::ForeignKeys for ProcedureForeignKeys {}
