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
    /// The 32-bit integer value P1 is written into register P2.
    Integer {
        /// The integer value to store
        value: i32,
        /// Destination register
        dest: i32,
    },

    /// P4 is a pointer to a 64-bit integer value. Write that value into
    /// register P2.
    Int64 {
        /// The integer value to store (in P4)
        value: i64,
        /// Destination register
        dest: i32,
    },

    /// P4 is a pointer to a 64-bit floating point value. Write that value into
    /// register P2.
    Real {
        /// The floating-point value to store (in P4)
        value: f64,
        /// Destination register
        dest: i32,
    },

    /// P4 points to a nul terminated UTF-8 string. This opcode is transformed
    /// into a String opcode before it is executed for the first time. During
    /// this transformation, the length of string P4 is computed and stored as
    /// the P1 parameter.
    String8 {
        /// The string value to store (in P4)
        value: String,
        /// Destination register
        dest: i32,
    },

    /// Write a NULL into registers P2. If P3 greater than P2, then also write
    /// NULL into register P3 and every register in between P2 and P3. If P3 is
    /// less than P2 (typically P3 is zero) then only register P2 is set to
    /// NULL.
    ///
    /// If the P1 value is non-zero, then also set the MEM_Cleared flag so that
    /// NULL values will not compare equal even if SQLITE_NULLEQ is set on Ne or
    /// Eq.
    Null {
        /// First register to set to NULL
        dest: i32,
        /// Number of consecutive registers to set (default 1)
        count: i32,
    },

    // =========================================================================
    // Arithmetic - Binary operations on registers
    // =========================================================================
    /// Add the value in register P1 to the value in register P2 and store the
    /// result in register P3. If either input is NULL, the result is NULL.
    Add {
        /// Left operand register
        lhs: i32,
        /// Right operand register
        rhs: i32,
        /// Destination register
        dest: i32,
    },

    /// Subtract the value in register P1 from the value in register P2 and
    /// store the result in register P3. If either input is NULL, the result is
    /// NULL.
    Subtract {
        /// Left operand register (minuend)
        lhs: i32,
        /// Right operand register (subtrahend)
        rhs: i32,
        /// Destination register
        dest: i32,
    },

    /// Multiply the value in register P1 by the value in register P2 and store
    /// the result in register P3. If either input is NULL, the result is NULL.
    Multiply {
        /// Left operand register
        lhs: i32,
        /// Right operand register
        rhs: i32,
        /// Destination register
        dest: i32,
    },

    /// Divide the value in register P1 by the value in register P2 and store
    /// the result in register P3 (P3=P2/P1). If the value in register P1 is
    /// zero, then the result is NULL. If either input is NULL, the result is
    /// NULL.
    Divide {
        /// Dividend register (numerator)
        lhs: i32,
        /// Divisor register (denominator)
        rhs: i32,
        /// Destination register
        dest: i32,
    },

    /// Compute the remainder after integer register P2 is divided by register
    /// P1 and store the result in register P3. If the value in register P1 is
    /// zero the result is NULL. If either operand is NULL, the result is NULL.
    Remainder {
        /// Dividend register
        lhs: i32,
        /// Divisor register
        rhs: i32,
        /// Destination register
        dest: i32,
    },

    /// Add the text in register P1 onto the end of the text in register P2 and
    /// store the result in register P3. If either the P1 or P2 text are NULL
    /// then store NULL in P3.
    ///
    /// P3 = P2 || P1
    ///
    /// It is illegal for P1 and P3 to be the same register. Sometimes, if P3 is
    /// the same register as P2, the implementation is able to avoid a memcpy().
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
    /// Take the bit-wise AND of the values in register P1 and P2 and store the
    /// result in register P3. If either input is NULL, the result is NULL.
    BitAnd { lhs: i32, rhs: i32, dest: i32 },

    /// Take the bit-wise OR of the values in register P1 and P2 and store the
    /// result in register P3. If either input is NULL, the result is NULL.
    BitOr { lhs: i32, rhs: i32, dest: i32 },

    /// Shift the integer value in register P2 to the left by the number of bits
    /// specified by the integer in register P1. Store the result in register
    /// P3. If either input is NULL, the result is NULL.
    ShiftLeft { lhs: i32, rhs: i32, dest: i32 },

    /// Shift the integer value in register P2 to the right by the number of
    /// bits specified by the integer in register P1. Store the result in
    /// register P3. If either input is NULL, the result is NULL.
    ShiftRight { lhs: i32, rhs: i32, dest: i32 },

    /// Interpret the content of register P1 as an integer. Store the
    /// ones-complement of the P1 value into register P2. If P1 holds a NULL
    /// then store a NULL in P2.
    BitNot {
        /// Source register
        src: i32,
        /// Destination register
        dest: i32,
    },

    // =========================================================================
    // Logical Operations
    // =========================================================================
    /// Interpret the value in register P1 as a boolean value. Store the boolean
    /// complement in register P2. If the value in register P1 is NULL, then a
    /// NULL is stored in P2.
    Not {
        /// Source register
        src: i32,
        /// Destination register
        dest: i32,
    },

    /// Add the constant P2 to the value in register P1. The result is always an
    /// integer.
    ///
    /// To force any register to be an integer, just add 0.
    AddImm {
        /// Register to modify
        dest: i32,
        /// Immediate value to add
        value: i32,
    },

    // =========================================================================
    // Register Operations - Copy and move values
    // =========================================================================
    /// Make a copy of registers P1..P1+P3 into registers P2..P2+P3.
    ///
    /// If the 0x0002 bit of P5 is set then also clear the MEM_Subtype flag in
    /// the destination. The 0x0001 bit of P5 indicates that this Copy opcode
    /// cannot be merged. The 0x0001 bit is used by the query planner and does
    /// not come into play during query execution.
    ///
    /// This instruction makes a deep copy of the value. A duplicate is made of
    /// any string or blob constant. See also SCopy.
    Copy {
        /// Source register (first in range)
        src: i32,
        /// Destination register (first in range)
        dest: i32,
        /// Number of registers to copy
        count: i32,
    },

    /// Make a shallow copy of register P1 into register P2.
    ///
    /// This instruction makes a shallow copy of the value. If the value is a
    /// string or blob, then the copy is only a pointer to the original and
    /// hence if the original changes so will the copy. Worse, if the original
    /// is deallocated, the copy becomes invalid. Thus the program must
    /// guarantee that the original will not change during the lifetime of the
    /// copy. Use Copy to make a complete copy.
    SCopy {
        /// Source register
        src: i32,
        /// Destination register
        dest: i32,
    },

    /// Move the P3 values in register P1..P1+P3-1 over into registers
    /// P2..P2+P3-1. Registers P1..P1+P3-1 are left holding a NULL. It is an
    /// error for register ranges P1..P1+P3-1 and P2..P2+P3-1 to overlap. It is
    /// an error for P3 to be less than 1.
    Move {
        /// Source register (first in range)
        src: i32,
        /// Destination register (first in range)
        dest: i32,
        /// Number of registers to move
        count: i32,
    },

    /// Transfer the integer value held in register P1 into register P2.
    ///
    /// This is an optimized version of SCopy that works only for integer
    /// values.
    IntCopy {
        /// Source register
        src: i32,
        /// Destination register
        dest: i32,
    },

    // =========================================================================
    // Control Flow
    // =========================================================================
    /// Exit immediately. All open cursors, etc are closed automatically.
    ///
    /// P1 is the result code returned by sqlite3_exec(), sqlite3_reset(), or
    /// sqlite3_finalize(). For a normal halt, this should be SQLITE_OK (0). For
    /// errors, it can be some other value. If P1!=0 then P2 will determine
    /// whether or not to rollback the current transaction. Do not rollback if
    /// P2==OE_Fail. Do the rollback if P2==OE_Rollback. If P2==OE_Abort, then
    /// back out all changes that have occurred during this execution of the
    /// VDBE, but do not rollback the transaction.
    ///
    /// If P3 is not zero and P4 is NULL, then P3 is a register that holds the
    /// text of an error message.
    ///
    /// If P3 is zero and P4 is not null then the error message string is held
    /// in P4.
    ///
    /// P5 is a value between 1 and 4, inclusive, then the P4 error message
    /// string is modified as follows:
    ///
    /// 1: NOT NULL constraint failed: P4 2: UNIQUE constraint failed: P4 3:
    /// CHECK constraint failed: P4 4: FOREIGN KEY constraint failed: P4
    ///
    /// If P3 is zero and P5 is not zero and P4 is NULL, then everything after
    /// the ":" is omitted.
    ///
    /// There is an implied "Halt 0 0 0" instruction inserted at the very end of
    /// every program. So a jump past the last instruction of the program is the
    /// same as executing Halt.
    Halt,

    /// Exit immediately. All open cursors, etc are closed automatically.
    ///
    /// P1 is the result code returned by sqlite3_exec(), sqlite3_reset(), or
    /// sqlite3_finalize(). For a normal halt, this should be SQLITE_OK (0). For
    /// errors, it can be some other value. If P1!=0 then P2 will determine
    /// whether or not to rollback the current transaction. Do not rollback if
    /// P2==OE_Fail. Do the rollback if P2==OE_Rollback. If P2==OE_Abort, then
    /// back out all changes that have occurred during this execution of the
    /// VDBE, but do not rollback the transaction.
    ///
    /// If P3 is not zero and P4 is NULL, then P3 is a register that holds the
    /// text of an error message.
    ///
    /// If P3 is zero and P4 is not null then the error message string is held
    /// in P4.
    ///
    /// P5 is a value between 1 and 4, inclusive, then the P4 error message
    /// string is modified as follows:
    ///
    /// 1: NOT NULL constraint failed: P4 2: UNIQUE constraint failed: P4 3:
    /// CHECK constraint failed: P4 4: FOREIGN KEY constraint failed: P4
    ///
    /// If P3 is zero and P5 is not zero and P4 is NULL, then everything after
    /// the ":" is omitted.
    ///
    /// There is an implied "Halt 0 0 0" instruction inserted at the very end of
    /// every program. So a jump past the last instruction of the program is the
    /// same as executing Halt.
    HaltWithError {
        /// Error code (SQLITE_OK for normal halt)
        error_code: i32,
        /// What to do on error (0=nothing, 1=fail, 2=abort, 3=rollback)
        on_error: i32,
    },

    /// Check the value in register P3. If it is NULL then Halt using parameter
    /// P1, P2, and P4 as if this were a Halt instruction. If the value in
    /// register P3 is not NULL, then this routine is a no-op. The P5 parameter
    /// should be 1.
    HaltIfNull {
        /// Register to test
        src: i32,
        /// Error code if NULL
        error_code: i32,
        /// Jump target if not NULL (0 = continue)
        target: i32,
    },

    /// An unconditional jump to address P2. The next instruction executed will
    /// be the one at index P2 from the beginning of the program.
    ///
    /// The P1 parameter is not actually used by this opcode. However, it is
    /// sometimes set to 1 instead of 0 as a hint to the command-line shell that
    /// this Goto is the bottom of a loop and that the lines from P2 down to the
    /// current line should be indented for EXPLAIN output.
    Goto {
        /// Target instruction address
        target: i32,
    },

    /// Write the current address onto register P1 and then jump to address P2.
    Gosub {
        /// Register to store return address
        return_reg: i32,
        /// Target instruction address
        target: i32,
    },

    /// Jump to the address stored in register P1. If P1 is a return address
    /// register, then this accomplishes a return from a subroutine.
    ///
    /// If P3 is 1, then the jump is only taken if register P1 holds an integer
    /// values, otherwise execution falls through to the next opcode, and the
    /// Return becomes a no-op. If P3 is 0, then register P1 must hold an
    /// integer or else an assert() is raised. P3 should be set to 1 when this
    /// opcode is used in combination with BeginSubrtn, and set to 0 otherwise.
    ///
    /// The value in register P1 is unchanged by this opcode.
    ///
    /// P2 is not used by the byte-code engine. However, if P2 is positive and
    /// also less than the current address, then the "EXPLAIN" output formatter
    /// in the CLI will indent all opcodes from the P2 opcode up to be not
    /// including the current Return. P2 should be the first opcode in the
    /// subroutine from which this opcode is returning. Thus the P2 value is a
    /// byte-code indentation hint. See tag-20220407a in wherecode.c and
    /// shell.c.
    Return {
        /// Register containing return address
        return_reg: i32,
    },

    /// Jump to P2 if the value in register P1 is true. The value is considered
    /// true if it is numeric and non-zero. If the value in P1 is NULL then take
    /// the jump if and only if P3 is non-zero.
    If {
        /// Register to test
        src: i32,
        /// Target address if true
        target: i32,
        /// If true, treat NULL as true; if false, treat NULL as false
        jump_if_null: bool,
    },

    /// Jump to P2 if the value in register P1 is False. The value is considered
    /// false if it has a numeric value of zero. If the value in P1 is NULL then
    /// take the jump if and only if P3 is non-zero.
    IfNot {
        /// Register to test
        src: i32,
        /// Target address if false
        target: i32,
        /// If true, treat NULL as true; if false, treat NULL as false
        jump_if_null: bool,
    },

    /// Jump to P2 if the value in register P1 is NULL.
    IsNull {
        /// Register to test
        src: i32,
        /// Target address if NULL
        target: i32,
    },

    /// Jump to P2 if the value in register P1 is not NULL.
    NotNull {
        /// Register to test
        src: i32,
        /// Target address if not NULL
        target: i32,
    },

    /// Fall through to the next instruction the first time this opcode is
    /// encountered on each invocation of the byte-code program. Jump to P2 on
    /// the second and all subsequent encounters during the same invocation.
    ///
    /// Top-level programs determine first invocation by comparing the P1
    /// operand against the P1 operand on the Init opcode at the beginning of
    /// the program. If the P1 values differ, then fall through and make the P1
    /// of this opcode equal to the P1 of Init. If P1 values are the same then
    /// take the jump.
    ///
    /// For subprograms, there is a bitmask in the VdbeFrame that determines
    /// whether or not the jump should be taken. The bitmask is necessary
    /// because the self-altering code trick does not work for recursive
    /// triggers.
    ///
    /// The P3 operand is not used directly by this opcode. However P3 is used
    /// by the code generator as follows: If this opcode is the start of a
    /// subroutine and that subroutine uses a Bloom filter, then P3 will be the
    /// register that holds that Bloom filter. See tag-202407032019 in the
    /// source code for implementation details.
    Once {
        /// Target address to jump to on subsequent executions
        target: i32,
    },

    /// Jump to the instruction at address P1, P2, or P3 depending on whether in
    /// the most recent Compare instruction the P1 vector was less than, equal
    /// to, or greater than the P2 vector, respectively.
    ///
    /// This opcode must immediately follow an Compare opcode.
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
    /// Compare the values in register P1 and P3. If reg(P3)==reg(P1) then jump
    /// to address P2.
    ///
    /// The SQLITE_AFF_MASK portion of P5 must be an affinity character -
    /// SQLITE_AFF_TEXT, SQLITE_AFF_INTEGER, and so forth. An attempt is made to
    /// coerce both inputs according to this affinity before the comparison is
    /// made. If the SQLITE_AFF_MASK is 0x00, then numeric affinity is used.
    /// Note that the affinity conversions are stored back into the input
    /// registers P1 and P3. So this opcode can cause persistent changes to
    /// registers P1 and P3.
    ///
    /// Once any conversions have taken place, and neither value is NULL, the
    /// values are compared. If both values are blobs then memcmp() is used to
    /// determine the results of the comparison. If both values are text, then
    /// the appropriate collating function specified in P4 is used to do the
    /// comparison. If P4 is not specified then memcmp() is used to compare text
    /// string. If both values are numeric, then a numeric comparison is used.
    /// If the two values are of different types, then numbers are considered
    /// less than strings and strings are considered less than blobs.
    ///
    /// If SQLITE_NULLEQ is set in P5 then the result of comparison is always
    /// either true or false and is never NULL. If both operands are NULL then
    /// the result of comparison is true. If either operand is NULL then the
    /// result is false. If neither operand is NULL the result is the same as it
    /// would be if the SQLITE_NULLEQ flag were omitted from P5.
    ///
    /// This opcode saves the result of comparison for use by the new Jump
    /// opcode.
    Eq {
        /// Left operand register
        lhs: i32,
        /// Right operand register
        rhs: i32,
        /// Target address if equal
        target: i32,
    },

    /// This works just like the Eq opcode except that the jump is taken if the
    /// operands in registers P1 and P3 are not equal. See the Eq opcode for
    /// additional information.
    Ne {
        /// Left operand register
        lhs: i32,
        /// Right operand register
        rhs: i32,
        /// Target address if not equal
        target: i32,
    },

    /// Compare the values in register P1 and P3. If reg(P3)<reg(P1) then jump
    /// to address P2.
    ///
    /// If the SQLITE_JUMPIFNULL bit of P5 is set and either reg(P1) or reg(P3)
    /// is NULL then the take the jump. If the SQLITE_JUMPIFNULL bit is clear
    /// then fall through if either operand is NULL.
    ///
    /// The SQLITE_AFF_MASK portion of P5 must be an affinity character -
    /// SQLITE_AFF_TEXT, SQLITE_AFF_INTEGER, and so forth. An attempt is made to
    /// coerce both inputs according to this affinity before the comparison is
    /// made. If the SQLITE_AFF_MASK is 0x00, then numeric affinity is used.
    /// Note that the affinity conversions are stored back into the input
    /// registers P1 and P3. So this opcode can cause persistent changes to
    /// registers P1 and P3.
    ///
    /// Once any conversions have taken place, and neither value is NULL, the
    /// values are compared. If both values are blobs then memcmp() is used to
    /// determine the results of the comparison. If both values are text, then
    /// the appropriate collating function specified in P4 is used to do the
    /// comparison. If P4 is not specified then memcmp() is used to compare text
    /// string. If both values are numeric, then a numeric comparison is used.
    /// If the two values are of different types, then numbers are considered
    /// less than strings and strings are considered less than blobs.
    ///
    /// This opcode saves the result of comparison for use by the new Jump
    /// opcode.
    Lt {
        /// Left operand register
        lhs: i32,
        /// Right operand register
        rhs: i32,
        /// Target address if less than
        target: i32,
    },

    /// This works just like the Lt opcode except that the jump is taken if the
    /// content of register P3 is less than or equal to the content of register
    /// P1. See the Lt opcode for additional information.
    Le {
        /// Left operand register
        lhs: i32,
        /// Right operand register
        rhs: i32,
        /// Target address if less than or equal
        target: i32,
    },

    /// This works just like the Lt opcode except that the jump is taken if the
    /// content of register P3 is greater than the content of register P1. See
    /// the Lt opcode for additional information.
    Gt {
        /// Left operand register
        lhs: i32,
        /// Right operand register
        rhs: i32,
        /// Target address if greater than
        target: i32,
    },

    /// This works just like the Lt opcode except that the jump is taken if the
    /// content of register P3 is greater than or equal to the content of
    /// register P1. See the Lt opcode for additional information.
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
    /// Register P1 must contain an integer. If the value of register P1 is 1 or
    /// greater, subtract P3 from the value in P1 and jump to P2.
    ///
    /// If the initial value of register P1 is less than 1, then the value is
    /// unchanged and control passes through to the next instruction.
    IfPos {
        /// Register to test
        src: i32,
        /// Target address if positive
        target: i32,
        /// Decrement amount (usually 0 or 1)
        decrement: i32,
    },

    /// Register P1 must contain an integer. If the content of register P1 is
    /// initially greater than zero, then decrement the value in register P1. If
    /// it is non-zero (negative or positive) and then also jump to P2. If
    /// register P1 is initially zero, leave it unchanged and fall through.
    IfNotZero {
        /// Register to test and decrement
        src: i32,
        /// Target address if not zero
        target: i32,
    },

    /// Register P1 must hold an integer. Decrement the value in P1 and jump to
    /// P2 if the new value is exactly zero.
    DecrJumpZero {
        /// Register to decrement
        src: i32,
        /// Target address if zero after decrement
        target: i32,
    },

    /// Force the value in register P1 to be an integer. If the value in P1 is
    /// not an integer and cannot be converted into an integer without data
    /// loss, then jump immediately to P2, or if P2==0 raise an SQLITE_MISMATCH
    /// exception.
    MustBeInt {
        /// Register to convert
        src: i32,
        /// Target address if not convertible
        target: i32,
    },

    // =========================================================================
    // Result Output
    // =========================================================================
    /// The registers P1 through P1+P2-1 contain a single row of results. This
    /// opcode causes the sqlite3_step() call to terminate with an SQLITE_ROW
    /// return code and it sets up the sqlite3_stmt structure to provide access
    /// to the r(P1)..r(P1+P2-1) values as the result row.
    ResultRow {
        /// First register of result row
        start: i32,
        /// Number of columns
        count: i32,
    },

    // =========================================================================
    // Cursor Operations - Database access
    // =========================================================================
    /// Open a read-only cursor for the database table whose root page is P2 in
    /// a database file. The database file is determined by P3. P3==0 means the
    /// main database, P3==1 means the database used for temporary tables, and
    /// P3>1 means used the corresponding attached database. Give the new cursor
    /// an identifier of P1. The P1 values need not be contiguous but all P1
    /// values should be small integers. It is an error for P1 to be negative.
    ///
    /// Allowed P5 bits: 0x02 OPFLAG_SEEKEQ: This cursor will only be used for
    /// equality lookups (implemented as a pair of opcodes SeekGE/IdxGT of
    /// SeekLE/IdxLT)
    ///
    /// The P4 value may be either an integer (P4_INT32) or a pointer to a
    /// KeyInfo structure (P4_KEYINFO). If it is a pointer to a KeyInfo object,
    /// then table being opened must be an index b-tree where the KeyInfo object
    /// defines the content and collating sequence of that index b-tree.
    /// Otherwise, if P4 is an integer value, then the table being opened must
    /// be a table b-tree with a number of columns no less than the value of P4.
    ///
    /// See also: OpenWrite, ReopenIdx
    OpenRead {
        /// Cursor number
        cursor: i32,
        /// Root page number (or register if P5 has OPFLAG_P2ISREG)
        root_page: i32,
        /// Database index (0=main, 1=temp)
        db_num: i32,
    },

    /// Open a read/write cursor named P1 on the table or index whose root page
    /// is P2 (or whose root page is held in register P2 if the OPFLAG_P2ISREG
    /// bit is set in P5 - see below).
    ///
    /// The P4 value may be either an integer (P4_INT32) or a pointer to a
    /// KeyInfo structure (P4_KEYINFO). If it is a pointer to a KeyInfo object,
    /// then table being opened must be an index b-tree where the KeyInfo object
    /// defines the content and collating sequence of that index b-tree.
    /// Otherwise, if P4 is an integer value, then the table being opened must
    /// be a table b-tree with a number of columns no less than the value of P4.
    ///
    /// Allowed P5 bits: 0x02 OPFLAG_SEEKEQ: This cursor will only be used for
    /// equality lookups (implemented as a pair of opcodes SeekGE/IdxGT of
    /// SeekLE/IdxLT) 0x08 OPFLAG_FORDELETE: This cursor is used only to seek
    /// and subsequently delete entries in an index btree. This is a hint to the
    /// storage engine that the storage engine is allowed to ignore. The hint is
    /// not used by the official SQLite b*tree storage engine, but is used by
    /// COMDB2. 0x10 OPFLAG_P2ISREG: Use the content of register P2 as the root
    /// page, not the value of P2 itself.
    ///
    /// This instruction works like OpenRead except that it opens the cursor in
    /// read/write mode.
    ///
    /// See also: OpenRead, ReopenIdx
    OpenWrite {
        /// Cursor number
        cursor: i32,
        /// Root page number (or register if P5 has OPFLAG_P2ISREG)
        root_page: i32,
        /// Database index (0=main, 1=temp)
        db_num: i32,
    },

    /// Open a new cursor P1 to a transient table. The cursor is always opened
    /// read/write even if the main database is read-only. The ephemeral table
    /// is deleted automatically when the cursor is closed.
    ///
    /// If the cursor P1 is already opened on an ephemeral table, the table is
    /// cleared (all content is erased).
    ///
    /// P2 is the number of columns in the ephemeral table. The cursor points to
    /// a BTree table if P4==0 and to a BTree index if P4 is not 0. If P4 is not
    /// NULL, it points to a KeyInfo structure that defines the format of keys
    /// in the index.
    ///
    /// The P5 parameter can be a mask of the BTREE_* flags defined in btree.h.
    /// These flags control aspects of the operation of the btree. The
    /// BTREE_OMIT_JOURNAL and BTREE_SINGLE flags are added automatically.
    ///
    /// If P3 is positive, then reg[P3] is modified slightly so that it can be
    /// used as zero-length data for Insert. This is an optimization that avoids
    /// an extra Blob opcode to initialize that register.
    OpenEphemeral {
        /// Cursor number
        cursor: i32,
        /// Number of columns
        num_columns: i32,
    },

    /// Close a cursor previously opened as P1. If P1 is not currently open,
    /// this instruction is a no-op.
    Close {
        /// Cursor number to close
        cursor: i32,
    },

    /// The next use of the Rowid or Column or Next instruction for P1 will
    /// refer to the first entry in the database table or index. If the table or
    /// index is empty, jump immediately to P2. If the table or index is not
    /// empty, fall through to the following instruction.
    ///
    /// If P2 is zero, that is an assertion that the P1 table is never empty and
    /// hence the jump will never be taken.
    ///
    /// This opcode leaves the cursor configured to move in forward order, from
    /// the beginning toward the end. In other words, the cursor is configured
    /// to use Next, not Prev.
    Rewind {
        /// Cursor number
        cursor: i32,
        /// Target address if table is empty
        target: i32,
    },

    /// Advance cursor P1 so that it points to the next key/data pair in its
    /// table or index. If there are no more key/value pairs then fall through
    /// to the following instruction. But if the cursor advance was successful,
    /// jump immediately to P2.
    ///
    /// The Next opcode is only valid following an SeekGT, SeekGE, or Rewind
    /// opcode used to position the cursor. Next is not allowed to follow
    /// SeekLT, SeekLE, or Last.
    ///
    /// The P1 cursor must be for a real table, not a pseudo-table. P1 must have
    /// been opened prior to this opcode or the program will segfault.
    ///
    /// The P3 value is a hint to the btree implementation. If P3==1, that means
    /// P1 is an SQL index and that this instruction could have been omitted if
    /// that index had been unique. P3 is usually 0. P3 is always either 0 or 1.
    ///
    /// If P5 is positive and the jump is taken, then event counter number P5-1
    /// in the prepared statement is incremented.
    ///
    /// See also: Prev
    Next {
        /// Cursor number
        cursor: i32,
        /// Target address if another row exists
        target: i32,
    },

    /// Back up cursor P1 so that it points to the previous key/data pair in its
    /// table or index. If there is no previous key/value pairs then fall
    /// through to the following instruction. But if the cursor backup was
    /// successful, jump immediately to P2.
    ///
    /// The Prev opcode is only valid following an SeekLT, SeekLE, or Last
    /// opcode used to position the cursor. Prev is not allowed to follow
    /// SeekGT, SeekGE, or Rewind.
    ///
    /// The P1 cursor must be for a real table, not a pseudo-table. If P1 is not
    /// open then the behavior is undefined.
    ///
    /// The P3 value is a hint to the btree implementation. If P3==1, that means
    /// P1 is an SQL index and that this instruction could have been omitted if
    /// that index had been unique. P3 is usually 0. P3 is always either 0 or 1.
    ///
    /// If P5 is positive and the jump is taken, then event counter number P5-1
    /// in the prepared statement is incremented.
    Prev {
        /// Cursor number
        cursor: i32,
        /// Target address if another row exists
        target: i32,
    },

    /// The next use of the Rowid or Column or Prev instruction for P1 will
    /// refer to the last entry in the database table or index. If the table or
    /// index is empty and P2>0, then jump immediately to P2. If P2 is 0 or if
    /// the table or index is not empty, fall through to the following
    /// instruction.
    ///
    /// This opcode leaves the cursor configured to move in reverse order, from
    /// the end toward the beginning. In other words, the cursor is configured
    /// to use Prev, not Next.
    Last {
        /// Cursor number
        cursor: i32,
        /// Target address if table is empty
        target: i32,
    },

    /// If cursor P1 refers to an SQL table (B-Tree that uses integer keys), use
    /// the value in register P3 as the key. If cursor P1 refers to an SQL
    /// index, then P3 is the first in an array of P4 registers that are used as
    /// an unpacked index key.
    ///
    /// Reposition cursor P1 so that it points to the smallest entry that is
    /// greater than or equal to the key value. If there are no records greater
    /// than or equal to the key and P2 is not zero, then jump to P2.
    ///
    /// If the cursor P1 was opened using the OPFLAG_SEEKEQ flag, then this
    /// opcode will either land on a record that exactly matches the key, or
    /// else it will cause a jump to P2. When the cursor is OPFLAG_SEEKEQ, this
    /// opcode must be followed by an IdxLE opcode with the same arguments. The
    /// IdxGT opcode will be skipped if this opcode succeeds, but the IdxGT
    /// opcode will be used on subsequent loop iterations. The OPFLAG_SEEKEQ
    /// flags is a hint to the btree layer to say that this is an equality
    /// search.
    ///
    /// This opcode leaves the cursor configured to move in forward order, from
    /// the beginning toward the end. In other words, the cursor is configured
    /// to use Next, not Prev.
    ///
    /// See also: Found, NotFound, SeekLt, SeekGt, SeekLe
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

    /// If cursor P1 refers to an SQL table (B-Tree that uses integer keys), use
    /// the value in register P3 as a key. If cursor P1 refers to an SQL index,
    /// then P3 is the first in an array of P4 registers that are used as an
    /// unpacked index key.
    ///
    /// Reposition cursor P1 so that it points to the smallest entry that is
    /// greater than the key value. If there are no records greater than the key
    /// and P2 is not zero, then jump to P2.
    ///
    /// This opcode leaves the cursor configured to move in forward order, from
    /// the beginning toward the end. In other words, the cursor is configured
    /// to use Next, not Prev.
    ///
    /// See also: Found, NotFound, SeekLt, SeekGe, SeekLe
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

    /// If cursor P1 refers to an SQL table (B-Tree that uses integer keys), use
    /// the value in register P3 as a key. If cursor P1 refers to an SQL index,
    /// then P3 is the first in an array of P4 registers that are used as an
    /// unpacked index key.
    ///
    /// Reposition cursor P1 so that it points to the largest entry that is less
    /// than or equal to the key value. If there are no records less than or
    /// equal to the key and P2 is not zero, then jump to P2.
    ///
    /// This opcode leaves the cursor configured to move in reverse order, from
    /// the end toward the beginning. In other words, the cursor is configured
    /// to use Prev, not Next.
    ///
    /// If the cursor P1 was opened using the OPFLAG_SEEKEQ flag, then this
    /// opcode will either land on a record that exactly matches the key, or
    /// else it will cause a jump to P2. When the cursor is OPFLAG_SEEKEQ, this
    /// opcode must be followed by an IdxLE opcode with the same arguments. The
    /// IdxGE opcode will be skipped if this opcode succeeds, but the IdxGE
    /// opcode will be used on subsequent loop iterations. The OPFLAG_SEEKEQ
    /// flags is a hint to the btree layer to say that this is an equality
    /// search.
    ///
    /// See also: Found, NotFound, SeekGt, SeekGe, SeekLt
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

    /// If cursor P1 refers to an SQL table (B-Tree that uses integer keys), use
    /// the value in register P3 as a key. If cursor P1 refers to an SQL index,
    /// then P3 is the first in an array of P4 registers that are used as an
    /// unpacked index key.
    ///
    /// Reposition cursor P1 so that it points to the largest entry that is less
    /// than the key value. If there are no records less than the key and P2 is
    /// not zero, then jump to P2.
    ///
    /// This opcode leaves the cursor configured to move in reverse order, from
    /// the end toward the beginning. In other words, the cursor is configured
    /// to use Prev, not Next.
    ///
    /// See also: Found, NotFound, SeekGt, SeekGe, SeekLe
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

    /// P1 is the index of a cursor open on an SQL table btree (with integer
    /// keys). If register P3 does not contain an integer or if P1 does not
    /// contain a record with rowid P3 then jump immediately to P2. Or, if P2 is
    /// 0, raise an SQLITE_CORRUPT error. If P1 does contain a record with rowid
    /// P3 then leave the cursor pointing at that record and fall through to the
    /// next instruction.
    ///
    /// The NotExists opcode performs the same operation, but with NotExists the
    /// P3 register must be guaranteed to contain an integer value. With this
    /// opcode, register P3 might not contain an integer.
    ///
    /// The NotFound opcode performs the same operation on index btrees (with
    /// arbitrary multi-value keys).
    ///
    /// This opcode leaves the cursor in a state where it cannot be advanced in
    /// either direction. In other words, the Next and Prev opcodes will not
    /// work following this opcode.
    ///
    /// See also: Found, NotFound, NoConflict, SeekRowid
    SeekRowid {
        /// Cursor number
        cursor: i32,
        /// Target address if not found
        target: i32,
        /// Register containing rowid
        rowid: i32,
    },

    /// Interpret the data that cursor P1 points to as a structure built using
    /// the MakeRecord instruction. (See the MakeRecord opcode for additional
    /// information about the format of the data.) Extract the P2-th column from
    /// this record. If there are less than (P2+1) values in the record, extract
    /// a NULL.
    ///
    /// The value extracted is stored in register P3.
    ///
    /// If the record contains fewer than P2 fields, then extract a NULL. Or, if
    /// the P4 argument is a P4_MEM use the value of the P4 argument as the
    /// result.
    ///
    /// If the OPFLAG_LENGTHARG bit is set in P5 then the result is guaranteed
    /// to only be used by the length() function or the equivalent. The content
    /// of large blobs is not loaded, thus saving CPU cycles. If the
    /// OPFLAG_TYPEOFARG bit is set then the result will only be used by the
    /// typeof() function or the IS NULL or IS NOT NULL operators or the
    /// equivalent. In this case, all content loading can be omitted.
    Column {
        /// Cursor number
        cursor: i32,
        /// Column index (0-based)
        column: i32,
        /// Destination register
        dest: i32,
    },

    /// Store in register P2 an integer which is the key of the table entry that
    /// P1 is currently point to.
    ///
    /// P1 can be either an ordinary table or a virtual table. There used to be
    /// a separate OP_VRowid opcode for use with virtual tables, but this one
    /// opcode now works for both table types.
    Rowid {
        /// Cursor number
        cursor: i32,
        /// Destination register
        dest: i32,
    },

    /// Get a new integer record number (a.k.a "rowid") used as the key to a
    /// table. The record number is not previously used as a key in the database
    /// table that cursor P1 points to. The new record number is written written
    /// to register P2.
    ///
    /// If P3>0 then P3 is a register in the root frame of this VDBE that holds
    /// the largest previously generated record number. No new record numbers
    /// are allowed to be less than this value. When this value reaches its
    /// maximum, an SQLITE_FULL error is generated. The P3 register is updated
    /// with the ' generated record number. This P3 mechanism is used to help
    /// implement the AUTOINCREMENT feature.
    NewRowid {
        /// Cursor number
        cursor: i32,
        /// Destination register for new rowid
        dest: i32,
        /// Previous rowid hint register (or 0)
        prev_rowid: i32,
    },

    /// Write an entry into the table of cursor P1. A new entry is created if it
    /// doesn't already exist or the data for an existing entry is overwritten.
    /// The data is the value MEM_Blob stored in register number P2. The key is
    /// stored in register P3. The key must be a MEM_Int.
    ///
    /// If the OPFLAG_NCHANGE flag of P5 is set, then the row change count is
    /// incremented (otherwise not). If the OPFLAG_LASTROWID flag of P5 is set,
    /// then rowid is stored for subsequent return by the
    /// sqlite3_last_insert_rowid() function (otherwise it is unmodified).
    ///
    /// If the OPFLAG_USESEEKRESULT flag of P5 is set, the implementation might
    /// run faster by avoiding an unnecessary seek on cursor P1. However, the
    /// OPFLAG_USESEEKRESULT flag must only be set if there have been no prior
    /// seeks on the cursor or if the most recent seek used a key equal to P3.
    ///
    /// If the OPFLAG_ISUPDATE flag is set, then this opcode is part of an
    /// UPDATE operation. Otherwise (if the flag is clear) then this opcode is
    /// part of an INSERT operation. The difference is only important to the
    /// update hook.
    ///
    /// Parameter P4 may point to a Table structure, or may be NULL. If it is
    /// not NULL, then the update-hook (sqlite3.xUpdateCallback) is invoked
    /// following a successful insert.
    ///
    /// (WARNING/TODO: If P1 is a pseudo-cursor and P2 is dynamically allocated,
    /// then ownership of P2 is transferred to the pseudo-cursor and register P2
    /// becomes ephemeral. If the cursor is changed, the value of register P2
    /// will then change. Make sure this does not cause any problems.)
    ///
    /// This instruction only works on tables. The equivalent instruction for
    /// indices is IdxInsert.
    Insert {
        /// Cursor number
        cursor: i32,
        /// Register containing row data
        data: i32,
        /// Register containing rowid
        rowid: i32,
    },

    /// Delete the record at which the P1 cursor is currently pointing.
    ///
    /// If the OPFLAG_SAVEPOSITION bit of the P5 parameter is set, then the
    /// cursor will be left pointing at either the next or the previous record
    /// in the table. If it is left pointing at the next record, then the next
    /// Next instruction will be a no-op. As a result, in this case it is ok to
    /// delete a record from within a Next loop. If OPFLAG_SAVEPOSITION bit of
    /// P5 is clear, then the cursor will be left in an undefined state.
    ///
    /// If the OPFLAG_AUXDELETE bit is set on P5, that indicates that this
    /// delete is one of several associated with deleting a table row and all
    /// its associated index entries. Exactly one of those deletes is the
    /// "primary" delete. The others are all on OPFLAG_FORDELETE cursors or else
    /// are marked with the AUXDELETE flag.
    ///
    /// If the OPFLAG_NCHANGE (0x01) flag of P2 (NB: P2 not P5) is set, then the
    /// row change count is incremented (otherwise not).
    ///
    /// If the OPFLAG_ISNOOP (0x40) flag of P2 (not P5!) is set, then the
    /// pre-update-hook for deletes is run, but the btree is otherwise
    /// unchanged. This happens when the Delete is to be shortly followed by an
    /// Insert with the same key, causing the btree entry to be overwritten.
    ///
    /// P1 must not be pseudo-table. It has to be a real table with multiple
    /// rows.
    ///
    /// If P4 is not NULL then it points to a Table object. In this case either
    /// the update or pre-update hook, or both, may be invoked. The P1 cursor
    /// must have been positioned using NotFound prior to invoking this opcode
    /// in this case. Specifically, if one is configured, the pre-update hook is
    /// invoked if P4 is not NULL. The update-hook is invoked if one is
    /// configured, P4 is not NULL, and the OPFLAG_NCHANGE flag is set in P2.
    ///
    /// If the OPFLAG_ISUPDATE flag is set in P2, then P3 contains the address
    /// of the memory cell that contains the value that the rowid of the row
    /// will be set to by the update.
    Delete {
        /// Cursor number
        cursor: i32,
    },

    /// Convert P2 registers beginning with P1 into the record format use as a
    /// data record in a database table or as a key in an index. The Column
    /// opcode can decode the record later.
    ///
    /// P4 may be a string that is P2 characters long. The N-th character of the
    /// string indicates the column affinity that should be used for the N-th
    /// field of the index key.
    ///
    /// The mapping from character to affinity is given by the SQLITE_AFF_
    /// macros defined in sqliteInt.h.
    ///
    /// If P4 is NULL then all index fields have the affinity BLOB.
    ///
    /// The meaning of P5 depends on whether or not the SQLITE_ENABLE_NULL_TRIM
    /// compile-time option is enabled:
    ///
    /// - If SQLITE_ENABLE_NULL_TRIM is enabled, then the P5 is the index of the
    ///   right-most table that can be null-trimmed.
    ///
    /// - If SQLITE_ENABLE_NULL_TRIM is omitted, then P5 has the value
    ///   OPFLAG_NOCHNG_MAGIC if the MakeRecord opcode is allowed to accept
    ///   no-change records with serial_type 10. This value is only used inside an
    ///   assert() and does not affect the end result.
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
    /// Register P2 holds an SQL index key made using the MakeRecord
    /// instructions. This opcode writes that key into the index P1. Data for
    /// the entry is nil.
    ///
    /// If P4 is not zero, then it is the number of values in the unpacked key
    /// of reg(P2). In that case, P3 is the index of the first register for the
    /// unpacked key. The availability of the unpacked key can sometimes be an
    /// optimization.
    ///
    /// If P5 has the OPFLAG_APPEND bit set, that is a hint to the b-tree layer
    /// that this insert is likely to be an append.
    ///
    /// If P5 has the OPFLAG_NCHANGE bit set, then the change counter is
    /// incremented by this instruction. If the OPFLAG_NCHANGE bit is clear,
    /// then the change counter is unchanged.
    ///
    /// If the OPFLAG_USESEEKRESULT flag of P5 is set, the implementation might
    /// run faster by avoiding an unnecessary seek on cursor P1. However, the
    /// OPFLAG_USESEEKRESULT flag must only be set if there have been no prior
    /// seeks on the cursor or if the most recent seek used a key equivalent to
    /// P2.
    ///
    /// This instruction only works for indices. The equivalent instruction for
    /// tables is Insert.
    IdxInsert {
        /// Cursor number (index)
        cursor: i32,
        /// Register containing key
        key: i32,
    },

    /// The content of P3 registers starting at register P2 form an unpacked
    /// index key. This opcode removes that entry from the index opened by
    /// cursor P1.
    ///
    /// If P5 is not zero, then raise an SQLITE_CORRUPT_INDEX error if no
    /// matching index entry is found. This happens when running an UPDATE or
    /// DELETE statement and the index entry to be updated or deleted is not
    /// found. For some uses of IdxDelete (example: the EXCEPT operator) it does
    /// not matter that no matching entry is found. For those cases, P5 is zero.
    /// Also, do not raise this (self-correcting and non-critical) error if in
    /// writable_schema mode.
    IdxDelete {
        /// Cursor number (index)
        cursor: i32,
        /// Key register
        key: i32,
        /// Number of key fields
        num_fields: i32,
    },

    /// Write into register P2 an integer which is the last entry in the record
    /// at the end of the index key pointed to by cursor P1. This integer should
    /// be the rowid of the table entry to which this index entry points.
    ///
    /// See also: Rowid, MakeRecord.
    IdxRowid {
        /// Cursor number
        cursor: i32,
        /// Destination register
        dest: i32,
    },

    // =========================================================================
    // Program Initialization
    // =========================================================================
    /// Programs contain a single instance of this opcode as the very first
    /// opcode.
    ///
    /// If tracing is enabled (by the sqlite3_trace()) interface, then the UTF-8
    /// string contained in P4 is emitted on the trace callback. Or if P4 is
    /// blank, use the string returned by sqlite3_sql().
    ///
    /// If P2 is not zero, jump to instruction P2.
    ///
    /// Increment the value of P1 so that Once opcodes will jump the first time
    /// they are evaluated for this run.
    ///
    /// If P3 is not zero, then it is an address to jump to if an SQLITE_CORRUPT
    /// error is encountered.
    Init {
        /// Target address to begin execution
        target: i32,
    },

    // =========================================================================
    // Coroutines
    // =========================================================================
    /// Set up register P1 so that it will Yield to the coroutine located at
    /// address P3.
    ///
    /// If P2!=0 then the coroutine implementation immediately follows this
    /// opcode. So jump over the coroutine implementation to address P2.
    ///
    /// See also: EndCoroutine
    InitCoroutine {
        /// Coroutine register
        coroutine: i32,
        /// Initial entry point
        target: i32,
        /// End address
        end: i32,
    },

    /// Swap the program counter with the value in register P1. This has the
    /// effect of yielding to a coroutine.
    ///
    /// If the coroutine that is launched by this instruction ends with Yield or
    /// Return then continue to the next instruction. But if the coroutine
    /// launched by this instruction ends with EndCoroutine, then jump to P2
    /// rather than continuing with the next instruction.
    ///
    /// See also: InitCoroutine
    Yield {
        /// Coroutine register
        coroutine: i32,
    },

    /// The instruction at the address in register P1 is a Yield. Jump to the P2
    /// parameter of that Yield. After the jump, the value register P1 is left
    /// with a value such that subsequent OP_Yields go back to the this same
    /// EndCoroutine instruction.
    ///
    /// See also: InitCoroutine
    EndCoroutine {
        /// Coroutine register
        coroutine: i32,
    },

    // =========================================================================
    // Aggregation
    // =========================================================================
    /// Execute the xStep function for an aggregate. The function has P5
    /// arguments. P4 is a pointer to the FuncDef structure that specifies the
    /// function. Register P3 is the accumulator.
    ///
    /// The P5 arguments are taken from register P2 and its successors.
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

    /// P1 is the memory location that is the accumulator for an aggregate or
    /// window function. Execute the finalizer function for an aggregate and
    /// store the result in P1.
    ///
    /// P2 is the number of arguments that the step function takes and P4 is a
    /// pointer to the FuncDef for this function. The P2 argument is not used by
    /// this opcode. It is only there to disambiguate functions that can take
    /// varying numbers of arguments. The P4 argument is only needed for the
    /// case where the step function was not previously called.
    AggFinal {
        /// Accumulator register
        accum: i32,
        /// Number of arguments (for finalization)
        num_args: i32,
    },

    // =========================================================================
    // Miscellaneous
    // =========================================================================
    /// Do nothing. Continue downward to the next opcode.
    Noop,

    /// This is the same as Noop during normal query execution. The purpose of
    /// this opcode is to hold information about the query plan for the purpose
    /// of EXPLAIN QUERY PLAN output.
    ///
    /// The P4 value is human-readable text that describes the query plan
    /// element. Something like "SCAN t1" or "SEARCH t2 USING INDEX t2x1".
    ///
    /// The P1 value is the ID of the current element and P2 is the parent
    /// element for the case of nested query plan elements. If P2 is zero then
    /// this element is a top-level element.
    ///
    /// For loop elements, P3 is the estimated code of each invocation of this
    /// element.
    ///
    /// As with all opcodes, the meanings of the parameters for Explain are
    /// subject to change from one release to the next. Applications should not
    /// attempt to interpret or use any of the information contained in the
    /// Explain opcode. The information provided by this opcode is intended for
    /// testing and debugging use only.
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
            Insn::ShiftLeft { lhs, rhs, dest } => (*rhs, *lhs, *dest, 0), // P2 << P1
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
            Insn::HaltWithError {
                error_code,
                on_error,
            } => (*error_code, *on_error, 0, 0),
            Insn::HaltIfNull {
                src,
                error_code,
                target,
            } => (*src, *target, *error_code, 0),
            Insn::Goto { target } => (0, *target, 0, 0),
            Insn::Gosub { return_reg, target } => (*return_reg, *target, 0, 0),
            Insn::Return { return_reg } => (*return_reg, 0, 0, 0),
            Insn::If {
                src,
                target,
                jump_if_null,
            } => (*src, *target, if *jump_if_null { 1 } else { 0 }, 0),
            Insn::IfNot {
                src,
                target,
                jump_if_null,
            } => (*src, *target, if *jump_if_null { 1 } else { 0 }, 0),
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
            Insn::IfPos {
                src,
                target,
                decrement,
            } => (*src, *target, *decrement, 0),
            Insn::IfNotZero { src, target } => (*src, *target, 0, 0),
            Insn::DecrJumpZero { src, target } => (*src, *target, 0, 0),
            Insn::MustBeInt { src, target } => (*src, *target, 0, 0),

            // Results
            Insn::ResultRow { start, count } => (*start, *count, 0, 0),

            // Cursor operations
            Insn::OpenRead {
                cursor,
                root_page,
                db_num,
            } => (*cursor, *root_page, *db_num, 0),
            Insn::OpenWrite {
                cursor,
                root_page,
                db_num,
            } => (*cursor, *root_page, *db_num, 0),
            Insn::OpenEphemeral {
                cursor,
                num_columns,
            } => (*cursor, *num_columns, 0, 0),
            Insn::Close { cursor } => (*cursor, 0, 0, 0),
            Insn::Rewind { cursor, target } => (*cursor, *target, 0, 0),
            Insn::Next { cursor, target } => (*cursor, *target, 0, 0),
            Insn::Prev { cursor, target } => (*cursor, *target, 0, 0),
            Insn::Last { cursor, target } => (*cursor, *target, 0, 0),
            Insn::SeekGE {
                cursor,
                target,
                key,
                num_fields,
            } => (*cursor, *target, *key, *num_fields as u16),
            Insn::SeekGT {
                cursor,
                target,
                key,
                num_fields,
            } => (*cursor, *target, *key, *num_fields as u16),
            Insn::SeekLE {
                cursor,
                target,
                key,
                num_fields,
            } => (*cursor, *target, *key, *num_fields as u16),
            Insn::SeekLT {
                cursor,
                target,
                key,
                num_fields,
            } => (*cursor, *target, *key, *num_fields as u16),
            Insn::SeekRowid {
                cursor,
                target,
                rowid,
            } => (*cursor, *target, *rowid, 0),
            Insn::Column {
                cursor,
                column,
                dest,
            } => (*cursor, *column, *dest, 0),
            Insn::Rowid { cursor, dest } => (*cursor, *dest, 0, 0),
            Insn::NewRowid {
                cursor,
                dest,
                prev_rowid,
            } => (*cursor, *dest, *prev_rowid, 0),
            Insn::Insert {
                cursor,
                data,
                rowid,
            } => (*cursor, *data, *rowid, 0),
            Insn::Delete { cursor } => (*cursor, 0, 0, 0),
            Insn::MakeRecord { start, count, dest } => (*start, *count, *dest, 0),

            // Index operations
            Insn::IdxInsert { cursor, key } => (*cursor, *key, 0, 0),
            Insn::IdxDelete {
                cursor,
                key,
                num_fields,
            } => (*cursor, *key, *num_fields, 0),
            Insn::IdxRowid { cursor, dest } => (*cursor, *dest, 0, 0),

            // Init
            Insn::Init { target } => (0, *target, 0, 0),

            // Coroutines
            Insn::InitCoroutine {
                coroutine,
                target,
                end,
            } => (*coroutine, *target, *end, 0),
            Insn::Yield { coroutine } => (*coroutine, 0, 0, 0),
            Insn::EndCoroutine { coroutine } => (*coroutine, 0, 0, 0),

            // Aggregation
            Insn::AggStep {
                args,
                accum,
                num_args,
                ..
            } => (*args, 0, *accum, *num_args as u16),
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
            Insn::Raw {
                p4: P4::String(s), ..
            } => Some(InsnP4::String(s.clone())),
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

        let insn = Insn::Add {
            lhs: 1,
            rhs: 2,
            dest: 3,
        };
        assert_eq!(insn.operands(), (1, 2, 3, 0));

        // Test that Subtract swaps operands
        let insn = Insn::Subtract {
            lhs: 1,
            rhs: 2,
            dest: 3,
        };
        assert_eq!(insn.operands(), (2, 1, 3, 0)); // P2-P1, so swap
    }

    #[test]
    fn test_insn_name() {
        assert_eq!(Insn::Halt.name(), "Halt");
        assert_eq!(Insn::Integer { value: 0, dest: 0 }.name(), "Integer");
    }
}
