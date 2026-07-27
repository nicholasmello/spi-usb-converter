use crate::{Deserialize, Serialize, SerializedData};
use alloc::vec;
use alloc::vec::Vec;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct TransferPacket {
    /// Data to be sent over the wire
    pub data: Vec<u8>,

    /// Device number for chip select
    pub device: u8,
}

impl Serialize for TransferPacket {
    fn serialize(&self) -> SerializedData {
        let mut data = vec![self.device];
        data.extend_from_slice(&self.data);

        data
    }
}

impl Deserialize for TransferPacket {
    fn deserialize(data: SerializedData) -> Option<Self> {
        Some(TransferPacket {
            device: *data.first()?,
            data: Vec::from(data.get(1..).unwrap_or_default()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_deserialize_transferpacket_default() {
        let packet = TransferPacket::default();
        let serialized = packet.serialize();
        assert_eq!(
            packet,
            TransferPacket::deserialize(serialized).expect("Failed to deserialize")
        );
    }

    #[test]
    fn test_serialize_deserialize_transferpacket_data() {
        let packet = TransferPacket {
            data: vec![0x5; 41],
            device: 3,
        };
        let serialized = packet.serialize();
        assert_eq!(
            packet,
            TransferPacket::deserialize(serialized).expect("Failed to deserialize")
        );
    }
}
