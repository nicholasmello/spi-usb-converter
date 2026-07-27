mod acktype;
mod error;
mod spimode;
mod txpacket;

use crate::{Deserialize, Serialize};
use acktype::AckType;
use alloc::vec;
use alloc::vec::Vec;
use error::ProtocolError;
use spimode::SpiMode;
use txpacket::TransferPacket;

#[derive(Clone, Debug, PartialEq)]
pub enum Message {
    /// Request to make sure the device is there
    Ping,

    /// Response to a `Message::Ping` request
    Pong,

    /// Command to set the SPI Mode
    SetMode { mode: SpiMode },

    /// Command to set the SPI Frequency
    SetFrequency { hz: u32 },

    /// Acknowledgement of `SetMode` or `SetFrequency` command
    Ack { ack_type: AckType },

    /// Command to transfer data
    Transfer { packet: TransferPacket },

    /// Response to a data transfer
    TransferResult { rx: Vec<u8> },

    /// Report of an error from the other side of the bus
    Error { error: ProtocolError },
}

impl Serialize for Message {
    fn serialize(&self) -> crate::SerializedData {
        match self {
            Self::Ping => vec![0x1],
            Self::Pong => vec![0x2],
            Self::SetMode { mode } => {
                let mut data = vec![0x3];
                data.extend_from_slice(&mode.serialize());
                data
            }
            Self::SetFrequency { hz } => {
                let mut data = vec![0x4];
                data.extend_from_slice(&hz.to_be_bytes());
                data
            }
            Self::Ack { ack_type } => {
                let mut data = vec![0x5];
                data.extend_from_slice(&ack_type.serialize());
                data
            }
            Self::Transfer { packet } => {
                let mut data = vec![0x6];
                data.extend_from_slice(&packet.serialize());
                data
            }
            Self::TransferResult { rx } => {
                let mut data = vec![0x7];
                data.extend_from_slice(rx);
                data
            }
            Self::Error { error } => {
                let mut data = vec![0x8];
                data.extend_from_slice(&error.serialize());
                data
            }
        }
    }
}

impl Deserialize for Message {
    fn deserialize(data: crate::SerializedData) -> Option<Self> {
        Some(match data.first()? {
            0x1 => Self::Ping,
            0x2 => Self::Pong,
            0x3 => Self::SetMode {
                mode: SpiMode::deserialize(Vec::from(data.get(1..).unwrap_or_default()))?,
            },
            0x4 => Self::SetFrequency {
                hz: u32::from_be_bytes(data.get(1..5).unwrap_or_default().try_into().ok()?),
            },
            0x5 => Self::Ack {
                ack_type: AckType::deserialize(Vec::from(data.get(1..).unwrap_or_default()))?,
            },
            0x6 => Self::Transfer {
                packet: TransferPacket::deserialize(Vec::from(data.get(1..).unwrap_or_default()))?,
            },
            0x7 => Self::TransferResult {
                rx: Vec::from(data.get(1..).unwrap_or_default()),
            },
            0x8 => Self::Error {
                error: ProtocolError::deserialize(Vec::from(data.get(1..).unwrap_or_default()))?,
            },
            _ => return None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn compare_deserialize(message: Message) {
        let serialized = message.serialize();
        assert_eq!(
            message,
            Message::deserialize(serialized).expect("Failed to deserialize message")
        );
    }

    #[test]
    fn test_serialize_deserialize_message() {
        compare_deserialize(Message::SetMode {
            mode: SpiMode::Mode2,
        });

        compare_deserialize(Message::Ack {
            ack_type: AckType::SetMode,
        });

        compare_deserialize(Message::Transfer {
            packet: TransferPacket {
                device: 2,
                data: vec![3; 86],
            },
        });

        compare_deserialize(Message::TransferResult { rx: vec![3; 86] });

        compare_deserialize(Message::Error {
            error: ProtocolError::UnknownMessage(90),
        });
    }
}
