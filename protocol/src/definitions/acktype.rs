use crate::{Deserialize, Serialize, SerializeTest};

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize, SerializeTest)]
pub enum AckType {
    SetMode,
    SetFrequency,
}
