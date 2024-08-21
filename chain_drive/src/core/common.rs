use std::any::Any;

pub trait ChainPayload {}

pub struct ChainJumpResult {
    func: Box<dyn FnOnce()>
}
impl ChainJumpResult {
    pub(crate) fn from_func(func: Box<dyn FnOnce()>) -> ChainJumpResult {
        ChainJumpResult {
            func
        }
    }

    pub(crate) fn from_blank() -> ChainJumpResult {
        ChainJumpResult {
            func: Box::new(|| {})
        }
    }
    pub fn enter(self) {
        (self.func)()
    }
}

pub trait ChainCrumb: Any+Send+Sync {}