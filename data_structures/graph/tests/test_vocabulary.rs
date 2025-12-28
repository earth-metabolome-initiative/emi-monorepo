//! Simple test for vocabulary.

use ::graph::prelude::*;
use sorted_vec::prelude::SortedVec;

#[test]
/// First simple test for vocabulary.
pub fn test_vocabulary() {
    let nodes: Vec<usize> = vec![1, 2, 3, 4, 5];
    let nodes: SortedVec<usize> = GenericVocabularyBuilder::default()
        .expected_number_of_symbols(nodes.len())
        .symbols(nodes.into_iter().enumerate())
        .build()
        .unwrap();
    assert_eq!(nodes.len(), 5);
    assert_eq!(nodes.get(0), Some(&1));
    assert_eq!(nodes.get(1), Some(&2));
    assert_eq!(nodes.get(2), Some(&3));
    assert_eq!(nodes.get(3), Some(&4));
    assert_eq!(nodes.get(4), Some(&5));
    assert_eq!(nodes.get(5), None);
    assert_eq!(nodes.get(6), None);
    assert_eq!(nodes.get(7), None);
    assert_eq!(nodes.sources().collect::<Vec<usize>>(), vec![0, 1, 2, 3, 4]);
    assert_eq!(nodes.destinations().collect::<Vec<usize>>(), vec![1, 2, 3, 4, 5]);
}
