//! Submodule providing an iterator over the digits of a `CAS` number.

/// Struct iterating over the digits of a `CAS` number.
pub struct Digits {
    value: u32,
}

impl From<crate::CAS> for Digits {
    /// Creates a new `Digits` iterator from a `CAS` number.
    fn from(cas: crate::CAS) -> Self {
        Self { value: cas.into() }
    }
}

impl Iterator for Digits {
    type Item = u8;

    /// Returns the next digit of the `CAS` number.
    fn next(&mut self) -> Option<Self::Item> {
        if self.value == 0 {
            return None;
        }

        let digit = u8::try_from(self.value % 10).unwrap();
        self.value /= 10;
        Some(digit)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_digits() {
        let cas = crate::CAS(7732_18_5);
        let digits: Vec<u8> = cas.digits().collect();
        assert_eq!(digits, vec![5, 8, 1, 2, 3, 7, 7]);
    }
}
