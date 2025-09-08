impl<
    C: diesel::connection::LoadConnection,
    ProcedureTemplate,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureTemplateBuilder<
    ProcedureTemplate,
>
where
    diesel::query_builder::InsertStatement<
        <crate::FractioningProcedureTemplate as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureTemplate as diesel::Insertable<
            <crate::FractioningProcedureTemplate as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::FractioningProcedureTemplate,
    >,
    C: diesel::connection::LoadConnection,
    ProcedureTemplate: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
    Self: crate::codegen::structs_codegen::tables::insertables::FractioningProcedureTemplateSettable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::FractioningProcedureTemplateAttribute,
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
    type Row = crate::FractioningProcedureTemplate;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureTemplate;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::FractioningProcedureTemplateAttribute,
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
        self.set_most_concrete_table("fractioning_procedure_templates");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureTemplate = self
            .try_insert(user_id, conn)?;
        if !insertable_struct.procedure_template(conn)?.can_update(user_id, conn)? {
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
        if !insertable_struct
            .procedure_template_fragment_placed_into_model(conn)?
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
            procedure_template_weighed_with_model,
        ) = self.procedure_template_weighed_with_model
        {
            let procedure_template_asset_models = crate::ProcedureTemplateAssetModel::read(
                procedure_template_weighed_with_model,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::FractioningProcedureTemplateSettable>::weighed_with_model(
                self,
                procedure_template_asset_models.asset_model,
            )?;
        }
        if let web_common_traits::database::IdOrBuilder::Id(
            procedure_template_fragment_container_model,
        ) = self.procedure_template_fragment_container_model
        {
            let procedure_template_asset_models = crate::ProcedureTemplateAssetModel::read(
                procedure_template_fragment_container_model,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::FractioningProcedureTemplateSettable>::fragment_container_model(
                self,
                procedure_template_asset_models.asset_model,
            )?;
        }
        if let web_common_traits::database::IdOrBuilder::Id(
            procedure_template_fragment_placed_into_model,
        ) = self.procedure_template_fragment_placed_into_model
        {
            let procedure_template_asset_models = crate::ProcedureTemplateAssetModel::read(
                procedure_template_fragment_placed_into_model,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::FractioningProcedureTemplateSettable>::fragment_placed_into_model(
                self,
                procedure_template_asset_models.asset_model,
            )?;
        }
        let kilograms = self
            .kilograms
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::FractioningProcedureTemplateAttribute::Kilograms,
                ),
            )?;
        let tolerance_percentage = self
            .tolerance_percentage
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::FractioningProcedureTemplateAttribute::TolerancePercentage,
                ),
            )?;
        let weighed_with_model = self
            .weighed_with_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::FractioningProcedureTemplateAttribute::WeighedWithModel,
                ),
            )?;
        let fragment_container_model = self
            .fragment_container_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::FractioningProcedureTemplateAttribute::FragmentContainerModel,
                ),
            )?;
        let fragment_placed_into_model = self
            .fragment_placed_into_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::FractioningProcedureTemplateAttribute::FragmentPlacedIntoModel,
                ),
            )?;
        let procedure_template = self
            .procedure_template
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::FractioningProcedureTemplateAttribute::Extension(
                    crate::codegen::structs_codegen::tables::insertables::FractioningProcedureTemplateExtensionAttribute::ProcedureTemplate(
                        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAttribute::ProcedureTemplate,
                    ),
                ))
            })?;
        let procedure_template_weighed_with_model = match self
            .procedure_template_weighed_with_model
        {
            web_common_traits::database::IdOrBuilder::Id(id) => id,
            web_common_traits::database::IdOrBuilder::Builder(
                mut procedure_template_weighed_with_model,
            ) => {
                procedure_template_weighed_with_model = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelSettable>::procedure_template(
                        procedure_template_weighed_with_model,
                        procedure_template,
                    )
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::FractioningProcedureTemplateAttribute::ProcedureTemplateWeighedWithModel,
                        )
                    })?;
                procedure_template_weighed_with_model
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::FractioningProcedureTemplateAttribute::ProcedureTemplateWeighedWithModel,
                        )
                    })?
            }
        };
        let procedure_template_fragment_container_model = match self
            .procedure_template_fragment_container_model
        {
            web_common_traits::database::IdOrBuilder::Id(id) => id,
            web_common_traits::database::IdOrBuilder::Builder(
                mut procedure_template_fragment_container_model,
            ) => {
                procedure_template_fragment_container_model = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelSettable>::procedure_template(
                        procedure_template_fragment_container_model,
                        procedure_template,
                    )
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::FractioningProcedureTemplateAttribute::ProcedureTemplateFragmentContainerModel,
                        )
                    })?;
                procedure_template_fragment_container_model
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::FractioningProcedureTemplateAttribute::ProcedureTemplateFragmentContainerModel,
                        )
                    })?
            }
        };
        let procedure_template_fragment_placed_into_model = match self
            .procedure_template_fragment_placed_into_model
        {
            web_common_traits::database::IdOrBuilder::Id(id) => id,
            web_common_traits::database::IdOrBuilder::Builder(
                mut procedure_template_fragment_placed_into_model,
            ) => {
                procedure_template_fragment_placed_into_model = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelSettable>::procedure_template(
                        procedure_template_fragment_placed_into_model,
                        procedure_template,
                    )
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::FractioningProcedureTemplateAttribute::ProcedureTemplateFragmentPlacedIntoModel,
                        )
                    })?;
                procedure_template_fragment_placed_into_model
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::FractioningProcedureTemplateAttribute::ProcedureTemplateFragmentPlacedIntoModel,
                        )
                    })?
            }
        };
        Ok(Self::InsertableVariant {
            procedure_template,
            kilograms,
            tolerance_percentage,
            weighed_with_model,
            procedure_template_weighed_with_model,
            fragment_container_model,
            procedure_template_fragment_container_model,
            fragment_placed_into_model,
            procedure_template_fragment_placed_into_model,
        })
    }
}
