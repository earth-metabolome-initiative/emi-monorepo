impl<Procedure> web_common_traits::database::DispatchableInsertVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureBuilder<
        Procedure,
    >
{
    type Row =
        crate::codegen::structs_codegen::tables::fractioning_procedures::FractioningProcedure;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::FractioningProcedureAttribute,
    >;
}
impl<Procedure> web_common_traits::database::InsertableVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureBuilder<
        Procedure,
    >
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedure;
}
#[cfg(feature = "backend")]
impl<Procedure> web_common_traits::database::BackendInsertableVariant
    for crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureBuilder<
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
for crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureBuilder<
    Procedure,
>
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::fractioning_procedures::FractioningProcedure as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedure as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::fractioning_procedures::FractioningProcedure as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::fractioning_procedures::FractioningProcedure,
    >,
    Self: web_common_traits::database::InsertableVariant<
        C,
        InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedure,
        Row = crate::codegen::structs_codegen::tables::fractioning_procedures::FractioningProcedure,
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::FractioningProcedureAttribute,
        >,
    >,
    Procedure: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = ::rosetta_uuid::Uuid,
    >,
    Self: crate::codegen::structs_codegen::tables::insertables::FractioningProcedureSettable<
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::FractioningProcedureAttribute,
        >,
    >,
    crate::codegen::structs_codegen::tables::fractioning_procedure_templates::FractioningProcedureTemplate: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder: web_common_traits::database::TryInsertGeneric<
        C,
        Attribute = crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute,
        PrimaryKey = ::rosetta_uuid::Uuid,
    >,
    crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset: web_common_traits::database::Read<
        C,
    >,
    Self: web_common_traits::database::MostConcreteTable,
    crate::codegen::structs_codegen::tables::insertables::FractioningProcedureExtensionAttribute: From<
        <Procedure as common_traits::builder::Attributed>::Attribute,
    >,
{
    fn insert(mut self, user_id: i32, conn: &mut C) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::InsertableVariant;
        use web_common_traits::database::MostConcreteTable;
        self.set_most_concrete_table("fractioning_procedures");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedure = self
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
for crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureBuilder<
    Procedure,
>
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::fractioning_procedures::FractioningProcedure as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedure as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::fractioning_procedures::FractioningProcedure as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::fractioning_procedures::FractioningProcedure,
    >,
    Procedure: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = ::rosetta_uuid::Uuid,
    >,
    Self: crate::codegen::structs_codegen::tables::insertables::FractioningProcedureSettable<
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::FractioningProcedureAttribute,
        >,
    >,
    crate::codegen::structs_codegen::tables::fractioning_procedure_templates::FractioningProcedureTemplate: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder: web_common_traits::database::TryInsertGeneric<
        C,
        Attribute = crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute,
        PrimaryKey = ::rosetta_uuid::Uuid,
    >,
    crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset: web_common_traits::database::Read<
        C,
    >,
    Self: web_common_traits::database::MostConcreteTable,
    crate::codegen::structs_codegen::tables::insertables::FractioningProcedureExtensionAttribute: From<
        <Procedure as common_traits::builder::Attributed>::Attribute,
    >,
{
    fn try_insert(
        mut self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::InsertableVariant, Self::Error> {
        use web_common_traits::database::TryInsertGeneric;
        use web_common_traits::database::Read;
        if let Some(procedure_template) = self.procedure_template {
            let fractioning_procedure_templates = crate::codegen::structs_codegen::tables::fractioning_procedure_templates::FractioningProcedureTemplate::read(
                procedure_template,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::FractioningProcedureSettable>::procedure_template_fragment_container_model(
                self,
                fractioning_procedure_templates
                    .procedure_template_fragment_container_model,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::FractioningProcedureSettable>::procedure_template_fragment_placed_into_model(
                self,
                fractioning_procedure_templates
                    .procedure_template_fragment_placed_into_model,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::FractioningProcedureSettable>::procedure_template_weighed_with_model(
                self,
                fractioning_procedure_templates.procedure_template_weighed_with_model,
            )?;
        }
        if let web_common_traits::database::IdOrBuilder::Id(
            procedure_fragment_container,
        ) = self.procedure_fragment_container
        {
            let procedure_assets = crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::read(
                procedure_fragment_container,
                conn,
            )?;
            if let Some(asset) = procedure_assets.asset {
                self = <Self as crate::codegen::structs_codegen::tables::insertables::FractioningProcedureSettable>::fragment_container(
                    self,
                    asset,
                )?;
            }
            self = <Self as crate::codegen::structs_codegen::tables::insertables::FractioningProcedureSettable>::procedure_template_fragment_container_model(
                self,
                procedure_assets.procedure_template_asset_model,
            )?;
        }
        if let web_common_traits::database::IdOrBuilder::Id(
            procedure_fragment_placed_into,
        ) = self.procedure_fragment_placed_into
        {
            let procedure_assets = crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::read(
                procedure_fragment_placed_into,
                conn,
            )?;
            if let Some(asset) = procedure_assets.asset {
                self = <Self as crate::codegen::structs_codegen::tables::insertables::FractioningProcedureSettable>::fragment_placed_into(
                    self,
                    asset,
                )?;
            }
            self = <Self as crate::codegen::structs_codegen::tables::insertables::FractioningProcedureSettable>::procedure_template_fragment_placed_into_model(
                self,
                procedure_assets.procedure_template_asset_model,
            )?;
        }
        if let web_common_traits::database::IdOrBuilder::Id(procedure_weighed_with) = self
            .procedure_weighed_with
        {
            let procedure_assets = crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::read(
                procedure_weighed_with,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::FractioningProcedureSettable>::procedure_template_weighed_with_model(
                self,
                procedure_assets.procedure_template_asset_model,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::FractioningProcedureSettable>::weighed_with(
                self,
                procedure_assets.asset,
            )?;
        }
        let procedure_template = self
            .procedure_template
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::FractioningProcedureAttribute::ProcedureTemplate,
                ),
            )?;
        let fragment_container = self
            .fragment_container
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::FractioningProcedureAttribute::FragmentContainer,
                ),
            )?;
        let procedure_template_fragment_container_model = self
            .procedure_template_fragment_container_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::FractioningProcedureAttribute::ProcedureTemplateFragmentContainerModel,
                ),
            )?;
        let fragment_placed_into = self
            .fragment_placed_into
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::FractioningProcedureAttribute::FragmentPlacedInto,
                ),
            )?;
        let procedure_template_fragment_placed_into_model = self
            .procedure_template_fragment_placed_into_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::FractioningProcedureAttribute::ProcedureTemplateFragmentPlacedIntoModel,
                ),
            )?;
        let kilograms = self
            .kilograms
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::FractioningProcedureAttribute::Kilograms,
                ),
            )?;
        let procedure_template_weighed_with_model = self
            .procedure_template_weighed_with_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::FractioningProcedureAttribute::ProcedureTemplateWeighedWithModel,
                ),
            )?;
        let procedure = self
            .procedure
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(|attribute| {
                    crate::codegen::structs_codegen::tables::insertables::FractioningProcedureAttribute::Extension(
                        From::from(attribute),
                    )
                })
            })?;
        let procedure_fragment_container = match self.procedure_fragment_container {
            web_common_traits::database::IdOrBuilder::Id(id) => id,
            web_common_traits::database::IdOrBuilder::Builder(
                mut procedure_fragment_container,
            ) => {
                procedure_fragment_container = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure(
                        procedure_fragment_container,
                        procedure,
                    )
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::FractioningProcedureAttribute::ProcedureFragmentContainer,
                        )
                    })?;
                procedure_fragment_container
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::FractioningProcedureAttribute::ProcedureFragmentContainer,
                        )
                    })?
            }
        };
        let procedure_fragment_placed_into = match self.procedure_fragment_placed_into {
            web_common_traits::database::IdOrBuilder::Id(id) => id,
            web_common_traits::database::IdOrBuilder::Builder(
                mut procedure_fragment_placed_into,
            ) => {
                procedure_fragment_placed_into = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure(
                        procedure_fragment_placed_into,
                        procedure,
                    )
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::FractioningProcedureAttribute::ProcedureFragmentPlacedInto,
                        )
                    })?;
                procedure_fragment_placed_into
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::FractioningProcedureAttribute::ProcedureFragmentPlacedInto,
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
                            crate::codegen::structs_codegen::tables::insertables::FractioningProcedureAttribute::ProcedureWeighedWith,
                        )
                    })?;
                procedure_weighed_with
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::FractioningProcedureAttribute::ProcedureWeighedWith,
                        )
                    })?
            }
        };
        Ok(Self::InsertableVariant {
            procedure,
            procedure_template,
            fragment_container,
            procedure_template_fragment_container_model,
            procedure_fragment_container,
            fragment_placed_into,
            procedure_template_fragment_placed_into_model,
            procedure_fragment_placed_into,
            kilograms,
            weighed_with: self.weighed_with,
            procedure_template_weighed_with_model,
            procedure_weighed_with,
        })
    }
}
