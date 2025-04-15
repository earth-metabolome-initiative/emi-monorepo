//! A Lambda reader that replaces a provided string with the provided separator
//! in each line being read.

use std::io::{BufRead, Read};

/// A Lambda reader that replaces a provided string with the provided separator
/// in each line being read.
pub struct SeparatorFixedReader<'a, R> {
    reader: R,
    separator: &'a str,
    needle: &'a str,
}

impl<'a, R> SeparatorFixedReader<'a, R> {
    /// Creates a new `SeparatorFixedReader` from the provided reader,
    /// separator, and needle.
    pub fn new(reader: R, separator: &'a str, needle: &'a str) -> Self {
        Self { reader, separator, needle }
    }
}

/// Implementation of `Read` for `SeparatorFixedReader`, which
/// replaces the needle with the separator in each line being read.
impl<'a, R: BufRead> Read for SeparatorFixedReader<'a, R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut buffer = String::new();
        let bytes_read = self.reader.read_line(&mut buffer)?;

        if bytes_read == 0 {
            return Ok(0); // EOF
        }

        let replaced = buffer.replace(&self.needle, &self.separator);

        let bytes = replaced.as_bytes();
        let len = bytes.len();
        let to_copy = std::cmp::min(len, buf.len());

        buf[..to_copy].copy_from_slice(&bytes[..to_copy]);

        Ok(to_copy)
    }
}

/// Implements BufRead for `SeparatorFixedReader`.
impl<'a, R: BufRead> BufRead for SeparatorFixedReader<'a, R> {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        self.reader.fill_buf()
    }

    fn consume(&mut self, amt: usize) {
        self.reader.consume(amt);
    }

    fn read_line(&mut self, buf: &mut String) -> std::io::Result<usize> {
        let mut buffer = String::new();
        let bytes_read = self.reader.read_line(&mut buffer)?;

        if bytes_read == 0 {
            return Ok(0); // EOF
        }

        let replaced = buffer.replace(self.needle, self.separator);

        buf.push_str(&replaced);

        Ok(replaced.len())
    }
}
