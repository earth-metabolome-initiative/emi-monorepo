#[derive(Debug, Clone, PartialEq, Default, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NameplateCategoryForeignKeys {
    pub permanence_category:
        Option<crate::codegen::structs_codegen::tables::permanence_categories::PermanenceCategory>,
    pub color: Option<crate::codegen::structs_codegen::tables::colors::Color>,
}
impl web_common_traits::prelude::HasForeignKeys
    for crate::codegen::structs_codegen::tables::nameplate_categories::NameplateCategory
{
    type ForeignKeys = NameplateCategoryForeignKeys;
    type Row = crate::codegen::tables::row::Row;
    fn load_foreign_keys<C>(&self, connector: &C)
    where
        C: web_common_traits::crud::Connector<Row = Self::Row>,
    {
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::PermanenceCategory(
                self.permanence_category_id,
            ),
        ));
        connector.send(web_common_traits::crud::CrudPrimaryKeyOperation::Read(
            crate::codegen::tables::table_primary_keys::TablePrimaryKey::Color(self.color_id),
        ));
    }
    fn foreign_keys_loaded(&self, foreign_keys: &Self::ForeignKeys) -> bool {
        foreign_keys.permanence_category.is_some() && foreign_keys.color.is_some()
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
                crate::codegen::tables::row::Row::PermanenceCategory(permanence_categories),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if permanence_categories.id == self.permanence_category_id {
                    foreign_keys.permanence_category = Some(permanence_categories);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::PermanenceCategory(permanence_categories),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if permanence_categories.id == self.permanence_category_id {
                    foreign_keys.permanence_category = None;
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Color(colors),
                web_common_traits::crud::CRUD::Read
                | web_common_traits::crud::CRUD::Create
                | web_common_traits::crud::CRUD::Update,
            ) => {
                if colors.id == self.color_id {
                    foreign_keys.color = Some(colors);
                    updated = true;
                }
            }
            (
                crate::codegen::tables::row::Row::Color(colors),
                web_common_traits::crud::CRUD::Delete,
            ) => {
                if colors.id == self.color_id {
                    foreign_keys.color = None;
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
impl web_common_traits::prelude::ForeignKeys for NameplateCategoryForeignKeys {}
