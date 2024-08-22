mod core;
pub use core::{ChainDrive, ChainJumper, ChainJumperCore, ChainPayload, InitPayload, ChainJumpResult, ChainBBack, ChainBFront, ChainBlock, ChainCrumb};
pub mod in_macro {
    pub use crate::core::{ChainBlockRef};
}