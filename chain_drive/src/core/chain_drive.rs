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

    fn get_channel<P: ChainPayload + 'static>(&mut self) -> &mut ChainChannel<P> {
        let entry = self.channels
            .entry(TypeId::of::<P>())
            .or_insert_with(|| Box::new(ChainChannel::<P>::new()));

        if let Some(channel) = entry.downcast_mut::<ChainChannel<P>>() {
            return channel
        }
        panic!("Channel type mismatch for {}", std::any::type_name::<P>());
    }

    pub fn push_front<P: ChainPayload + 'static, T: ChainBlock<P> + 'static>(&mut self, block: T) {
        self.get_channel().push_front(block);
    }

    pub fn push_back<P: ChainPayload + 'static, T: ChainBlock<P> + 'static>(&mut self, block: T) {
        self.get_channel().push_back(block);
    }

    pub fn run<P: ChainPayload + 'static>(&mut self, payload: P) {
        self.get_channel().run(payload);
    }
}