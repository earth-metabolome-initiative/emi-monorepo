#[derive(Debug, Clone, PartialEq, Copy, PartialOrd)]
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
        crate::codegen::structs_codegen::tables::volume_measuring_device_models::VolumeMeasuringDeviceModel,
        foreign_key = measured_with_model
    )
)]
#[diesel(primary_key(procedure_template_id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::pouring_procedure_templates::pouring_procedure_templates
)]
pub struct PouringProcedureTemplate {
    pub procedure_template: i32,
    pub measured_with_model: i32,
    pub procedure_template_measured_with_model: i32,
    pub poured_from_model: i32,
    pub procedure_template_poured_from_model: i32,
    pub poured_into_model: i32,
    pub procedure_template_poured_into_model: i32,
    pub liters: f32,
}
impl web_common_traits::prelude::TableName for PouringProcedureTemplate {
    const TABLE_NAME: &'static str = "pouring_procedure_templates";
}
impl<'a> From<&'a PouringProcedureTemplate>
for web_common_traits::database::IdOrBuilder<
    i32,
    crate::codegen::structs_codegen::tables::insertables::InsertablePouringProcedureTemplateBuilder,
> {
    fn from(value: &'a PouringProcedureTemplate) -> Self {
        web_common_traits::database::IdOrBuilder::Id(value.procedure_template_id)
    }
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate,
    > for PouringProcedureTemplate
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{
}
impl web_common_traits::prelude::ExtensionTable<
    crate::codegen::structs_codegen::tables::pouring_procedure_templates::PouringProcedureTemplate,
> for PouringProcedureTemplate
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{}
impl diesel::Identifiable for PouringProcedureTemplate {
    type Id = i32;
    fn id(self) -> Self::Id {
        self.procedure_template
    }
}
impl web_common_traits::database::PrimaryKeyLike for PouringProcedureTemplate {
    type PrimaryKey = i32;
    fn primary_key(&self) -> Self::PrimaryKey {
        self.procedure_template
    }
}
impl PouringProcedureTemplate {
    pub fn measured_with_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::volume_measuring_device_models::VolumeMeasuringDeviceModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::volume_measuring_device_models::VolumeMeasuringDeviceModel: web_common_traits::database::Read<
            C,
        >,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::volume_measuring_device_models::VolumeMeasuringDeviceModel::read(
            self.measured_with_model,
            conn,
        )
    }
    pub fn poured_from_model<C: diesel::connection::LoadConnection>(
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
            self.poured_from_model,
            conn,
        )
    }
    pub fn poured_into_model<C: diesel::connection::LoadConnection>(
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
            self.poured_into_model,
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn pouring_procedure_templates_procedure_template_measured_w_fkey1(
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
                    .eq(&self.procedure_template_measured_with_model)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::dsl::asset_model
                            .eq(&self.measured_with_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
            >(conn)
    }
    pub fn procedure_template_measured_with_model<C: diesel::connection::LoadConnection>(
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
            self.procedure_template_measured_with_model,
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn pouring_procedure_templates_procedure_template_poured_fro_fkey1(
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
                    .eq(&self.procedure_template_poured_from_model)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::dsl::asset_model
                            .eq(&self.poured_from_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
            >(conn)
    }
    pub fn procedure_template_poured_from_model<C: diesel::connection::LoadConnection>(
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
            self.procedure_template_poured_from_model,
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn pouring_procedure_templates_procedure_template_poured_int_fkey1(
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
                    .eq(&self.procedure_template_poured_into_model)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_template_asset_models::procedure_template_asset_models::dsl::asset_model
                            .eq(&self.poured_into_model),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
            >(conn)
    }
    pub fn procedure_template_poured_into_model<C: diesel::connection::LoadConnection>(
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
            self.procedure_template_poured_into_model,
            conn,
        )
    }
    pub fn from_poured_from_model<C>(
        poured_from_model: i32,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::pouring_procedure_templates::pouring_procedure_templates::poured_from_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::pouring_procedure_templates::pouring_procedure_templates::poured_from_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::pouring_procedure_templates::pouring_procedure_templates::procedure_template,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::pouring_procedure_templates::pouring_procedure_templates::poured_from_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::pouring_procedure_templates::pouring_procedure_templates::procedure_template,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::pouring_procedure_templates::pouring_procedure_templates;
        Self::table()
            .filter(pouring_procedure_templates::poured_from_model.eq(poured_from_model))
            .order_by(pouring_procedure_templates::procedure_template.asc())
            .load::<Self>(conn)
    }
    pub fn from_poured_into_model<C>(
        poured_into_model: i32,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::pouring_procedure_templates::pouring_procedure_templates::poured_into_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::pouring_procedure_templates::pouring_procedure_templates::poured_into_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::pouring_procedure_templates::pouring_procedure_templates::procedure_template,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::pouring_procedure_templates::pouring_procedure_templates::poured_into_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::pouring_procedure_templates::pouring_procedure_templates::procedure_template,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::pouring_procedure_templates::pouring_procedure_templates;
        Self::table()
            .filter(pouring_procedure_templates::poured_into_model.eq(poured_into_model))
            .order_by(pouring_procedure_templates::procedure_template.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_measured_with_model_and_measured_with_model(
        procedure_template_measured_with_model: i32,
        measured_with_model: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::pouring_procedure_templates::pouring_procedure_templates;
        Self::table()
            .filter(
                pouring_procedure_templates::procedure_template_measured_with_model
                    .eq(procedure_template_measured_with_model)
                    .and(pouring_procedure_templates::measured_with_model.eq(measured_with_model)),
            )
            .order_by(pouring_procedure_templates::procedure_template.asc())
            .load::<Self>(conn)
    }
    pub fn from_procedure_template_measured_with_model<C>(
        procedure_template_measured_with_model: i32,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::pouring_procedure_templates::pouring_procedure_templates::procedure_template_measured_with_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::pouring_procedure_templates::pouring_procedure_templates::procedure_template_measured_with_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::pouring_procedure_templates::pouring_procedure_templates::procedure_template,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::pouring_procedure_templates::pouring_procedure_templates::procedure_template_measured_with_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::pouring_procedure_templates::pouring_procedure_templates::procedure_template,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::pouring_procedure_templates::pouring_procedure_templates;
        Self::table()
            .filter(
                pouring_procedure_templates::procedure_template_measured_with_model
                    .eq(procedure_template_measured_with_model),
            )
            .order_by(pouring_procedure_templates::procedure_template.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_poured_from_model_and_poured_from_model(
        procedure_template_poured_from_model: i32,
        poured_from_model: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::pouring_procedure_templates::pouring_procedure_templates;
        Self::table()
            .filter(
                pouring_procedure_templates::procedure_template_poured_from_model
                    .eq(procedure_template_poured_from_model)
                    .and(pouring_procedure_templates::poured_from_model.eq(poured_from_model)),
            )
            .order_by(pouring_procedure_templates::procedure_template.asc())
            .load::<Self>(conn)
    }
    pub fn from_procedure_template_poured_from_model<C>(
        procedure_template_poured_from_model: i32,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::pouring_procedure_templates::pouring_procedure_templates::procedure_template_poured_from_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::pouring_procedure_templates::pouring_procedure_templates::procedure_template_poured_from_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::pouring_procedure_templates::pouring_procedure_templates::procedure_template,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::pouring_procedure_templates::pouring_procedure_templates::procedure_template_poured_from_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::pouring_procedure_templates::pouring_procedure_templates::procedure_template,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::pouring_procedure_templates::pouring_procedure_templates;
        Self::table()
            .filter(
                pouring_procedure_templates::procedure_template_poured_from_model
                    .eq(procedure_template_poured_from_model),
            )
            .order_by(pouring_procedure_templates::procedure_template.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_poured_into_model_and_poured_into_model(
        procedure_template_poured_into_model: i32,
        poured_into_model: i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::pouring_procedure_templates::pouring_procedure_templates;
        Self::table()
            .filter(
                pouring_procedure_templates::procedure_template_poured_into_model
                    .eq(procedure_template_poured_into_model)
                    .and(pouring_procedure_templates::poured_into_model.eq(poured_into_model)),
            )
            .order_by(pouring_procedure_templates::procedure_template.asc())
            .load::<Self>(conn)
    }
    pub fn from_procedure_template_poured_into_model<C>(
        procedure_template_poured_into_model: i32,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>
    where
        C: diesel::connection::LoadConnection,
        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::pouring_procedure_templates::pouring_procedure_templates::procedure_template_poured_into_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >,
        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::pouring_procedure_templates::pouring_procedure_templates::procedure_template_poured_into_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output: diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::pouring_procedure_templates::pouring_procedure_templates::procedure_template,
            >,
        >,
        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
            <crate::codegen::diesel_codegen::tables::pouring_procedure_templates::pouring_procedure_templates::procedure_template_poured_into_model as diesel::expression_methods::EqAll<
                i32,
            >>::Output,
        >>::Output as diesel::query_dsl::methods::OrderDsl<
            diesel::helper_types::Asc<
                crate::codegen::diesel_codegen::tables::pouring_procedure_templates::pouring_procedure_templates::procedure_template,
            >,
        >>::Output: diesel::RunQueryDsl<C>
            + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
    {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::pouring_procedure_templates::pouring_procedure_templates;
        Self::table()
            .filter(
                pouring_procedure_templates::procedure_template_poured_into_model
                    .eq(procedure_template_poured_into_model),
            )
            .order_by(pouring_procedure_templates::procedure_template.asc())
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
            pouring_procedure_templates::pouring_procedure_templates,
            procedure_templates::procedure_templates,
        };
        Self::table()
            .inner_join(
                procedure_templates::table.on(pouring_procedure_templates::procedure_template
                    .eq(procedure_templates::procedure_template_id)),
            )
            .filter(procedure_templates::name.eq(name))
            .order_by(pouring_procedure_templates::procedure_template.asc())
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
            pouring_procedure_templates::pouring_procedure_templates,
            procedure_templates::procedure_templates,
        };
        Self::table()
            .inner_join(
                procedure_templates::table.on(pouring_procedure_templates::procedure_template
                    .eq(procedure_templates::procedure_template_id)),
            )
            .filter(procedure_templates::description.eq(description))
            .order_by(pouring_procedure_templates::procedure_template.asc())
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
            pouring_procedure_templates::pouring_procedure_templates,
            procedure_templates::procedure_templates,
        };
        Self::table()
            .inner_join(
                procedure_templates::table.on(pouring_procedure_templates::procedure_template
                    .eq(procedure_templates::procedure_template_id)),
            )
            .filter(procedure_templates::created_by.eq(created_by))
            .order_by(pouring_procedure_templates::procedure_template.asc())
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
            pouring_procedure_templates::pouring_procedure_templates,
            procedure_templates::procedure_templates,
        };
        Self::table()
            .inner_join(
                procedure_templates::table.on(pouring_procedure_templates::procedure_template
                    .eq(procedure_templates::procedure_template_id)),
            )
            .filter(procedure_templates::updated_by.eq(updated_by))
            .order_by(pouring_procedure_templates::procedure_template.asc())
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
            pouring_procedure_templates::pouring_procedure_templates,
            procedure_templates::procedure_templates,
        };
        Self::table()
            .inner_join(
                procedure_templates::table.on(pouring_procedure_templates::procedure_template
                    .eq(procedure_templates::procedure_template_id)),
            )
            .filter(procedure_templates::deprecated.eq(deprecated))
            .order_by(pouring_procedure_templates::procedure_template.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
}
impl AsRef<PouringProcedureTemplate> for PouringProcedureTemplate {
    fn as_ref(&self) -> &PouringProcedureTemplate {
        self
    }
}
