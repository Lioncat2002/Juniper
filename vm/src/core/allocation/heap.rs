use crate::core::juniper_val::JnpVal;

use super::error::AllocationError;

struct AllocFrame{
    data: JnpVal,
    idx:usize
}

impl AllocFrame {
    pub fn new(data: JnpVal,idx:usize)->Self{
        AllocFrame{
            data,
            idx
        }
    }
}

pub struct Heap {
    memory: Vec<AllocFrame>,
}
//TODO: rethink heap impl
impl Heap {
    pub fn new() -> Self {
        Heap {
            memory: vec![],
            
        }
    }

    pub fn alloc(&mut self, data: JnpVal) -> Result<usize, AllocationError> {
        if self.memory.len()>1024*1024{
            return Err(AllocationError::new(
                "Failed to allocate memory; not enough space available",
            ));
        }
        self.memory.push(AllocFrame::new(data, self.memory.len()-1));
        Ok(self.memory.len())
    }

    pub fn free(&mut self, idx:usize) {
        self.memory.remove(idx);
    }
}
