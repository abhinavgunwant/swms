pub enum ErrorType {
    NotFound,
    InternalError,
}

pub struct Error<'a> {
    pub error_type: ErrorType,
    pub message: &'a str,
}

impl<'a> Error<'a> {
    pub fn new(error_type: ErrorType, message: &'a str) -> Self {
        Error { error_type, message }
    }
}

