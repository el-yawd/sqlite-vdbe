//! VDBE Instructions with semantically named fields
//!
//! This module provides a high-level, ergonomic API for building VDBE programs.
//! Each instruction variant contains named fields that correspond to the
//! SQLite VDBE operands (P1, P2, P3, P4, P5).
//!
//! # Example
//!
//! ```no_run
//! use sqlite_vdbe::{Connection, Insn};
//!
//! let mut conn = Connection::open_in_memory()?;
//! let mut builder = conn.new_program()?;
//!
//! let r1 = builder.alloc_register();
//! let r2 = builder.alloc_register();
//! let r3 = builder.alloc_register();
//!
//! // Load constants
//! builder.add(Insn::Integer { value: 10, dest: r1 });
//! builder.add(Insn::Integer { value: 32, dest: r2 });
//!
//! // Add them
//! builder.add(Insn::Add { lhs: r1, rhs: r2, dest: r3 });
//!
//! // Output result
//! builder.add(Insn::ResultRow { start: r3, count: 1 });
//! builder.add(Insn::Halt);
//!
//! # Ok::<(), sqlite_vdbe::Error>(())
//! ```


/// Raw opcode values from SQLite's opcodes.h
///
/// These are the numeric values that map to SQLite's internal opcodes.
/// They must match the SQLite version being linked against (3.45.0).
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum RawOpcode {
    Savepoint = 0,
    AutoCommit = 1,
    Transaction = 2,
    Checkpoint = 3,
    JournalMode = 4,
    Vacuum = 5,
    VFilter = 6,
    VUpdate = 7,
    Init = 8,
    Goto = 9,
    Gosub = 10,
    InitCoroutine = 11,
    Yield = 12,
    MustBeInt = 13,
    Jump = 14,
    Once = 15,
    If = 16,
    IfNot = 17,
    IsType = 18,
    Not = 19,
    IfNullRow = 20,
    SeekLT = 21,
    SeekLE = 22,
    SeekGE = 23,
    SeekGT = 24,
    IfNotOpen = 25,
    IfNoHope = 26,
    NoConflict = 27,
    NotFound = 28,
    Found = 29,
    SeekRowid = 30,
    NotExists = 31,
    Last = 32,
    IfSmaller = 33,
    SorterSort = 34,
    Sort = 35,
    Rewind = 36,
    SorterNext = 37,
    Prev = 38,
    Next = 39,
    IdxLE = 40,
    IdxGT = 41,
    IdxLT = 42,
    Or = 43,
    And = 44,
    IdxGE = 45,
    RowSetRead = 46,
    RowSetTest = 47,
    Program = 48,
    FkIfZero = 49,
    IsNull = 50,
    NotNull = 51,
    Ne = 52,
    Eq = 53,
    Gt = 54,
    Le = 55,
    Lt = 56,
    Ge = 57,
    ElseEq = 58,
    IfPos = 59,
    IfNotZero = 60,
    DecrJumpZero = 61,
    IncrVacuum = 62,
    VNext = 63,
    Filter = 64,
    PureFunc = 65,
    Function = 66,
    Return = 67,
    EndCoroutine = 68,
    HaltIfNull = 69,
    Halt = 70,
    Integer = 71,
    Int64 = 72,
    String = 73,
    BeginSubrtn = 74,
    Null = 75,
    SoftNull = 76,
    Blob = 77,
    Variable = 78,
    Move = 79,
    Copy = 80,
    SCopy = 81,
    IntCopy = 82,
    FkCheck = 83,
    ResultRow = 84,
    CollSeq = 85,
    AddImm = 86,
    RealAffinity = 87,
    Cast = 88,
    Permutation = 89,
    Compare = 90,
    IsTrue = 91,
    ZeroOrNull = 92,
    Offset = 93,
    Column = 94,
    TypeCheck = 95,
    Affinity = 96,
    MakeRecord = 97,
    Count = 98,
    ReadCookie = 99,
    SetCookie = 100,
    ReopenIdx = 101,
    BitAnd = 102,
    BitOr = 103,
    ShiftLeft = 104,
    ShiftRight = 105,
    Add = 106,
    Subtract = 107,
    Multiply = 108,
    Divide = 109,
    Remainder = 110,
    Concat = 111,
    OpenRead = 112,
    OpenWrite = 113,
    BitNot = 114,
    OpenDup = 115,
    OpenAutoindex = 116,
    String8 = 117,
    OpenEphemeral = 118,
    SorterOpen = 119,
    SequenceTest = 120,
    OpenPseudo = 121,
    Close = 122,
    ColumnsUsed = 123,
    SeekScan = 124,
    SeekHit = 125,
    Sequence = 126,
    NewRowid = 127,
    Insert = 128,
    RowCell = 129,
    Delete = 130,
    ResetCount = 131,
    SorterCompare = 132,
    SorterData = 133,
    RowData = 134,
    Rowid = 135,
    NullRow = 136,
    SeekEnd = 137,
    IdxInsert = 138,
    SorterInsert = 139,
    IdxDelete = 140,
    DeferredSeek = 141,
    IdxRowid = 142,
    FinishSeek = 143,
    Destroy = 144,
    Clear = 145,
    ResetSorter = 146,
    CreateBtree = 147,
    SqlExec = 148,
    ParseSchema = 149,
    LoadAnalysis = 150,
    DropTable = 151,
    DropIndex = 152,
    Real = 153,
    DropTrigger = 154,
    IntegrityCk = 155,
    RowSetAdd = 156,
    Param = 157,
    MemMax = 159,
    OffsetLimit = 160,
    AggInverse = 161,
    AggStep = 162,
    AggStep1 = 163,
    AggValue = 164,
    AggFinal = 165,
    Expire = 166,
    CursorLock = 167,
    CursorUnlock = 168,
    TableLock = 169,
    VBegin = 170,
    VCreate = 171,
    VDestroy = 172,
    VOpen = 173,
    VCheck = 174,
    VInitIn = 175,
    VColumn = 176,
    VRename = 177,
    Pagecount = 178,
    MaxPgcnt = 179,
    ClrSubtype = 180,
    GetSubtype = 181,
    SetSubtype = 182,
    FilterAdd = 183,
    Trace = 184,
    CursorHint = 185,
    ReleaseReg = 186,
    Noop = 187,
    Explain = 188,
    Abortable = 189,
}

