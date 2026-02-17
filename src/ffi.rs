//! FFI bindings to SQLite and VDBE internals
//!
//! This module contains raw C function declarations and type definitions.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use std::os::raw::{c_char, c_int, c_void};

// =============================================================================
// Opaque types
// =============================================================================

/// Opaque database connection handle
pub enum sqlite3 {}

/// Opaque prepared statement handle (actually a Vdbe*)
pub enum sqlite3_stmt {}

/// Opaque VDBE program handle
pub enum Vdbe {}

// =============================================================================
// Result codes
// =============================================================================

pub const SQLITE_OK: c_int = 0;
pub const SQLITE_ERROR: c_int = 1;
pub const SQLITE_INTERNAL: c_int = 2;
pub const SQLITE_PERM: c_int = 3;
pub const SQLITE_ABORT: c_int = 4;
pub const SQLITE_BUSY: c_int = 5;
pub const SQLITE_LOCKED: c_int = 6;
pub const SQLITE_NOMEM: c_int = 7;
pub const SQLITE_READONLY: c_int = 8;
pub const SQLITE_INTERRUPT: c_int = 9;
pub const SQLITE_IOERR: c_int = 10;
pub const SQLITE_CORRUPT: c_int = 11;
pub const SQLITE_NOTFOUND: c_int = 12;
pub const SQLITE_FULL: c_int = 13;
pub const SQLITE_CANTOPEN: c_int = 14;
pub const SQLITE_PROTOCOL: c_int = 15;
pub const SQLITE_EMPTY: c_int = 16;
pub const SQLITE_SCHEMA: c_int = 17;
pub const SQLITE_TOOBIG: c_int = 18;
pub const SQLITE_CONSTRAINT: c_int = 19;
pub const SQLITE_MISMATCH: c_int = 20;
pub const SQLITE_MISUSE: c_int = 21;
pub const SQLITE_NOLFS: c_int = 22;
pub const SQLITE_AUTH: c_int = 23;
pub const SQLITE_FORMAT: c_int = 24;
pub const SQLITE_RANGE: c_int = 25;
pub const SQLITE_NOTADB: c_int = 26;
pub const SQLITE_NOTICE: c_int = 27;
pub const SQLITE_WARNING: c_int = 28;
pub const SQLITE_ROW: c_int = 100;
pub const SQLITE_DONE: c_int = 101;

// =============================================================================
// Open flags
// =============================================================================

pub const SQLITE_OPEN_READONLY: c_int = 0x00000001;
pub const SQLITE_OPEN_READWRITE: c_int = 0x00000002;
pub const SQLITE_OPEN_CREATE: c_int = 0x00000004;
pub const SQLITE_OPEN_URI: c_int = 0x00000040;
pub const SQLITE_OPEN_MEMORY: c_int = 0x00000080;
pub const SQLITE_OPEN_NOMUTEX: c_int = 0x00008000;
pub const SQLITE_OPEN_FULLMUTEX: c_int = 0x00010000;
pub const SQLITE_OPEN_SHAREDCACHE: c_int = 0x00020000;
pub const SQLITE_OPEN_PRIVATECACHE: c_int = 0x00040000;

// =============================================================================
// P4 type constants (from vdbe.h)
// =============================================================================

pub const P4_NOTUSED: c_int = 0;
pub const P4_TRANSIENT: c_int = 0;
pub const P4_STATIC: c_int = -1;
pub const P4_COLLSEQ: c_int = -2;
pub const P4_INT32: c_int = -3;
pub const P4_SUBPROGRAM: c_int = -4;
pub const P4_TABLE: c_int = -5;
pub const P4_DYNAMIC: c_int = -6;
pub const P4_FUNCDEF: c_int = -7;
pub const P4_KEYINFO: c_int = -8;
pub const P4_EXPR: c_int = -9;
pub const P4_MEM: c_int = -10;
pub const P4_VTAB: c_int = -11;
pub const P4_REAL: c_int = -12;
pub const P4_INT64: c_int = -13;
pub const P4_INTARRAY: c_int = -14;
pub const P4_FUNCCTX: c_int = -15;
pub const P4_TABLEREF: c_int = -16;
pub const P4_SUBRTNSIG: c_int = -17;

