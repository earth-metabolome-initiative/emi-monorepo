impl<Procedure> web_common_traits::database::DispatchableInsertVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableGeolocationProcedureBuilder<
        Procedure,
    >
{
    type Row =
        crate::codegen::structs_codegen::tables::geolocation_procedures::GeolocationProcedure;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::GeolocationProcedureAttribute,
    >;
}
impl<Procedure> web_common_traits::database::InsertableVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableGeolocationProcedureBuilder<
        Procedure,
    >
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableGeolocationProcedure;
}
#[cfg(feature = "backend")]
impl<Procedure> web_common_traits::database::BackendInsertableVariant
    for crate::codegen::structs_codegen::tables::insertables::InsertableGeolocationProcedureBuilder<
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
for crate::codegen::structs_codegen::tables::insertables::InsertableGeolocationProcedureBuilder<
    Procedure,
>
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::geolocation_procedures::GeolocationProcedure as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableGeolocationProcedure as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::geolocation_procedures::GeolocationProcedure as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::geolocation_procedures::GeolocationProcedure,
    >,
    Self: web_common_traits::database::InsertableVariant<
        C,
        InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableGeolocationProcedure,
        Row = crate::codegen::structs_codegen::tables::geolocation_procedures::GeolocationProcedure,
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::GeolocationProcedureAttribute,
        >,
    >,
    Self: web_common_traits::database::MostConcreteTable,
{
    fn insert(mut self, user_id: i32, conn: &mut C) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::InsertableVariant;
        use web_common_traits::database::MostConcreteTable;
        self.set_most_concrete_table("geolocation_procedures");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableGeolocationProcedure = self
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
for crate::codegen::structs_codegen::tables::insertables::InsertableGeolocationProcedureBuilder<
    Procedure,
>
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::geolocation_procedures::GeolocationProcedure as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableGeolocationProcedure as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::geolocation_procedures::GeolocationProcedure as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::geolocation_procedures::GeolocationProcedure,
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
    Self: crate::codegen::structs_codegen::tables::insertables::GeolocationProcedureSettable<
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::GeolocationProcedureAttribute,
        >,
    >,
    crate::codegen::structs_codegen::tables::geolocation_procedure_templates::GeolocationProcedureTemplate: web_common_traits::database::Read<
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
            let geolocation_procedure_templates = crate::codegen::structs_codegen::tables::geolocation_procedure_templates::GeolocationProcedureTemplate::read(
                procedure_template,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::GeolocationProcedureSettable>::procedure_template_geolocated_asset_model(
                self,
                geolocation_procedure_templates.procedure_template_geolocated_asset_model,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::GeolocationProcedureSettable>::procedure_template_geolocated_with_model(
                self,
                geolocation_procedure_templates.procedure_template_geolocated_with_model,
            )?;
        }
        if let web_common_traits::database::IdOrBuilder::Id(
            procedure_geolocated_asset,
        ) = self.procedure_geolocated_asset
        {
            let procedure_assets = crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::read(
                procedure_geolocated_asset,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::GeolocationProcedureSettable>::geolocated_asset(
                self,
                procedure_assets.asset,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::GeolocationProcedureSettable>::procedure_template_geolocated_asset_model(
                self,
                procedure_assets.procedure_template_asset_model,
            )?;
        }
        if let web_common_traits::database::IdOrBuilder::Id(procedure_geolocated_with) = self
            .procedure_geolocated_with
        {
            let procedure_assets = crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::read(
                procedure_geolocated_with,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::GeolocationProcedureSettable>::geolocated_with(
                self,
                procedure_assets.asset,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::GeolocationProcedureSettable>::procedure_template_geolocated_with_model(
                self,
                procedure_assets.procedure_template_asset_model,
            )?;
        }
        let procedure_template = self
            .procedure_template
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::GeolocationProcedureAttribute::ProcedureTemplate,
                ),
            )?;
        let procedure_template_geolocated_asset_model = self
            .procedure_template_geolocated_asset_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::GeolocationProcedureAttribute::ProcedureTemplateGeolocatedAssetModel,
                ),
            )?;
        let procedure_template_geolocated_with_model = self
            .procedure_template_geolocated_with_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::GeolocationProcedureAttribute::ProcedureTemplateGeolocatedWithModel,
                ),
            )?;
        let location = self
            .location
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::GeolocationProcedureAttribute::Location,
                ),
            )?;
        let procedure = self
            .procedure
            .mint_primary_key(user_id, conn)
            .map_err(Self::Error::from_extension)?;
        let procedure_geolocated_asset = match self.procedure_geolocated_asset {
            web_common_traits::database::IdOrBuilder::Id(id) => id,
            web_common_traits::database::IdOrBuilder::Builder(
                mut procedure_geolocated_asset,
            ) => {
                procedure_geolocated_asset = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure(
                        procedure_geolocated_asset,
                        procedure,
                    )
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::GeolocationProcedureAttribute::ProcedureGeolocatedAsset,
                        )
                    })?;
                procedure_geolocated_asset
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::GeolocationProcedureAttribute::ProcedureGeolocatedAsset,
                        )
                    })?
            }
        };
        let procedure_geolocated_with = match self.procedure_geolocated_with {
            web_common_traits::database::IdOrBuilder::Id(id) => id,
            web_common_traits::database::IdOrBuilder::Builder(
                mut procedure_geolocated_with,
            ) => {
                procedure_geolocated_with = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure(
                        procedure_geolocated_with,
                        procedure,
                    )
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::GeolocationProcedureAttribute::ProcedureGeolocatedWith,
                        )
                    })?;
                procedure_geolocated_with
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::GeolocationProcedureAttribute::ProcedureGeolocatedWith,
                        )
                    })?
            }
        };
        Ok(Self::InsertableVariant {
            procedure,
            procedure_template,
            geolocated_asset: self.geolocated_asset,
            procedure_template_geolocated_asset_model,
            procedure_geolocated_asset,
            geolocated_with: self.geolocated_with,
            procedure_geolocated_with,
            procedure_template_geolocated_with_model,
            location,
        })
    }
}
