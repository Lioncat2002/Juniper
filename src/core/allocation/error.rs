use std::{error::Error, fmt};

#[derive(Debug,Clone)]
pub struct AllocationError{
    details: String
}

impl AllocationError{
    pub fn new(details: &str)->Self{
        AllocationError{
            details: details.to_string()
        }
    }
}

impl fmt::Display for AllocationError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}",self.details)
    }
}

impl Error for AllocationError {
    fn description(&self) -> &str {
        &self.details.as_str()
    }
}
