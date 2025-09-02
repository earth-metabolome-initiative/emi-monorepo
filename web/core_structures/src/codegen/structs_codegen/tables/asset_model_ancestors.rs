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
impl AssetModelAncestor {
    pub fn descendant_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::asset_models::AssetModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::asset_models::AssetModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::asset_models::AssetModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::asset_models::AssetModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::asset_models::AssetModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::asset_models::AssetModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::asset_models::AssetModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::asset_models::AssetModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::asset_models::AssetModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::asset_models::AssetModel::table(),
                self.descendant_model,
            ),
            conn,
        )
    }
    pub fn ancestor_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::asset_models::AssetModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::asset_models::AssetModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::asset_models::AssetModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::asset_models::AssetModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::asset_models::AssetModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::asset_models::AssetModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::asset_models::AssetModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::asset_models::AssetModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::asset_models::AssetModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::asset_models::AssetModel::table(),
                self.ancestor_model,
            ),
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn from_descendant_model(
        descendant_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
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
    #[cfg(feature = "postgres")]
    pub fn from_ancestor_model(
        ancestor_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
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
}
impl AsRef<AssetModelAncestor> for AssetModelAncestor {
    fn as_ref(&self) -> &AssetModelAncestor {
        self
    }
}
