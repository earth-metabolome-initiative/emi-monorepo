impl From<crate::codegen::structs_codegen::tables::storage_procedures::StorageProcedure>
    for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::storage_procedures::StorageProcedure,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::storage_procedures::StorageProcedure>>
    for super::Rows
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::storage_procedures::StorageProcedure>,
    ) -> Self {
        super::Rows::StorageProcedure(value)
    }
}
impl From<super::Rows>
    for Option<Vec<crate::codegen::structs_codegen::tables::storage_procedures::StorageProcedure>>
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::StorageProcedure(v) => Some(v),
            _ => None,
        }
    }
}
