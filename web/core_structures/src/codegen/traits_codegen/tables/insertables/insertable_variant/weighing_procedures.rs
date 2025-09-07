impl<
    C: diesel::connection::LoadConnection,
    Procedure,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureBuilder<
    Procedure,
>
where
    <C as diesel::Connection>::Backend: diesel::backend::DieselReserveSpecialization,
    diesel::query_builder::InsertStatement<
        <crate::WeighingProcedure as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedure as diesel::Insertable<
            <crate::WeighingProcedure as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<'query, C, crate::WeighingProcedure>,
    C: diesel::connection::LoadConnection,
    Procedure: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = ::rosetta_uuid::Uuid,
    >,
    Self: crate::codegen::structs_codegen::tables::insertables::WeighingProcedureSettable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::WeighingProcedureAttribute,
    >,
    crate::Procedure: diesel::Identifiable
        + web_common_traits::database::Updatable<C, UserId = i32>,
    <crate::Procedure as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
        <crate::Procedure as diesel::Identifiable>::Id,
    >,
    <<crate::Procedure as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::Procedure as diesel::Identifiable>::Id,
    >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
    <<<crate::Procedure as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::Procedure as diesel::Identifiable>::Id,
    >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
        'a,
        C,
        crate::Procedure,
    >,
    crate::ProcedureAsset: web_common_traits::database::Read<C>,
    crate::ProcedureAsset: web_common_traits::database::Read<C>,
    crate::ProcedureTemplateAssetModel: diesel::Identifiable
        + web_common_traits::database::Updatable<C, UserId = i32>,
    <crate::ProcedureTemplateAssetModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
        <crate::ProcedureTemplateAssetModel as diesel::Identifiable>::Id,
    >,
    <<crate::ProcedureTemplateAssetModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::ProcedureTemplateAssetModel as diesel::Identifiable>::Id,
    >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
    <<<crate::ProcedureTemplateAssetModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::ProcedureTemplateAssetModel as diesel::Identifiable>::Id,
    >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
        'a,
        C,
        crate::ProcedureTemplateAssetModel,
    >,
    crate::WeighingProcedureTemplate: web_common_traits::database::Read<C>,
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder: web_common_traits::database::TryInsertGeneric<
        C,
        Attributes = crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute,
        PrimaryKey = ::rosetta_uuid::Uuid,
    >,
    Self: web_common_traits::database::MostConcreteTable,
{
    type Row = crate::WeighingProcedure;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedure;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::WeighingProcedureAttribute,
    >;
    type UserId = i32;
    fn insert(
        mut self,
        user_id: Self::UserId,
        conn: &mut C,
    ) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::Updatable;
        use web_common_traits::database::MostConcreteTable;
        self.set_most_concrete_table("weighing_procedures");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedure = self
            .try_insert(user_id, conn)?;
        if !insertable_struct.procedure(conn)?.can_update(user_id, conn)? {
            return Err(
                generic_backend_request_errors::GenericBackendRequestError::Unauthorized
                    .into(),
            );
        }
        if !insertable_struct
            .procedure_template_weighed_with_model(conn)?
            .can_update(user_id, conn)?
        {
            return Err(
                generic_backend_request_errors::GenericBackendRequestError::Unauthorized
                    .into(),
            );
        }
        Ok(
            diesel::insert_into(Self::Row::table())
                .values(insertable_struct)
                .get_result(conn)?,
        )
    }
    fn try_insert(
        mut self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::InsertableVariant, Self::Error> {
        use web_common_traits::database::TryInsertGeneric;
        use web_common_traits::database::Read;
        if let Some(procedure_template) = self.procedure_template {
            let weighing_procedure_templates = crate::WeighingProcedureTemplate::read(
                procedure_template,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::WeighingProcedureSettable>::procedure_template_weighed_container_model(
                self,
                weighing_procedure_templates.procedure_template_weighed_container_model,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::WeighingProcedureSettable>::procedure_template_weighed_with_model(
                self,
                weighing_procedure_templates.procedure_template_weighed_with_model,
            )?;
        }
        if let web_common_traits::database::IdOrBuilder::Id(
            procedure_weighed_container,
        ) = self.procedure_weighed_container
        {
            let procedure_assets = crate::ProcedureAsset::read(
                procedure_weighed_container,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::WeighingProcedureSettable>::procedure_template_weighed_container_model(
                self,
                procedure_assets.procedure_template_asset_model,
            )?;
            if let Some(asset) = procedure_assets.asset {
                self = <Self as crate::codegen::structs_codegen::tables::insertables::WeighingProcedureSettable>::weighed_container(
                    self,
                    asset,
                )?;
            }
        }
        if let web_common_traits::database::IdOrBuilder::Id(procedure_weighed_with) = self
            .procedure_weighed_with
        {
            let procedure_assets = crate::ProcedureAsset::read(
                procedure_weighed_with,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::WeighingProcedureSettable>::procedure_template_weighed_with_model(
                self,
                procedure_assets.procedure_template_asset_model,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::WeighingProcedureSettable>::weighed_with(
                self,
                procedure_assets.asset,
            )?;
        }
        let procedure_template = self
            .procedure_template
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::WeighingProcedureAttribute::ProcedureTemplate,
                ),
            )?;
        let weighed_container = self
            .weighed_container
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::WeighingProcedureAttribute::WeighedContainer,
                ),
            )?;
        let procedure_template_weighed_container_model = self
            .procedure_template_weighed_container_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::WeighingProcedureAttribute::ProcedureTemplateWeighedContainerModel,
                ),
            )?;
        let kilograms = self
            .kilograms
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::WeighingProcedureAttribute::Kilograms,
                ),
            )?;
        let procedure_template_weighed_with_model = self
            .procedure_template_weighed_with_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::WeighingProcedureAttribute::ProcedureTemplateWeighedWithModel,
                ),
            )?;
        let procedure = self
            .procedure
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::WeighingProcedureAttribute::Extension(
                    crate::codegen::structs_codegen::tables::insertables::WeighingProcedureExtensionAttribute::Procedure(
                        crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute::Procedure,
                    ),
                ))
            })?;
        let procedure_weighed_container = match self.procedure_weighed_container {
            web_common_traits::database::IdOrBuilder::Id(id) => id,
            web_common_traits::database::IdOrBuilder::Builder(
                mut procedure_weighed_container,
            ) => {
                procedure_weighed_container = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure(
                        procedure_weighed_container,
                        procedure,
                    )
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::WeighingProcedureAttribute::ProcedureWeighedContainer,
                        )
                    })?;
                procedure_weighed_container
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::WeighingProcedureAttribute::ProcedureWeighedContainer,
                        )
                    })?
            }
        };
        let procedure_weighed_with = match self.procedure_weighed_with {
            web_common_traits::database::IdOrBuilder::Id(id) => id,
            web_common_traits::database::IdOrBuilder::Builder(
                mut procedure_weighed_with,
            ) => {
                procedure_weighed_with = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure(
                        procedure_weighed_with,
                        procedure,
                    )
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::WeighingProcedureAttribute::ProcedureWeighedWith,
                        )
                    })?;
                procedure_weighed_with
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::WeighingProcedureAttribute::ProcedureWeighedWith,
                        )
                    })?
            }
        };
        Ok(Self::InsertableVariant {
            procedure,
            procedure_template,
            weighed_container,
            procedure_template_weighed_container_model,
            procedure_weighed_container,
            kilograms,
            weighed_with: self.weighed_with,
            procedure_template_weighed_with_model,
            procedure_weighed_with,
        })
    }
}
