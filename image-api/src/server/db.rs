use std::fmt::{ Display, Formatter, Result as FmtResult };

use log::error;
use mysql::Error::{
    IoError, DriverError, UrlError, TlsError, CodecError, MySqlError
};

#[derive(PartialEq)]
pub enum DBError {
    /// Database or table does not exist/not found
    NotFound,
    IOError,
    AccessDenied,
    DriverError,
    ConnectionError,
    OtherError,
}

impl Display for DBError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::NotFound=> write!(f, "DBError: NotFound"),
            Self::IOError=> write!(f, "DBError: IOError"),
            Self::DriverError=> write!(f, "DBError: DriverError"),
            Self::ConnectionError => write!( f, "DBError: ConnectionError"),
            Self::OtherError => return write!(f, "DBError: OtherError"),
            Self::AccessDenied => return write!(f, "DBError: AccessDenied"),
        }
    }
}

pub fn mysql_to_db_error(msg: &str, e: mysql::Error) -> DBError {
    match e {
        IoError(err) => { error!("{}: {}", msg, err); DBError::IOError }
        DriverError(err) => {error!("{}: {}", msg, err); DBError::DriverError}
        UrlError(err) => {error!("{}: {}", msg, err); DBError::ConnectionError}
        TlsError(err) => {error!("{}: {}", msg, err); DBError::ConnectionError}
        CodecError(err) => { error!("{}: {}", msg, err); DBError::OtherError }
        MySqlError(err) => { error!("{}: {}", msg, err); DBError::OtherError }
        _ => { error!("{}", msg); DBError::OtherError }
    }
}

