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

    fn run_in_channel<P, F>(&mut self, func: F)
    where
        P: ChainPayload + 'static,
        F: FnOnce(&mut ChainChannel<P>)
    {
        let entry = self.channels
            .entry(TypeId::of::<P>())
            .or_insert_with(|| Box::new(ChainChannel::<P>::new()));

        if let Some(channel) = entry.downcast_mut::<ChainChannel<P>>() {
            func(channel);
        } else {
            panic!("Channel type mismatch for {}", std::any::type_name::<P>());
        }
    }

    pub fn push_front<P: ChainPayload + 'static, T: ChainBlock<P> + 'static>(&mut self, block: T) {
        self.run_in_channel(move |channel: &mut ChainChannel<P>| {
            channel.push_front(block);
        });
    }

    pub fn run<P: ChainPayload + 'static>(&mut self, payload: P) {
        self.run_in_channel(move |channel: &mut ChainChannel<P>| {
            channel.run(payload);
        });
    }
}