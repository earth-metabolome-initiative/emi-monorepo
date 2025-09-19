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

impl From<(u8, u8, u8)> for Color {
    fn from(rgb: (u8, u8, u8)) -> Self {
        Color { red: rgb.0, green: rgb.1, blue: rgb.2 }
    }
}

impl From<Rgb> for Color {
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    fn from(rgb: Rgb) -> Self {
        Color { red: rgb.red() as u8, green: rgb.green() as u8, blue: rgb.blue() as u8 }
    }
}

impl From<Hsl> for Color {
    fn from(hsl: Hsl) -> Self {
        let rgb = Rgb::from(hsl);
        Color::from(rgb)
    }
}

impl From<Color> for Rgb {
    fn from(color: Color) -> Self {
        Rgb::new(f64::from(color.red), f64::from(color.green), f64::from(color.blue), None)
    }
}

impl From<Color> for Hsl {
    fn from(color: Color) -> Self {
        let rgb: Rgb = color.into();
        Hsl::from(rgb)
    }
}

impl Color {
    #[must_use]
    /// Returns a new pastel red color.
    pub fn pastel_red() -> Self {
        Color { red: 240, green: 116, blue: 108 }
    }

    #[must_use]
    /// Returns a new pastel blue color.
    pub fn pastel_blue() -> Self {
        Color { red: 108, green: 116, blue: 240 }
    }

    #[must_use]
    /// Returns a new pastel cyan color.
    pub fn pastel_cyan() -> Self {
        Color { red: 167, green: 239, blue: 240 }
    }

    #[must_use]
    /// Returns the color as a hexadecimal string.
    pub fn to_hex(self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.red, self.green, self.blue)
    }

    #[must_use]
    /// Returns `n` maximally distinct colors.
    ///
    /// # Arguments
    ///
    /// * `n` - The number of distinct colors to generate.
    /// * `saturation` - The saturation of the colors (0-100).
    /// * `lightness` - The lightness of the colors (0-100).
    pub fn maximally_distinct(n: u16, saturation: u8, lightness: u8) -> Vec<Color> {
        let mut colors = Vec::with_capacity(usize::from(n));
        let saturation = f64::from(saturation);
        let lightness = f64::from(lightness);
        for i in 0..n {
            let hue = f64::from(i) * 360.0 / f64::from(n);
            let hsl = Hsl::new(hue, saturation, lightness, None);
            colors.push(hsl.into());
        }
        colors
    }

    #[must_use]
    /// Returns the color darkened by the provided amount (0-100).
    pub fn darken(self, amount: u8) -> Color {
        let hsl: Hsl = self.into();
        let new_lightness = (hsl.lightness() - f64::from(amount)).max(0.0);
        let new_hsl = Hsl::new(hsl.hue(), hsl.saturation(), new_lightness, None);
        new_hsl.into()
    }

    #[must_use]
    /// Returns the color lightened by the provided amount (0-100).
    pub fn lighten(self, amount: u8) -> Color {
        let hsl: Hsl = self.into();
        let new_lightness = (hsl.lightness() + f64::from(amount)).min(100.0);
        let new_hsl = Hsl::new(hsl.hue(), hsl.saturation(), new_lightness, None);
        new_hsl.into()
    }
}
