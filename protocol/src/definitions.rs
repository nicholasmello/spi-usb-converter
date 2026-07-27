mod transferpacket;

use crate::{Deserialize, Serialize, SerializeTest};
use alloc::vec::Vec;
use transferpacket::TransferPacket;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Default, Serialize, Deserialize, SerializeTest)]
pub enum SpiMode {
    /// CPOL = 0 / CPHA = 0
    #[default]
    Mode0,

    /// CPOL = 0 / CPHA = 1
    Mode1,

    /// CPOL = 1 / CPHA = 1
    Mode2,

    /// CPOL = 1 / CPHA = 0
    Mode3,
}

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum ProtocolError {
    /// Error packetizing the data
    #[error("Error packetizing data")]
    Packetizing,

    /// Error depacketizing the data
    #[error("Error depacketizing data")]
    Depacketizing,

    /// Transfer specified an invalid device
    #[error("Device {0} does not exist")]
    InvalidDevice(u8),

    /// Unknow message type
    #[error("Unknown message type {0} specified")]
    UnknownMessage(u8),

    /// Internal packet formatting error
    #[error("Packet contents malformed")]
    MalformedPacket,
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize, SerializeTest)]
pub enum AckType {
    SetMode,
    SetFrequency,
}

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
    Error(ProtocolError),
}
