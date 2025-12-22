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
        crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel,
        foreign_key = weighed_container_model
    )
)]
#[diesel(
    belongs_to(
        crate::codegen::structs_codegen::tables::weighing_device_models::WeighingDeviceModel,
        foreign_key = weighed_with_model
    )
)]
#[diesel(primary_key(procedure_template_id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::weighing_procedure_templates::weighing_procedure_templates
)]
pub struct WeighingProcedureTemplate {
    pub procedure_template: i32,
    pub weighed_container_model: i32,
    pub procedure_template_weighed_container_model: i32,
    pub weighed_with_model: i32,
    pub procedure_template_weighed_with_model: i32,
}
impl web_common_traits::prelude::TableName for WeighingProcedureTemplate {
    const TABLE_NAME: &'static str = "weighing_procedure_templates";
}
impl<'a> From<&'a WeighingProcedureTemplate>
for web_common_traits::database::IdOrBuilder<
    i32,
    crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureTemplateBuilder,
> {
    fn from(value: &'a WeighingProcedureTemplate) -> Self {
        web_common_traits::database::IdOrBuilder::Id(value.procedure_template_id)
    }
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate,
    > for WeighingProcedureTemplate
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{
}
impl web_common_traits::prelude::ExtensionTable<
    crate::codegen::structs_codegen::tables::weighing_procedure_templates::WeighingProcedureTemplate,
> for WeighingProcedureTemplate
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{}
impl diesel::Identifiable for WeighingProcedureTemplate {
    type Id = i32;
    fn id(self) -> Self::Id {
        self.procedure_template
    }
}
impl web_common_traits::database::PrimaryKeyLike for WeighingProcedureTemplate {
    type PrimaryKey = i32;
    fn primary_key(&self) -> Self::PrimaryKey {
        self.procedure_template
    }
}
impl WeighingProcedureTemplate {
    #[cfg(feature = "postgres")]
    pub fn weighing_procedure_templates_procedure_template_weighed_c_fkey1(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
        diesel::result::Error,
    >{
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::dsl::id
                    .eq(&self.procedure_template_weighed_container_model)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::dsl::asset_model
                            .eq(&self.weighed_container_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
            >(conn)
    }
    pub fn procedure_template_weighed_container_model<
        C: diesel::connection::LoadConnection,
    >(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel: web_common_traits::database::Read<
            C,
        >,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel::read(
            self.procedure_template_weighed_container_model,
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn weighing_procedure_templates_procedure_template_weighed_w_fkey1(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
        diesel::result::Error,
    >{
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::dsl::id
                    .eq(&self.procedure_template_weighed_with_model)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::dsl::asset_model
                            .eq(&self.weighed_with_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
            >(conn)
    }
    pub fn procedure_template_weighed_with_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel: web_common_traits::database::Read<
            C,
        >,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel::read(
            self.procedure_template_weighed_with_model,
            conn,
        )
    }
    pub fn weighed_container_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel: web_common_traits::database::Read<
            C,
        >,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel::read(
            self.weighed_container_model,
            conn,
        )
    }
    pub fn weighed_with_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::weighing_device_models::WeighingDeviceModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::weighing_device_models::WeighingDeviceModel:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::weighing_device_models::WeighingDeviceModel::read(
            self.weighed_with_model,
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_weighed_container_model_and_weighed_container_model(
        procedure_template_weighed_container_model: i32,
        weighed_container_model: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::weighing_procedure_templates::weighing_procedure_templates;
        Self::table()
            .filter(
                weighing_procedure_templates::procedure_template_weighed_container_model
                    .eq(procedure_template_weighed_container_model)
                    .and(
                        weighing_procedure_templates::weighed_container_model
                            .eq(weighed_container_model),
                    ),
            )
            .order_by(weighing_procedure_templates::procedure_template.asc())
            .load::<Self>(conn)
    }
    pub fn from_procedure_template_weighed_container_model<C>(
        procedure_template_weighed_container_model: i32,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::weighing_procedure_templates::weighing_procedure_templates::procedure_template_weighed_container_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::weighing_procedure_templates::weighing_procedure_templates::procedure_template_weighed_container_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::weighing_procedure_templates::weighing_procedure_templates::procedure_template,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::weighing_procedure_templates::weighing_procedure_templates::procedure_template_weighed_container_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::weighing_procedure_templates::weighing_procedure_templates::procedure_template,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::weighing_procedure_templates::weighing_procedure_templates;
        Self::table()
            .filter(
                weighing_procedure_templates::procedure_template_weighed_container_model
                    .eq(procedure_template_weighed_container_model),
            )
            .order_by(weighing_procedure_templates::procedure_template.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_weighed_with_model_and_weighed_with_model(
        procedure_template_weighed_with_model: i32,
        weighed_with_model: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::weighing_procedure_templates::weighing_procedure_templates;
        Self::table()
            .filter(
                weighing_procedure_templates::procedure_template_weighed_with_model
                    .eq(procedure_template_weighed_with_model)
                    .and(weighing_procedure_templates::weighed_with_model.eq(weighed_with_model)),
            )
            .order_by(weighing_procedure_templates::procedure_template.asc())
            .load::<Self>(conn)
    }
    pub fn from_procedure_template_weighed_with_model<C>(
        procedure_template_weighed_with_model: i32,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::weighing_procedure_templates::weighing_procedure_templates::procedure_template_weighed_with_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::weighing_procedure_templates::weighing_procedure_templates::procedure_template_weighed_with_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::weighing_procedure_templates::weighing_procedure_templates::procedure_template,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::weighing_procedure_templates::weighing_procedure_templates::procedure_template_weighed_with_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::weighing_procedure_templates::weighing_procedure_templates::procedure_template,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::weighing_procedure_templates::weighing_procedure_templates;
        Self::table()
            .filter(
                weighing_procedure_templates::procedure_template_weighed_with_model
                    .eq(procedure_template_weighed_with_model),
            )
            .order_by(weighing_procedure_templates::procedure_template.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_name(
        name: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            procedure_templates::procedure_templates,
            weighing_procedure_templates::weighing_procedure_templates,
        };
        Self::table()
            .inner_join(
                procedure_templates::table.on(weighing_procedure_templates::procedure_template
                    .eq(procedure_templates::procedure_template_id)),
            )
            .filter(procedure_templates::name.eq(name))
            .order_by(weighing_procedure_templates::procedure_template.asc())
            .select(Self::as_select())
            .first::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_description(
        description: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            procedure_templates::procedure_templates,
            weighing_procedure_templates::weighing_procedure_templates,
        };
        Self::table()
            .inner_join(
                procedure_templates::table.on(weighing_procedure_templates::procedure_template
                    .eq(procedure_templates::procedure_template_id)),
            )
            .filter(procedure_templates::description.eq(description))
            .order_by(weighing_procedure_templates::procedure_template.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_created_by(
        created_by: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            procedure_templates::procedure_templates,
            weighing_procedure_templates::weighing_procedure_templates,
        };
        Self::table()
            .inner_join(
                procedure_templates::table.on(weighing_procedure_templates::procedure_template
                    .eq(procedure_templates::procedure_template_id)),
            )
            .filter(procedure_templates::created_by.eq(created_by))
            .order_by(weighing_procedure_templates::procedure_template.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_updated_by(
        updated_by: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            procedure_templates::procedure_templates,
            weighing_procedure_templates::weighing_procedure_templates,
        };
        Self::table()
            .inner_join(
                procedure_templates::table.on(weighing_procedure_templates::procedure_template
                    .eq(procedure_templates::procedure_template_id)),
            )
            .filter(procedure_templates::updated_by.eq(updated_by))
            .order_by(weighing_procedure_templates::procedure_template.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_deprecated(
        deprecated: bool,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            procedure_templates::procedure_templates,
            weighing_procedure_templates::weighing_procedure_templates,
        };
        Self::table()
            .inner_join(
                procedure_templates::table.on(weighing_procedure_templates::procedure_template
                    .eq(procedure_templates::procedure_template_id)),
            )
            .filter(procedure_templates::deprecated.eq(deprecated))
            .order_by(weighing_procedure_templates::procedure_template.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
}
impl AsRef<WeighingProcedureTemplate> for WeighingProcedureTemplate {
    fn as_ref(&self) -> &WeighingProcedureTemplate {
        self
    }
}
