//! Submodule testing the Intersection struct and the IntersectionIter trait.
use iter_utils::prelude::*;

#[test]
fn test_intersection_iter() {
    let left = vec![1, 2, 3, 4, 5];
    let right = vec![3, 4, 5, 6, 7];
    let intersection: Vec<_> = left.iter().sorted_intersection(right.iter()).collect();
    assert_eq!(intersection, vec![&3, &4, &5]);
}

#[test]
fn test_intersection_iter_empty() {
    let left: Vec<i32> = vec![];
    let right: Vec<i32> = vec![];
    let intersection: Vec<i32> =
        left.iter().copied().sorted_intersection(right.iter().copied()).collect();
    assert_eq!(intersection, vec![]);

    let filled_left: Vec<i32> = vec![1, 2, 3];
    let intersection: Vec<_> =
        filled_left.iter().copied().sorted_intersection(right.iter().copied()).collect();
    assert_eq!(intersection, vec![]);

    let filled_right: Vec<i32> = vec![1, 2, 3];
    let intersection: Vec<_> =
        left.iter().copied().sorted_intersection(filled_right.iter().copied()).collect();
    assert_eq!(intersection, vec![]);
}
