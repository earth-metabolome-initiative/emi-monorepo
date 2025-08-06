#[derive(Debug, Clone, PartialEq, Default, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PackagingProcedureModelForeignKeys {
    pub procedure_model:
        Option<crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel>,
    pub packaged_with:
        Option<crate::codegen::structs_codegen::tables::container_models::ContainerModel>,
}
impl web_common_traits::prelude::HasForeignKeys
    for crate::codegen::structs_codegen::tables::packaging_procedure_models::PackagingProcedureModel
{
    type ForeignKeys = PackagingProcedureModelForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ProcedureModel(
                self.procedure_model_id,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::ContainerModel(
                self.packaged_with,
            ),
        ));
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.procedure_model.is_some() && foreign_keys.packaged_with.is_some()
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
                crate::codegen::tables::row::Row::ContainerModel(container_models),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.packaged_with == container_models.id {
                    foreign_keys.packaged_with = Some(container_models);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ContainerModel(container_models),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.packaged_with == container_models.id {
                    foreign_keys.packaged_with = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ProcedureModel(procedure_models),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.procedure_model_id == procedure_models.id {
                    foreign_keys.procedure_model = Some(procedure_models);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::ProcedureModel(procedure_models),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.procedure_model_id == procedure_models.id {
                    foreign_keys.procedure_model = None;
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
impl web_common_traits::prelude::ForeignKeys for PackagingProcedureModelForeignKeys {}
