//! VDBE program building and execution
//!
//! This module provides the core types for building and executing VDBE programs:
//!
//! - [`ProgramBuilder`] - Build programs by adding instructions
//! - [`Program`] - Execute programs and retrieve results
//! - [`InsnRecord`] - Inspect recorded instructions for display/debugging
//!
//! # EXPLAIN Format Display
//!
//! The [`Program`] type implements [`Display`](std::fmt::Display) to output
//! bytecode in SQLite's `EXPLAIN` format:
//!
//! ```text
//! addr  opcode         p1    p2    p3    p4             p5  comment
//! ----  -------------  ----  ----  ----  -------------  --  -------------
//! 0     Init           0     4     0                    0   Start at 4
//! 1     Add            2     2     1                    0   r[1]=r[2]+r[2]
//! 2     ResultRow      1     1     0                    0   output=r[1]
//! 3     Halt           0     0     0                    0
//! ```
//!
//! Use [`ProgramBuilder::add_with_comment`] to attach comments to instructions.

use std::ffi::CString;
use std::fmt;
use std::marker::PhantomData;

use crate::error::{Error, Result};
use crate::ffi;
use crate::insn::{Insn, InsnP4};
use crate::value::Value;

// Re-export for backwards compatibility
#[doc(hidden)]
pub use crate::insn::RawOpcode as Opcode;

/// Address of an instruction in the VDBE program
///
/// Addresses are 0-based indices into the opcode array.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Address(pub i32);

impl Address {
    /// Get the raw address value
    #[inline]
    pub fn raw(&self) -> i32 {
        self.0
    }
}

impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "@{}", self.0)
    }
}

/// Record of an instruction for display and inspection
///
/// This stores the information needed to display instructions in SQLite's
/// EXPLAIN format. Each record captures the opcode name, operands (P1-P5),
/// and an optional comment.
///
/// # Example
///
/// ```no_run
/// use sqlite_vdbe::{Connection, Insn};
///
/// let mut conn = Connection::open_in_memory()?;
/// let mut builder = conn.new_program()?;
///
/// let r1 = builder.alloc_register();
/// builder.add_with_comment(Insn::Integer { value: 42, dest: r1 }, "the answer");
/// builder.add(Insn::Halt);
///
/// let program = builder.finish(1)?;
///
/// // Access individual instruction records
/// for insn in program.instructions() {
///     println!("{}: p1={}, p2={}", insn.opcode, insn.p1, insn.p2);
/// }
/// # Ok::<(), sqlite_vdbe::Error>(())
/// ```
#[derive(Debug, Clone)]
pub struct InsnRecord {
    /// Opcode name (e.g., "Add", "Integer", "Goto")
    pub opcode: String,
    /// P1 operand
    pub p1: i32,
    /// P2 operand
    pub p2: i32,
    /// P3 operand
    pub p3: i32,
    /// P4 operand as string (empty if not used)
    pub p4: String,
    /// P5 operand
    pub p5: u16,
    /// Optional comment for display
    pub comment: String,
}

/// A VDBE program under construction
///
/// Use this to build a VDBE program by adding instructions one at a time.
/// When finished, call `finish()` to get an executable `Program`.
///
/// # Example
///
/// ```no_run
/// use sqlite_vdbe::{Connection, Insn};
///
/// let mut conn = Connection::open_in_memory()?;
/// let mut builder = conn.new_program()?;
///
/// // Allocate registers
/// let r1 = builder.alloc_register();
/// let r2 = builder.alloc_register();
/// let r3 = builder.alloc_register();
///
/// // Build program: compute 1 + 2
/// builder.add(Insn::Integer { value: 1, dest: r1 });
/// builder.add(Insn::Integer { value: 2, dest: r2 });
/// builder.add(Insn::Add { lhs: r1, rhs: r2, dest: r3 });
/// builder.add(Insn::ResultRow { start: r3, count: 1 });
/// builder.add(Insn::Halt);
///
/// // Finish and execute
/// let mut program = builder.finish(1)?;
///
/// # Ok::<(), sqlite_vdbe::Error>(())
/// ```
pub struct ProgramBuilder {
    raw: *mut ffi::Vdbe,
    db: *mut ffi::sqlite3,
    next_register: i32,
    next_cursor: i32,
    /// Recorded instructions for display purposes
    instructions: Vec<InsnRecord>,
    // Mark as !Send and !Sync
    _marker: PhantomData<*const ()>,
}

