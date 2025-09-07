#[derive(Debug, Clone, PartialEq, Default, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NextProcedureTemplateForeignKeys {
    pub created_by: Option<crate::User>,
    pub parent: Option<crate::ProcedureTemplate>,
    pub next_procedure_templates_parent_predecessor_fkey: Option<crate::ParentProcedureTemplate>,
    pub next_procedure_templates_parent_successor_fkey: Option<crate::ParentProcedureTemplate>,
    pub predecessor: Option<crate::ProcedureTemplate>,
    pub successor: Option<crate::ProcedureTemplate>,
}
impl web_common_traits::prelude::HasForeignKeys for crate::NextProcedureTemplate {
    type ForeignKeys = NextProcedureTemplateForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::User(self.created_by),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplate(
                self.parent,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ParentProcedureTemplate((
                self.parent,
                self.predecessor,
            )),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ParentProcedureTemplate((
                self.parent,
                self.successor,
            )),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplate(
                self.predecessor,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureTemplate(
                self.successor,
            ),
        ));
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.created_by.is_some()
            && foreign_keys.parent.is_some()
            && foreign_keys.next_procedure_templates_parent_predecessor_fkey.is_some()
            && foreign_keys.next_procedure_templates_parent_successor_fkey.is_some()
            && foreign_keys.predecessor.is_some()
            && foreign_keys.successor.is_some()
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
                if self.parent == parent_procedure_templates.parent
                    && self.predecessor == parent_procedure_templates.child
                {
                    foreign_keys.next_procedure_templates_parent_predecessor_fkey =
                        Some(parent_procedure_templates);
                    updated = true;
                }
                if self.parent == parent_procedure_templates.parent
                    && self.successor == parent_procedure_templates.child
                {
                    foreign_keys.next_procedure_templates_parent_successor_fkey =
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
                if self.parent == parent_procedure_templates.parent
                    && self.predecessor == parent_procedure_templates.child
                {
                    foreign_keys.next_procedure_templates_parent_predecessor_fkey = None;
                    updated = true;
                }
                if self.parent == parent_procedure_templates.parent
                    && self.successor == parent_procedure_templates.child
                {
                    foreign_keys.next_procedure_templates_parent_successor_fkey = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ProcedureTemplate(procedure_templates),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.parent == procedure_templates.procedure_template {
                    foreign_keys.parent = Some(procedure_templates.clone());
                    updated = true;
                }
                if self.predecessor == procedure_templates.procedure_template {
                    foreign_keys.predecessor = Some(procedure_templates.clone());
                    updated = true;
                }
                if self.successor == procedure_templates.procedure_template {
                    foreign_keys.successor = Some(procedure_templates.clone());
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ProcedureTemplate(procedure_templates),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.parent == procedure_templates.procedure_template {
                    foreign_keys.parent = None;
                    updated = true;
                }
                if self.predecessor == procedure_templates.procedure_template {
                    foreign_keys.predecessor = None;
                    updated = true;
                }
                if self.successor == procedure_templates.procedure_template {
                    foreign_keys.successor = None;
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
impl web_common_traits::prelude::ForeignKeys for NextProcedureTemplateForeignKeys {}
