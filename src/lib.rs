use std::iter::Sum;
use std::ops::Add;

/// Length of raw data in bytes in on-the-wire format.
///
/// On-the-wire data is the byte representation used when data is sent over
/// a network or stored in wire format, before any in-memory deserialization.
pub trait WireLength<T> {
    /// Returns the length of this value in bytes when encoded in wire format.
    fn wire_length(&self) -> T;
}

impl<T: WireLength<K>, K: Add<Output = K> + Sum<K>> WireLength<K> for Vec<T> {
    fn wire_length(&self) -> K {
        self.iter().map(|item| item.wire_length()).sum()
    }
}
