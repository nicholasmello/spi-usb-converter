// Re-export macros with everything else
pub use protocol_derive::{Deserialize, Serialize, SerializeTest};

use alloc::vec::Vec;

pub type SerializedData = Vec<u8>;

pub trait Serialize {
    fn serialize(&self) -> SerializedData;
}

pub trait Deserialize {
    fn deserialize(data: SerializedData) -> Option<Self>
    where
        Self: Sized;
}
