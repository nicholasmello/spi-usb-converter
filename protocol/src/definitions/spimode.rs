use crate::{Deserialize, Serialize, SerializeTest};

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
