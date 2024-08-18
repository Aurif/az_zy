use std::sync::{Arc, Mutex};
use crate::{ChainDrive, ChainJumper, ChainPayload};
use crate::core::common::ChainJumpResult;

pub(crate) trait ChainB {}
pub struct ChainBFront;
impl ChainB for ChainBFront {}
pub struct ChainBBack;
impl ChainB for ChainBBack {}


pub trait ChainBlock<P: ChainPayload, B: ChainB>: Send+Sync {
    fn run(&self, payload: P, jump: ChainJumper<P>) -> ChainJumpResult;
}

pub trait ChainBlockRef {
    fn insert_into(self_ref: Arc<Mutex<Self>>, chain_drive: &mut ChainDrive);
}

pub(crate) trait ChainBlockInserter {
    fn insert_into(self, chain_drive: &mut ChainDrive);
}

impl<R: ChainBlockRef> ChainBlockInserter for R {
    fn insert_into(self, chain_drive: &mut ChainDrive) {
        let block = Arc::new(Mutex::new(self));
        ChainBlockRef::insert_into(block, chain_drive)
    }
}

impl<R: ChainBlockRef> ChainBlockInserter for Arc<Mutex<R>> {
    fn insert_into(self, chain_drive: &mut ChainDrive) {
        ChainBlockRef::insert_into(self, chain_drive)
    }
}

#[macro_export]
macro_rules! define_block {
    ( $vis:vis struct $name:ident; $(impl { $($impl_code:tt)* })? $(impl for $pos:ident, $t:ty { $($code:tt)* })* ) => {
        mod __inner {
            pub use std::sync::{Arc, Mutex};
            pub use chain_drive::{ChainDrive};
            pub use chain_drive::in_macro::{ChainBlock, ChainBlockRef};
            macro_rules! chain_block_insert {
                (ChainBFront, $ti:ty, $rf:ident, $chain_drive:ident) => {
                    $chain_drive.push_front($rf.clone() as __inner::Arc<__inner::Mutex<dyn __inner::ChainBlock<$ti, ChainBFront>>>);
                };
                (ChainBBack, $ti:ty, $rf:ident, $chain_drive:ident) => {
                    $chain_drive.push_back($rf.clone() as __inner::Arc<__inner::Mutex<dyn __inner::ChainBlock<$ti, ChainBBack>>>);
                };
            }
            pub(crate) use chain_block_insert;
        }
        $vis struct $name;
        $(
            impl $name {
                $($impl_code)*
            }
        )?
        $(
            impl __inner::ChainBlock<$t, $pos> for $name {
                $($code)*
            }
        )*
        impl __inner::ChainBlockRef for $name {
            fn insert_into(self_ref: __inner::Arc<__inner::Mutex<Self>>, chain_drive: &mut __inner::ChainDrive) {
                $(
                    __inner::chain_block_insert!($pos, $t, self_ref, chain_drive);
                )*
            }
        }
    };
    ( $vis:vis struct $name:ident $({ $($struct_code:tt)* })? $(impl { $($impl_code:tt)* })? $(impl for $pos:ident, $t:ty { $($code:tt)* })* ) => {
        mod __inner {
            pub use std::sync::{Arc, Mutex};
            pub use chain_drive::{ChainDrive};
            pub use chain_drive::in_macro::{ChainBlock, ChainBlockRef};
            macro_rules! chain_block_insert {
                (ChainBFront, $ti:ty, $rf:ident, $chain_drive:ident) => {
                    $chain_drive.push_front($rf.clone() as __inner::Arc<__inner::Mutex<dyn __inner::ChainBlock<$ti, ChainBFront>>>);
                };
                (ChainBBack, $ti:ty, $rf:ident, $chain_drive:ident) => {
                    $chain_drive.push_back($rf.clone() as __inner::Arc<__inner::Mutex<dyn __inner::ChainBlock<$ti, ChainBBack>>>);
                };
            }
            pub(crate) use chain_block_insert;
        }
        $vis struct $name $(
            {
                $($struct_code)*
            }
        )?
        $(
            impl $name {
                $($impl_code)*
            }
        )?
        $(
            impl __inner::ChainBlock<$t, $pos> for $name {
                $($code)*
            }
        )*
        impl __inner::ChainBlockRef for $name {
            fn insert_into(self_ref: __inner::Arc<__inner::Mutex<Self>>, chain_drive: &mut __inner::ChainDrive) {
                $(
                    __inner::chain_block_insert!($pos, $t, self_ref, chain_drive);
                )*
            }
        }
    }
}