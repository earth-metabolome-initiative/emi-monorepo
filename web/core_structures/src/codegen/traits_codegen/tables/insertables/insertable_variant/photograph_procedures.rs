impl<Procedure> web_common_traits::database::InsertableVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureBuilder<
        Procedure,
    >
{
    type Row = crate::codegen::structs_codegen::tables::photograph_procedures::PhotographProcedure;
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedure;
    type UserId = i32;
}
impl<
    C: diesel::connection::LoadConnection,
    Procedure,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureBuilder<
    Procedure,
>
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::photograph_procedures::PhotographProcedure as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedure as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::photograph_procedures::PhotographProcedure as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::photograph_procedures::PhotographProcedure,
    >,
    Procedure: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = ::rosetta_uuid::Uuid,
    >,
    Self: crate::codegen::structs_codegen::tables::insertables::PhotographProcedureSettable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::PhotographProcedureAttribute,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder: web_common_traits::database::TryInsertGeneric<
        C,
        Attribute = crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute,
        PrimaryKey = ::rosetta_uuid::Uuid,
    >,
    crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset: web_common_traits::database::Read<
        C,
    >,
    Self: web_common_traits::database::MostConcreteTable,
    crate::codegen::structs_codegen::tables::insertables::PhotographProcedureExtensionAttribute: From<
        <Procedure as common_traits::builder::Attributed>::Attribute,
    >,
{
    fn insert(
        mut self,
        user_id: Self::UserId,
        conn: &mut C,
    ) -> Result<
        Self::Row,
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::PhotographProcedureAttribute,
        >,
    > {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::MostConcreteTable;
        self.set_most_concrete_table("photograph_procedures");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedure = self
            .try_insert(user_id, conn)?;
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
    ) -> Result<
        Self::InsertableVariant,
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::PhotographProcedureAttribute,
        >,
    > {
        use web_common_traits::database::TryInsertGeneric;
        use web_common_traits::database::Read;
        if let Some(procedure_template) = self.procedure_template {
            let photograph_procedure_templates = crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate::read(
                procedure_template,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::PhotographProcedureSettable>::procedure_template_photographed_asset_model(
                self,
                photograph_procedure_templates
                    .procedure_template_photographed_asset_model,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::PhotographProcedureSettable>::procedure_template_photograph_model(
                self,
                photograph_procedure_templates.procedure_template_photograph_model,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::PhotographProcedureSettable>::procedure_template_photographed_with_model(
                self,
                photograph_procedure_templates.procedure_template_photographed_with_model,
            )?;
        }
        if let web_common_traits::database::IdOrBuilder::Id(
            procedure_photographed_asset,
        ) = self.procedure_photographed_asset
        {
            let procedure_assets = crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::read(
                procedure_photographed_asset,
                conn,
            )?;
            if let Some(asset) = procedure_assets.asset {
                self = <Self as crate::codegen::structs_codegen::tables::insertables::PhotographProcedureSettable>::photographed_asset(
                    self,
                    asset,
                )?;
            }
            self = <Self as crate::codegen::structs_codegen::tables::insertables::PhotographProcedureSettable>::procedure_template_photographed_asset_model(
                self,
                procedure_assets.procedure_template_asset_model,
            )?;
        }
        if let web_common_traits::database::IdOrBuilder::Id(
            procedure_photographed_with,
        ) = self.procedure_photographed_with
        {
            let procedure_assets = crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::read(
                procedure_photographed_with,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::PhotographProcedureSettable>::photographed_with(
                self,
                procedure_assets.asset,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::PhotographProcedureSettable>::procedure_template_photographed_with_model(
                self,
                procedure_assets.procedure_template_asset_model,
            )?;
        }
        if let web_common_traits::database::IdOrBuilder::Id(procedure_photograph) = self
            .procedure_photograph
        {
            let procedure_assets = crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::read(
                procedure_photograph,
                conn,
            )?;
            if let Some(asset) = procedure_assets.asset {
                self = <Self as crate::codegen::structs_codegen::tables::insertables::PhotographProcedureSettable>::photograph(
                    self,
                    asset,
                )?;
            }
            self = <Self as crate::codegen::structs_codegen::tables::insertables::PhotographProcedureSettable>::procedure_template_photograph_model(
                self,
                procedure_assets.procedure_template_asset_model,
            )?;
        }
        let procedure_template = self
            .procedure_template
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::PhotographProcedureAttribute::ProcedureTemplate,
                ),
            )?;
        let photographed_asset = self
            .photographed_asset
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::PhotographProcedureAttribute::PhotographedAsset,
                ),
            )?;
        let procedure_template_photographed_asset_model = self
            .procedure_template_photographed_asset_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::PhotographProcedureAttribute::ProcedureTemplatePhotographedAssetModel,
                ),
            )?;
        let procedure_template_photographed_with_model = self
            .procedure_template_photographed_with_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::PhotographProcedureAttribute::ProcedureTemplatePhotographedWithModel,
                ),
            )?;
        let photograph = self
            .photograph
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::PhotographProcedureAttribute::Photograph,
                ),
            )?;
        let procedure_template_photograph_model = self
            .procedure_template_photograph_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::PhotographProcedureAttribute::ProcedureTemplatePhotographModel,
                ),
            )?;
        let procedure = self
            .procedure
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(|attribute| {
                    crate::codegen::structs_codegen::tables::insertables::PhotographProcedureAttribute::Extension(
                        From::from(attribute),
                    )
                })
            })?;
        let procedure_photographed_asset = match self.procedure_photographed_asset {
            web_common_traits::database::IdOrBuilder::Id(id) => id,
            web_common_traits::database::IdOrBuilder::Builder(
                mut procedure_photographed_asset,
            ) => {
                procedure_photographed_asset = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure(
                        procedure_photographed_asset,
                        procedure,
                    )
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::PhotographProcedureAttribute::ProcedurePhotographedAsset,
                        )
                    })?;
                procedure_photographed_asset
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::PhotographProcedureAttribute::ProcedurePhotographedAsset,
                        )
                    })?
            }
        };
        let procedure_photographed_with = match self.procedure_photographed_with {
            web_common_traits::database::IdOrBuilder::Id(id) => id,
            web_common_traits::database::IdOrBuilder::Builder(
                mut procedure_photographed_with,
            ) => {
                procedure_photographed_with = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure(
                        procedure_photographed_with,
                        procedure,
                    )
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::PhotographProcedureAttribute::ProcedurePhotographedWith,
                        )
                    })?;
                procedure_photographed_with
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::PhotographProcedureAttribute::ProcedurePhotographedWith,
                        )
                    })?
            }
        };
        let procedure_photograph = match self.procedure_photograph {
            web_common_traits::database::IdOrBuilder::Id(id) => id,
            web_common_traits::database::IdOrBuilder::Builder(
                mut procedure_photograph,
            ) => {
                procedure_photograph = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure(
                        procedure_photograph,
                        procedure,
                    )
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::PhotographProcedureAttribute::ProcedurePhotograph,
                        )
                    })?;
                procedure_photograph
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::PhotographProcedureAttribute::ProcedurePhotograph,
                        )
                    })?
            }
        };
        Ok(Self::InsertableVariant {
            procedure,
            procedure_template,
            photographed_asset,
            procedure_template_photographed_asset_model,
            procedure_photographed_asset,
            photographed_with: self.photographed_with,
            procedure_template_photographed_with_model,
            procedure_photographed_with,
            photograph,
            procedure_template_photograph_model,
            procedure_photograph,
        })
    }
}
