#![doc = include_str!("../README.md")]

#[cfg(feature = "diesel")]
mod diesel_impls;
#[cfg(feature = "pgrx")]
mod pgrx_impls;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// A wrapper around the `uuid` crate's `Uuid` type.
pub struct Uuid(uuid::Uuid);

impl From<uuid::Uuid> for Uuid {
    fn from(uuid: uuid::Uuid) -> Self {
        Self(uuid)
    }
}

impl From<Uuid> for uuid::Uuid {
    fn from(uuid: Uuid) -> Self {
        uuid.0
    }
}

impl From<[u8; 16]> for Uuid {
	fn from(bytes: [u8; 16]) -> Self {
		Self(uuid::Uuid::from_bytes(bytes))
	}
}

impl From<Uuid> for [u8; 16] {
	fn from(uuid: Uuid) -> Self {
		uuid.0.as_bytes().clone()
	}
}

impl<'a> From<&'a [u8; 16]> for Uuid {
	fn from(bytes: &'a [u8; 16]) -> Self {
		Self(uuid::Uuid::from_bytes(*bytes))
	}
}

impl AsRef<uuid::Uuid> for Uuid {
    fn as_ref(&self) -> &uuid::Uuid {
        &self.0
    }
}

impl AsMut<uuid::Uuid> for Uuid {
    fn as_mut(&mut self) -> &mut uuid::Uuid {
        &mut self.0
    }
}

impl AsRef<[u8; 16]> for Uuid {
    fn as_ref(&self) -> &[u8; 16] {
        self.0.as_bytes()
    }
}

impl core::ops::Deref for Uuid {
	type Target = uuid::Uuid;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl core::fmt::Display for Uuid {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:-x}")
    }
}

impl<'a> core::fmt::LowerHex for Uuid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error>
{         self.format(f, UuidFormatCase::Lowercase)
    }
}

impl<'a> core::fmt::UpperHex for Uuid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error>
{         self.format(f, UuidFormatCase::Uppercase)
    }
}