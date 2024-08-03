use std::any::{Any, TypeId};
use std::collections::HashMap;
use crate::{ChainBlock, ChainPayload};
use crate::core::ChainChannel;

pub struct ChainDrive {
    pub channels: HashMap<TypeId, Box<dyn Any>>
}
impl ChainDrive {
    pub fn new() -> ChainDrive {
        return ChainDrive { channels: HashMap::new() }
    }

    pub fn push_front<P: ChainPayload + 'static, T: ChainBlock<P> + 'static>(&mut self, block: T) {
        match self.channels.remove(&TypeId::of::<P>()) {
            Some(channel) => {
                if let Ok(mut channel) = channel.downcast::<ChainChannel<P>>() {
                    channel.push_front(block);
                    self.channels.insert(TypeId::of::<P>(), channel);
                } else {
                    panic!("Something went wrong in the chain drive, chain channel for {} is mismatched", std::any::type_name::<P>())
                }
            }
            None => {
                let mut channel = ChainChannel::new();
                channel.push_front(block);
                self.channels.insert(TypeId::of::<P>(), Box::new(channel));
            }
        }
    }

    pub fn run<P: ChainPayload + 'static>(&mut self, payload: P) {
        match self.channels.remove(&TypeId::of::<P>()) {
            Some(channel) => {
                if let Ok(mut channel) = channel.downcast::<ChainChannel<P>>() {
                    channel.run(payload);
                    self.channels.insert(TypeId::of::<P>(), channel);
                } else {
                    panic!("Something went wrong in the chain drive, chain channel for {} is mismatched", std::any::type_name::<P>())
                }
            }
            None => {
                let mut channel = ChainChannel::new();
                channel.run(payload);
                self.channels.insert(TypeId::of::<P>(), Box::new(channel));
            }
        }
    }
}