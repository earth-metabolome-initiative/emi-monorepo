impl<Procedure> web_common_traits::database::DispatchableInsertVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureBuilder<
        Procedure,
    >
{
    type Row = crate::codegen::structs_codegen::tables::centrifuge_procedures::CentrifugeProcedure;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::CentrifugeProcedureAttribute,
    >;
}
impl<Procedure> web_common_traits::database::InsertableVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureBuilder<
        Procedure,
    >
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedure;
}
#[cfg(feature = "backend")]
impl<Procedure> web_common_traits::database::BackendInsertableVariant
    for crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureBuilder<
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
for crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureBuilder<
    Procedure,
>
where
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
    Self: web_common_traits::database::InsertableVariant<
        C,
        InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedure,
        Row = crate::codegen::structs_codegen::tables::centrifuge_procedures::CentrifugeProcedure,
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CentrifugeProcedureAttribute,
        >,
    >,
    Self: web_common_traits::database::MostConcreteTable,
{
    fn insert(mut self, user_id: i32, conn: &mut C) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::InsertableVariant;
        use web_common_traits::database::MostConcreteTable;
        self.set_most_concrete_table("centrifuge_procedures");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedure = self
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
for crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureBuilder<
    Procedure,
>
where
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
    Self: crate::codegen::structs_codegen::tables::insertables::CentrifugeProcedureSettable<
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CentrifugeProcedureAttribute,
        >,
    >,
    crate::codegen::structs_codegen::tables::assets::Asset: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::centrifuge_procedure_templates::CentrifugeProcedureTemplate: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset: web_common_traits::database::Read<
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
                procedure_assets.asset_model_id,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::CentrifugeProcedureSettable>::procedure_template_centrifuged_container_model(
                self,
                procedure_assets.procedure_template_asset_model_id,
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
                procedure_assets.asset_model_id,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::CentrifugeProcedureSettable>::procedure_template_centrifuged_with_model(
                self,
                procedure_assets.procedure_template_asset_model_id,
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
            .map_err(Self::Error::from_extension)?;
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
                        err.replace_field_name(
                            crate::codegen::structs_codegen::tables::insertables::CentrifugeProcedureAttribute::ProcedureCentrifugedContainer,
                        )
                    })?;
                procedure_centrifuged_container
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.replace_field_name(
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
                        err.replace_field_name(
                            crate::codegen::structs_codegen::tables::insertables::CentrifugeProcedureAttribute::ProcedureCentrifugedWith,
                        )
                    })?;
                procedure_centrifuged_with
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.replace_field_name(
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
