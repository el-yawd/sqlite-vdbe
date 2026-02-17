//! Database connection management

use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::path::Path;
use std::ptr;
use std::sync::Once;

use crate::error::{Error, Result};
use crate::ffi;
use crate::program::ProgramBuilder;

/// Ensures SQLite is initialized exactly once before any connection is opened.
static SQLITE_INIT: Once = Once::new();

/// Initialize SQLite library. Called automatically on first connection.
fn ensure_sqlite_initialized() {
    SQLITE_INIT.call_once(|| unsafe {
        ffi::sqlite3_initialize();
    });
}

/// A connection to a SQLite database
///
/// This wraps an `sqlite3*` connection handle and provides methods to
/// create VDBE programs without SQL parsing.
///
/// # Thread Safety
///
/// `Connection` is `!Send` and `!Sync` by design. SQLite connections should
/// only be used from the thread that created them.
///
/// # Example
///
/// ```no_run
/// use sqlite_vdbe::Connection;
///
/// let conn = Connection::open_in_memory()?;
/// # Ok::<(), sqlite_vdbe::Error>(())
/// ```
pub struct Connection {
    raw: *mut ffi::sqlite3,
    // Mark as !Send and !Sync using PhantomData with a raw pointer type
    _marker: PhantomData<*const ()>,
}

impl Connection {
    /// Open a database file
    ///
    /// Creates or opens the database at the given path with read-write access.
    /// If the file doesn't exist, it will be created.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the database file, or ":memory:" for in-memory database
    ///
    /// # Example
    ///
    /// ```no_run
    /// use sqlite_vdbe::Connection;
    ///
    /// let conn = Connection::open("test.db")?;
    /// # Ok::<(), sqlite_vdbe::Error>(())
    /// ```
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path_str = path.as_ref().to_str().ok_or(Error::InvalidPath)?;
        Self::open_with_flags(
            path_str,
            ffi::SQLITE_OPEN_READWRITE | ffi::SQLITE_OPEN_CREATE,
        )
    }

    /// Open an in-memory database
    ///
    /// Creates a temporary database that exists only in memory.
    /// The database is destroyed when the connection is closed.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use sqlite_vdbe::Connection;
    ///
    /// let conn = Connection::open_in_memory()?;
    /// # Ok::<(), sqlite_vdbe::Error>(())
    /// ```
    pub fn open_in_memory() -> Result<Self> {
        Self::open(":memory:")
    }

    /// Open a database with specific flags
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the database file
    /// * `flags` - Combination of `SQLITE_OPEN_*` flags
    pub fn open_with_flags(path: &str, flags: i32) -> Result<Self> {
        // Ensure SQLite is initialized (thread-safe, called once)
        ensure_sqlite_initialized();

        let c_path = CString::new(path)?;
        let mut db: *mut ffi::sqlite3 = ptr::null_mut();

        let rc = unsafe { ffi::sqlite3_open_v2(c_path.as_ptr(), &mut db, flags, ptr::null()) };

        if rc != ffi::SQLITE_OK {
            // Get error message before potentially closing
            let msg = if !db.is_null() {
                unsafe {
                    let err = ffi::sqlite3_errmsg(db);
                    if !err.is_null() {
                        CStr::from_ptr(err).to_string_lossy().into_owned()
                    } else {
                        String::new()
                    }
                }
            } else {
                String::new()
            };

            // Close the partially opened connection
            if !db.is_null() {
                unsafe { ffi::sqlite3_close(db) };
            }

            return Err(Error::from_code_with_message(rc, msg));
        }

        Ok(Connection {
            raw: db,
            _marker: PhantomData,
        })
    }

    /// Create a new VDBE program builder
    ///
    /// This creates a new VDBE program that can be populated with instructions
    /// and executed without SQL parsing.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use sqlite_vdbe::{Connection, Insn};
    ///
    /// let mut conn = Connection::open_in_memory()?;
    /// let mut builder = conn.new_program()?;
    ///
    /// // Add instructions...
    /// let reg = builder.alloc_register();
    /// builder.add(Insn::Integer { value: 42, dest: reg });
    ///
    /// # Ok::<(), sqlite_vdbe::Error>(())
    /// ```
    pub fn new_program(&mut self) -> Result<ProgramBuilder> {
        ProgramBuilder::new(self.raw)
    }

    /// Get the last error message from the connection
    pub fn last_error(&self) -> Option<String> {
        unsafe {
            let err = ffi::sqlite3_errmsg(self.raw);
            if err.is_null() {
                None
            } else {
                Some(CStr::from_ptr(err).to_string_lossy().into_owned())
            }
        }
    }

    /// Get the last error code from the connection
    pub fn last_error_code(&self) -> i32 {
        unsafe { ffi::sqlite3_errcode(self.raw) }
    }

    /// Get the raw connection pointer
    ///
    /// # Safety
    ///
    /// The returned pointer is valid as long as the `Connection` is alive.
    /// Do not call `sqlite3_close` on this pointer.
    pub unsafe fn raw_ptr(&self) -> *mut ffi::sqlite3 {
        self.raw
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe {
                // Use close_v2 which handles zombie connections gracefully
                ffi::sqlite3_close_v2(self.raw);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_in_memory() {
        let conn = Connection::open_in_memory();
        assert!(conn.is_ok());
    }

    #[test]
    fn test_open_invalid_path() {
        // Try to open a non-existent directory
        let result = Connection::open_with_flags(
            "/nonexistent/path/to/db.sqlite",
            ffi::SQLITE_OPEN_READONLY,
        );
        assert!(result.is_err());
    }
}
