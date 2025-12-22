impl<Procedure> web_common_traits::database::DispatchableInsertVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureBuilder<
        Procedure,
    >
{
    type Row = crate::codegen::structs_codegen::tables::weighing_procedures::WeighingProcedure;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::WeighingProcedureAttribute,
    >;
}
impl<Procedure> web_common_traits::database::InsertableVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureBuilder<
        Procedure,
    >
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedure;
}
#[cfg(feature = "backend")]
impl<Procedure> web_common_traits::database::BackendInsertableVariant
    for crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureBuilder<
        Procedure,
    >
where
    Self: web_common_traits::database::DispatchableInsertableVariant<diesel::PgConnection>,
{
}
impl<
    C: diesel::connection::LoadConnection,
    Procedure,
> web_common_traits::database::DispatchableInsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureBuilder<
    Procedure,
>
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::weighing_procedures::WeighingProcedure as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedure as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::weighing_procedures::WeighingProcedure as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::weighing_procedures::WeighingProcedure,
    >,
    Self: web_common_traits::database::InsertableVariant<
        C,
        InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedure,
        Row = crate::codegen::structs_codegen::tables::weighing_procedures::WeighingProcedure,
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::WeighingProcedureAttribute,
        >,
    >,
    Self: web_common_traits::database::MostConcreteTable,
{
    fn insert(mut self, user_id: i32, conn: &mut C) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::InsertableVariant;
        use web_common_traits::database::MostConcreteTable;
        self.set_most_concrete_table("weighing_procedures");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedure = self
            .try_insert(user_id, conn)?;
        Ok(
            diesel::insert_into(Self::table())
                .values(insertable_struct)
                .get_result(conn)?,
        )
    }
}
impl<
    C: diesel::connection::LoadConnection,
    Procedure,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureBuilder<
    Procedure,
>
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::weighing_procedures::WeighingProcedure as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedure as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::weighing_procedures::WeighingProcedure as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::weighing_procedures::WeighingProcedure,
    >,
    Self::Error: web_common_traits::database::FromExtension<
        <Procedure as web_common_traits::database::TryInsertGeneric<C>>::Error,
    >,
    Procedure: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = ::rosetta_uuid::Uuid,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    Self: crate::codegen::structs_codegen::tables::insertables::WeighingProcedureSettable<
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::WeighingProcedureAttribute,
        >,
    >,
    crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::weighing_procedure_templates::WeighingProcedureTemplate: web_common_traits::database::Read<
        C,
    >,
{
    fn try_insert(
        mut self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::InsertableVariant, Self::Error> {
        use web_common_traits::database::FromExtension;
        use web_common_traits::database::TryInsertGeneric;
        use web_common_traits::database::Read;
        if let Some(procedure_template_id) = self.procedure_template {
            let weighing_procedure_templates = crate::codegen::structs_codegen::tables::weighing_procedure_templates::WeighingProcedureTemplate::read(
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
            let procedure_assets = crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::read(
                procedure_weighed_container,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::WeighingProcedureSettable>::procedure_template_weighed_container_model(
                self,
                procedure_assets.procedure_template_asset_model_id,
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
            let procedure_assets = crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::read(
                procedure_weighed_with,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::WeighingProcedureSettable>::procedure_template_weighed_with_model(
                self,
                procedure_assets.procedure_template_asset_model_id,
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
            .map_err(Self::Error::from_extension)?;
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
                        err.replace_field_name(
                            crate::codegen::structs_codegen::tables::insertables::WeighingProcedureAttribute::ProcedureWeighedContainer,
                        )
                    })?;
                procedure_weighed_container
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.replace_field_name(
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
                        err.replace_field_name(
                            crate::codegen::structs_codegen::tables::insertables::WeighingProcedureAttribute::ProcedureWeighedWith,
                        )
                    })?;
                procedure_weighed_with
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.replace_field_name(
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
