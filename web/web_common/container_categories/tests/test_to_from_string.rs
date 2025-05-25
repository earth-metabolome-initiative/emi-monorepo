//! Test submodule to verify that the `to_string` and `from_str` methods
//! work as expected for the enumeration.

use std::str::FromStr;

use container_categories::ContainerCategory;
use strum::IntoEnumIterator;

#[test]
fn test_to_from_string() {
    for category in ContainerCategory::iter() {
        // Convert the enum variant to a string
        let category_str = category.to_string();

        // Convert the string back to the enum variant
        let parsed_category = ContainerCategory::from_str(&category_str)
            .expect("Failed to parse ContainerCategory from string");

        // Assert that the original and parsed categories are the same
        assert_eq!(category, parsed_category, "Mismatch for category: {}", category_str);
    }
}
