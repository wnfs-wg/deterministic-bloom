//! Internally-used Utilities

use serde::de::Visitor;
use std::fmt::{self, Debug};

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

pub(crate) struct ByteArrayVisitor<const N: usize>;

/// Helper newtype for rendering given debug field as hex string
pub(crate) struct HexFieldDebug<A: AsRef<[u8]>>(pub(crate) A);

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl<'de, const N: usize> Visitor<'de> for ByteArrayVisitor<N> {
    type Value = [u8; N];

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "a byte array of length {N}")
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let bytes: [u8; N] = v.try_into().map_err(E::custom)?;
        Ok(bytes)
    }
}

impl<A: AsRef<[u8]>> Debug for HexFieldDebug<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x")?;
        for byte in self.0.as_ref().iter() {
            write!(f, "{byte:02X}")?;
        }

        Ok(())
    }
}
