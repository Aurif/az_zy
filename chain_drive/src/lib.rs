mod core;
pub use core::{ChainDrive, ChainJumper, ChainJumperCore, ChainPayload, InitPayload, ChainJumpResult, ChainBBack, ChainBFront, ChainCrumb};
pub mod in_macro {
    pub use crate::core::{ChainBlock, ChainBlockRef};
}