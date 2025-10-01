#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PlacingProcedureForeignKeys {
    pub procedure: Option<
        crate::codegen::structs_codegen::tables::geolocation_procedures::GeolocationProcedure,
    >,
    pub procedure_template: Option<
        crate::codegen::structs_codegen::tables::placing_procedure_templates::PlacingProcedureTemplate,
    >,
}
impl web_common_traits::prelude::HasForeignKeys
    for crate::codegen::structs_codegen::tables::placing_procedures::PlacingProcedure
{
    type ForeignKeys = PlacingProcedureForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::GeolocationProcedure(
                self.procedure,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::PlacingProcedureTemplate(
                self.procedure_template,
            ),
        ));
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.procedure.is_some() && foreign_keys.procedure_template.is_some()
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
                crate::codegen::tables::row::Row::GeolocationProcedure(geolocation_procedures),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.procedure == geolocation_procedures.procedure {
                    foreign_keys.procedure = Some(geolocation_procedures);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::GeolocationProcedure(geolocation_procedures),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.procedure == geolocation_procedures.procedure {
                    foreign_keys.procedure = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::PlacingProcedureTemplate(
                    placing_procedure_templates,
                ),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if self.procedure_template == placing_procedure_templates.procedure_template {
                    foreign_keys.procedure_template = Some(placing_procedure_templates);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::PlacingProcedureTemplate(
                    placing_procedure_templates,
                ),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if self.procedure_template == placing_procedure_templates.procedure_template {
                    foreign_keys.procedure_template = None;
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
impl web_common_traits::prelude::ForeignKeys for PlacingProcedureForeignKeys {}
