//! Submodule providing traits for several common numerical constants.

/// Trait for the number zero.
pub trait Zero {
    /// The zero value.
    const ZERO: Self;
}
/// Trait for the number one.
pub trait One {
    /// The one value.
    const ONE: Self;
}

impl Zero for i8 {
    const ZERO: Self = 0;
}
impl Zero for i16 {
    const ZERO: Self = 0;
}
impl Zero for i32 {
    const ZERO: Self = 0;
}
impl Zero for i64 {
    const ZERO: Self = 0;
}
impl Zero for i128 {
    const ZERO: Self = 0;
}
impl Zero for isize {
    const ZERO: Self = 0;
}
impl Zero for u8 {
    const ZERO: Self = 0;
}
impl Zero for u16 {
    const ZERO: Self = 0;
}
impl Zero for u32 {
    const ZERO: Self = 0;
}
impl Zero for u64 {
    const ZERO: Self = 0;
}
impl Zero for u128 {
    const ZERO: Self = 0;
}
impl Zero for usize {
    const ZERO: Self = 0;
}
impl Zero for f32 {
    const ZERO: Self = 0.0;
}
impl Zero for f64 {
    const ZERO: Self = 0.0;
}

impl One for i8 {
    const ONE: Self = 1;
}
impl One for i16 {
    const ONE: Self = 1;
}
impl One for i32 {
    const ONE: Self = 1;
}
impl One for i64 {
    const ONE: Self = 1;
}
impl One for i128 {
    const ONE: Self = 1;
}
impl One for isize {
    const ONE: Self = 1;
}
impl One for u8 {
    const ONE: Self = 1;
}
impl One for u16 {
    const ONE: Self = 1;
}
impl One for u32 {
    const ONE: Self = 1;
}
impl One for u64 {
    const ONE: Self = 1;
}
impl One for u128 {
    const ONE: Self = 1;
}
impl One for usize {
    const ONE: Self = 1;
}
impl One for f32 {
    const ONE: Self = 1.0;
}
impl One for f64 {
    const ONE: Self = 1.0;
}

/// Trait representing a bounded value.
pub trait Bounded {
    /// The minimum value.
    const MIN: Self;
    /// The maximum value.
    const MAX: Self;
}

impl Bounded for i8 {
    const MIN: Self = i8::MIN;
    const MAX: Self = i8::MAX;
}

impl Bounded for i16 {
    const MIN: Self = i16::MIN;
    const MAX: Self = i16::MAX;
}

impl Bounded for i32 {
    const MIN: Self = i32::MIN;
    const MAX: Self = i32::MAX;
}

impl Bounded for i64 {
    const MIN: Self = i64::MIN;
    const MAX: Self = i64::MAX;
}

impl Bounded for i128 {
    const MIN: Self = i128::MIN;
    const MAX: Self = i128::MAX;
}

impl Bounded for isize {
    const MIN: Self = isize::MIN;
    const MAX: Self = isize::MAX;
}

impl Bounded for u8 {
    const MIN: Self = u8::MIN;
    const MAX: Self = u8::MAX;
}

impl Bounded for u16 {
    const MIN: Self = u16::MIN;
    const MAX: Self = u16::MAX;
}

impl Bounded for u32 {
    const MIN: Self = u32::MIN;
    const MAX: Self = u32::MAX;
}

impl Bounded for u64 {
    const MIN: Self = u64::MIN;
    const MAX: Self = u64::MAX;
}

impl Bounded for u128 {
    const MIN: Self = u128::MIN;
    const MAX: Self = u128::MAX;
}

impl Bounded for usize {
    const MIN: Self = usize::MIN;
    const MAX: Self = usize::MAX;
}

impl Bounded for f32 {
    const MIN: Self = f32::MIN;
    const MAX: Self = f32::MAX;
}

impl Bounded for f64 {
    const MIN: Self = f64::MIN;
    const MAX: Self = f64::MAX;
}