impl ProgramBuilder {
    /// Create a new program builder
    ///
    /// This is called internally by `Connection::new_program()`.
    pub(crate) fn new(db: *mut ffi::sqlite3) -> Result<Self> {
        let raw = unsafe { ffi::sqlite3_vdbe_create(db) };
        if raw.is_null() {
            return Err(Error::AllocationFailed);
        }

        Ok(ProgramBuilder {
            raw,
            db,
            next_register: 1, // Register 0 is reserved
            next_cursor: 0,
            instructions: Vec::new(),
            _marker: PhantomData,
        })
    }

    /// Allocate a register and return its index
    ///
    /// Registers are 1-based (register 0 is reserved).
    pub fn alloc_register(&mut self) -> i32 {
        let reg = self.next_register;
        self.next_register += 1;
        reg
    }

    /// Allocate multiple consecutive registers
    ///
    /// Returns the index of the first register. The allocated registers
    /// are `first..first+count`.
    pub fn alloc_registers(&mut self, count: i32) -> i32 {
        let first = self.next_register;
        self.next_register += count;
        first
    }

    /// Allocate a cursor slot and return its index
    ///
    /// Cursors are 0-based.
    pub fn alloc_cursor(&mut self) -> i32 {
        let cursor = self.next_cursor;
        self.next_cursor += 1;
        cursor
    }

    /// Get the number of registers allocated so far
    pub fn register_count(&self) -> i32 {
        self.next_register
    }

    /// Get the number of cursors allocated so far
    pub fn cursor_count(&self) -> i32 {
        self.next_cursor
    }

    /// Add an instruction to the program
    ///
    /// This is the primary method for building VDBE programs. Each instruction
    /// variant contains all the operands needed for that specific opcode.
    ///
    /// Returns the address of the added instruction.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use sqlite_vdbe::{Connection, Insn};
    ///
    /// let mut conn = Connection::open_in_memory()?;
    /// let mut builder = conn.new_program()?;
    ///
    /// let r1 = builder.alloc_register();
    /// let r2 = builder.alloc_register();
    /// let r3 = builder.alloc_register();
    ///
    /// builder.add(Insn::Integer { value: 10, dest: r1 });
    /// builder.add(Insn::Integer { value: 32, dest: r2 });
    /// builder.add(Insn::Add { lhs: r1, rhs: r2, dest: r3 });
    /// builder.add(Insn::ResultRow { start: r3, count: 1 });
    /// builder.add(Insn::Halt);
    ///
    /// # Ok::<(), sqlite_vdbe::Error>(())
    /// ```
    pub fn add(&mut self, insn: Insn) -> Address {
        self.add_with_comment(insn, "")
    }

