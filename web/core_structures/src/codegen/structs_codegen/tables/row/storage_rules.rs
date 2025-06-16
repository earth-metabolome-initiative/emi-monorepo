impl From<crate::codegen::structs_codegen::tables::storage_rules::StorageRule> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::storage_rules::StorageRule) -> Self {
        super::Row::StorageRule(value)
    }
}
impl TryFrom<super::Row> for crate::codegen::structs_codegen::tables::storage_rules::StorageRule {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::StorageRule(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
