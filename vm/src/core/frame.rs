#[derive(Debug)]
pub struct Frame {
    pub ret_addr: usize,
}

impl Frame {
    pub fn new(ret_addr: usize) -> Frame {
        Frame { ret_addr: ret_addr }
    }
}
