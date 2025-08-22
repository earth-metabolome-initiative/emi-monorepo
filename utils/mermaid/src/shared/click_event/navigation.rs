//! Submodule handling navigation click events in Mermaid diagrams, which
//! can include anchor-like links or JavaScript function calls navigating to
//! external resources or internal sections, with or without opening in a new
//! tab.

use std::fmt::Display;

/// Represents a navigation event triggered by a click on a node in a Mermaid
/// diagram. This can include external links, with options for opening in a new
/// tab and whether to use an anchor-like link or a JavaScript function for
/// navigation.
///
/// # Example
///
/// Some example of mermaid syntax for a navigation event:
///
/// ```mermaid
/// click A "https://www.github.com" _blank
/// click B "https://www.github.com" "Open this in a new tab" _blank
/// click C href "https://www.github.com" _blank
/// click D href "https://www.github.com" "Open this in a new tab" _blank
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Navigation {
    /// The URL to navigate to when the node is clicked.
    url: String,
    /// Whether to open the link in a new tab.
    new_tab: bool,
    /// Whether to employ an anchor-like link or a JavaScript function for
    /// navigation.
    anchor: bool,
    /// Descriptive tooltip for the navigation link.
    tooltip: Option<String>,
}

impl Display for Navigation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // We omit the `click {node_name}` part as it is not relevant for the
        // display of the navigation event, and is handled by the parent
        // `ClickEvent` enum.
        if self.anchor {
            write!(f, "href")?;
        }
        write!(f, " \"{}\"", self.url)?;

        if let Some(tooltip) = &self.tooltip {
            write!(f, " \"{tooltip}\"")?;
        }

        if self.new_tab {
            write!(f, " _blank")?;
        }

        Ok(())
    }
}
