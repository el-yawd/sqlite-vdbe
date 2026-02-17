//! Error types for the VDBE API

use std::ffi::NulError;
use std::fmt;

use crate::ffi;

/// Error type for VDBE operations
#[derive(Debug)]
pub enum Error {
    /// SQLite returned an error code
    Sqlite { code: i32, message: Option<String> },
    /// Invalid path (non-UTF8 or contains null byte)
    InvalidPath,
    /// String conversion failed (contains null byte)
    NulError(NulError),
    /// Memory allocation failed
    AllocationFailed,
    /// Program is in wrong state for operation
    InvalidState {
        expected: &'static str,
        actual: &'static str,
    },
    /// Register index out of bounds
    RegisterOutOfBounds { index: i32, max: i32 },
    /// Cursor index out of bounds
    CursorOutOfBounds { index: i32, max: i32 },
    /// Invalid opcode
    InvalidOpcode(u8),
}

impl Error {
    /// Create an error from a SQLite error code
    pub fn from_code(code: i32) -> Self {
        Error::Sqlite {
            code,
            message: None,
        }
    }

    /// Create an error from a SQLite error code with message
    pub fn from_code_with_message(code: i32, message: String) -> Self {
        Error::Sqlite {
            code,
            message: Some(message),
        }
    }

    /// Get the SQLite error code if this is a SQLite error
    pub fn sqlite_code(&self) -> Option<i32> {
        match self {
            Error::Sqlite { code, .. } => Some(*code),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Sqlite { code, message } => {
                let code_name = match *code {
                    ffi::SQLITE_ERROR => "SQLITE_ERROR",
                    ffi::SQLITE_INTERNAL => "SQLITE_INTERNAL",
                    ffi::SQLITE_PERM => "SQLITE_PERM",
                    ffi::SQLITE_ABORT => "SQLITE_ABORT",
                    ffi::SQLITE_BUSY => "SQLITE_BUSY",
                    ffi::SQLITE_LOCKED => "SQLITE_LOCKED",
                    ffi::SQLITE_NOMEM => "SQLITE_NOMEM",
                    ffi::SQLITE_READONLY => "SQLITE_READONLY",
                    ffi::SQLITE_INTERRUPT => "SQLITE_INTERRUPT",
                    ffi::SQLITE_IOERR => "SQLITE_IOERR",
                    ffi::SQLITE_CORRUPT => "SQLITE_CORRUPT",
                    ffi::SQLITE_NOTFOUND => "SQLITE_NOTFOUND",
                    ffi::SQLITE_FULL => "SQLITE_FULL",
                    ffi::SQLITE_CANTOPEN => "SQLITE_CANTOPEN",
                    ffi::SQLITE_PROTOCOL => "SQLITE_PROTOCOL",
                    ffi::SQLITE_SCHEMA => "SQLITE_SCHEMA",
                    ffi::SQLITE_TOOBIG => "SQLITE_TOOBIG",
                    ffi::SQLITE_CONSTRAINT => "SQLITE_CONSTRAINT",
                    ffi::SQLITE_MISMATCH => "SQLITE_MISMATCH",
                    ffi::SQLITE_MISUSE => "SQLITE_MISUSE",
                    ffi::SQLITE_RANGE => "SQLITE_RANGE",
                    ffi::SQLITE_NOTADB => "SQLITE_NOTADB",
                    _ => "SQLITE_UNKNOWN",
                };
                if let Some(msg) = message {
                    write!(f, "{} ({}): {}", code_name, code, msg)
                } else {
                    write!(f, "{} ({})", code_name, code)
                }
            }
            Error::InvalidPath => write!(f, "Invalid path: non-UTF8 or contains null byte"),
            Error::NulError(e) => write!(f, "String contains null byte: {}", e),
            Error::AllocationFailed => write!(f, "Memory allocation failed"),
            Error::InvalidState { expected, actual } => {
                write!(f, "Invalid state: expected {}, got {}", expected, actual)
            }
            Error::RegisterOutOfBounds { index, max } => {
                write!(f, "Register {} out of bounds (max: {})", index, max)
            }
            Error::CursorOutOfBounds { index, max } => {
                write!(f, "Cursor {} out of bounds (max: {})", index, max)
            }
            Error::InvalidOpcode(op) => write!(f, "Invalid opcode: {}", op),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::NulError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<NulError> for Error {
    fn from(e: NulError) -> Self {
        Error::NulError(e)
    }
}

/// Result type for VDBE operations
pub type Result<T> = std::result::Result<T, Error>;
