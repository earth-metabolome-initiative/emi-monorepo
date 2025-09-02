#[derive(Debug, Clone, PartialEq, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    diesel::Selectable,
    diesel::Insertable,
    diesel::AsChangeset,
    diesel::Queryable,
    diesel::Identifiable,
)]
#[cfg_attr(feature = "yew", derive(yew::prelude::Properties))]
#[diesel(primary_key(procedure))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::fractioning_procedures::fractioning_procedures
)]
pub struct FractioningProcedure {
    pub procedure: ::rosetta_uuid::Uuid,
    pub procedure_template: i32,
    pub foreign_procedure_template: i32,
    pub foreign_procedure: ::rosetta_uuid::Uuid,
    pub fragment_container: ::rosetta_uuid::Uuid,
    pub fragment_placed_into: ::rosetta_uuid::Uuid,
    pub kilograms: f32,
    pub weighed_with: Option<::rosetta_uuid::Uuid>,
    pub weighed_with_model: i32,
}
impl web_common_traits::prelude::TableName for FractioningProcedure {
    const TABLE_NAME: &'static str = "fractioning_procedures";
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::procedures::Procedure,
    > for FractioningProcedure
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a ::rosetta_uuid::Uuid>,
{
}
impl diesel::Identifiable for FractioningProcedure {
    type Id = ::rosetta_uuid::Uuid;
    fn id(self) -> Self::Id {
        self.procedure
    }
}
impl FractioningProcedure {
    pub fn foreign_procedure_template<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate::table(),
                self.foreign_procedure_template,
            ),
            conn,
        )
    }
    pub fn foreign_procedure<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedures::Procedure,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedures::Procedure: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::procedures::Procedure as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedures::Procedure as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::procedures::Procedure as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedures::Procedure as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::procedures::Procedure as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedures::Procedure as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::procedures::Procedure,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::procedures::Procedure::table(),
                self.foreign_procedure,
            ),
            conn,
        )
    }
    pub fn fragment_container<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer::table(),
                self.fragment_container,
            ),
            conn,
        )
    }
    pub fn procedure<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedures::Procedure,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedures::Procedure: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::procedures::Procedure as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedures::Procedure as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::procedures::Procedure as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedures::Procedure as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::procedures::Procedure as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedures::Procedure as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::procedures::Procedure,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::procedures::Procedure::table(),
                self.procedure,
            ),
            conn,
        )
    }
    pub fn procedure_template<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::fractioning_procedure_templates::FractioningProcedureTemplate,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::fractioning_procedure_templates::FractioningProcedureTemplate: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::fractioning_procedure_templates::FractioningProcedureTemplate as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::fractioning_procedure_templates::FractioningProcedureTemplate as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::fractioning_procedure_templates::FractioningProcedureTemplate as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::fractioning_procedure_templates::FractioningProcedureTemplate as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::fractioning_procedure_templates::FractioningProcedureTemplate as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::fractioning_procedure_templates::FractioningProcedureTemplate as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::fractioning_procedure_templates::FractioningProcedureTemplate,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::fractioning_procedure_templates::FractioningProcedureTemplate::table(),
                self.procedure_template,
            ),
            conn,
        )
    }
    pub fn fragment_placed_into<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::volumetric_containers::VolumetricContainer::table(),
                self.fragment_placed_into,
            ),
            conn,
        )
    }
    pub fn weighed_with<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        Option<
            crate::codegen::structs_codegen::tables::weighing_devices::WeighingDevice,
        >,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::weighing_devices::WeighingDevice: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::weighing_devices::WeighingDevice as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::weighing_devices::WeighingDevice as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::weighing_devices::WeighingDevice as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::weighing_devices::WeighingDevice as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::weighing_devices::WeighingDevice as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::weighing_devices::WeighingDevice as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::weighing_devices::WeighingDevice,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        let Some(weighed_with) = self.weighed_with else {
            return Ok(None);
        };
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::weighing_devices::WeighingDevice::table(),
                weighed_with,
            ),
            conn,
        )
        .map(Some)
    }
    pub fn weighed_with_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::weighing_device_models::WeighingDeviceModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::weighing_device_models::WeighingDeviceModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::weighing_device_models::WeighingDeviceModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::weighing_device_models::WeighingDeviceModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::weighing_device_models::WeighingDeviceModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::weighing_device_models::WeighingDeviceModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::weighing_device_models::WeighingDeviceModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::weighing_device_models::WeighingDeviceModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::weighing_device_models::WeighingDeviceModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::weighing_device_models::WeighingDeviceModel::table(),
                self.weighed_with_model,
            ),
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn fractioning_procedures_foreign_procedure_fragment_containe_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
        diesel::result::Error,
    > {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::procedure
                    .eq(&self.foreign_procedure)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset
                            .eq(&self.fragment_container),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn fractioning_procedures_procedure_fragment_placed_into_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
        diesel::result::Error,
    > {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::procedure
                    .eq(&self.procedure)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset
                            .eq(&self.fragment_placed_into),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
    }
    pub fn fractioning_procedures_procedure_weighed_with_model_fkey<
        C: diesel::connection::LoadConnection,
    >(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::table(),
                (self.procedure, self.weighed_with_model),
            ),
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn fractioning_procedures_procedure_weighed_with_fkey(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset>,
        diesel::result::Error,
    > {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };
        let Some(weighed_with) = self.weighed_with else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::table()
            .filter(
                crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::procedure
                    .eq(&self.procedure)
                    .and(
                        crate::codegen::diesel_codegen::tables::procedure_assets::procedure_assets::dsl::asset
                            .eq(weighed_with),
                    ),
            )
            .first::<
                crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset,
            >(conn)
            .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template(
        procedure_template: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::fractioning_procedures::fractioning_procedures;
        Self::table()
            .filter(fractioning_procedures::procedure_template.eq(procedure_template))
            .order_by(fractioning_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_foreign_procedure_template(
        foreign_procedure_template: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::fractioning_procedures::fractioning_procedures;
        Self::table()
            .filter(
                fractioning_procedures::foreign_procedure_template.eq(foreign_procedure_template),
            )
            .order_by(fractioning_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_foreign_procedure(
        foreign_procedure: &::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::fractioning_procedures::fractioning_procedures;
        Self::table()
            .filter(fractioning_procedures::foreign_procedure.eq(foreign_procedure))
            .order_by(fractioning_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_fragment_container(
        fragment_container: &::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::fractioning_procedures::fractioning_procedures;
        Self::table()
            .filter(fractioning_procedures::fragment_container.eq(fragment_container))
            .order_by(fractioning_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_fragment_placed_into(
        fragment_placed_into: &::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::fractioning_procedures::fractioning_procedures;
        Self::table()
            .filter(fractioning_procedures::fragment_placed_into.eq(fragment_placed_into))
            .order_by(fractioning_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_weighed_with(
        weighed_with: &::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::fractioning_procedures::fractioning_procedures;
        Self::table()
            .filter(fractioning_procedures::weighed_with.eq(weighed_with))
            .order_by(fractioning_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_weighed_with_model(
        weighed_with_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::fractioning_procedures::fractioning_procedures;
        Self::table()
            .filter(fractioning_procedures::weighed_with_model.eq(weighed_with_model))
            .order_by(fractioning_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_and_procedure_template(
        procedure: &::rosetta_uuid::Uuid,
        procedure_template: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::fractioning_procedures::fractioning_procedures;
        Self::table()
            .filter(
                fractioning_procedures::procedure
                    .eq(procedure)
                    .and(fractioning_procedures::procedure_template.eq(procedure_template)),
            )
            .order_by(fractioning_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_foreign_procedure_and_foreign_procedure_template(
        foreign_procedure: &::rosetta_uuid::Uuid,
        foreign_procedure_template: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::fractioning_procedures::fractioning_procedures;
        Self::table()
            .filter(fractioning_procedures::foreign_procedure.eq(foreign_procedure).and(
                fractioning_procedures::foreign_procedure_template.eq(foreign_procedure_template),
            ))
            .order_by(fractioning_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_foreign_procedure_and_fragment_container(
        foreign_procedure: &::rosetta_uuid::Uuid,
        fragment_container: &::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::fractioning_procedures::fractioning_procedures;
        Self::table()
            .filter(
                fractioning_procedures::foreign_procedure
                    .eq(foreign_procedure)
                    .and(fractioning_procedures::fragment_container.eq(fragment_container)),
            )
            .order_by(fractioning_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_and_fragment_placed_into(
        procedure: &::rosetta_uuid::Uuid,
        fragment_placed_into: &::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::fractioning_procedures::fractioning_procedures;
        Self::table()
            .filter(
                fractioning_procedures::procedure
                    .eq(procedure)
                    .and(fractioning_procedures::fragment_placed_into.eq(fragment_placed_into)),
            )
            .order_by(fractioning_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_and_weighed_with_model(
        procedure: &::rosetta_uuid::Uuid,
        weighed_with_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::fractioning_procedures::fractioning_procedures;
        Self::table()
            .filter(
                fractioning_procedures::procedure
                    .eq(procedure)
                    .and(fractioning_procedures::weighed_with_model.eq(weighed_with_model)),
            )
            .order_by(fractioning_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_and_weighed_with(
        procedure: &::rosetta_uuid::Uuid,
        weighed_with: &::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::fractioning_procedures::fractioning_procedures;
        Self::table()
            .filter(
                fractioning_procedures::procedure
                    .eq(procedure)
                    .and(fractioning_procedures::weighed_with.eq(weighed_with)),
            )
            .order_by(fractioning_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_weighed_with_and_weighed_with_model(
        weighed_with: &::rosetta_uuid::Uuid,
        weighed_with_model: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::fractioning_procedures::fractioning_procedures;
        Self::table()
            .filter(
                fractioning_procedures::weighed_with
                    .eq(weighed_with)
                    .and(fractioning_procedures::weighed_with_model.eq(weighed_with_model)),
            )
            .order_by(fractioning_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_and_foreign_procedure_template(
        procedure_template: &i32,
        foreign_procedure_template: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::fractioning_procedures::fractioning_procedures;
        Self::table()
            .filter(fractioning_procedures::procedure_template.eq(procedure_template).and(
                fractioning_procedures::foreign_procedure_template.eq(foreign_procedure_template),
            ))
            .order_by(fractioning_procedures::procedure.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_procedure_template_and_procedure(
        procedure_template: &i32,
        procedure: &::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl,
            SelectableHelper, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            fractioning_procedures::fractioning_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(fractioning_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(
                procedures::procedure_template
                    .eq(procedure_template)
                    .and(procedures::procedure.eq(procedure)),
            )
            .order_by(fractioning_procedures::procedure.asc())
            .select(Self::as_select())
            .first::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_most_concrete_table(
        most_concrete_table: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            fractioning_procedures::fractioning_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(fractioning_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::most_concrete_table.eq(most_concrete_table))
            .order_by(fractioning_procedures::procedure.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_created_by(
        created_by: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            fractioning_procedures::fractioning_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(fractioning_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::created_by.eq(created_by))
            .order_by(fractioning_procedures::procedure.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_created_at(
        created_at: &::rosetta_timestamp::TimestampUTC,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            fractioning_procedures::fractioning_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(fractioning_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::created_at.eq(created_at))
            .order_by(fractioning_procedures::procedure.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_updated_by(
        updated_by: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            fractioning_procedures::fractioning_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(fractioning_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::updated_by.eq(updated_by))
            .order_by(fractioning_procedures::procedure.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_updated_at(
        updated_at: &::rosetta_timestamp::TimestampUTC,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{
            ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper,
            associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::{
            fractioning_procedures::fractioning_procedures, procedures::procedures,
        };
        Self::table()
            .inner_join(
                procedures::table.on(fractioning_procedures::procedure.eq(procedures::procedure)),
            )
            .filter(procedures::updated_at.eq(updated_at))
            .order_by(fractioning_procedures::procedure.asc())
            .select(Self::as_select())
            .load::<Self>(conn)
    }
}
impl AsRef<FractioningProcedure> for FractioningProcedure {
    fn as_ref(&self) -> &FractioningProcedure {
        self
    }
}