// =============================================================================
// VDBE states
// =============================================================================

pub const VDBE_INIT_STATE: c_int = 0;
pub const VDBE_READY_STATE: c_int = 1;
pub const VDBE_RUN_STATE: c_int = 2;
pub const VDBE_HALT_STATE: c_int = 3;

// =============================================================================
// Column types
// =============================================================================

pub const SQLITE_INTEGER: c_int = 1;
pub const SQLITE_FLOAT: c_int = 2;
pub const SQLITE_TEXT: c_int = 3;
pub const SQLITE_BLOB: c_int = 4;
pub const SQLITE_NULL: c_int = 5;

extern "C" {
    // =========================================================================
    // Library initialization
    // =========================================================================

    /// Initialize the SQLite library
    ///
    /// This must be called before any other SQLite functions in a multi-threaded
    /// environment. It is safe to call multiple times.
    pub fn sqlite3_initialize() -> c_int;

    /// Shutdown the SQLite library
    pub fn sqlite3_shutdown() -> c_int;

    // =========================================================================
    // Memory allocation (using SQLite's allocator)
    // =========================================================================

    /// Allocate memory using SQLite's allocator
    ///
    /// Memory allocated with this function must be freed with `sqlite3_free`.
    pub fn sqlite3_malloc(n: c_int) -> *mut c_void;

    /// Allocate memory using SQLite's allocator (64-bit size)
    pub fn sqlite3_malloc64(n: u64) -> *mut c_void;

    /// Free memory allocated with `sqlite3_malloc`
    pub fn sqlite3_free(ptr: *mut c_void);

    // =========================================================================
    // Database connection management
    // =========================================================================

    pub fn sqlite3_open(filename: *const c_char, ppDb: *mut *mut sqlite3) -> c_int;

    pub fn sqlite3_open_v2(
        filename: *const c_char,
        ppDb: *mut *mut sqlite3,
        flags: c_int,
        zVfs: *const c_char,
    ) -> c_int;

    pub fn sqlite3_close(db: *mut sqlite3) -> c_int;

    pub fn sqlite3_close_v2(db: *mut sqlite3) -> c_int;

    pub fn sqlite3_errmsg(db: *mut sqlite3) -> *const c_char;

    pub fn sqlite3_errcode(db: *mut sqlite3) -> c_int;

    // =========================================================================
    // Statement execution (sqlite3_stmt is really Vdbe*)
    // =========================================================================

    pub fn sqlite3_step(pStmt: *mut sqlite3_stmt) -> c_int;

    pub fn sqlite3_reset(pStmt: *mut sqlite3_stmt) -> c_int;

    pub fn sqlite3_finalize(pStmt: *mut sqlite3_stmt) -> c_int;

    pub fn sqlite3_clear_bindings(pStmt: *mut sqlite3_stmt) -> c_int;

    // =========================================================================
    // Column access
    // =========================================================================

    pub fn sqlite3_column_count(pStmt: *mut sqlite3_stmt) -> c_int;

    pub fn sqlite3_column_type(pStmt: *mut sqlite3_stmt, i: c_int) -> c_int;

    pub fn sqlite3_column_int(pStmt: *mut sqlite3_stmt, i: c_int) -> c_int;

    pub fn sqlite3_column_int64(pStmt: *mut sqlite3_stmt, i: c_int) -> i64;

    pub fn sqlite3_column_double(pStmt: *mut sqlite3_stmt, i: c_int) -> f64;

    pub fn sqlite3_column_text(pStmt: *mut sqlite3_stmt, i: c_int) -> *const u8;

    pub fn sqlite3_column_blob(pStmt: *mut sqlite3_stmt, i: c_int) -> *const c_void;

    pub fn sqlite3_column_bytes(pStmt: *mut sqlite3_stmt, i: c_int) -> c_int;

    pub fn sqlite3_column_name(pStmt: *mut sqlite3_stmt, i: c_int) -> *const c_char;

    // =========================================================================
    // Internal VDBE functions (from vdbe.h)
    // These are normally internal but we expose them via vdbe_rust.c
    // =========================================================================

    pub fn sqlite3VdbeAddOp0(p: *mut Vdbe, op: c_int) -> c_int;

    pub fn sqlite3VdbeAddOp1(p: *mut Vdbe, op: c_int, p1: c_int) -> c_int;

    pub fn sqlite3VdbeAddOp2(p: *mut Vdbe, op: c_int, p1: c_int, p2: c_int) -> c_int;

    pub fn sqlite3VdbeAddOp3(
        p: *mut Vdbe,
        op: c_int,
        p1: c_int,
        p2: c_int,
        p3: c_int,
    ) -> c_int;

    pub fn sqlite3VdbeAddOp4(
        p: *mut Vdbe,
        op: c_int,
        p1: c_int,
        p2: c_int,
        p3: c_int,
        zP4: *const c_char,
        p4type: c_int,
    ) -> c_int;

    pub fn sqlite3VdbeAddOp4Int(
        p: *mut Vdbe,
        op: c_int,
        p1: c_int,
        p2: c_int,
        p3: c_int,
        p4: c_int,
    ) -> c_int;

    pub fn sqlite3VdbeChangeP5(p: *mut Vdbe, p5: u16);

    pub fn sqlite3VdbeChangeP4(p: *mut Vdbe, addr: c_int, zP4: *const c_char, n: c_int);

    pub fn sqlite3VdbeJumpHere(p: *mut Vdbe, addr: c_int);

    pub fn sqlite3VdbeCurrentAddr(p: *mut Vdbe) -> c_int;

    pub fn sqlite3VdbeSetNumCols(p: *mut Vdbe, nResColumn: c_int);

    pub fn sqlite3VdbeResolveLabel(p: *mut Vdbe, x: c_int);

    // =========================================================================
    // Custom helper functions from vdbe_rust.c
    // =========================================================================

    /// Create a new VDBE program without SQL parsing
    pub fn sqlite3_vdbe_create(db: *mut sqlite3) -> *mut Vdbe;

    /// Prepare a VDBE program for execution
    pub fn sqlite3_vdbe_make_ready(p: *mut Vdbe, nMem: c_int, nCursor: c_int);

    /// Get the number of opcodes in the program
    pub fn sqlite3_vdbe_op_count(p: *mut Vdbe) -> c_int;

    /// Get the current VDBE state
    pub fn sqlite3_vdbe_state(p: *mut Vdbe) -> c_int;

    /// Set a register to an integer value
    pub fn sqlite3_vdbe_set_int(p: *mut Vdbe, reg: c_int, value: i64) -> c_int;

    /// Get an integer value from a register
    pub fn sqlite3_vdbe_get_int(p: *mut Vdbe, reg: c_int) -> i64;

    /// Set a register to a double value
    pub fn sqlite3_vdbe_set_double(p: *mut Vdbe, reg: c_int, value: f64) -> c_int;

    /// Get a double value from a register
    pub fn sqlite3_vdbe_get_double(p: *mut Vdbe, reg: c_int) -> f64;

    /// Set a register to NULL
    pub fn sqlite3_vdbe_set_null(p: *mut Vdbe, reg: c_int) -> c_int;

    /// Check if a register value is NULL
    pub fn sqlite3_vdbe_is_null(p: *mut Vdbe, reg: c_int) -> c_int;

    /// Get the number of memory registers allocated
    pub fn sqlite3_vdbe_mem_count(p: *mut Vdbe) -> c_int;

    /// Get the number of cursors allocated
    pub fn sqlite3_vdbe_cursor_count(p: *mut Vdbe) -> c_int;

    /// Create a label for forward jumps
    pub fn sqlite3_vdbe_make_label(p: *mut Vdbe) -> c_int;

    /// Resolve a label to a specific address
    pub fn sqlite3_vdbe_resolve_label(p: *mut Vdbe, label: c_int);

    /// Test function that creates and runs a simple VDBE program
    /// Returns 42 if successful, or a negative error code
    pub fn sqlite3_vdbe_test_simple(db: *mut sqlite3) -> c_int;
}
