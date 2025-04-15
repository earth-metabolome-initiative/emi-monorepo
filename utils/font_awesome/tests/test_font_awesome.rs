//! Small test suite for the `font_awesome` module.

use font_awesome::Icon;
use strum::IntoEnumIterator;

#[test]
fn test_icon_count() {
    let icons = Icon::iter().collect::<Vec<_>>();
    assert_eq!(icons.len(), 1465);
}

#[test]
fn test_write_to_file() {
    Icon::to_csv("icons.csv").unwrap();

    // We delete the file after the test
    std::fs::remove_file("icons.csv").unwrap();
}

#[test]
fn test_description_and_class() {
    let water = Icon::Water;
    assert_eq!(water.description(), "A water droplet, representing liquid or hydration.");
    assert_eq!(water.class(), "water");
}
