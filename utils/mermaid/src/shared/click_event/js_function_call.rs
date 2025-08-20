//! Submodule handling click events in Mermaid nodes which lead to calling
//! JavaScript functions, including the typing of the function signature
//! and the function call itself.

use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Represents a JavaScript function call that can be triggered by a click event
/// on a Mermaid node.
pub struct JsFunctionCall {
    /// The name of the JavaScript function to call.
    function_name: String,
    /// The arguments to pass to the JavaScript function.
    args: Vec<String>,
}

impl Display for JsFunctionCall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let args_str = if self.args.is_empty() {
            String::new()
        } else {
            format!("({})", self.args.join(", "))
        };
        write!(f, "{}{}", self.function_name, args_str)
    }
}
