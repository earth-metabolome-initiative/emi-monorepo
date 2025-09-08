#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
#[diesel(belongs_to(crate::AssetModel, foreign_key = asset_model))]
#[diesel(belongs_to(crate::User, foreign_key = created_by))]
#[diesel(belongs_to(crate::ProcedureTemplate, foreign_key = procedure_template))]
#[diesel(primary_key(id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models
)]
pub struct ProcedureTemplateAssetModel {
    pub id: i32,
    pub name: String,
    pub procedure_template: i32,
    pub based_on: Option<i32>,
    pub asset_model: i32,
    pub created_by: i32,
    pub created_at: ::rosetta_timestamp::TimestampUTC,
}
impl web_common_traits::prelude::TableName for ProcedureTemplateAssetModel {
    const TABLE_NAME: &'static str = "procedure_template_asset_models";
}
impl<'a> From<&'a ProcedureTemplateAssetModel>
for web_common_traits::database::IdOrBuilder<
    i32,
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
> {
    fn from(value: &'a ProcedureTemplateAssetModel) -> Self {
        web_common_traits::database::IdOrBuilder::Id(value.id)
    }
}
impl web_common_traits::prelude::ExtensionTable<crate::ProcedureTemplateAssetModel>
    for ProcedureTemplateAssetModel
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{
}
impl<C> web_common_traits::prelude::Ancestor<C> for ProcedureTemplateAssetModel
where
    Self: web_common_traits::prelude::TableName + Sized,
    C: diesel::connection::LoadConnection,
    <C as diesel::Connection>::Backend: diesel::backend::DieselReserveSpecialization
        + diesel::sql_types::HasSqlType<diesel::sql_types::Integer>
        + 'static,
    web_common_traits::prelude::AncestorExists: diesel::deserialize::FromSqlRow<
            diesel::sql_types::Untyped,
            <C as diesel::Connection>::Backend,
        >,
    for<'a> &'a Self: diesel::Identifiable,
    for<'a> <&'a Self as diesel::Identifiable>::Id:
        diesel::serialize::ToSql<diesel::sql_types::Integer, C::Backend>,
{
    const PARENT_ID: &'static str = "based_on";
    const ID: &'static str = "id";
    type SqlType = diesel::sql_types::Integer;
}
impl web_common_traits::prelude::Descendant<ProcedureTemplateAssetModel>
    for ProcedureTemplateAssetModel
{
    fn parent(&self) -> Option<<&Self as diesel::Identifiable>::Id> {
        self.based_on.as_ref()
    }
}
impl diesel::Identifiable for ProcedureTemplateAssetModel {
    type Id = i32;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl ProcedureTemplateAssetModel {
    pub fn asset_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::AssetModel, diesel::result::Error>
    where
        crate::AssetModel: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::AssetModel::read(self.asset_model, conn)
    }
    #[cfg(feature = "postgres")]
    pub fn procedure_template_asset_models_based_on_asset_model_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<crate::ProcedureTemplateAssetModel>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        let Some(based_on) = self.based_on else {
            return Ok(None);
        };
        crate::ProcedureTemplateAssetModel::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::dsl::id
                    .eq(based_on)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::dsl::asset_model
                            .eq(&self.asset_model),
                    ),
            )
            .first::<crate::ProcedureTemplateAssetModel>(conn)
            .map(Some)
    }
    pub fn based_on<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<Option<crate::ProcedureTemplateAssetModel>, diesel::result::Error>
    where
        crate::ProcedureTemplateAssetModel: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        let Some(based_on) = self.based_on else {
            return Ok(None);
        };
        crate::ProcedureTemplateAssetModel::read(based_on, conn).map(Some)
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
    pub fn procedure_template<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::ProcedureTemplate, diesel::result::Error>
    where
        crate::ProcedureTemplate: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::ProcedureTemplate::read(self.procedure_template, conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_name_and_procedure_template(
        name: &str,
        procedure_template: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models;
        Self::table()
            .filter(
                procedure_template_asset_models::name.eq(name).and(
                    procedure_template_asset_models::procedure_template.eq(procedure_template),
                ),
            )
            .order_by(procedure_template_asset_models::id.asc())
            .first::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_and_id(
        procedure_template: i32,
        id: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models;
        Self::table()
            .filter(
                procedure_template_asset_models::procedure_template
                    .eq(procedure_template)
                    .and(procedure_template_asset_models::id.eq(id)),
            )
            .order_by(procedure_template_asset_models::id.asc())
            .first::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_asset_model_and_id(
        asset_model: i32,
        id: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models;
        Self::table()
            .filter(
                procedure_template_asset_models::asset_model
                    .eq(asset_model)
                    .and(procedure_template_asset_models::id.eq(id)),
            )
            .order_by(procedure_template_asset_models::id.asc())
            .first::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_based_on_and_asset_model(
        based_on: i32,
        asset_model: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models;
        Self::table()
            .filter(
                procedure_template_asset_models::based_on
                    .eq(based_on)
                    .and(procedure_template_asset_models::asset_model.eq(asset_model)),
            )
            .order_by(procedure_template_asset_models::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_based_on(
        based_on: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models;
        Self::table()
            .filter(procedure_template_asset_models::based_on.eq(based_on))
            .order_by(procedure_template_asset_models::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_name(
        name: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models;
        Self::table()
            .filter(procedure_template_asset_models::name.eq(name))
            .order_by(procedure_template_asset_models::id.asc())
            .load::<Self>(conn)
    }
    pub fn from_created_at<C>(
        created_at: ::rosetta_timestamp::TimestampUTC,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::created_at as diesel::expression_methods::EqAll<
                ::rosetta_timestamp::TimestampUTC,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::created_at as diesel::expression_methods::EqAll<
                ::rosetta_timestamp::TimestampUTC,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::id,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::created_at as diesel::expression_methods::EqAll<
                ::rosetta_timestamp::TimestampUTC,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::id,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models;
        Self::table()
            .filter(procedure_template_asset_models::created_at.eq(created_at))
            .order_by(procedure_template_asset_models::id.asc())
            .load::<Self>(conn)
    }
}
impl AsRef<ProcedureTemplateAssetModel> for ProcedureTemplateAssetModel {
    fn as_ref(&self) -> &ProcedureTemplateAssetModel {
        self
    }
}