/// P4 parameter type for instructions that need it
#[derive(Debug, Clone)]
pub enum P4 {
    /// No P4 value
    None,
    /// Integer value
    Int(i32),
    /// String value (will be copied)
    String(String),
}

/// A VDBE instruction with semantically named fields
///
/// Each variant represents a specific VDBE opcode with its operands
/// named according to their semantic meaning.
#[derive(Debug, Clone)]
pub enum Insn {
    // =========================================================================
    // Constants - Load values into registers
    // =========================================================================

    /// Load a 32-bit integer constant into a register
    ///
    /// `dest = value`
    Integer {
        /// The integer value to store
        value: i32,
        /// Destination register
        dest: i32,
    },

    /// Load a 64-bit integer constant into a register
    ///
    /// `dest = value`
    Int64 {
        /// The integer value to store (in P4)
        value: i64,
        /// Destination register
        dest: i32,
    },

    /// Load a floating-point constant into a register
    ///
    /// `dest = value`
    Real {
        /// The floating-point value to store (in P4)
        value: f64,
        /// Destination register
        dest: i32,
    },

    /// Load a string constant into a register
    ///
    /// `dest = value`
    String8 {
        /// The string value to store (in P4)
        value: String,
        /// Destination register
        dest: i32,
    },

    /// Set registers to NULL
    ///
    /// Sets `dest` to NULL. If `count` > 1, sets registers `dest` through
    /// `dest + count - 1` to NULL.
    Null {
        /// First register to set to NULL
        dest: i32,
        /// Number of consecutive registers to set (default 1)
        count: i32,
    },

    // =========================================================================
    // Arithmetic - Binary operations on registers
    // =========================================================================

    /// Add two registers
    ///
    /// `dest = lhs + rhs`
    Add {
        /// Left operand register
        lhs: i32,
        /// Right operand register
        rhs: i32,
        /// Destination register
        dest: i32,
    },

    /// Subtract two registers
    ///
    /// `dest = lhs - rhs`
    ///
    /// Note: Maps to SQLite's OP_Subtract where P2-P1 is computed,
    /// so we swap the parameters internally.
    Subtract {
        /// Left operand register (minuend)
        lhs: i32,
        /// Right operand register (subtrahend)
        rhs: i32,
        /// Destination register
        dest: i32,
    },

    /// Multiply two registers
    ///
    /// `dest = lhs * rhs`
    Multiply {
        /// Left operand register
        lhs: i32,
        /// Right operand register
        rhs: i32,
        /// Destination register
        dest: i32,
    },

    /// Divide two registers
    ///
    /// `dest = lhs / rhs`
    ///
    /// Returns NULL if rhs is zero.
    Divide {
        /// Dividend register (numerator)
        lhs: i32,
        /// Divisor register (denominator)
        rhs: i32,
        /// Destination register
        dest: i32,
    },

    /// Compute remainder (modulo)
    ///
    /// `dest = lhs % rhs`
    Remainder {
        /// Dividend register
        lhs: i32,
        /// Divisor register
        rhs: i32,
        /// Destination register
        dest: i32,
    },

    /// Concatenate two strings
    ///
    /// `dest = lhs || rhs`
    Concat {
        /// Left string register
        lhs: i32,
        /// Right string register
        rhs: i32,
        /// Destination register
        dest: i32,
    },

    // =========================================================================
    // Bitwise Operations
    // =========================================================================

    /// Bitwise AND
    ///
    /// `dest = lhs & rhs`
    BitAnd {
        lhs: i32,
        rhs: i32,
        dest: i32,
    },

    /// Bitwise OR
    ///
    /// `dest = lhs | rhs`
    BitOr {
        lhs: i32,
        rhs: i32,
        dest: i32,
    },

    /// Left shift
    ///
    /// `dest = lhs << rhs`
    ShiftLeft {
        lhs: i32,
        rhs: i32,
        dest: i32,
    },

    /// Right shift
    ///
    /// `dest = lhs >> rhs`
    ShiftRight {
        lhs: i32,
        rhs: i32,
        dest: i32,
    },

    /// Bitwise NOT (one's complement)
    ///
    /// `dest = ~src`
    BitNot {
        /// Source register
        src: i32,
        /// Destination register
        dest: i32,
    },

    // =========================================================================
    // Logical Operations
    // =========================================================================

    /// Logical NOT
    ///
    /// `dest = NOT src`
    Not {
        /// Source register
        src: i32,
        /// Destination register
        dest: i32,
    },

    /// Add immediate value to register
    ///
    /// `dest += value`
    AddImm {
        /// Register to modify
        dest: i32,
        /// Immediate value to add
        value: i32,
    },