    /// Add an instruction to the program with a comment
    ///
    /// Same as `add()`, but allows attaching a comment that will be displayed
    /// when the program is printed in EXPLAIN format.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use sqlite_vdbe::{Connection, Insn};
    ///
    /// let mut conn = Connection::open_in_memory()?;
    /// let mut builder = conn.new_program()?;
    ///
    /// let r1 = builder.alloc_register();
    /// builder.add_with_comment(Insn::Integer { value: 42, dest: r1 }, "r[1]=42");
    ///
    /// # Ok::<(), sqlite_vdbe::Error>(())
    /// ```
    pub fn add_with_comment(&mut self, insn: Insn, comment: &str) -> Address {
        let opcode = insn.raw_opcode() as i32;
        let (p1, p2, p3, p5) = insn.operands();
        let name = insn.name().to_string();

        // Get P4 string representation for display
        let p4_str = match insn.p4() {
            Some(InsnP4::Int(i)) => i.to_string(),
            Some(InsnP4::Int64(i)) => i.to_string(),
            Some(InsnP4::Real(r)) => format!("{:?}", r),
            Some(InsnP4::String(ref s)) => s.clone(),
            None => String::new(),
        };

        // Record instruction for display
        self.instructions.push(InsnRecord {
            opcode: name,
            p1,
            p2,
            p3,
            p4: p4_str,
            p5,
            comment: comment.to_string(),
        });

        // Handle P4 if present
        if let Some(p4) = insn.p4() {
            let addr = match p4 {
                InsnP4::Int(i) => unsafe {
                    ffi::sqlite3VdbeAddOp4Int(self.raw, opcode, p1, p2, p3, i)
                },
                InsnP4::Int64(i) => unsafe {
                    // For Int64, we need to use sqlite3_malloc because SQLite will
                    // call sqlite3_free on the P4 pointer when the VDBE is finalized
                    let addr = ffi::sqlite3VdbeAddOp2(self.raw, opcode, p1, p2);
                    let ptr = ffi::sqlite3_malloc(std::mem::size_of::<i64>() as i32);
                    if !ptr.is_null() {
                        *(ptr as *mut i64) = i;
                        ffi::sqlite3VdbeChangeP4(self.raw, addr, ptr as *const i8, ffi::P4_INT64);
                    }
                    addr
                },
                InsnP4::Real(r) => unsafe {
                    // For Real, we need to use sqlite3_malloc because SQLite will
                    // call sqlite3_free on the P4 pointer when the VDBE is finalized
                    let addr = ffi::sqlite3VdbeAddOp2(self.raw, opcode, p1, p2);
                    let ptr = ffi::sqlite3_malloc(std::mem::size_of::<f64>() as i32);
                    if !ptr.is_null() {
                        *(ptr as *mut f64) = r;
                        ffi::sqlite3VdbeChangeP4(self.raw, addr, ptr as *const i8, ffi::P4_REAL);
                    }
                    addr
                },
                InsnP4::String(ref s) => {
                    if let Ok(c_str) = CString::new(s.as_str()) {
                        let bytes = c_str.as_bytes_with_nul();
                        unsafe {
                            // Allocate with sqlite3_malloc so SQLite can free it
                            let ptr = ffi::sqlite3_malloc(bytes.len() as i32);
                            if !ptr.is_null() {
                                std::ptr::copy_nonoverlapping(
                                    bytes.as_ptr(),
                                    ptr as *mut u8,
                                    bytes.len(),
                                );
                                ffi::sqlite3VdbeAddOp4(
                                    self.raw,
                                    opcode,
                                    p1,
                                    p2,
                                    p3,
                                    ptr as *const i8,
                                    ffi::P4_DYNAMIC,
                                )
                            } else {
                                // Allocation failed, fall back to op3
                                ffi::sqlite3VdbeAddOp3(self.raw, opcode, p1, p2, p3)
                            }
                        }
                    } else {
                        // Fallback to op3 if string conversion fails
                        unsafe { ffi::sqlite3VdbeAddOp3(self.raw, opcode, p1, p2, p3) }
                    }
                }
            };
            if p5 != 0 {
                unsafe { ffi::sqlite3VdbeChangeP5(self.raw, p5) };
            }
            return Address(addr);
        }

        // No P4, use appropriate add function
        let addr = if p3 != 0 {
            unsafe { ffi::sqlite3VdbeAddOp3(self.raw, opcode, p1, p2, p3) }
        } else if p2 != 0 || matches!(insn, Insn::Goto { .. } | Insn::Integer { value: 0, .. }) {
            unsafe { ffi::sqlite3VdbeAddOp2(self.raw, opcode, p1, p2) }
        } else if p1 != 0 {
            unsafe { ffi::sqlite3VdbeAddOp1(self.raw, opcode, p1) }
        } else {
            unsafe { ffi::sqlite3VdbeAddOp0(self.raw, opcode) }
        };

        if p5 != 0 {
            unsafe { ffi::sqlite3VdbeChangeP5(self.raw, p5) };
        }

        Address(addr)
    }

