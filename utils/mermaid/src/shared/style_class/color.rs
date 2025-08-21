//! Submodule defining the `Color` struct for Mermaid diagrams.

use colorsys::{Hsl, Rgb};

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

    /// Returns `n` maximally distinct colors.
    pub fn maximally_distinct(n: usize) -> Vec<Color> {
        let mut colors = Vec::with_capacity(n);
        for i in 0..n {
            let hue = (i as f64 / n as f64) * 360.0;
            let saturation = 70.0;
            let lightness = 80.0;
            let hsl = Hsl::new(hue, saturation, lightness, None);
            let rgb = Rgb::from(hsl);
            colors.push(Color {
                red: rgb.red() as u8,
                green: rgb.green() as u8,
                blue: rgb.blue() as u8,
            });
        }
        colors
    }
}
