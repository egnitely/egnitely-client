use std::error;
use std::fmt;

pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Debug)]
pub struct HandlerError {
    error_message: String,
    error_code: String,
    status_code: u32,
}

impl HandlerError {
    pub fn new(error_code: String, error_message: String, status_code: u32) -> Box<Self> {
        Box::new(Self {
            error_code,
            error_message,
            status_code,
        })
    }
}

impl fmt::Display for HandlerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({} ({}): {})",
            self.error_code, self.status_code, self.error_message
        )
    }
}

impl error::Error for HandlerError {}