    // =========================================================================
    // Legacy API (kept for backwards compatibility)
    // =========================================================================

    /// Add an opcode with no operands
    ///
    /// **Deprecated**: Use `add(Insn::...)` instead.
    ///
    /// Returns the address of the added instruction.
    pub fn add_op0(&mut self, op: Opcode) -> Address {
        let addr = unsafe { ffi::sqlite3VdbeAddOp0(self.raw, op as i32) };
        Address(addr)
    }

    /// Add an opcode with one operand (P1)
    ///
    /// Returns the address of the added instruction.
    pub fn add_op1(&mut self, op: Opcode, p1: i32) -> Address {
        let addr = unsafe { ffi::sqlite3VdbeAddOp1(self.raw, op as i32, p1) };
        Address(addr)
    }

    /// Add an opcode with two operands (P1, P2)
    ///
    /// Returns the address of the added instruction.
    pub fn add_op2(&mut self, op: Opcode, p1: i32, p2: i32) -> Address {
        let addr = unsafe { ffi::sqlite3VdbeAddOp2(self.raw, op as i32, p1, p2) };
        Address(addr)
    }

    /// Add an opcode with three operands (P1, P2, P3)
    ///
    /// Returns the address of the added instruction.
    pub fn add_op3(&mut self, op: Opcode, p1: i32, p2: i32, p3: i32) -> Address {
        let addr = unsafe { ffi::sqlite3VdbeAddOp3(self.raw, op as i32, p1, p2, p3) };
        Address(addr)
    }

    /// Add an opcode with P4 as a 32-bit integer
    ///
    /// Returns the address of the added instruction.
    pub fn add_op4_int(&mut self, op: Opcode, p1: i32, p2: i32, p3: i32, p4: i32) -> Address {
        let addr = unsafe { ffi::sqlite3VdbeAddOp4Int(self.raw, op as i32, p1, p2, p3, p4) };
        Address(addr)
    }

    /// Add an opcode with P4 as a string
    ///
    /// The string is copied into SQLite-managed memory.
    /// Returns the address of the added instruction.
    pub fn add_op4_str(
        &mut self,
        op: Opcode,
        p1: i32,
        p2: i32,
        p3: i32,
        p4: &str,
    ) -> Result<Address> {
        let c_str = CString::new(p4)?;
        let addr = unsafe {
            ffi::sqlite3VdbeAddOp4(
                self.raw,
                op as i32,
                p1,
                p2,
                p3,
                c_str.as_ptr(),
                ffi::P4_DYNAMIC, // SQLite will take ownership and free
            )
        };
        Ok(Address(addr))
    }

    /// Set the P5 flags on the last added instruction
    pub fn change_p5(&mut self, p5: u16) {
        unsafe {
            ffi::sqlite3VdbeChangeP5(self.raw, p5);
        }
    }

    /// Get the address of the next instruction to be added
    pub fn current_addr(&self) -> Address {
        let addr = unsafe { ffi::sqlite3VdbeCurrentAddr(self.raw) };
        Address(addr)
    }

    /// Make the instruction at `addr` jump to the current position
    ///
    /// This is useful for patching forward jumps after the target is known.
    pub fn jump_here(&mut self, addr: Address) {
        unsafe {
            ffi::sqlite3VdbeJumpHere(self.raw, addr.0);
        }
    }

    /// Create a label for forward jumps
    ///
    /// Labels are negative numbers that get resolved to actual addresses
    /// when `resolve_label()` is called.
    pub fn make_label(&mut self) -> i32 {
        unsafe { ffi::sqlite3_vdbe_make_label(self.raw) }
    }

