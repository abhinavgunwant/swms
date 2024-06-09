use std::{ error::Error, fmt::{ Display, Formatter } };

#[derive(Debug)]
pub enum SWMSErrorType {
    ActixServer,
    Repository,
    Other,
}

/// Represents error at the application level
#[derive(Debug)]
pub struct SWMSError {
    error_type: SWMSErrorType,
}

impl Error for SWMSError {
    fn description(&self) -> &str {
        match self.error_type {
            SWMSErrorType::ActixServer =>
                "API server error. Check error logs above for more info.",
            SWMSErrorType::Repository =>
                "Repository error. Check error logs above for more info.",
            SWMSErrorType::Other =>
                "Unknown error. Checking error logs above might help.",
        }
    }
}

impl Display for SWMSError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error 1234")
    }
}

impl SWMSError {
    pub fn repository() -> Self { Self { error_type: SWMSErrorType::Repository } }
    pub fn actix_server() -> Self { Self { error_type: SWMSErrorType::ActixServer } }
    pub fn other() -> Self { Self { error_type: SWMSErrorType::Other } }
}

