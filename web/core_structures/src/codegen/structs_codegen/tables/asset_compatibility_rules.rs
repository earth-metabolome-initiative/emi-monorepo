#[derive(Debug, Clone, PartialEq, Copy, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    diesel::Selectable,
    diesel::Insertable,
    diesel::AsChangeset,
    diesel::Queryable,
    diesel::Identifiable,
    diesel::Associations,
)]
#[cfg_attr(feature = "yew", derive(yew::prelude::Properties))]
#[diesel(belongs_to(crate::User, foreign_key = created_by))]
#[diesel(primary_key(left_asset_model, right_asset_model))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::asset_compatibility_rules::asset_compatibility_rules
)]
pub struct AssetCompatibilityRule {
    pub left_asset_model: i32,
    pub right_asset_model: i32,
    pub created_by: i32,
    pub created_at: ::rosetta_timestamp::TimestampUTC,
}
impl web_common_traits::prelude::TableName for AssetCompatibilityRule {
    const TABLE_NAME: &'static str = "asset_compatibility_rules";
}
impl<'a> From<&'a AssetCompatibilityRule>
for web_common_traits::database::IdOrBuilder<
    (i32, i32),
    crate::codegen::structs_codegen::tables::insertables::InsertableAssetCompatibilityRuleBuilder,
> {
    fn from(value: &'a AssetCompatibilityRule) -> Self {
        web_common_traits::database::IdOrBuilder::Id((
            value.left_asset_model,
            value.right_asset_model,
        ))
    }
}
impl web_common_traits::prelude::ExtensionTable<crate::AssetCompatibilityRule>
    for AssetCompatibilityRule
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a (i32, i32)>,
{
}
impl diesel::Identifiable for AssetCompatibilityRule {
    type Id = (i32, i32);
    fn id(self) -> Self::Id {
        (self.left_asset_model, self.right_asset_model)
    }
}
impl AssetCompatibilityRule {
    pub fn left_asset_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::AssetModel, diesel::result::Error>
    where
        crate::AssetModel: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::AssetModel::read(self.left_asset_model, conn)
    }
    pub fn right_asset_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::AssetModel, diesel::result::Error>
    where
        crate::AssetModel: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::AssetModel::read(self.right_asset_model, conn)
    }
    pub fn created_by<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::User, diesel::result::Error>
    where
        crate::User: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::User::read(self.created_by, conn)
    }
    pub fn from_left_asset_model<C>(
        left_asset_model: i32,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::asset_compatibility_rules::asset_compatibility_rules::left_asset_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::asset_compatibility_rules::asset_compatibility_rules::left_asset_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            (
                diesel::helper_types::Asc<
                    crate::codegen::diesel_codegen::tables::asset_compatibility_rules::asset_compatibility_rules::left_asset_model,
                >,
                diesel::helper_types::Asc<
                    crate::codegen::diesel_codegen::tables::asset_compatibility_rules::asset_compatibility_rules::right_asset_model,
                >,
            ),
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::asset_compatibility_rules::asset_compatibility_rules::left_asset_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            (
                diesel::helper_types::Asc<
                    crate::codegen::diesel_codegen::tables::asset_compatibility_rules::asset_compatibility_rules::left_asset_model,
                >,
                diesel::helper_types::Asc<
                    crate::codegen::diesel_codegen::tables::asset_compatibility_rules::asset_compatibility_rules::right_asset_model,
                >,
            ),
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::asset_compatibility_rules::asset_compatibility_rules;
        Self::table()
            .filter(asset_compatibility_rules::left_asset_model.eq(left_asset_model))
            .order_by((
                asset_compatibility_rules::left_asset_model.asc(),
                asset_compatibility_rules::right_asset_model.asc(),
            ))
            .load::<Self>(conn)
    }
    pub fn from_right_asset_model<C>(
        right_asset_model: i32,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::asset_compatibility_rules::asset_compatibility_rules::right_asset_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::asset_compatibility_rules::asset_compatibility_rules::right_asset_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            (
                diesel::helper_types::Asc<
                    crate::codegen::diesel_codegen::tables::asset_compatibility_rules::asset_compatibility_rules::left_asset_model,
                >,
                diesel::helper_types::Asc<
                    crate::codegen::diesel_codegen::tables::asset_compatibility_rules::asset_compatibility_rules::right_asset_model,
                >,
            ),
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::asset_compatibility_rules::asset_compatibility_rules::right_asset_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            (
                diesel::helper_types::Asc<
                    crate::codegen::diesel_codegen::tables::asset_compatibility_rules::asset_compatibility_rules::left_asset_model,
                >,
                diesel::helper_types::Asc<
                    crate::codegen::diesel_codegen::tables::asset_compatibility_rules::asset_compatibility_rules::right_asset_model,
                >,
            ),
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::asset_compatibility_rules::asset_compatibility_rules;
        Self::table()
            .filter(asset_compatibility_rules::right_asset_model.eq(right_asset_model))
            .order_by((
                asset_compatibility_rules::left_asset_model.asc(),
                asset_compatibility_rules::right_asset_model.asc(),
            ))
            .load::<Self>(conn)
    }
    pub fn from_created_at<C>(
        created_at: ::rosetta_timestamp::TimestampUTC,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::asset_compatibility_rules::asset_compatibility_rules::created_at as diesel::expression_methods::EqAll<
                ::rosetta_timestamp::TimestampUTC,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::asset_compatibility_rules::asset_compatibility_rules::created_at as diesel::expression_methods::EqAll<
                ::rosetta_timestamp::TimestampUTC,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            (
                diesel::helper_types::Asc<
                    crate::codegen::diesel_codegen::tables::asset_compatibility_rules::asset_compatibility_rules::left_asset_model,
                >,
                diesel::helper_types::Asc<
                    crate::codegen::diesel_codegen::tables::asset_compatibility_rules::asset_compatibility_rules::right_asset_model,
                >,
            ),
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::asset_compatibility_rules::asset_compatibility_rules::created_at as diesel::expression_methods::EqAll<
                ::rosetta_timestamp::TimestampUTC,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            (
                diesel::helper_types::Asc<
                    crate::codegen::diesel_codegen::tables::asset_compatibility_rules::asset_compatibility_rules::left_asset_model,
                >,
                diesel::helper_types::Asc<
                    crate::codegen::diesel_codegen::tables::asset_compatibility_rules::asset_compatibility_rules::right_asset_model,
                >,
            ),
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::asset_compatibility_rules::asset_compatibility_rules;
        Self::table()
            .filter(asset_compatibility_rules::created_at.eq(created_at))
            .order_by((
                asset_compatibility_rules::left_asset_model.asc(),
                asset_compatibility_rules::right_asset_model.asc(),
            ))
            .load::<Self>(conn)
    }
}
impl AsRef<AssetCompatibilityRule> for AssetCompatibilityRule {
    fn as_ref(&self) -> &AssetCompatibilityRule {
        self
    }
}