    // =========================================================================
    // Register Operations - Copy and move values
    // =========================================================================

    /// Deep copy a range of registers
    ///
    /// Copies `count` registers starting at `src` to registers starting at `dest`.
    Copy {
        /// Source register (first in range)
        src: i32,
        /// Destination register (first in range)
        dest: i32,
        /// Number of registers to copy
        count: i32,
    },

    /// Shallow copy a register
    ///
    /// For strings and blobs, only the pointer is copied (not the data).
    SCopy {
        /// Source register
        src: i32,
        /// Destination register
        dest: i32,
    },

    /// Move registers (copy then clear source)
    ///
    /// Copies `count` registers from `src` to `dest`, then sets source registers to NULL.
    Move {
        /// Source register (first in range)
        src: i32,
        /// Destination register (first in range)
        dest: i32,
        /// Number of registers to move
        count: i32,
    },

    /// Copy integer value only
    ///
    /// Like SCopy but specifically for integers.
    IntCopy {
        /// Source register
        src: i32,
        /// Destination register
        dest: i32,
    },

    // =========================================================================
    // Control Flow
    // =========================================================================

    /// Halt program execution
    ///
    /// Terminates the VDBE program. With no parameters, halts normally (success).
    Halt,

    /// Halt with error code
    ///
    /// Terminates with the specified error code and behavior.
    HaltWithError {
        /// Error code (SQLITE_OK for normal halt)
        error_code: i32,
        /// What to do on error (0=nothing, 1=fail, 2=abort, 3=rollback)
        on_error: i32,
    },

    /// Halt if register is NULL
    ///
    /// If the register is NULL, halt with an error.
    HaltIfNull {
        /// Register to test
        src: i32,
        /// Error code if NULL
        error_code: i32,
        /// Jump target if not NULL (0 = continue)
        target: i32,
    },

    /// Unconditional jump
    ///
    /// Jump to the specified address.
    Goto {
        /// Target instruction address
        target: i32,
    },

    /// Call subroutine
    ///
    /// Save return address in `return_reg` and jump to `target`.
    Gosub {
        /// Register to store return address
        return_reg: i32,
        /// Target instruction address
        target: i32,
    },

    /// Return from subroutine
    ///
    /// Jump to the address stored in the register.
    Return {
        /// Register containing return address
        return_reg: i32,
    },

    /// Jump if register is true (non-zero)
    ///
    /// If `src` contains a non-zero numeric value, jump to `target`.
    If {
        /// Register to test
        src: i32,
        /// Target address if true
        target: i32,
        /// If true, treat NULL as true; if false, treat NULL as false
        jump_if_null: bool,
    },

    /// Jump if register is false (zero or NULL)
    ///
    /// If `src` contains zero or NULL, jump to `target`.
    IfNot {
        /// Register to test
        src: i32,
        /// Target address if false
        target: i32,
        /// If true, treat NULL as true; if false, treat NULL as false
        jump_if_null: bool,
    },

    /// Jump if register is NULL
    IsNull {
        /// Register to test
        src: i32,
        /// Target address if NULL
        target: i32,
    },

    /// Jump if register is not NULL
    NotNull {
        /// Register to test
        src: i32,
        /// Target address if not NULL
        target: i32,
    },

    /// Execute once per prepared statement
    ///
    /// On first execution, falls through. On subsequent executions, jumps to target.
    Once {
        /// Target address to jump to on subsequent executions
        target: i32,
    },

    /// Three-way branch
    ///
    /// Jump based on value in a register:
    /// - If value < 0: jump to `neg`
    /// - If value == 0: jump to `zero`
    /// - If value > 0: jump to `pos`
    Jump {
        /// Target if negative
        neg: i32,
        /// Target if zero
        zero: i32,
        /// Target if positive
        pos: i32,
    },

    // =========================================================================
    // Comparison Operations - Compare and branch
    // =========================================================================

    /// Jump if equal
    ///
    /// Compare values in `lhs` and `rhs`, jump to `target` if equal.
    Eq {
        /// Left operand register
        lhs: i32,
        /// Right operand register
        rhs: i32,
        /// Target address if equal
        target: i32,
    },

    /// Jump if not equal
    ///
    /// Compare values in `lhs` and `rhs`, jump to `target` if not equal.
    Ne {
        /// Left operand register
        lhs: i32,
        /// Right operand register
        rhs: i32,
        /// Target address if not equal
        target: i32,
    },

    /// Jump if less than
    ///
    /// Jump to `target` if `lhs < rhs`.
    Lt {
        /// Left operand register
        lhs: i32,
        /// Right operand register
        rhs: i32,
        /// Target address if less than
        target: i32,
    },

    /// Jump if less than or equal
    ///
    /// Jump to `target` if `lhs <= rhs`.
    Le {
        /// Left operand register
        lhs: i32,
        /// Right operand register
        rhs: i32,
        /// Target address if less than or equal
        target: i32,
    },

    /// Jump if greater than
    ///
    /// Jump to `target` if `lhs > rhs`.
    Gt {
        /// Left operand register
        lhs: i32,
        /// Right operand register
        rhs: i32,
        /// Target address if greater than
        target: i32,
    },

    /// Jump if greater than or equal
    ///
    /// Jump to `target` if `lhs >= rhs`.
    Ge {
        /// Left operand register
        lhs: i32,
        /// Right operand register
        rhs: i32,
        /// Target address if greater than or equal
        target: i32,
    },

