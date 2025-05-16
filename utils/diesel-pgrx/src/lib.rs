#![doc = include_str!("../README.md")]

/// Trait marked defining a type that can be used with both `diesel` and `pgrx`
pub trait DieselPGRX {}

/// A constant representing the size of the "varlena" header, typically 4 bytes.
pub const VARHDRSZ: usize = std::mem::size_of::<i32>();

/// Encodes a serializable input into a CBOR-encoded Vec<u8> with a 4-byte length prefix.
///
/// The first 4 bytes are a length header, where the length is left-shifted by 2 bits,
/// mimicking PostgreSQL's varlena 4-byte header format.
pub fn cbor_encode<T>(input: T) -> Vec<u8>
where
    T: serde::Serialize,
{
    let mut buf = Vec::with_capacity(128);
    
    // Reserve `VARHDRSZ` bytes for the header
    buf.extend_from_slice(&[0x69u8; VARHDRSZ]);

    // Write the CBOR-encoded data
    serde_cbor::to_writer(&mut buf, &input).expect("failed to encode as CBOR");

    // We check that the header was left unchanged
    assert_eq!(buf[0..VARHDRSZ], [0x69u8; VARHDRSZ]);

    // Set the first `VARHDRSZ` bytes to the shifted length (PostgreSQL-style header)
    let len = buf.len() as u32;
    // len = len.next_multiple_of(8);

    println!("Length of serialized data: {}", len);

    let header = len << 2;
    buf[0..VARHDRSZ].copy_from_slice(&header.to_be_bytes());

    // Print the encoded buffer for debugging in binary format
    for byte in &buf {
        print!("{:02x} ", byte);
    }
    println!("");

    for byte in &buf {
        print!("{:08b} ", byte);
    }
    println!("");

    // println!("Press enter to send cbor encoded data");
    // let mut input = String::new();
    // std::io::stdin().read_line(&mut input).expect("Failed to read line");
    
    buf
}

#[cfg(feature = "macro")]
pub use byteorder;
#[cfg(feature = "macro")]
pub use diesel;
#[cfg(feature = "macro")]
#[doc(hidden)]
pub use diesel_pgrx_derive::DieselPGRX;
#[cfg(feature = "macro")]
pub use serde_cbor;
