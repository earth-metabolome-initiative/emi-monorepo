/// Returns the name of the parameter set
pub trait NamedParametersSet {
    fn parameter_set_name() -> &'static str;
}