    // =========================================================================
    // Register Tests
    // =========================================================================

    /// Jump if register is positive
    ///
    /// If `src > 0`, jump to `target`.
    IfPos {
        /// Register to test
        src: i32,
        /// Target address if positive
        target: i32,
        /// Decrement amount (usually 0 or 1)
        decrement: i32,
    },

    /// Jump if register is not zero
    ///
    /// If `src != 0`, decrement and jump.
    IfNotZero {
        /// Register to test and decrement
        src: i32,
        /// Target address if not zero
        target: i32,
    },

    /// Decrement and jump if zero
    ///
    /// Decrement `src`, then jump to `target` if it becomes zero.
    DecrJumpZero {
        /// Register to decrement
        src: i32,
        /// Target address if zero after decrement
        target: i32,
    },

    /// Force register to integer type
    ///
    /// If the register is not an integer, abort with an error.
    MustBeInt {
        /// Register to convert
        src: i32,
        /// Target address if not convertible
        target: i32,
    },

    // =========================================================================
    // Result Output
    // =========================================================================

    /// Output a row of results
    ///
    /// Return the contents of registers `start` through `start + count - 1`
    /// as a result row.
    ResultRow {
        /// First register of result row
        start: i32,
        /// Number of columns
        count: i32,
    },

    // =========================================================================
    // Cursor Operations - Database access
    // =========================================================================

    /// Open a read-only cursor on a table or index
    OpenRead {
        /// Cursor number
        cursor: i32,
        /// Root page number (or register if P5 has OPFLAG_P2ISREG)
        root_page: i32,
        /// Database index (0=main, 1=temp)
        db_num: i32,
    },

    /// Open a read/write cursor on a table or index
    OpenWrite {
        /// Cursor number
        cursor: i32,
        /// Root page number (or register if P5 has OPFLAG_P2ISREG)
        root_page: i32,
        /// Database index (0=main, 1=temp)
        db_num: i32,
    },

    /// Open an ephemeral (temporary) table
    OpenEphemeral {
        /// Cursor number
        cursor: i32,
        /// Number of columns
        num_columns: i32,
    },

    /// Close a cursor
    Close {
        /// Cursor number to close
        cursor: i32,
    },

    /// Position cursor at start of table
    ///
    /// If the table is empty, jump to `target`.
    Rewind {
        /// Cursor number
        cursor: i32,
        /// Target address if table is empty
        target: i32,
    },

    /// Move cursor to next row
    ///
    /// Advance cursor to next entry. If there is a next entry, jump to `target`.
    Next {
        /// Cursor number
        cursor: i32,
        /// Target address if another row exists
        target: i32,
    },

    /// Move cursor to previous row
    ///
    /// Move cursor to previous entry. If there is a previous entry, jump to `target`.
    Prev {
        /// Cursor number
        cursor: i32,
        /// Target address if another row exists
        target: i32,
    },

    /// Position cursor at end of table
    ///
    /// If the table is empty, jump to `target`.
    Last {
        /// Cursor number
        cursor: i32,
        /// Target address if table is empty
        target: i32,
    },

    /// Seek to row with key >= given key
    SeekGE {
        /// Cursor number
        cursor: i32,
        /// Target address if no matching row
        target: i32,
        /// Key register
        key: i32,
        /// Number of key columns (for multi-column keys)
        num_fields: i32,
    },

    /// Seek to row with key > given key
    SeekGT {
        /// Cursor number
        cursor: i32,
        /// Target address if no matching row
        target: i32,
        /// Key register
        key: i32,
        /// Number of key columns
        num_fields: i32,
    },

    /// Seek to row with key <= given key
    SeekLE {
        /// Cursor number
        cursor: i32,
        /// Target address if no matching row
        target: i32,
        /// Key register
        key: i32,
        /// Number of key columns
        num_fields: i32,
    },

    /// Seek to row with key < given key
    SeekLT {
        /// Cursor number
        cursor: i32,
        /// Target address if no matching row
        target: i32,
        /// Key register
        key: i32,
        /// Number of key columns
        num_fields: i32,
    },

    /// Seek to specific rowid
    SeekRowid {
        /// Cursor number
        cursor: i32,
        /// Target address if not found
        target: i32,
        /// Register containing rowid
        rowid: i32,
    },

    /// Extract column from cursor row
    Column {
        /// Cursor number
        cursor: i32,
        /// Column index (0-based)
        column: i32,
        /// Destination register
        dest: i32,
    },

    /// Get rowid of current cursor position
    Rowid {
        /// Cursor number
        cursor: i32,
        /// Destination register
        dest: i32,
    },

    /// Create a new rowid
    NewRowid {
        /// Cursor number
        cursor: i32,
        /// Destination register for new rowid
        dest: i32,
        /// Previous rowid hint register (or 0)
        prev_rowid: i32,
    },

    /// Insert a row
    Insert {
        /// Cursor number
        cursor: i32,
        /// Register containing row data
        data: i32,
        /// Register containing rowid
        rowid: i32,
    },

    /// Delete the row at cursor position
    Delete {
        /// Cursor number
        cursor: i32,
    },

    /// Build a record from registers
    MakeRecord {
        /// First register of data
        start: i32,
        /// Number of registers
        count: i32,
        /// Destination register for record
        dest: i32,
    },

    // =========================================================================
    // Index Operations
    // =========================================================================

