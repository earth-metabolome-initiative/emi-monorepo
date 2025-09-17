#[derive(Debug, Clone, PartialEq, Copy, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(diesel::Selectable, diesel::Insertable, diesel::Queryable, diesel::Identifiable)]
#[cfg_attr(feature = "yew", derive(yew::prelude::Properties))]
#[diesel(primary_key(descendant_model, ancestor_model))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::asset_model_ancestors::asset_model_ancestors
)]
pub struct AssetModelAncestor {
    pub descendant_model: i32,
    pub ancestor_model: i32,
}
impl web_common_traits::prelude::TableName for AssetModelAncestor {
    const TABLE_NAME: &'static str = "asset_model_ancestors";
}
impl<'a> From<&'a AssetModelAncestor>
    for web_common_traits::database::IdOrBuilder<
        (i32, i32),
        crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelAncestorBuilder,
    >
{
    fn from(value: &'a AssetModelAncestor) -> Self {
        web_common_traits::database::IdOrBuilder::Id((value.descendant_model, value.ancestor_model))
    }
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::asset_model_ancestors::AssetModelAncestor,
    > for AssetModelAncestor
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a (i32, i32)>,
{
}
impl diesel::Identifiable for AssetModelAncestor {
    type Id = (i32, i32);
    fn id(self) -> Self::Id {
        (self.descendant_model, self.ancestor_model)
    }
}
impl web_common_traits::database::PrimaryKeyLike for AssetModelAncestor {
    type PrimaryKey = (i32, i32);
    fn primary_key(&self) -> Self::PrimaryKey {
        (self.descendant_model, self.ancestor_model)
    }
}
impl AssetModelAncestor {
    pub fn ancestor_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::asset_models::AssetModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::asset_models::AssetModel:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::asset_models::AssetModel::read(
            self.ancestor_model,
            conn,
        )
    }
    pub fn descendant_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::asset_models::AssetModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::asset_models::AssetModel:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::asset_models::AssetModel::read(
            self.descendant_model,
            conn,
        )
    }
    pub fn from_ancestor_model<C>(
        ancestor_model: i32,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::asset_model_ancestors::asset_model_ancestors::ancestor_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::asset_model_ancestors::asset_model_ancestors::ancestor_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            (
                diesel::helper_types::Asc<
                    crate::codegen::diesel_codegen::tables::asset_model_ancestors::asset_model_ancestors::descendant_model,
                >,
                diesel::helper_types::Asc<
                    crate::codegen::diesel_codegen::tables::asset_model_ancestors::asset_model_ancestors::ancestor_model,
                >,
            ),
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::asset_model_ancestors::asset_model_ancestors::ancestor_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            (
                diesel::helper_types::Asc<
                    crate::codegen::diesel_codegen::tables::asset_model_ancestors::asset_model_ancestors::descendant_model,
                >,
                diesel::helper_types::Asc<
                    crate::codegen::diesel_codegen::tables::asset_model_ancestors::asset_model_ancestors::ancestor_model,
                >,
            ),
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::asset_model_ancestors::asset_model_ancestors;
        Self::table()
            .filter(asset_model_ancestors::ancestor_model.eq(ancestor_model))
            .order_by((
                asset_model_ancestors::descendant_model.asc(),
                asset_model_ancestors::ancestor_model.asc(),
            ))
            .load::<Self>(conn)
    }
    pub fn from_descendant_model<C>(
        descendant_model: i32,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::asset_model_ancestors::asset_model_ancestors::descendant_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::asset_model_ancestors::asset_model_ancestors::descendant_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            (
                diesel::helper_types::Asc<
                    crate::codegen::diesel_codegen::tables::asset_model_ancestors::asset_model_ancestors::descendant_model,
                >,
                diesel::helper_types::Asc<
                    crate::codegen::diesel_codegen::tables::asset_model_ancestors::asset_model_ancestors::ancestor_model,
                >,
            ),
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::asset_model_ancestors::asset_model_ancestors::descendant_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            (
                diesel::helper_types::Asc<
                    crate::codegen::diesel_codegen::tables::asset_model_ancestors::asset_model_ancestors::descendant_model,
                >,
                diesel::helper_types::Asc<
                    crate::codegen::diesel_codegen::tables::asset_model_ancestors::asset_model_ancestors::ancestor_model,
                >,
            ),
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::asset_model_ancestors::asset_model_ancestors;
        Self::table()
            .filter(asset_model_ancestors::descendant_model.eq(descendant_model))
            .order_by((
                asset_model_ancestors::descendant_model.asc(),
                asset_model_ancestors::ancestor_model.asc(),
            ))
            .load::<Self>(conn)
    }
}
impl AsRef<AssetModelAncestor> for AssetModelAncestor {
    fn as_ref(&self) -> &AssetModelAncestor {
        self
    }
}
