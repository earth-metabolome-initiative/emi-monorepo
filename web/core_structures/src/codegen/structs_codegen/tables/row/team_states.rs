impl From<crate::codegen::structs_codegen::tables::team_states::TeamState> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::team_states::TeamState) -> Self {
        super::Row::TeamState(value)
    }
}
impl From<super::Row> for Option<crate::codegen::structs_codegen::tables::team_states::TeamState> {
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::TeamState(v) => Some(v),
            _ => None,
        }
    }
}