    /// Insert into an index
    IdxInsert {
        /// Cursor number (index)
        cursor: i32,
        /// Register containing key
        key: i32,
    },

    /// Delete from an index
    IdxDelete {
        /// Cursor number (index)
        cursor: i32,
        /// Key register
        key: i32,
        /// Number of key fields
        num_fields: i32,
    },

    /// Get rowid from index cursor
    IdxRowid {
        /// Cursor number
        cursor: i32,
        /// Destination register
        dest: i32,
    },

    // =========================================================================
    // Program Initialization
    // =========================================================================

    /// Initialize program
    ///
    /// Always the first instruction. Jump to `target` to begin execution.
    Init {
        /// Target address to begin execution
        target: i32,
    },

    // =========================================================================
    // Coroutines
    // =========================================================================

    /// Initialize a coroutine
    InitCoroutine {
        /// Coroutine register
        coroutine: i32,
        /// Initial entry point
        target: i32,
        /// End address
        end: i32,
    },

    /// Yield from a coroutine
    Yield {
        /// Coroutine register
        coroutine: i32,
    },

    /// End a coroutine
    EndCoroutine {
        /// Coroutine register
        coroutine: i32,
    },

    // =========================================================================
    // Aggregation
    // =========================================================================

    /// Step an aggregate function
    AggStep {
        /// Function definition (P4)
        func_def: i32,
        /// First argument register
        args: i32,
        /// Accumulator register
        accum: i32,
        /// Number of arguments
        num_args: i32,
    },

    /// Get final aggregate value
    AggFinal {
        /// Accumulator register
        accum: i32,
        /// Number of arguments (for finalization)
        num_args: i32,
    },

    // =========================================================================
    // Miscellaneous
    // =========================================================================

    /// No operation (placeholder)
    Noop,

    /// Explain query plan (debugging)
    Explain,

    // =========================================================================
    // Raw Opcode - For opcodes not yet wrapped
    // =========================================================================

    /// Raw opcode for advanced use
    ///
    /// Use this for opcodes that don't have a dedicated variant yet.
    Raw {
        /// The raw opcode value
        opcode: RawOpcode,
        /// P1 operand
        p1: i32,
        /// P2 operand
        p2: i32,
        /// P3 operand
        p3: i32,
        /// P4 operand
        p4: P4,
        /// P5 operand
        p5: u16,
    },
}

impl Insn {
    /// Get the raw opcode value for this instruction
    pub fn raw_opcode(&self) -> u8 {
        match self {
            Insn::Integer { .. } => RawOpcode::Integer as u8,
            Insn::Int64 { .. } => RawOpcode::Int64 as u8,
            Insn::Real { .. } => RawOpcode::Real as u8,
            Insn::String8 { .. } => RawOpcode::String8 as u8,
            Insn::Null { .. } => RawOpcode::Null as u8,

            Insn::Add { .. } => RawOpcode::Add as u8,
            Insn::Subtract { .. } => RawOpcode::Subtract as u8,
            Insn::Multiply { .. } => RawOpcode::Multiply as u8,
            Insn::Divide { .. } => RawOpcode::Divide as u8,
            Insn::Remainder { .. } => RawOpcode::Remainder as u8,
            Insn::Concat { .. } => RawOpcode::Concat as u8,

            Insn::BitAnd { .. } => RawOpcode::BitAnd as u8,
            Insn::BitOr { .. } => RawOpcode::BitOr as u8,
            Insn::ShiftLeft { .. } => RawOpcode::ShiftLeft as u8,
            Insn::ShiftRight { .. } => RawOpcode::ShiftRight as u8,
            Insn::BitNot { .. } => RawOpcode::BitNot as u8,

            Insn::Not { .. } => RawOpcode::Not as u8,
            Insn::AddImm { .. } => RawOpcode::AddImm as u8,

            Insn::Copy { .. } => RawOpcode::Copy as u8,
            Insn::SCopy { .. } => RawOpcode::SCopy as u8,
            Insn::Move { .. } => RawOpcode::Move as u8,
            Insn::IntCopy { .. } => RawOpcode::IntCopy as u8,

            Insn::Halt | Insn::HaltWithError { .. } => RawOpcode::Halt as u8,
            Insn::HaltIfNull { .. } => RawOpcode::HaltIfNull as u8,
            Insn::Goto { .. } => RawOpcode::Goto as u8,
            Insn::Gosub { .. } => RawOpcode::Gosub as u8,
            Insn::Return { .. } => RawOpcode::Return as u8,
            Insn::If { .. } => RawOpcode::If as u8,
            Insn::IfNot { .. } => RawOpcode::IfNot as u8,
            Insn::IsNull { .. } => RawOpcode::IsNull as u8,
            Insn::NotNull { .. } => RawOpcode::NotNull as u8,
            Insn::Once { .. } => RawOpcode::Once as u8,
            Insn::Jump { .. } => RawOpcode::Jump as u8,

            Insn::Eq { .. } => RawOpcode::Eq as u8,
            Insn::Ne { .. } => RawOpcode::Ne as u8,
            Insn::Lt { .. } => RawOpcode::Lt as u8,
            Insn::Le { .. } => RawOpcode::Le as u8,
            Insn::Gt { .. } => RawOpcode::Gt as u8,
            Insn::Ge { .. } => RawOpcode::Ge as u8,

            Insn::IfPos { .. } => RawOpcode::IfPos as u8,
            Insn::IfNotZero { .. } => RawOpcode::IfNotZero as u8,
            Insn::DecrJumpZero { .. } => RawOpcode::DecrJumpZero as u8,
            Insn::MustBeInt { .. } => RawOpcode::MustBeInt as u8,

            Insn::ResultRow { .. } => RawOpcode::ResultRow as u8,

            Insn::OpenRead { .. } => RawOpcode::OpenRead as u8,
            Insn::OpenWrite { .. } => RawOpcode::OpenWrite as u8,
            Insn::OpenEphemeral { .. } => RawOpcode::OpenEphemeral as u8,
            Insn::Close { .. } => RawOpcode::Close as u8,
            Insn::Rewind { .. } => RawOpcode::Rewind as u8,
            Insn::Next { .. } => RawOpcode::Next as u8,
            Insn::Prev { .. } => RawOpcode::Prev as u8,
            Insn::Last { .. } => RawOpcode::Last as u8,
            Insn::SeekGE { .. } => RawOpcode::SeekGE as u8,
            Insn::SeekGT { .. } => RawOpcode::SeekGT as u8,
            Insn::SeekLE { .. } => RawOpcode::SeekLE as u8,
            Insn::SeekLT { .. } => RawOpcode::SeekLT as u8,
            Insn::SeekRowid { .. } => RawOpcode::SeekRowid as u8,
            Insn::Column { .. } => RawOpcode::Column as u8,
            Insn::Rowid { .. } => RawOpcode::Rowid as u8,
            Insn::NewRowid { .. } => RawOpcode::NewRowid as u8,
            Insn::Insert { .. } => RawOpcode::Insert as u8,
            Insn::Delete { .. } => RawOpcode::Delete as u8,
            Insn::MakeRecord { .. } => RawOpcode::MakeRecord as u8,

            Insn::IdxInsert { .. } => RawOpcode::IdxInsert as u8,
            Insn::IdxDelete { .. } => RawOpcode::IdxDelete as u8,
            Insn::IdxRowid { .. } => RawOpcode::IdxRowid as u8,

            Insn::Init { .. } => RawOpcode::Init as u8,

            Insn::InitCoroutine { .. } => RawOpcode::InitCoroutine as u8,
            Insn::Yield { .. } => RawOpcode::Yield as u8,
            Insn::EndCoroutine { .. } => RawOpcode::EndCoroutine as u8,

            Insn::AggStep { .. } => RawOpcode::AggStep as u8,
            Insn::AggFinal { .. } => RawOpcode::AggFinal as u8,

            Insn::Noop => RawOpcode::Noop as u8,
            Insn::Explain => RawOpcode::Explain as u8,

            Insn::Raw { opcode, .. } => *opcode as u8,
        }
    }

