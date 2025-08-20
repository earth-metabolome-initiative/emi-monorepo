//! Submodule defining the `Color` struct for Mermaid diagrams.

/// Represents a color in the Mermaid diagram style class.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Color {
    /// Red component of the color (0-255).
    red: u8,
    /// Green component of the color (0-255).
    green: u8,
    /// Blue component of the color (0-255).
    blue: u8,
}

impl Color {
    /// Returns the color as a hexadecimal string.
    pub fn to_hex(self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.red, self.green, self.blue)
    }
}
