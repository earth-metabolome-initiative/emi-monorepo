impl web_common_traits::database::DispatchableInsertVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableAddressBuilder
{
    type Row = crate::codegen::structs_codegen::tables::addresses::Address;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::AddressAttribute,
    >;
}
impl web_common_traits::database::InsertableVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableAddressBuilder
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableAddress;
}
#[cfg(feature = "backend")]
impl web_common_traits::database::BackendInsertableVariant
    for crate::codegen::structs_codegen::tables::insertables::InsertableAddressBuilder
where
    Self: web_common_traits::database::DispatchableInsertableVariant<diesel::PgConnection>,
{
}
impl<
    C: diesel::connection::LoadConnection,
> web_common_traits::database::DispatchableInsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableAddressBuilder
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::addresses::Address as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableAddress as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::addresses::Address as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::addresses::Address,
    >,
    Self: web_common_traits::database::InsertableVariant<
        C,
        InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableAddress,
        Row = crate::codegen::structs_codegen::tables::addresses::Address,
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::AddressAttribute,
        >,
    >,
{
    fn insert(self, user_id: i32, conn: &mut C) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::InsertableVariant;
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableAddress = self
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
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableAddressBuilder
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::addresses::Address as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableAddress as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::addresses::Address as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::addresses::Address,
    >,
{
    fn try_insert(
        self,
        _user_id: i32,
        _conn: &mut C,
    ) -> Result<Self::InsertableVariant, Self::Error> {
        let city_id = self
            .city_id
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::AddressAttribute::CityId,
                ),
            )?;
        let street_name = self
            .street_name
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::AddressAttribute::StreetName,
                ),
            )?;
        let house_number = self
            .house_number
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::AddressAttribute::HouseNumber,
                ),
            )?;
        let postal_code = self
            .postal_code
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::AddressAttribute::PostalCode,
                ),
            )?;
        let geolocation = self
            .geolocation
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::AddressAttribute::Geolocation,
                ),
            )?;
        Ok(Self::InsertableVariant {
            city_id,
            street_name,
            house_number,
            postal_code,
            geolocation,
        })
    }
}
