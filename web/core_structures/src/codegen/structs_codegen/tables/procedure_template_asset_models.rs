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
#[diesel(
    belongs_to(
        crate::codegen::structs_codegen::tables::asset_models::AssetModel,
        foreign_key = asset_model
    )
)]
#[diesel(
    belongs_to(
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate,
        foreign_key = procedure_template
    )
)]
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
impl web_common_traits::prelude::ExtensionTable<
    crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
> for ProcedureTemplateAssetModel
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{}
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
impl web_common_traits::database::PrimaryKeyLike for ProcedureTemplateAssetModel {
    type PrimaryKey = i32;
    fn primary_key(&self) -> Self::PrimaryKey {
        self.id
    }
}
impl ProcedureTemplateAssetModel {
    pub fn asset_model<C: diesel::connection::LoadConnection>(
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
            self.asset_model_id,
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn procedure_template_asset_models_based_on_asset_model_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        Option<
            crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
        >,
        diesel::result::Error,
    >{
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl,
            associations::HasTable,
        };
        let Some(based_on) = self.based_on else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::dsl::id
                    .eq(based_on)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::dsl::asset_model
                            .eq(&self.asset_model_id),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
            >(conn)
            .optional()
    }
    pub fn based_on<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        Option<
            crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
        >,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel: web_common_traits::database::Read<
            C,
        >,
    {
        use diesel::OptionalExtension;
        use web_common_traits::database::Read;
        let Some(based_on) = self.based_on else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel::read(
                based_on,
                conn,
            )
            .optional()
    }
    pub fn procedure_template<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate::read(
            self.procedure_template,
            conn,
        )
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
                    procedure_template_asset_models::procedure_template.eq(procedure_template_id),
                ),
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
                    .and(procedure_template_asset_models::asset_model.eq(asset_model_id)),
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
}
impl AsRef<ProcedureTemplateAssetModel> for ProcedureTemplateAssetModel {
    fn as_ref(&self) -> &ProcedureTemplateAssetModel {
        self
    }
}
