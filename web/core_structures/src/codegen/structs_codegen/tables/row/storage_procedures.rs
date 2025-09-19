impl From<crate::codegen::structs_codegen::tables::storage_procedures::StorageProcedure>
    for super::Row
{
    fn from(
        value: crate::codegen::structs_codegen::tables::storage_procedures::StorageProcedure,
    ) -> Self {
        super::Row::StorageProcedure(value)
    }
}
impl From<super::Row>
    for Option<crate::codegen::structs_codegen::tables::storage_procedures::StorageProcedure>
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::StorageProcedure(v) => Some(v),
            _ => None,
        }
    }
}