    /// Resolve a label to the current address
    ///
    /// After this call, any instructions that jump to the label will
    /// jump to the current position.
    pub fn resolve_label(&mut self, label: i32) {
        unsafe {
            ffi::sqlite3_vdbe_resolve_label(self.raw, label);
        }
    }

    /// Get the number of opcodes currently in the program
    pub fn op_count(&self) -> i32 {
        unsafe { ffi::sqlite3_vdbe_op_count(self.raw) }
    }

    /// Finish building the program and prepare for execution
    ///
    /// # Arguments
    ///
    /// * `num_columns` - Number of result columns (for ResultRow opcode)
    ///
    /// # Returns
    ///
    /// An executable `Program` that can be stepped through.
    pub fn finish(mut self, num_columns: u16) -> Result<Program> {
        unsafe {
            // Set the number of result columns
            ffi::sqlite3VdbeSetNumCols(self.raw, num_columns as i32);

            // Prepare the program for execution
            ffi::sqlite3_vdbe_make_ready(self.raw, self.next_register, self.next_cursor);
        }

        // Take ownership of instructions before forget
        let instructions = std::mem::take(&mut self.instructions);

        // Transfer ownership to Program (don't drop the Vdbe here)
        let program = Program {
            raw: self.raw,
            db: self.db,
            done: false,
            instructions,
            _marker: PhantomData,
        };

        // Prevent the builder from finalizing the Vdbe
        std::mem::forget(self);

        Ok(program)
    }

    /// Get the raw Vdbe pointer (for advanced use)
    ///
    /// # Safety
    ///
    /// The returned pointer is valid as long as the builder is alive.
    pub unsafe fn raw_ptr(&self) -> *mut ffi::Vdbe {
        self.raw
    }
}

impl Drop for ProgramBuilder {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe {
                // Finalize the Vdbe to clean up resources
                ffi::sqlite3_finalize(self.raw as *mut ffi::sqlite3_stmt);
            }
        }
    }
}

/// Result of a single step in program execution
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepResult {
    /// A row of data is available
    Row,
    /// Execution completed successfully
    Done,
}

/// A prepared VDBE program ready for execution
///
/// Execute the program by calling `step()` repeatedly until it returns
/// `StepResult::Done`. After each `Row` result, use the `column_*` methods
/// to retrieve values.
///
/// # Example
///
/// ```no_run
/// use sqlite_vdbe::{Connection, Insn, StepResult};
///
/// let mut conn = Connection::open_in_memory()?;
/// let mut builder = conn.new_program()?;
///
/// let r1 = builder.alloc_register();
/// builder.add(Insn::Integer { value: 42, dest: r1 });
/// builder.add(Insn::ResultRow { start: r1, count: 1 });
/// builder.add(Insn::Halt);
///
/// let mut program = builder.finish(1)?;
///
/// while let StepResult::Row = program.step()? {
///     let value = program.column_int(0);
///     println!("Got value: {}", value);
/// }
///
/// # Ok::<(), sqlite_vdbe::Error>(())
/// ```
pub struct Program {
    raw: *mut ffi::Vdbe,
    db: *mut ffi::sqlite3,
    done: bool,
    /// Recorded instructions for display purposes
    instructions: Vec<InsnRecord>,
    // Mark as !Send and !Sync
    _marker: PhantomData<*const ()>,
}

impl Program {
    /// Execute one step of the program
    ///
    /// Returns `Row` if a result row is available (retrieve with `column_*`),
    /// or `Done` if execution completed.
    pub fn step(&mut self) -> Result<StepResult> {
        let rc = unsafe { ffi::sqlite3_step(self.raw as *mut ffi::sqlite3_stmt) };

        match rc {
            ffi::SQLITE_ROW => Ok(StepResult::Row),
            ffi::SQLITE_DONE => {
                self.done = true;
                Ok(StepResult::Done)
            }
            _ => {
                // Get error message from connection
                let msg = unsafe {
                    let err = ffi::sqlite3_errmsg(self.db);
                    if err.is_null() {
                        String::new()
                    } else {
                        std::ffi::CStr::from_ptr(err).to_string_lossy().into_owned()
                    }
                };
                Err(Error::from_code_with_message(rc, msg))
            }
        }
    }