    /// Extract the operands (P1, P2, P3, P5) for this instruction
    ///
    /// Returns (p1, p2, p3, p5). P4 is handled separately.
    pub(crate) fn operands(&self) -> (i32, i32, i32, u16) {
        match self {
            // Constants
            Insn::Integer { value, dest } => (*value, *dest, 0, 0),
            Insn::Int64 { dest, .. } => (0, *dest, 0, 0),
            Insn::Real { dest, .. } => (0, *dest, 0, 0),
            Insn::String8 { dest, .. } => (0, *dest, 0, 0),
            Insn::Null { dest, count } => (0, *dest, dest + count - 1, 0),

            // Arithmetic - Note: SQLite's Subtract/Divide compute P2 op P1, not P1 op P2
            Insn::Add { lhs, rhs, dest } => (*lhs, *rhs, *dest, 0),
            Insn::Subtract { lhs, rhs, dest } => (*rhs, *lhs, *dest, 0), // Swap for P2-P1
            Insn::Multiply { lhs, rhs, dest } => (*lhs, *rhs, *dest, 0),
            Insn::Divide { lhs, rhs, dest } => (*rhs, *lhs, *dest, 0), // Swap for P2/P1
            Insn::Remainder { lhs, rhs, dest } => (*rhs, *lhs, *dest, 0), // Swap for P2%P1
            Insn::Concat { lhs, rhs, dest } => (*rhs, *lhs, *dest, 0), // P2||P1, so swap

            // Bitwise - Note: SQLite computes P2 op P1
            Insn::BitAnd { lhs, rhs, dest } => (*lhs, *rhs, *dest, 0),
            Insn::BitOr { lhs, rhs, dest } => (*lhs, *rhs, *dest, 0),
            Insn::ShiftLeft { lhs, rhs, dest } => (*rhs, *lhs, *dest, 0),  // P2 << P1
            Insn::ShiftRight { lhs, rhs, dest } => (*rhs, *lhs, *dest, 0), // P2 >> P1
            Insn::BitNot { src, dest } => (*src, *dest, 0, 0),

            // Logical
            Insn::Not { src, dest } => (*src, *dest, 0, 0),
            Insn::AddImm { dest, value } => (*dest, *value, 0, 0),

            // Register operations
            Insn::Copy { src, dest, count } => (*src, *dest, *count, 0),
            Insn::SCopy { src, dest } => (*src, *dest, 0, 0),
            Insn::Move { src, dest, count } => (*src, *dest, *count, 0),
            Insn::IntCopy { src, dest } => (*src, *dest, 0, 0),

            // Control flow
            Insn::Halt => (0, 0, 0, 0),
            Insn::HaltWithError { error_code, on_error } => (*error_code, *on_error, 0, 0),
            Insn::HaltIfNull { src, error_code, target } => (*src, *target, *error_code, 0),
            Insn::Goto { target } => (0, *target, 0, 0),
            Insn::Gosub { return_reg, target } => (*return_reg, *target, 0, 0),
            Insn::Return { return_reg } => (*return_reg, 0, 0, 0),
            Insn::If { src, target, jump_if_null } => (*src, *target, if *jump_if_null { 1 } else { 0 }, 0),
            Insn::IfNot { src, target, jump_if_null } => (*src, *target, if *jump_if_null { 1 } else { 0 }, 0),
            Insn::IsNull { src, target } => (*src, *target, 0, 0),
            Insn::NotNull { src, target } => (*src, *target, 0, 0),
            Insn::Once { target } => (0, *target, 0, 0),
            Insn::Jump { neg, zero, pos } => (*neg, *zero, *pos, 0),

            // Comparisons - Jump to P2 if P3 op P1
            // For lhs op rhs: P1=rhs, P3=lhs, P2=target
            Insn::Eq { lhs, rhs, target } => (*rhs, *target, *lhs, 0),
            Insn::Ne { lhs, rhs, target } => (*rhs, *target, *lhs, 0),
            Insn::Lt { lhs, rhs, target } => (*rhs, *target, *lhs, 0),
            Insn::Le { lhs, rhs, target } => (*rhs, *target, *lhs, 0),
            Insn::Gt { lhs, rhs, target } => (*rhs, *target, *lhs, 0),
            Insn::Ge { lhs, rhs, target } => (*rhs, *target, *lhs, 0),

            // Register tests
            Insn::IfPos { src, target, decrement } => (*src, *target, *decrement, 0),
            Insn::IfNotZero { src, target } => (*src, *target, 0, 0),
            Insn::DecrJumpZero { src, target } => (*src, *target, 0, 0),
            Insn::MustBeInt { src, target } => (*src, *target, 0, 0),

            // Results
            Insn::ResultRow { start, count } => (*start, *count, 0, 0),

            // Cursor operations
            Insn::OpenRead { cursor, root_page, db_num } => (*cursor, *root_page, *db_num, 0),
            Insn::OpenWrite { cursor, root_page, db_num } => (*cursor, *root_page, *db_num, 0),
            Insn::OpenEphemeral { cursor, num_columns } => (*cursor, *num_columns, 0, 0),
            Insn::Close { cursor } => (*cursor, 0, 0, 0),
            Insn::Rewind { cursor, target } => (*cursor, *target, 0, 0),
            Insn::Next { cursor, target } => (*cursor, *target, 0, 0),
            Insn::Prev { cursor, target } => (*cursor, *target, 0, 0),
            Insn::Last { cursor, target } => (*cursor, *target, 0, 0),
            Insn::SeekGE { cursor, target, key, num_fields } => (*cursor, *target, *key, *num_fields as u16),
            Insn::SeekGT { cursor, target, key, num_fields } => (*cursor, *target, *key, *num_fields as u16),
            Insn::SeekLE { cursor, target, key, num_fields } => (*cursor, *target, *key, *num_fields as u16),
            Insn::SeekLT { cursor, target, key, num_fields } => (*cursor, *target, *key, *num_fields as u16),
            Insn::SeekRowid { cursor, target, rowid } => (*cursor, *target, *rowid, 0),
            Insn::Column { cursor, column, dest } => (*cursor, *column, *dest, 0),
            Insn::Rowid { cursor, dest } => (*cursor, *dest, 0, 0),
            Insn::NewRowid { cursor, dest, prev_rowid } => (*cursor, *dest, *prev_rowid, 0),
            Insn::Insert { cursor, data, rowid } => (*cursor, *data, *rowid, 0),
            Insn::Delete { cursor } => (*cursor, 0, 0, 0),
            Insn::MakeRecord { start, count, dest } => (*start, *count, *dest, 0),

            // Index operations
            Insn::IdxInsert { cursor, key } => (*cursor, *key, 0, 0),
            Insn::IdxDelete { cursor, key, num_fields } => (*cursor, *key, *num_fields, 0),
            Insn::IdxRowid { cursor, dest } => (*cursor, *dest, 0, 0),

            // Init
            Insn::Init { target } => (0, *target, 0, 0),

            // Coroutines
            Insn::InitCoroutine { coroutine, target, end } => (*coroutine, *target, *end, 0),
            Insn::Yield { coroutine } => (*coroutine, 0, 0, 0),
            Insn::EndCoroutine { coroutine } => (*coroutine, 0, 0, 0),

            // Aggregation
            Insn::AggStep { args, accum, num_args, .. } => (*args, 0, *accum, *num_args as u16),
            Insn::AggFinal { accum, num_args } => (*accum, *num_args, 0, 0),

            // Misc
            Insn::Noop => (0, 0, 0, 0),
            Insn::Explain => (0, 0, 0, 0),

            // Raw
            Insn::Raw { p1, p2, p3, p5, .. } => (*p1, *p2, *p3, *p5),
        }
    }

