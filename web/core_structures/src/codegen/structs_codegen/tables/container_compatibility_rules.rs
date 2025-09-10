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
#[diesel(
    belongs_to(
        crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel,
        foreign_key = contained_asset_model
    )
)]
#[diesel(
    belongs_to(
        crate::codegen::structs_codegen::tables::container_models::ContainerModel,
        foreign_key = container_model
    )
)]
#[diesel(
    belongs_to(
        crate::codegen::structs_codegen::tables::users::User,
        foreign_key = created_by
    )
)]
#[diesel(primary_key(container_model, contained_asset_model))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::container_compatibility_rules::container_compatibility_rules
)]
pub struct ContainerCompatibilityRule {
    pub container_model: i32,
    pub contained_asset_model: i32,
    pub quantity: Option<i16>,
    pub created_by: i32,
    pub created_at: ::rosetta_timestamp::TimestampUTC,
}
impl web_common_traits::prelude::TableName for ContainerCompatibilityRule {
    const TABLE_NAME: &'static str = "container_compatibility_rules";
}
impl<'a> From<&'a ContainerCompatibilityRule>
for web_common_traits::database::IdOrBuilder<
    (i32, i32),
    crate::codegen::structs_codegen::tables::insertables::InsertableContainerCompatibilityRuleBuilder,
> {
    fn from(value: &'a ContainerCompatibilityRule) -> Self {
        web_common_traits::database::IdOrBuilder::Id((
            value.container_model,
            value.contained_asset_model,
        ))
    }
}
impl web_common_traits::prelude::ExtensionTable<
    crate::codegen::structs_codegen::tables::container_compatibility_rules::ContainerCompatibilityRule,
> for ContainerCompatibilityRule
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a (i32, i32)>,
{}
impl diesel::Identifiable for ContainerCompatibilityRule {
    type Id = (i32, i32);
    fn id(self) -> Self::Id {
        (self.container_model, self.contained_asset_model)
    }
}
impl web_common_traits::database::PrimaryKeyLike for ContainerCompatibilityRule {
    type PrimaryKey = (i32, i32);
    fn primary_key(&self) -> Self::PrimaryKey {
        (self.container_model, self.contained_asset_model)
    }
}
impl ContainerCompatibilityRule {
    pub fn contained_asset_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel::read(
            self.contained_asset_model,
            conn,
        )
    }
    pub fn container_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::container_models::ContainerModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::container_models::ContainerModel:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::container_models::ContainerModel::read(
            self.container_model,
            conn,
        )
    }
    pub fn created_by<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::codegen::structs_codegen::tables::users::User, diesel::result::Error>
    where
        crate::codegen::structs_codegen::tables::users::User: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::users::User::read(self.created_by, conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_quantity(
        quantity: i16,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::container_compatibility_rules::container_compatibility_rules;
        Self::table()
            .filter(container_compatibility_rules::quantity.eq(quantity))
            .order_by((
                container_compatibility_rules::container_model.asc(),
                container_compatibility_rules::contained_asset_model.asc(),
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
            <crate::codegen::diesel_codegen::tables::container_compatibility_rules::container_compatibility_rules::created_at as diesel::expression_methods::EqAll<
                ::rosetta_timestamp::TimestampUTC,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::container_compatibility_rules::container_compatibility_rules::created_at as diesel::expression_methods::EqAll<
                ::rosetta_timestamp::TimestampUTC,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            (
                diesel::helper_types::Asc<
                    crate::codegen::diesel_codegen::tables::container_compatibility_rules::container_compatibility_rules::container_model,
                >,
                diesel::helper_types::Asc<
                    crate::codegen::diesel_codegen::tables::container_compatibility_rules::container_compatibility_rules::contained_asset_model,
                >,
            ),
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::container_compatibility_rules::container_compatibility_rules::created_at as diesel::expression_methods::EqAll<
                ::rosetta_timestamp::TimestampUTC,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            (
                diesel::helper_types::Asc<
                    crate::codegen::diesel_codegen::tables::container_compatibility_rules::container_compatibility_rules::container_model,
                >,
                diesel::helper_types::Asc<
                    crate::codegen::diesel_codegen::tables::container_compatibility_rules::container_compatibility_rules::contained_asset_model,
                >,
            ),
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::container_compatibility_rules::container_compatibility_rules;
        Self::table()
            .filter(container_compatibility_rules::created_at.eq(created_at))
            .order_by((
                container_compatibility_rules::container_model.asc(),
                container_compatibility_rules::contained_asset_model.asc(),
            ))
            .load::<Self>(conn)
    }
}
impl AsRef<ContainerCompatibilityRule> for ContainerCompatibilityRule {
    fn as_ref(&self) -> &ContainerCompatibilityRule {
        self
    }
}