    /// Check if execution has completed
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// Get the number of columns in the result set
    pub fn column_count(&self) -> i32 {
        unsafe { ffi::sqlite3_column_count(self.raw as *mut ffi::sqlite3_stmt) }
    }

    /// Get the type of column at index
    ///
    /// Returns one of: SQLITE_INTEGER, SQLITE_FLOAT, SQLITE_TEXT, SQLITE_BLOB, SQLITE_NULL
    pub fn column_type(&self, idx: i32) -> i32 {
        unsafe { ffi::sqlite3_column_type(self.raw as *mut ffi::sqlite3_stmt, idx) }
    }

    /// Get column value as a 32-bit integer
    pub fn column_int(&self, idx: i32) -> i32 {
        unsafe { ffi::sqlite3_column_int(self.raw as *mut ffi::sqlite3_stmt, idx) }
    }

    /// Get column value as a 64-bit integer
    pub fn column_int64(&self, idx: i32) -> i64 {
        unsafe { ffi::sqlite3_column_int64(self.raw as *mut ffi::sqlite3_stmt, idx) }
    }

    /// Get column value as a double (64-bit float)
    pub fn column_double(&self, idx: i32) -> f64 {
        unsafe { ffi::sqlite3_column_double(self.raw as *mut ffi::sqlite3_stmt, idx) }
    }

    /// Get column value as text (UTF-8)
    ///
    /// Returns `None` if the column is NULL.
    pub fn column_text(&self, idx: i32) -> Option<&str> {
        unsafe {
            let ptr = ffi::sqlite3_column_text(self.raw as *mut ffi::sqlite3_stmt, idx);
            if ptr.is_null() {
                return None;
            }
            let bytes = ffi::sqlite3_column_bytes(self.raw as *mut ffi::sqlite3_stmt, idx);
            let slice = std::slice::from_raw_parts(ptr, bytes as usize);
            std::str::from_utf8(slice).ok()
        }
    }

    /// Get column value as a blob (binary data)
    ///
    /// Returns `None` if the column is NULL.
    pub fn column_blob(&self, idx: i32) -> Option<&[u8]> {
        unsafe {
            let ptr = ffi::sqlite3_column_blob(self.raw as *mut ffi::sqlite3_stmt, idx);
            if ptr.is_null() {
                return None;
            }
            let bytes = ffi::sqlite3_column_bytes(self.raw as *mut ffi::sqlite3_stmt, idx);
            Some(std::slice::from_raw_parts(ptr as *const u8, bytes as usize))
        }
    }

    /// Get column value as a `Value`
    ///
    /// Automatically determines the type and returns the appropriate variant.
    pub fn column_value(&self, idx: i32) -> Value {
        let col_type = self.column_type(idx);
        match col_type {
            ffi::SQLITE_INTEGER => Value::Integer(self.column_int64(idx)),
            ffi::SQLITE_FLOAT => Value::Real(self.column_double(idx)),
            ffi::SQLITE_TEXT => self
                .column_text(idx)
                .map(|s| Value::Text(s.to_string()))
                .unwrap_or(Value::Null),
            ffi::SQLITE_BLOB => self
                .column_blob(idx)
                .map(|b| Value::Blob(b.to_vec()))
                .unwrap_or(Value::Null),
            ffi::SQLITE_NULL => Value::Null,
            _ => Value::Null,
        }
    }

    /// Reset the program for re-execution
    ///
    /// After reset, the program can be stepped through again from the beginning.
    pub fn reset(&mut self) {
        unsafe {
            ffi::sqlite3_reset(self.raw as *mut ffi::sqlite3_stmt);
        }
        self.done = false;
    }

    /// Clear all bound parameters
    pub fn clear_bindings(&mut self) {
        unsafe {
            ffi::sqlite3_clear_bindings(self.raw as *mut ffi::sqlite3_stmt);
        }
    }

