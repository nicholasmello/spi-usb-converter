mod acktype;
mod error;
mod spimode;
mod txpacket;

use acktype::AckType;
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
    Error(ProtocolError),
}
