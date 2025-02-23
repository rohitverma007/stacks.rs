pub use crate::crypto::base58::b58_decode;
pub use crate::crypto::base58::b58_encode;
pub use crate::crypto::base58::base58check_decode;
pub use crate::crypto::base58::base58check_encode;

pub use crate::crypto::c32::c32_address;
pub use crate::crypto::c32::c32_address_decode;
pub use crate::crypto::c32::c32_decode;
pub use crate::crypto::c32::c32_encode;
pub use crate::crypto::c32::c32check_decode;
pub use crate::crypto::c32::c32check_encode;

pub use crate::crypto::hex::bytes_to_hex;
pub use crate::crypto::hex::hex_to_bytes;

pub use crate::crypto::hash::DSha256Hash;
pub use crate::crypto::hash::Hash160;
pub use crate::crypto::hash::Sha256Hash;
pub use crate::crypto::hash::Sha512_256Hash;

pub use crate::crypto::bip32::ExtendedPrivateKey;

pub mod base58;
pub mod c32;
pub mod hash;
pub mod hex;

pub(crate) mod bip32;

/// Trait for serializing data to bytes.
pub trait Serialize {
    /// The error type returned by the serializer.
    type Err;

    /// Serialize the data to bytes.
    fn serialize(&self) -> Result<Vec<u8>, Self::Err>;
    /// Get the byte length of the serialized data.
    fn byte_length(&self) -> Result<u64, Self::Err> {
        let serialized = self.serialize()?;
        Ok(serialized.len() as u64)
    }
    /// Get the hex representation of the serialized data.
    fn to_hex(&self) -> Result<String, Self::Err> {
        let serialized = self.serialize()?;
        Ok(bytes_to_hex(&serialized))
    }
    /// Get the hex representation of the serialized data with a `0x` prefix.
    fn to_hex_prefixed(&self) -> Result<String, Self::Err> {
        Ok(format!("0x{}", self.to_hex()?))
    }
}

/// Trait for deserializing data from bytes.
pub trait Deserialize: Sized {
    /// The output type returned by the deserializer.
    type Output;
    /// The error type returned by the deserializer.
    type Err;

    /// Deserialize the data from bytes.
    fn deserialize(bytes: &[u8]) -> Result<Self::Output, Self::Err>;
}

macro_rules! impl_wrapped_array {
    ($type:ident, $ty:ty, $len:expr) => {
        impl $type {
            pub fn len(&self) -> usize {
                $len
            }

            pub fn is_empty(&self) -> bool {
                self.0.len() == 0
            }

            pub fn as_bytes(&self) -> &[$ty; $len] {
                &self.0
            }

            pub fn to_bytes(self) -> [$ty; $len] {
                self.0
            }

            pub fn into_bytes(self) -> [$ty; $len] {
                self.0
            }
        }
    };
}

pub(crate) use impl_wrapped_array;
