impl<
    C: diesel::connection::LoadConnection,
    Procedure,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureBuilder<
    Procedure,
>
where
    <C as diesel::Connection>::Backend: diesel::backend::DieselReserveSpecialization,
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::centrifuge_procedures::CentrifugeProcedure as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedure as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::centrifuge_procedures::CentrifugeProcedure as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::centrifuge_procedures::CentrifugeProcedure,
    >,
    C: diesel::connection::LoadConnection,
    Procedure: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = ::rosetta_uuid::Uuid,
    >,
    Self: crate::codegen::structs_codegen::tables::insertables::CentrifugeProcedureSettable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::CentrifugeProcedureAttribute,
    >,
    crate::codegen::structs_codegen::tables::assets::Asset: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::centrifuge_procedure_templates::CentrifugeProcedureTemplate: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder: web_common_traits::database::TryInsertGeneric<
        C,
        Attributes = crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute,
        PrimaryKey = ::rosetta_uuid::Uuid,
    >,
    crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::procedures::Procedure: diesel::Identifiable
        + web_common_traits::database::Updatable<C, UserId = i32>,
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
    Self: web_common_traits::database::MostConcreteTable,
{
    type Row = crate::codegen::structs_codegen::tables::centrifuge_procedures::CentrifugeProcedure;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedure;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::CentrifugeProcedureAttribute,
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
        self.set_most_concrete_table("centrifuge_procedures");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedure = self
            .try_insert(user_id, conn)?;
        if !insertable_struct.procedure(conn)?.can_update(user_id, conn)? {
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
            let centrifuge_procedure_templates = crate::codegen::structs_codegen::tables::centrifuge_procedure_templates::CentrifugeProcedureTemplate::read(
                procedure_template,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::CentrifugeProcedureSettable>::procedure_template_centrifuged_with_model(
                self,
                centrifuge_procedure_templates.procedure_template_centrifuged_with_model,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::CentrifugeProcedureSettable>::procedure_template_centrifuged_container_model(
                self,
                centrifuge_procedure_templates
                    .procedure_template_centrifuged_container_model,
            )?;
        }
        if let web_common_traits::database::IdOrBuilder::Id(
            procedure_centrifuged_container,
        ) = self.procedure_centrifuged_container
        {
            let procedure_assets = crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::read(
                procedure_centrifuged_container,
                conn,
            )?;
            if let Some(asset) = procedure_assets.asset {
                self = <Self as crate::codegen::structs_codegen::tables::insertables::CentrifugeProcedureSettable>::centrifuged_container(
                    self,
                    asset,
                )?;
            }
            self = <Self as crate::codegen::structs_codegen::tables::insertables::CentrifugeProcedureSettable>::centrifuged_container_model(
                self,
                procedure_assets.asset_model,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::CentrifugeProcedureSettable>::procedure_template_centrifuged_container_model(
                self,
                procedure_assets.procedure_template_asset_model,
            )?;
        }
        if let Some(centrifuged_with) = self.centrifuged_with {
            let assets = crate::codegen::structs_codegen::tables::assets::Asset::read(
                centrifuged_with,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::CentrifugeProcedureSettable>::centrifuged_with_model(
                self,
                assets.model,
            )?;
        }
        if let web_common_traits::database::IdOrBuilder::Id(
            procedure_centrifuged_with,
        ) = self.procedure_centrifuged_with
        {
            let procedure_assets = crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::read(
                procedure_centrifuged_with,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::CentrifugeProcedureSettable>::centrifuged_with(
                self,
                procedure_assets.asset,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::CentrifugeProcedureSettable>::centrifuged_with_model(
                self,
                procedure_assets.asset_model,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::CentrifugeProcedureSettable>::procedure_template_centrifuged_with_model(
                self,
                procedure_assets.procedure_template_asset_model,
            )?;
        }
        let procedure_template = self
            .procedure_template
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::CentrifugeProcedureAttribute::ProcedureTemplate,
                ),
            )?;
        let centrifuged_container = self
            .centrifuged_container
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::CentrifugeProcedureAttribute::CentrifugedContainer,
                ),
            )?;
        let centrifuged_container_model = self
            .centrifuged_container_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::CentrifugeProcedureAttribute::CentrifugedContainerModel,
                ),
            )?;
        let procedure_template_centrifuged_container_model = self
            .procedure_template_centrifuged_container_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::CentrifugeProcedureAttribute::ProcedureTemplateCentrifugedContainerModel,
                ),
            )?;
        let centrifuged_with_model = self
            .centrifuged_with_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::CentrifugeProcedureAttribute::CentrifugedWithModel,
                ),
            )?;
        let procedure_template_centrifuged_with_model = self
            .procedure_template_centrifuged_with_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::CentrifugeProcedureAttribute::ProcedureTemplateCentrifugedWithModel,
                ),
            )?;
        let procedure = self
            .procedure
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::CentrifugeProcedureAttribute::Extension(
                    crate::codegen::structs_codegen::tables::insertables::CentrifugeProcedureExtensionAttribute::Procedure(
                        crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute::Procedure,
                    ),
                ))
            })?;
        let procedure_centrifuged_container = match self.procedure_centrifuged_container
        {
            web_common_traits::database::IdOrBuilder::Id(id) => id,
            web_common_traits::database::IdOrBuilder::Builder(
                mut procedure_centrifuged_container,
            ) => {
                procedure_centrifuged_container = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure(
                        procedure_centrifuged_container,
                        procedure,
                    )
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::CentrifugeProcedureAttribute::ProcedureCentrifugedContainer,
                        )
                    })?;
                procedure_centrifuged_container
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::CentrifugeProcedureAttribute::ProcedureCentrifugedContainer,
                        )
                    })?
            }
        };
        let procedure_centrifuged_with = match self.procedure_centrifuged_with {
            web_common_traits::database::IdOrBuilder::Id(id) => id,
            web_common_traits::database::IdOrBuilder::Builder(
                mut procedure_centrifuged_with,
            ) => {
                procedure_centrifuged_with = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure(
                        procedure_centrifuged_with,
                        procedure,
                    )
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::CentrifugeProcedureAttribute::ProcedureCentrifugedWith,
                        )
                    })?;
                procedure_centrifuged_with
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::CentrifugeProcedureAttribute::ProcedureCentrifugedWith,
                        )
                    })?
            }
        };
        Ok(Self::InsertableVariant {
            procedure,
            procedure_template,
            centrifuged_container,
            centrifuged_container_model,
            procedure_template_centrifuged_container_model,
            procedure_centrifuged_container,
            centrifuged_with_model,
            centrifuged_with: self.centrifuged_with,
            procedure_template_centrifuged_with_model,
            procedure_centrifuged_with,
        })
    }
}
