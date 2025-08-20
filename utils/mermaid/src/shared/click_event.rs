//! Enumeration of events associated with a click action on a node in a Mermaid
//! diagram.

mod navigation;
use std::fmt::Display;

pub use navigation::Navigation;
mod js_function_call;
pub use js_function_call::JsFunctionCall;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ClickEvent {
    /// Represents a click event that triggers a navigation event,
    /// which can be a external link with or without opening in a new tab,
    /// and that can either be triggered via a real anchor link or a JavaScript
    /// function.
    Navigation(Navigation),
    /// Represents a click event that triggers a JavaScript function call.
    JsFunctionCall(JsFunctionCall),
}

impl Display for ClickEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClickEvent::Navigation(nav) => write!(f, "{nav}",),
            ClickEvent::JsFunctionCall(js_call) => write!(f, "{js_call}",),
        }
    }
}
