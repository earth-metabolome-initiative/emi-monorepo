impl<Procedure> web_common_traits::database::DispatchableInsertVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertablePouringProcedureBuilder<
        Procedure,
    >
{
    type Row = crate::codegen::structs_codegen::tables::pouring_procedures::PouringProcedure;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::PouringProcedureAttribute,
    >;
}
impl<Procedure> web_common_traits::database::InsertableVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertablePouringProcedureBuilder<
        Procedure,
    >
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertablePouringProcedure;
}
#[cfg(feature = "backend")]
impl<Procedure> web_common_traits::database::BackendInsertableVariant
    for crate::codegen::structs_codegen::tables::insertables::InsertablePouringProcedureBuilder<
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
for crate::codegen::structs_codegen::tables::insertables::InsertablePouringProcedureBuilder<
    Procedure,
>
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::pouring_procedures::PouringProcedure as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertablePouringProcedure as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::pouring_procedures::PouringProcedure as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::pouring_procedures::PouringProcedure,
    >,
    Self: web_common_traits::database::InsertableVariant<
        C,
        InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertablePouringProcedure,
        Row = crate::codegen::structs_codegen::tables::pouring_procedures::PouringProcedure,
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::PouringProcedureAttribute,
        >,
    >,
    Self: web_common_traits::database::MostConcreteTable,
{
    fn insert(mut self, user_id: i32, conn: &mut C) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::InsertableVariant;
        use web_common_traits::database::MostConcreteTable;
        self.set_most_concrete_table("pouring_procedures");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertablePouringProcedure = self
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
for crate::codegen::structs_codegen::tables::insertables::InsertablePouringProcedureBuilder<
    Procedure,
>
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::pouring_procedures::PouringProcedure as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertablePouringProcedure as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::pouring_procedures::PouringProcedure as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::pouring_procedures::PouringProcedure,
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
    Self: crate::codegen::structs_codegen::tables::insertables::PouringProcedureSettable<
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::PouringProcedureAttribute,
        >,
    >,
    crate::codegen::structs_codegen::tables::pouring_procedure_templates::PouringProcedureTemplate: web_common_traits::database::Read<
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
            let pouring_procedure_templates = crate::codegen::structs_codegen::tables::pouring_procedure_templates::PouringProcedureTemplate::read(
                procedure_template,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::PouringProcedureSettable>::procedure_template_poured_into_model(
                self,
                pouring_procedure_templates.procedure_template_poured_into_model,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::PouringProcedureSettable>::procedure_template_measured_with_model(
                self,
                pouring_procedure_templates.procedure_template_measured_with_model,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::PouringProcedureSettable>::procedure_template_poured_from_model(
                self,
                pouring_procedure_templates.procedure_template_poured_from_model,
            )?;
        }
        if let web_common_traits::database::IdOrBuilder::Id(procedure_poured_from) = self
            .procedure_poured_from
        {
            let procedure_assets = crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::read(
                procedure_poured_from,
                conn,
            )?;
            if let Some(asset) = procedure_assets.asset {
                self = <Self as crate::codegen::structs_codegen::tables::insertables::PouringProcedureSettable>::poured_from(
                    self,
                    asset,
                )?;
            }
            self = <Self as crate::codegen::structs_codegen::tables::insertables::PouringProcedureSettable>::procedure_template_poured_from_model(
                self,
                procedure_assets.procedure_template_asset_model,
            )?;
        }
        if let web_common_traits::database::IdOrBuilder::Id(procedure_measured_with) = self
            .procedure_measured_with
        {
            let procedure_assets = crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::read(
                procedure_measured_with,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::PouringProcedureSettable>::measured_with(
                self,
                procedure_assets.asset,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::PouringProcedureSettable>::procedure_template_measured_with_model(
                self,
                procedure_assets.procedure_template_asset_model,
            )?;
        }
        if let web_common_traits::database::IdOrBuilder::Id(procedure_poured_into) = self
            .procedure_poured_into
        {
            let procedure_assets = crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::read(
                procedure_poured_into,
                conn,
            )?;
            if let Some(asset) = procedure_assets.asset {
                self = <Self as crate::codegen::structs_codegen::tables::insertables::PouringProcedureSettable>::poured_into(
                    self,
                    asset,
                )?;
            }
            self = <Self as crate::codegen::structs_codegen::tables::insertables::PouringProcedureSettable>::procedure_template_poured_into_model(
                self,
                procedure_assets.procedure_template_asset_model,
            )?;
        }
        let procedure_template = self
            .procedure_template
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::PouringProcedureAttribute::ProcedureTemplate,
                ),
            )?;
        let poured_from = self
            .poured_from
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::PouringProcedureAttribute::PouredFrom,
                ),
            )?;
        let procedure_template_poured_from_model = self
            .procedure_template_poured_from_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::PouringProcedureAttribute::ProcedureTemplatePouredFromModel,
                ),
            )?;
        let procedure_template_measured_with_model = self
            .procedure_template_measured_with_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::PouringProcedureAttribute::ProcedureTemplateMeasuredWithModel,
                ),
            )?;
        let poured_into = self
            .poured_into
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::PouringProcedureAttribute::PouredInto,
                ),
            )?;
        let procedure_template_poured_into_model = self
            .procedure_template_poured_into_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::PouringProcedureAttribute::ProcedureTemplatePouredIntoModel,
                ),
            )?;
        let procedure = self
            .procedure
            .mint_primary_key(user_id, conn)
            .map_err(Self::Error::from_extension)?;
        let procedure_poured_from = match self.procedure_poured_from {
            web_common_traits::database::IdOrBuilder::Id(id) => id,
            web_common_traits::database::IdOrBuilder::Builder(
                mut procedure_poured_from,
            ) => {
                procedure_poured_from = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure(
                        procedure_poured_from,
                        procedure,
                    )
                    .map_err(|err| {
                        err.replace_field_name(
                            crate::codegen::structs_codegen::tables::insertables::PouringProcedureAttribute::ProcedurePouredFrom,
                        )
                    })?;
                procedure_poured_from
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.replace_field_name(
                            crate::codegen::structs_codegen::tables::insertables::PouringProcedureAttribute::ProcedurePouredFrom,
                        )
                    })?
            }
        };
        let procedure_measured_with = match self.procedure_measured_with {
            web_common_traits::database::IdOrBuilder::Id(id) => id,
            web_common_traits::database::IdOrBuilder::Builder(
                mut procedure_measured_with,
            ) => {
                procedure_measured_with = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure(
                        procedure_measured_with,
                        procedure,
                    )
                    .map_err(|err| {
                        err.replace_field_name(
                            crate::codegen::structs_codegen::tables::insertables::PouringProcedureAttribute::ProcedureMeasuredWith,
                        )
                    })?;
                procedure_measured_with
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.replace_field_name(
                            crate::codegen::structs_codegen::tables::insertables::PouringProcedureAttribute::ProcedureMeasuredWith,
                        )
                    })?
            }
        };
        let procedure_poured_into = match self.procedure_poured_into {
            web_common_traits::database::IdOrBuilder::Id(id) => id,
            web_common_traits::database::IdOrBuilder::Builder(
                mut procedure_poured_into,
            ) => {
                procedure_poured_into = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure(
                        procedure_poured_into,
                        procedure,
                    )
                    .map_err(|err| {
                        err.replace_field_name(
                            crate::codegen::structs_codegen::tables::insertables::PouringProcedureAttribute::ProcedurePouredInto,
                        )
                    })?;
                procedure_poured_into
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.replace_field_name(
                            crate::codegen::structs_codegen::tables::insertables::PouringProcedureAttribute::ProcedurePouredInto,
                        )
                    })?
            }
        };
        Ok(Self::InsertableVariant {
            procedure,
            procedure_template,
            poured_from,
            procedure_template_poured_from_model,
            procedure_poured_from,
            measured_with: self.measured_with,
            procedure_template_measured_with_model,
            procedure_measured_with,
            poured_into,
            procedure_template_poured_into_model,
            procedure_poured_into,
        })
    }
}
