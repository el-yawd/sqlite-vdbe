//! # sqlite-vdbe
//!
//! Low-level access to SQLite's VDBE (Virtual Database Engine) bytecode.
//!
//! This crate allows you to create and execute VDBE programs directly,
//! without going through SQL parsing. This is useful for:
//!
//! - Building custom query engines on top of SQLite's storage
//! - Testing VDBE behavior
//! - Learning how SQLite works internally
//! - Implementing specialized database operations
//!
//! ## Example
//!
//! ```no_run
//! use sqlite_vdbe::{Connection, Insn, StepResult};
//!
//! fn main() -> sqlite_vdbe::Result<()> {
//!     // Open an in-memory database
//!     let mut conn = Connection::open_in_memory()?;
//!
//!     // Create a program that computes 1 + 2
//!     let mut builder = conn.new_program()?;
//!
//!     // Allocate registers
//!     let r1 = builder.alloc_register();  // Will hold 1
//!     let r2 = builder.alloc_register();  // Will hold 2
//!     let r3 = builder.alloc_register();  // Will hold result
//!
//!     // Build the program using the ergonomic instruction API
//!     builder.add(Insn::Integer { value: 1, dest: r1 });
//!     builder.add(Insn::Integer { value: 2, dest: r2 });
//!     builder.add(Insn::Add { lhs: r1, rhs: r2, dest: r3 });
//!     builder.add(Insn::ResultRow { start: r3, count: 1 });
//!     builder.add(Insn::Halt);
//!
//!     // Finish building and execute
//!     let mut program = builder.finish(1)?;
//!
//!     match program.step()? {
//!         StepResult::Row => {
//!             let value = program.column_int(0);
//!             println!("Result: {}", value);  // Prints: Result: 3
//!         }
//!         StepResult::Done => {
//!             println!("No results");
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ## VDBE Overview
//!
//! The VDBE is a register-based virtual machine. Key concepts:
//!
//! - **Registers**: Memory cells that hold values (integers, floats, strings, blobs, NULL)
//! - **Instructions**: Operations that manipulate registers and database state
//! - **Cursors**: Handles for iterating over database tables/indexes
//! - **Program**: A sequence of instructions that can be executed
//!
//! ### Register Conventions
//!
//! - Registers are 1-based (register 0 is reserved)
//! - Use `builder.alloc_register()` to allocate registers
//! - Result values are stored in the register specified by the `dest` field
//!
//! ### Common Instruction Patterns
//!
//! ```no_run
//! use sqlite_vdbe::Insn;
//!
//! // Load a constant integer
//! let load = Insn::Integer { value: 42, dest: 1 };
//!
//! // Arithmetic: dest = lhs + rhs
//! let add = Insn::Add { lhs: 1, rhs: 2, dest: 3 };
//!
//! // Output a row of results
//! let output = Insn::ResultRow { start: 1, count: 3 };
//!
//! // Halt execution
//! let halt = Insn::Halt;
//!
//! // Unconditional jump
//! let jump = Insn::Goto { target: 10 };
//!
//! // Conditional jump
//! let branch = Insn::If { src: 1, target: 20, jump_if_null: false };
//! ```
//!
//! ## Thread Safety
//!
//! This crate is designed for single-threaded use. `Connection`, `ProgramBuilder`,
//! and `Program` are all `!Send` and `!Sync`. SQLite connections should only be
//! used from the thread that created them.
//!
//! ## Safety
//!
//! This crate uses `unsafe` internally to call into SQLite's C API. The safe
//! Rust wrappers ensure proper memory management and prevent common errors.

pub mod connection;
pub mod error;
pub mod ffi;
pub mod insn;
pub mod program;
pub mod value;

// Legacy module - kept for backwards compatibility
#[doc(hidden)]
pub mod opcode {
    pub use crate::insn::RawOpcode as Opcode;
}

// Re-export main types at crate root
pub use connection::Connection;
pub use error::{Error, Result};
pub use insn::{Insn, P4, RawOpcode};
pub use program::{Address, Program, ProgramBuilder, StepResult};
pub use value::Value;

// Legacy re-export for backwards compatibility
#[doc(hidden)]
pub use insn::RawOpcode as Opcode;

// Re-export FFI constants that users might need
pub use ffi::{
    SQLITE_BLOB, SQLITE_DONE, SQLITE_ERROR, SQLITE_FLOAT, SQLITE_INTEGER, SQLITE_NULL, SQLITE_OK,
    SQLITE_ROW, SQLITE_TEXT,
};
