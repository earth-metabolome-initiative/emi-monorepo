# Multi Ranged

Efficient data structures for representing and manipulating ranges of discrete values. The crate provides three range types with a unified `MultiRanged` trait: `SimpleRange` for contiguous ranges similar to Rust's [`std::ops::Range`](https://doc.rust-lang.org/std/ops/struct.Range.html) but with stable semantics, `BiRange` for ranges split into two parts, and `MultiRange` for arbitrary collections of disjoint ranges. All types support incremental insertion, merging, and efficient iteration over their elements. The `Step` trait abstracts over numeric types that can be used as range boundaries, providing operations for stepping forward and backward with [saturating arithmetic](https://en.wikipedia.org/wiki/Saturation_arithmetic).

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
multi_ranged = "0.1"
```

## Examples

### Simple Range

A contiguous range from start to end:

```rust
use multi_ranged::{SimpleRange, MultiRanged};

let mut range = SimpleRange::try_from((0, 10)).unwrap();
assert_eq!(range.len(), 10);
assert!(range.contains(5));
assert!(!range.contains(15));

// Extend the range
range.insert(10).unwrap();
assert_eq!(range.len(), 11);
```

### Multi Range

Multiple disjoint ranges that can be built incrementally:

```rust
use multi_ranged::{MultiRange, MultiRanged};

let mut range = MultiRange::try_from([1, 2, 3, 10, 11, 12]).unwrap();
assert!(!range.is_dense()); // Multiple separate ranges

// Insert values that bridge the gap
range.insert(4).unwrap();
range.insert(5).unwrap();
range.insert(6).unwrap();
range.insert(7).unwrap();
range.insert(8).unwrap();
range.insert(9).unwrap();

assert!(range.is_dense()); // Now a single contiguous range
assert_eq!(range.absolute_start(), Some(1));
assert_eq!(range.absolute_end(), Some(13));
```

## Trait Overview

The `MultiRanged` trait provides a common interface for all range types with methods for insertion, merging, containment checking, and iteration. The `Step` trait enables generic range operations over any numeric type supporting saturating arithmetic and ordering.
