impl<Procedure> web_common_traits::database::DispatchableInsertVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureBuilder<
        Procedure,
    >
{
    type Row =
        crate::codegen::structs_codegen::tables::freeze_drying_procedures::FreezeDryingProcedure;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::FreezeDryingProcedureAttribute,
    >;
}
impl<Procedure> web_common_traits::database::InsertableVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureBuilder<
        Procedure,
    >
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedure;
}
#[cfg(feature = "backend")]
impl<Procedure> web_common_traits::database::BackendInsertableVariant
    for crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureBuilder<
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
for crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureBuilder<
    Procedure,
>
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::freeze_drying_procedures::FreezeDryingProcedure as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedure as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::freeze_drying_procedures::FreezeDryingProcedure as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::freeze_drying_procedures::FreezeDryingProcedure,
    >,
    Self: web_common_traits::database::InsertableVariant<
        C,
        InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedure,
        Row = crate::codegen::structs_codegen::tables::freeze_drying_procedures::FreezeDryingProcedure,
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::FreezeDryingProcedureAttribute,
        >,
    >,
    Self: web_common_traits::database::MostConcreteTable,
{
    fn insert(mut self, user_id: i32, conn: &mut C) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::InsertableVariant;
        use web_common_traits::database::MostConcreteTable;
        self.set_most_concrete_table("freeze_drying_procedures");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedure = self
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
for crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureBuilder<
    Procedure,
>
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::freeze_drying_procedures::FreezeDryingProcedure as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedure as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::freeze_drying_procedures::FreezeDryingProcedure as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::freeze_drying_procedures::FreezeDryingProcedure,
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
    Self: crate::codegen::structs_codegen::tables::insertables::FreezeDryingProcedureSettable<
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::FreezeDryingProcedureAttribute,
        >,
    >,
    crate::codegen::structs_codegen::tables::assets::Asset: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::freeze_drying_procedure_templates::FreezeDryingProcedureTemplate: web_common_traits::database::Read<
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
        if let Some(procedure_template) = self.procedure_template {
            let freeze_drying_procedure_templates = crate::codegen::structs_codegen::tables::freeze_drying_procedure_templates::FreezeDryingProcedureTemplate::read(
                procedure_template,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::FreezeDryingProcedureSettable>::procedure_template_freeze_dried_container_model(
                self,
                freeze_drying_procedure_templates
                    .procedure_template_freeze_dried_container_model,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::FreezeDryingProcedureSettable>::procedure_template_freeze_dried_with_model(
                self,
                freeze_drying_procedure_templates
                    .procedure_template_freeze_dried_with_model,
            )?;
        }
        if let web_common_traits::database::IdOrBuilder::Id(
            procedure_freeze_dried_container,
        ) = self.procedure_freeze_dried_container
        {
            let procedure_assets = crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::read(
                procedure_freeze_dried_container,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::FreezeDryingProcedureSettable>::procedure_template_freeze_dried_container_model(
                self,
                procedure_assets.procedure_template_asset_model,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::FreezeDryingProcedureSettable>::freeze_dried_container_model(
                self,
                procedure_assets.asset_model,
            )?;
            if let Some(asset) = procedure_assets.asset {
                self = <Self as crate::codegen::structs_codegen::tables::insertables::FreezeDryingProcedureSettable>::freeze_dried_container(
                    self,
                    asset,
                )?;
            }
        }
        if let Some(freeze_dried_with) = self.freeze_dried_with {
            let assets = crate::codegen::structs_codegen::tables::assets::Asset::read(
                freeze_dried_with,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::FreezeDryingProcedureSettable>::freeze_dried_with_model(
                self,
                assets.model,
            )?;
        }
        if let web_common_traits::database::IdOrBuilder::Id(
            procedure_freeze_dried_with,
        ) = self.procedure_freeze_dried_with
        {
            let procedure_assets = crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::read(
                procedure_freeze_dried_with,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::FreezeDryingProcedureSettable>::freeze_dried_with(
                self,
                procedure_assets.asset,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::FreezeDryingProcedureSettable>::freeze_dried_with_model(
                self,
                procedure_assets.asset_model,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::FreezeDryingProcedureSettable>::procedure_template_freeze_dried_with_model(
                self,
                procedure_assets.procedure_template_asset_model,
            )?;
        }
        let procedure_template = self
            .procedure_template
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::FreezeDryingProcedureAttribute::ProcedureTemplate,
                ),
            )?;
        let freeze_dried_container = self
            .freeze_dried_container
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::FreezeDryingProcedureAttribute::FreezeDriedContainer,
                ),
            )?;
        let freeze_dried_container_model = self
            .freeze_dried_container_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::FreezeDryingProcedureAttribute::FreezeDriedContainerModel,
                ),
            )?;
        let procedure_template_freeze_dried_container_model = self
            .procedure_template_freeze_dried_container_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::FreezeDryingProcedureAttribute::ProcedureTemplateFreezeDriedContainerModel,
                ),
            )?;
        let freeze_dried_with_model = self
            .freeze_dried_with_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::FreezeDryingProcedureAttribute::FreezeDriedWithModel,
                ),
            )?;
        let procedure_template_freeze_dried_with_model = self
            .procedure_template_freeze_dried_with_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::FreezeDryingProcedureAttribute::ProcedureTemplateFreezeDriedWithModel,
                ),
            )?;
        let procedure = self
            .procedure
            .mint_primary_key(user_id, conn)
            .map_err(Self::Error::from_extension)?;
        let procedure_freeze_dried_container = match self
            .procedure_freeze_dried_container
        {
            web_common_traits::database::IdOrBuilder::Id(id) => id,
            web_common_traits::database::IdOrBuilder::Builder(
                mut procedure_freeze_dried_container,
            ) => {
                procedure_freeze_dried_container = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure(
                        procedure_freeze_dried_container,
                        procedure,
                    )
                    .map_err(|err| {
                        err.replace_field_name(
                            crate::codegen::structs_codegen::tables::insertables::FreezeDryingProcedureAttribute::ProcedureFreezeDriedContainer,
                        )
                    })?;
                procedure_freeze_dried_container
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.replace_field_name(
                            crate::codegen::structs_codegen::tables::insertables::FreezeDryingProcedureAttribute::ProcedureFreezeDriedContainer,
                        )
                    })?
            }
        };
        let procedure_freeze_dried_with = match self.procedure_freeze_dried_with {
            web_common_traits::database::IdOrBuilder::Id(id) => id,
            web_common_traits::database::IdOrBuilder::Builder(
                mut procedure_freeze_dried_with,
            ) => {
                procedure_freeze_dried_with = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure(
                        procedure_freeze_dried_with,
                        procedure,
                    )
                    .map_err(|err| {
                        err.replace_field_name(
                            crate::codegen::structs_codegen::tables::insertables::FreezeDryingProcedureAttribute::ProcedureFreezeDriedWith,
                        )
                    })?;
                procedure_freeze_dried_with
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.replace_field_name(
                            crate::codegen::structs_codegen::tables::insertables::FreezeDryingProcedureAttribute::ProcedureFreezeDriedWith,
                        )
                    })?
            }
        };
        Ok(Self::InsertableVariant {
            procedure,
            procedure_template,
            freeze_dried_container,
            freeze_dried_container_model,
            procedure_template_freeze_dried_container_model,
            procedure_freeze_dried_container,
            freeze_dried_with: self.freeze_dried_with,
            freeze_dried_with_model,
            procedure_template_freeze_dried_with_model,
            procedure_freeze_dried_with,
        })
    }
}