    /// Get the P4 value if this instruction has one
    pub(crate) fn p4(&self) -> Option<InsnP4> {
        match self {
            Insn::Int64 { value, .. } => Some(InsnP4::Int64(*value)),
            Insn::Real { value, .. } => Some(InsnP4::Real(*value)),
            Insn::String8 { value, .. } => Some(InsnP4::String(value.clone())),
            Insn::Raw { p4: P4::Int(i), .. } => Some(InsnP4::Int(*i)),
            Insn::Raw { p4: P4::String(s), .. } => Some(InsnP4::String(s.clone())),
            _ => None,
        }
    }

    /// Get a human-readable name for this instruction
    pub fn name(&self) -> &'static str {
        match self {
            Insn::Integer { .. } => "Integer",
            Insn::Int64 { .. } => "Int64",
            Insn::Real { .. } => "Real",
            Insn::String8 { .. } => "String8",
            Insn::Null { .. } => "Null",
            Insn::Add { .. } => "Add",
            Insn::Subtract { .. } => "Subtract",
            Insn::Multiply { .. } => "Multiply",
            Insn::Divide { .. } => "Divide",
            Insn::Remainder { .. } => "Remainder",
            Insn::Concat { .. } => "Concat",
            Insn::BitAnd { .. } => "BitAnd",
            Insn::BitOr { .. } => "BitOr",
            Insn::ShiftLeft { .. } => "ShiftLeft",
            Insn::ShiftRight { .. } => "ShiftRight",
            Insn::BitNot { .. } => "BitNot",
            Insn::Not { .. } => "Not",
            Insn::AddImm { .. } => "AddImm",
            Insn::Copy { .. } => "Copy",
            Insn::SCopy { .. } => "SCopy",
            Insn::Move { .. } => "Move",
            Insn::IntCopy { .. } => "IntCopy",
            Insn::Halt => "Halt",
            Insn::HaltWithError { .. } => "Halt",
            Insn::HaltIfNull { .. } => "HaltIfNull",
            Insn::Goto { .. } => "Goto",
            Insn::Gosub { .. } => "Gosub",
            Insn::Return { .. } => "Return",
            Insn::If { .. } => "If",
            Insn::IfNot { .. } => "IfNot",
            Insn::IsNull { .. } => "IsNull",
            Insn::NotNull { .. } => "NotNull",
            Insn::Once { .. } => "Once",
            Insn::Jump { .. } => "Jump",
            Insn::Eq { .. } => "Eq",
            Insn::Ne { .. } => "Ne",
            Insn::Lt { .. } => "Lt",
            Insn::Le { .. } => "Le",
            Insn::Gt { .. } => "Gt",
            Insn::Ge { .. } => "Ge",
            Insn::IfPos { .. } => "IfPos",
            Insn::IfNotZero { .. } => "IfNotZero",
            Insn::DecrJumpZero { .. } => "DecrJumpZero",
            Insn::MustBeInt { .. } => "MustBeInt",
            Insn::ResultRow { .. } => "ResultRow",
            Insn::OpenRead { .. } => "OpenRead",
            Insn::OpenWrite { .. } => "OpenWrite",
            Insn::OpenEphemeral { .. } => "OpenEphemeral",
            Insn::Close { .. } => "Close",
            Insn::Rewind { .. } => "Rewind",
            Insn::Next { .. } => "Next",
            Insn::Prev { .. } => "Prev",
            Insn::Last { .. } => "Last",
            Insn::SeekGE { .. } => "SeekGE",
            Insn::SeekGT { .. } => "SeekGT",
            Insn::SeekLE { .. } => "SeekLE",
            Insn::SeekLT { .. } => "SeekLT",
            Insn::SeekRowid { .. } => "SeekRowid",
            Insn::Column { .. } => "Column",
            Insn::Rowid { .. } => "Rowid",
            Insn::NewRowid { .. } => "NewRowid",
            Insn::Insert { .. } => "Insert",
            Insn::Delete { .. } => "Delete",
            Insn::MakeRecord { .. } => "MakeRecord",
            Insn::IdxInsert { .. } => "IdxInsert",
            Insn::IdxDelete { .. } => "IdxDelete",
            Insn::IdxRowid { .. } => "IdxRowid",
            Insn::Init { .. } => "Init",
            Insn::InitCoroutine { .. } => "InitCoroutine",
            Insn::Yield { .. } => "Yield",
            Insn::EndCoroutine { .. } => "EndCoroutine",
            Insn::AggStep { .. } => "AggStep",
            Insn::AggFinal { .. } => "AggFinal",
            Insn::Noop => "Noop",
            Insn::Explain => "Explain",
            Insn::Raw { .. } => "Raw",
        }
    }
}

