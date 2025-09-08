impl<
    C: diesel::connection::LoadConnection,
    ProcedureTemplate,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableStorageProcedureTemplateBuilder<
    ProcedureTemplate,
>
where
    diesel::query_builder::InsertStatement<
        <crate::StorageProcedureTemplate as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableStorageProcedureTemplate as diesel::Insertable<
            <crate::StorageProcedureTemplate as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::StorageProcedureTemplate,
    >,
    C: diesel::connection::LoadConnection,
    ProcedureTemplate: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
    Self: crate::codegen::structs_codegen::tables::insertables::StorageProcedureTemplateSettable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::StorageProcedureTemplateAttribute,
    >,
    crate::ProcedureTemplate: web_common_traits::database::Read<C>,
    crate::ProcedureTemplate: web_common_traits::database::Updatable<C, UserId = i32>,
    crate::ProcedureTemplateAssetModel: web_common_traits::database::Read<C>,
    crate::ProcedureTemplateAssetModel: web_common_traits::database::Updatable<
        C,
        UserId = i32,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder: web_common_traits::database::TryInsertGeneric<
        C,
        Attributes = crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelAttribute,
        PrimaryKey = i32,
    >,
    Self: web_common_traits::database::MostConcreteTable,
{
    type Row = crate::StorageProcedureTemplate;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableStorageProcedureTemplate;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::StorageProcedureTemplateAttribute,
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
        self.set_most_concrete_table("storage_procedure_templates");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableStorageProcedureTemplate = self
            .try_insert(user_id, conn)?;
        if !insertable_struct.procedure_template(conn)?.can_update(user_id, conn)? {
            return Err(
                generic_backend_request_errors::GenericBackendRequestError::Unauthorized
                    .into(),
            );
        }
        if !insertable_struct
            .procedure_template_stored_into_model(conn)?
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
        if let web_common_traits::database::IdOrBuilder::Id(
            procedure_template_stored_into_model,
        ) = self.procedure_template_stored_into_model
        {
            let procedure_template_asset_models = crate::ProcedureTemplateAssetModel::read(
                procedure_template_stored_into_model,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::StorageProcedureTemplateSettable>::stored_into_model(
                self,
                procedure_template_asset_models.asset_model,
            )?;
        }
        if let web_common_traits::database::IdOrBuilder::Id(
            procedure_template_stored_asset_model,
        ) = self.procedure_template_stored_asset_model
        {
            let procedure_template_asset_models = crate::ProcedureTemplateAssetModel::read(
                procedure_template_stored_asset_model,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::StorageProcedureTemplateSettable>::stored_asset_model(
                self,
                procedure_template_asset_models.asset_model,
            )?;
        }
        let kelvin = self
            .kelvin
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::StorageProcedureTemplateAttribute::Kelvin,
                ),
            )?;
        let kelvin_tolerance_percentage = self
            .kelvin_tolerance_percentage
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::StorageProcedureTemplateAttribute::KelvinTolerancePercentage,
                ),
            )?;
        let stored_into_model = self
            .stored_into_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::StorageProcedureTemplateAttribute::StoredIntoModel,
                ),
            )?;
        let stored_asset_model = self
            .stored_asset_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::StorageProcedureTemplateAttribute::StoredAssetModel,
                ),
            )?;
        let procedure_template = self
            .procedure_template
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::StorageProcedureTemplateAttribute::Extension(
                    crate::codegen::structs_codegen::tables::insertables::StorageProcedureTemplateExtensionAttribute::ProcedureTemplate(
                        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAttribute::ProcedureTemplate,
                    ),
                ))
            })?;
        let procedure_template_stored_into_model = match self
            .procedure_template_stored_into_model
        {
            web_common_traits::database::IdOrBuilder::Id(id) => id,
            web_common_traits::database::IdOrBuilder::Builder(
                mut procedure_template_stored_into_model,
            ) => {
                procedure_template_stored_into_model = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelSettable>::procedure_template(
                        procedure_template_stored_into_model,
                        procedure_template,
                    )
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::StorageProcedureTemplateAttribute::ProcedureTemplateStoredIntoModel,
                        )
                    })?;
                procedure_template_stored_into_model
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::StorageProcedureTemplateAttribute::ProcedureTemplateStoredIntoModel,
                        )
                    })?
            }
        };
        let procedure_template_stored_asset_model = match self
            .procedure_template_stored_asset_model
        {
            web_common_traits::database::IdOrBuilder::Id(id) => id,
            web_common_traits::database::IdOrBuilder::Builder(
                mut procedure_template_stored_asset_model,
            ) => {
                procedure_template_stored_asset_model = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelSettable>::procedure_template(
                        procedure_template_stored_asset_model,
                        procedure_template,
                    )
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::StorageProcedureTemplateAttribute::ProcedureTemplateStoredAssetModel,
                        )
                    })?;
                procedure_template_stored_asset_model
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::StorageProcedureTemplateAttribute::ProcedureTemplateStoredAssetModel,
                        )
                    })?
            }
        };
        Ok(Self::InsertableVariant {
            procedure_template,
            kelvin,
            kelvin_tolerance_percentage,
            stored_into_model,
            procedure_template_stored_into_model,
            stored_asset_model,
            procedure_template_stored_asset_model,
        })
    }
}