    /// Get the current VDBE state
    ///
    /// Returns one of: VDBE_INIT_STATE, VDBE_READY_STATE, VDBE_RUN_STATE, VDBE_HALT_STATE
    pub fn state(&self) -> i32 {
        unsafe { ffi::sqlite3_vdbe_state(self.raw) }
    }

    /// Get the number of registers in the program
    pub fn register_count(&self) -> i32 {
        unsafe { ffi::sqlite3_vdbe_mem_count(self.raw) }
    }

    /// Set a register value to an integer
    ///
    /// Note: This is for advanced use and should be called carefully.
    pub fn set_register_int(&mut self, reg: i32, value: i64) -> Result<()> {
        let rc = unsafe { ffi::sqlite3_vdbe_set_int(self.raw, reg, value) };
        if rc == ffi::SQLITE_OK {
            Ok(())
        } else {
            Err(Error::RegisterOutOfBounds {
                index: reg,
                max: self.register_count(),
            })
        }
    }

    /// Get an integer value from a register
    ///
    /// Note: This is for advanced use.
    pub fn get_register_int(&self, reg: i32) -> i64 {
        unsafe { ffi::sqlite3_vdbe_get_int(self.raw, reg) }
    }

    /// Set a register value to a double
    pub fn set_register_double(&mut self, reg: i32, value: f64) -> Result<()> {
        let rc = unsafe { ffi::sqlite3_vdbe_set_double(self.raw, reg, value) };
        if rc == ffi::SQLITE_OK {
            Ok(())
        } else {
            Err(Error::RegisterOutOfBounds {
                index: reg,
                max: self.register_count(),
            })
        }
    }

    /// Get a double value from a register
    pub fn get_register_double(&self, reg: i32) -> f64 {
        unsafe { ffi::sqlite3_vdbe_get_double(self.raw, reg) }
    }

    /// Set a register to NULL
    pub fn set_register_null(&mut self, reg: i32) -> Result<()> {
        let rc = unsafe { ffi::sqlite3_vdbe_set_null(self.raw, reg) };
        if rc == ffi::SQLITE_OK {
            Ok(())
        } else {
            Err(Error::RegisterOutOfBounds {
                index: reg,
                max: self.register_count(),
            })
        }
    }

    /// Check if a register value is NULL
    pub fn is_register_null(&self, reg: i32) -> bool {
        unsafe { ffi::sqlite3_vdbe_is_null(self.raw, reg) != 0 }
    }

    /// Get the raw Vdbe pointer (for advanced use)
    ///
    /// # Safety
    ///
    /// The returned pointer is valid as long as the Program is alive.
    pub unsafe fn raw_ptr(&self) -> *mut ffi::Vdbe {
        self.raw
    }

    /// Get the recorded instructions
    ///
    /// Returns a slice of all instructions that were added to this program.
    pub fn instructions(&self) -> &[InsnRecord] {
        &self.instructions
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Header
        writeln!(
            f,
            "{:<6}{:<15}{:<6}{:<6}{:<6}{:<15}{:<4}comment",
            "addr", "opcode", "p1", "p2", "p3", "p4", "p5"
        )?;
        writeln!(
            f,
            "{:<6}{:<15}{:<6}{:<6}{:<6}{:<15}{:<4}-------------",
            "----", "-------------", "----", "----", "----", "-------------", "--"
        )?;

        // Instructions
        for (addr, insn) in self.instructions.iter().enumerate() {
            writeln!(
                f,
                "{:<6}{:<15}{:<6}{:<6}{:<6}{:<15}{:<4}{}",
                addr, insn.opcode, insn.p1, insn.p2, insn.p3, insn.p4, insn.p5, insn.comment
            )?;
        }

        Ok(())
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe {
                ffi::sqlite3_finalize(self.raw as *mut ffi::sqlite3_stmt);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_address_display() {
        let addr = Address(42);
        assert_eq!(format!("{}", addr), "@42");
    }
}