/// Internal P4 representation for instruction emission
pub(crate) enum InsnP4 {
    Int(i32),
    Int64(i64),
    Real(f64),
    String(String),
}


impl std::fmt::Display for Insn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_raw_opcode_values() {
        // SQLite 3.45.0 opcode values
        assert_eq!(RawOpcode::Integer as u8, 71);
        assert_eq!(RawOpcode::Add as u8, 106);
        assert_eq!(RawOpcode::Halt as u8, 70);
        assert_eq!(RawOpcode::ResultRow as u8, 84);
    }

    #[test]
    fn test_insn_operands() {
        let insn = Insn::Integer { value: 42, dest: 1 };
        assert_eq!(insn.operands(), (42, 1, 0, 0));

        let insn = Insn::Add { lhs: 1, rhs: 2, dest: 3 };
        assert_eq!(insn.operands(), (1, 2, 3, 0));

        // Test that Subtract swaps operands
        let insn = Insn::Subtract { lhs: 1, rhs: 2, dest: 3 };
        assert_eq!(insn.operands(), (2, 1, 3, 0)); // P2-P1, so swap
    }

    #[test]
    fn test_insn_name() {
        assert_eq!(Insn::Halt.name(), "Halt");
        assert_eq!(Insn::Integer { value: 0, dest: 0 }.name(), "Integer");
    }
}
