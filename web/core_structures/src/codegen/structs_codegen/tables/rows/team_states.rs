impl From<crate::codegen::structs_codegen::tables::team_states::TeamState> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::team_states::TeamState) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::team_states::TeamState>> for super::Rows {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::team_states::TeamState>) -> Self {
        super::Rows::TeamState(value)
    }
}
impl From<super::Rows>
    for Option<Vec<crate::codegen::structs_codegen::tables::team_states::TeamState>>
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::TeamState(v) => Some(v),
            _ => None,
        }
    }
}
