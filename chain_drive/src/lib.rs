mod core;
pub use core::{ChainDrive, ChainJumper, ChainJumperCore, ChainPayload, InitPayload, ChainJumpResult};
pub mod in_macro {
    pub use crate::core::{ChainBlock, ChainBlockRef};
}