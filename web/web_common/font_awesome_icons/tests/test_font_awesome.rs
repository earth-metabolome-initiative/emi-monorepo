//! Small test suite for the `font_awesome` module.

use font_awesome_icons::FAIcon;
use strum::IntoEnumIterator;

#[test]
fn test_icon_count() {
    let icons = FAIcon::iter().collect::<Vec<_>>();
    assert_eq!(icons.len(), 1465);
}

#[test]
fn test_description_and_class() {
    let water = FAIcon::Water;
    assert_eq!(water.description(), "A water droplet, representing liquid or hydration.");
    assert_eq!(water.class(), "water");
}
