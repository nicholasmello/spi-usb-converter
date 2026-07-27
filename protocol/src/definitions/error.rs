use crate::{Deserialize, Serialize};
use alloc::vec;
use protocol_derive::SerializeTest;

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error, SerializeTest)]
pub enum ProtocolError {
    /// Error packetizing the data
    #[error("Error packetizing data")]
    Packetizing,

    /// Error depacketizing the data
    #[error("Error depacketizing data")]
    Depacketizing,

    /// Transfer specified an invalid device
    #[error("Device {0} does not exist")]
    #[skip_test]
    InvalidDevice(u8),

    /// Unknow message type
    #[error("Unknown message type {0} specified")]
    #[skip_test]
    UnknownMessage(u8),

    /// Internal packet formatting error
    #[error("Packet contents malformed")]
    MalformedPacket,
}

impl Serialize for ProtocolError {
    fn serialize(&self) -> crate::SerializedData {
        match self {
            Self::Packetizing => vec![0x1],
            Self::Depacketizing => vec![0x2],
            Self::InvalidDevice(dev) => vec![0x3, *dev],
            Self::UnknownMessage(msg) => vec![0x4, *msg],
            Self::MalformedPacket => vec![0x5],
        }
    }
}

impl Deserialize for ProtocolError {
    fn deserialize(data: crate::SerializedData) -> Option<Self> {
        Some(match data.first()? {
            0x1 => Self::Packetizing,
            0x2 => Self::Depacketizing,
            0x3 => Self::InvalidDevice(*data.get(1)?),
            0x4 => Self::UnknownMessage(*data.get(1)?),
            0x5 => Self::MalformedPacket,
            _ => return None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_deserialize_protocolerror_non_unit() {
        let serialized = ProtocolError::InvalidDevice(3).serialize();
        assert_eq!(
            ProtocolError::InvalidDevice(3),
            ProtocolError::deserialize(serialized).expect("Failed to deserialize")
        );

        let serialized = ProtocolError::UnknownMessage(3).serialize();
        assert_eq!(
            ProtocolError::UnknownMessage(3),
            ProtocolError::deserialize(serialized).expect("Failed to deserialize")
        );
    }
}
