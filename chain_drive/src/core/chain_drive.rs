use std::any::{Any, TypeId};
use std::collections::HashMap;
use crate::core::ChainChannel;
use crate::core::common::{ChainBlock, ChainPayload};

pub struct ChainDrive {
    channels: HashMap<TypeId, Box<dyn Any>>
}
impl ChainDrive {
    pub fn new() -> ChainDrive {
        return ChainDrive { channels: HashMap::new() }
    }

    fn get_channel_mut<P: ChainPayload + 'static>(&mut self) -> &mut ChainChannel<P> {
        let entry = self.channels
            .entry(TypeId::of::<P>())
            .or_insert_with(|| Box::new(ChainChannel::<P>::new()));

        if let Some(channel) = entry.downcast_mut::<ChainChannel<P>>() {
            return channel
        }
        panic!("Channel type mismatch for {}", std::any::type_name::<P>());
    }

    fn get_channel<P: ChainPayload + 'static>(&self) -> &ChainChannel<P> {
        let entry = self.channels
            .get(&TypeId::of::<P>())
            .expect(&format!("Channel missing for {}", std::any::type_name::<P>()));

        if let Some(channel) = entry.downcast_ref::<ChainChannel<P>>() {
            return channel;
        }
        panic!("Channel type mismatch for {}", std::any::type_name::<P>());
    }

    pub fn push_front<P: ChainPayload + 'static, T: ChainBlock<P> + 'static>(&mut self, block: T) {
        self.get_channel_mut().push_front(block);
    }

    pub fn push_back<P: ChainPayload + 'static, T: ChainBlock<P> + 'static>(&mut self, block: T) {
        self.get_channel_mut().push_back(block);
    }

    fn run_channel<P: ChainPayload + 'static>(&self, payload: P) {
        self.get_channel().run(payload, &ChainJumper {owner: self});
    }

    pub fn start(&self) {
        self.run_channel(InitPayload {});
    }
}
pub struct ChainJumper<'a> {
    owner: &'a ChainDrive
}
impl ChainJumper<'_> {
    pub fn to<P: ChainPayload + 'static>(&self, payload: P) {
        self.owner.run_channel(payload)
    }
}

pub struct InitPayload;
impl ChainPayload for InitPayload {}