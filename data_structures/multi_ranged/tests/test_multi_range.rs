//! Test submodule to validate the multi-range data structure

use multi_ranged::{MultiRange, MultiRanged};

#[test]
fn test_multi_range_creation() {
    let mut range = MultiRange::try_from([-3, -1, 0, 1, 2, 3, 4, 5, 6, 7]).unwrap();
    range.insert(-2).unwrap();

    assert_eq!(range.absolute_start(), Some(-3), "Range: {range:?}");
    assert_eq!(range.absolute_end(), Some(8), "Range: {range:?}");
    assert!(range.is_dense());
}
