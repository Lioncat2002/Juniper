use super::error::AllocationError;

struct FreeSpaceFrame {
    start_idx: usize,
    size: usize,
}

pub struct Heap {
    memory: [u64; 1024 * 1024],
    free_space: Vec<FreeSpaceFrame>,
}
//TODO: rethink heap impl
impl Heap {
    pub fn new() -> Self {
        Heap {
            memory: [0; 1024 * 1024],
            free_space: vec![FreeSpaceFrame {
                start_idx: 0,
                size: 1024 * 2024,
            }],
        }
    }

    pub fn alloc(&mut self, data: &[u64]) -> Result<usize, AllocationError> {
        match self.free_space.pop() {
            Some(addr) => {
                //TODO: rethink this logic to check if heap is full
                if addr.size < data.len() {
                    self.free_space.push(addr);
                    return Err(AllocationError::new(
                        "Failed to allocate memory; not enough space available",
                    ));
                }
                self.allocate_memory(data, addr.start_idx);
                self.free_space.push(FreeSpaceFrame {
                    start_idx: addr.start_idx + data.len(),
                    size: addr.size - data.len(),
                });
                self.sort_free_space();
                return Ok(addr.start_idx);
            }
            None => {
                return Err(AllocationError::new(
                    "Failed to allocate memory; heap is full",
                ));
            }
        }
    }

    pub fn free(&mut self, start_idx: usize, size: usize) {
        self.free_space.push(FreeSpaceFrame { start_idx, size });
        self.sort_free_space();
    }

    fn allocate_memory(&mut self, data: &[u64], start_idx: usize) {
        self.memory[start_idx..data.len()].copy_from_slice(data);
    }

    fn sort_free_space(&mut self){
        self.free_space.sort_unstable_by_key(|x| x.size);
    }
}
