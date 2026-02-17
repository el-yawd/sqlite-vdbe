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
    FkCounter = 158,
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
    /// If P3 is positive, then reg\[P3\] is modified slightly so that it can be
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
    /// * If SQLITE_ENABLE_NULL_TRIM is enabled, then the P5 is the index of the
    ///   right-most table that can be null-trimmed.
    ///
    /// * If SQLITE_ENABLE_NULL_TRIM is omitted, then P5 has the value
    ///   OPFLAG_NOCHNG_MAGIC if the MakeRecord opcode is allowed to accept
    ///   no-change records with serial_type 10. This value is only used
    ///   inside an assert() and does not affect the end result.
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

    /// Execute the xStep (if P1==0) or xInverse (if P1!=0) function for an
    /// aggregate. The function has P5 arguments. P4 is a pointer to the
    /// FuncDef structure that specifies the function. Register P3 is the
    /// accumulator.
    ///
    /// The P5 arguments are taken from register P2 and its successors.
    ///
    /// This opcode is initially coded as OP_AggStep0. On first evaluation,
    /// the FuncDef stored in P4 is converted into an sqlite3_context and
    /// the opcode is changed. In this way, the initialization of the
    /// sqlite3_context only happens once, instead of on each call to the
    /// step function.
    AggStep1 {
        /// 0 for xStep, non-zero for xInverse
        is_inverse: i32,
        /// First argument register
        args: i32,
        /// Accumulator register
        accum: i32,
        /// Number of arguments
        num_args: u16,
    },

    /// Invoke the xValue() function and store the result in register P3.
    ///
    /// P2 is the number of arguments that the step function takes and
    /// P4 is a pointer to the FuncDef for this function. The P2 argument
    /// is not used by this opcode. It is only there to disambiguate functions
    /// that can take varying numbers of arguments. The P4 argument is only
    /// needed for the case where the step function was not previously called.
    AggValue {
        /// Number of arguments (unused, for disambiguation)
        num_args: i32,
        /// Destination register
        dest: i32,
    },

    /// Execute the xInverse function for an aggregate.
    /// The function has P5 arguments. P4 is a pointer to the
    /// FuncDef structure that specifies the function. Register P3 is the
    /// accumulator.
    ///
    /// The P5 arguments are taken from register P2 and its successors.
    AggInverse {
        /// First argument register
        args: i32,
        /// Accumulator register
        accum: i32,
        /// Number of arguments
        num_args: u16,
    },

    /// Invoke a user function (P4 is a pointer to an sqlite3_context object that
    /// contains a pointer to the function to be run) with arguments taken
    /// from register P2 and successors. The number of arguments is in
    /// the sqlite3_context object that P4 points to.
    /// The result of the function is stored in register P3.
    /// Register P3 must not be one of the function inputs.
    ///
    /// P1 is a 32-bit bitmask indicating whether or not each argument to the
    /// function was determined to be constant at compile time. If the first
    /// argument was constant then bit 0 of P1 is set. This is used to determine
    /// whether meta data associated with a user function argument using the
    /// sqlite3_set_auxdata() API may be safely retained until the next
    /// invocation of this opcode.
    Function {
        /// Constant argument bitmask
        const_mask: i32,
        /// First argument register
        args: i32,
        /// Destination register
        dest: i32,
    },

    /// Invoke a pure user function (no side effects).
    ///
    /// Same as Function but for pure functions. P4 is a pointer to an
    /// sqlite3_context object that contains a pointer to the function to be run.
    /// Arguments are taken from register P2 and successors.
    /// The result is stored in register P3.
    ///
    /// P1 is a 32-bit bitmask indicating whether or not each argument to the
    /// function was determined to be constant at compile time.
    PureFunc {
        /// Constant argument bitmask
        const_mask: i32,
        /// First argument register
        args: i32,
        /// Destination register
        dest: i32,
    },

    // =========================================================================
    // Logical Operations
    // =========================================================================
    /// Take the logical AND of the values in registers P1 and P2 and write the
    /// result into register P3.
    ///
    /// If either P1 or P2 is 0 (false) then the result is 0 even if the other
    /// input is NULL. A NULL and true or two NULLs give a NULL output.
    And {
        /// First operand register
        lhs: i32,
        /// Second operand register
        rhs: i32,
        /// Destination register
        dest: i32,
    },

    /// Take the logical OR of the values in register P1 and P2 and store the
    /// answer in register P3.
    ///
    /// If either P1 or P2 is nonzero (true) then the result is 1 (true) even if
    /// the other input is NULL. A NULL and false or two NULLs give a NULL
    /// output.
    Or {
        /// First operand register
        lhs: i32,
        /// Second operand register
        rhs: i32,
        /// Destination register
        dest: i32,
    },

    // =========================================================================
    // Type Operations
    // =========================================================================
    /// Force the value in register P1 to be the type defined by P2.
    ///
    /// P2 values: 'A' = BLOB, 'B' = TEXT, 'C' = NUMERIC, 'D' = INTEGER,
    /// 'E' = REAL. A NULL value is not changed by this routine. It remains
    /// NULL.
    Cast {
        /// Register to cast
        src: i32,
        /// Type affinity character
        affinity: i32,
    },

    /// Apply affinities to a range of P2 registers starting with P1.
    ///
    /// P4 is a string that is P2 characters long. The N-th character of the
    /// string indicates the column affinity that should be used for the N-th
    /// memory cell in the range.
    Affinity {
        /// First register
        start: i32,
        /// Number of registers
        count: i32,
    },

    /// If register P1 holds an integer convert it to a real value.
    ///
    /// This opcode is used when extracting information from a column that has
    /// REAL affinity. Such column values may still be stored as integers, for
    /// space efficiency, but after extraction we want them to have only a real
    /// value.
    RealAffinity {
        /// Register to convert
        src: i32,
    },

    /// Generate an error if the type of the content in register P1 does not
    /// satisfy the type constraints given by P5.
    TypeCheck {
        /// Register to check
        src: i32,
        /// Type mask (P5)
        type_mask: u16,
    },

    /// Jump to P2 if the type of a column in a btree is one of the types
    /// specified by the P5 bitmask.
    IsType {
        /// Cursor number (P1, or -1 for register)
        cursor: i32,
        /// Jump target
        target: i32,
        /// Column or register (P3)
        column: i32,
        /// Type bitmask
        type_mask: u16,
    },

    /// Interpret the value in register P1 as a boolean value. Store that
    /// boolean in register P2.
    IsTrue {
        /// Source register
        src: i32,
        /// Destination register
        dest: i32,
        /// Value for NULL
        null_value: i32,
    },

    // =========================================================================
    // Blob and String Operations
    // =========================================================================
    /// P4 points to a blob of data P1 bytes long. Store this blob in register
    /// P2.
    Blob {
        /// Length of blob
        len: i32,
        /// Destination register
        dest: i32,
    },

    /// The string value P4 of length P1 (bytes) is stored in register P2.
    ///
    /// If P3 is not zero and the content of register P3 is equal to P5, then
    /// the datatype of the register P2 is converted to BLOB.
    String {
        /// Length of string
        len: i32,
        /// Destination register
        dest: i32,
        /// Optional blob conversion register
        blob_reg: i32,
    },

    /// Transfer the values of bound parameter P1 into register P2.
    Variable {
        /// Parameter number (1-based)
        param: i32,
        /// Destination register
        dest: i32,
    },

    // =========================================================================
    // Null Operations
    // =========================================================================
    /// Set register P1 to have the value NULL as seen by the MakeRecord
    /// instruction, but do not free any string or blob memory associated with
    /// the register, so that if the value was a string or blob that was
    /// previously copied, the copy will still work.
    SoftNull {
        /// Register to set
        dest: i32,
    },

    /// If register P1 contains an integer, set register P2 to NULL. Otherwise,
    /// set register P2 to the same value as register P1.
    ///
    /// If P3 is greater than zero, then also check register P3. If the content
    /// of register P3 is NULL, then set register P2 to NULL and continue
    /// immediately to the next opcode.
    ZeroOrNull {
        /// Source register
        src: i32,
        /// Destination register
        dest: i32,
        /// Optional NULL check register
        null_check: i32,
    },

    /// Set all columns of the cursor P1 to be NULL values.
    NullRow {
        /// Cursor number
        cursor: i32,
    },

    // =========================================================================
    // Subroutine Operations
    // =========================================================================
    /// Mark the beginning of a subroutine that can be entered in-line or that
    /// can be called using Gosub.
    ///
    /// The subroutine should be terminated by a Return instruction that has a
    /// P1 equal to the P1 of this opcode.
    BeginSubrtn {
        /// Subroutine return address register
        return_reg: i32,
        /// Jump target for direct entry
        target: i32,
    },

    // =========================================================================
    // Seek and Search Operations
    // =========================================================================
    /// If P4==0 then register P3 holds a blob constructed by MakeRecord. If
    /// P4>0 then register P3 is the first of P4 registers that form an
    /// unpacked record.
    ///
    /// Cursor P1 is on an index btree. If the record identified by P3 and P4 is
    /// a prefix of any entry in P1 then a jump is made to P2.
    Found {
        /// Cursor number
        cursor: i32,
        /// Jump target if found
        target: i32,
        /// Key register
        key: i32,
        /// Number of key fields (0 = blob)
        num_fields: i32,
    },

    /// If P4==0 then register P3 holds a blob constructed by MakeRecord. If
    /// P4>0 then register P3 is the first of P4 registers that form an
    /// unpacked record.
    ///
    /// Cursor P1 is on an index btree. If the record identified by P3 and P4 is
    /// not the prefix of any entry in P1 then a jump is made to P2.
    NotFound {
        /// Cursor number
        cursor: i32,
        /// Jump target if not found
        target: i32,
        /// Key register
        key: i32,
        /// Number of key fields (0 = blob)
        num_fields: i32,
    },

    /// P1 is the index of a cursor open on an SQL table btree (with integer
    /// keys). P3 is an integer rowid. If P1 does not contain a record with
    /// rowid P3 then jump immediately to P2.
    NotExists {
        /// Cursor number
        cursor: i32,
        /// Jump target if not exists
        target: i32,
        /// Rowid register
        rowid: i32,
    },

    /// If P4==0 then register P3 holds a blob constructed by MakeRecord. If
    /// P4>0 then register P3 is the first of P4 registers that form an
    /// unpacked record.
    ///
    /// Cursor P1 is on an index btree. If the record identified by P3 and P4
    /// contains any NULL value, jump immediately to P2. Otherwise, perform a
    /// seek on cursor P1 to find an entry that matches all but the last field.
    /// If any entries match, jump to P2. If there are no entries that match,
    /// fall through.
    NoConflict {
        /// Cursor number
        cursor: i32,
        /// Jump target
        target: i32,
        /// Key register
        key: i32,
        /// Number of key fields
        num_fields: i32,
    },

    /// This opcode is similar to NotFound with the following differences:
    ///
    /// 1. The cursor P1 is always a covering index cursor 2. If the cursor is
    ///    positioned to a row that matches the key, control falls through
    IfNoHope {
        /// Cursor number
        cursor: i32,
        /// Jump target
        target: i32,
        /// Key register
        key: i32,
        /// Number of key fields
        num_fields: i32,
    },

    /// If cursor P1 is not open or if P1 is set to a NULL row, then jump to
    /// instruction P2. Otherwise, fall through.
    IfNotOpen {
        /// Cursor number
        cursor: i32,
        /// Jump target
        target: i32,
    },

    /// If all columns in the current row of cursor P1 are NULL, then set
    /// register P3 to NULL and jump to P2. Otherwise, continue with the next
    /// instruction.
    IfNullRow {
        /// Cursor number
        cursor: i32,
        /// Jump target
        target: i32,
        /// Register to set to NULL
        dest: i32,
    },

    // =========================================================================
    // Index Comparison Operations
    // =========================================================================
    /// The P4 register values beginning with P3 form an unpacked index key that
    /// omits the PRIMARY KEY. Compare this key value against the index that P1
    /// is currently pointing to, ignoring the PRIMARY KEY.
    ///
    /// If the P1 index entry is greater than or equal to the key value then
    /// jump to P2. Otherwise fall through to the next instruction.
    IdxGE {
        /// Cursor number
        cursor: i32,
        /// Jump target
        target: i32,
        /// First key register
        key: i32,
        /// Number of key fields
        num_fields: i32,
    },

    /// The P4 register values beginning with P3 form an unpacked index key that
    /// omits the PRIMARY KEY. Compare this key value against the index that P1
    /// is currently pointing to, ignoring the PRIMARY KEY.
    ///
    /// If the P1 index entry is greater than the key value then jump to P2.
    /// Otherwise fall through to the next instruction.
    IdxGT {
        /// Cursor number
        cursor: i32,
        /// Jump target
        target: i32,
        /// First key register
        key: i32,
        /// Number of key fields
        num_fields: i32,
    },

    /// The P4 register values beginning with P3 form an unpacked index key that
    /// omits the PRIMARY KEY. Compare this key value against the index that P1
    /// is currently pointing to, ignoring the PRIMARY KEY.
    ///
    /// If the P1 index entry is less than or equal to the key value then jump
    /// to P2. Otherwise fall through to the next instruction.
    IdxLE {
        /// Cursor number
        cursor: i32,
        /// Jump target
        target: i32,
        /// First key register
        key: i32,
        /// Number of key fields
        num_fields: i32,
    },

    /// The P4 register values beginning with P3 form an unpacked index key that
    /// omits the PRIMARY KEY. Compare this key value against the index that P1
    /// is currently pointing to, ignoring the PRIMARY KEY.
    ///
    /// If the P1 index entry is less than the key value then jump to P2.
    /// Otherwise fall through to the next instruction.
    IdxLT {
        /// Cursor number
        cursor: i32,
        /// Jump target
        target: i32,
        /// First key register
        key: i32,
        /// Number of key fields
        num_fields: i32,
    },

    // =========================================================================
    // Advanced Cursor Operations
    // =========================================================================
    /// Return in register P2 the next available pseudo-rowid for cursor P1.
    Sequence {
        /// Cursor number
        cursor: i32,
        /// Destination register
        dest: i32,
    },

    /// P1 is a sorter cursor. If the sequence counter is currently zero, jump
    /// to P2. Regardless, increment the sequence counter.
    SequenceTest {
        /// Cursor number
        cursor: i32,
        /// Jump target
        target: i32,
    },

    /// Write into register P2 the complete row content for the current row of
    /// cursor P1.
    RowData {
        /// Cursor number
        cursor: i32,
        /// Destination register
        dest: i32,
    },

    /// P1 is an open index cursor and P3 is a cursor on the corresponding
    /// table. This opcode does a deferred seek of the P3 table cursor to the
    /// row that corresponds to the current row of P1.
    DeferredSeek {
        /// Index cursor
        cursor: i32,
        /// Target address (unused)
        target: i32,
        /// Table cursor
        table_cursor: i32,
    },

    /// If a deferred seek is pending on cursor P1, complete that seek now.
    FinishSeek {
        /// Cursor number
        cursor: i32,
    },

    /// Position cursor P1 at the end of the btree for the purpose of appending
    /// a new entry onto the btree.
    SeekEnd {
        /// Cursor number
        cursor: i32,
    },

    /// Increase or decrease the seekHit value for cursor P1 by P2.
    SeekHit {
        /// Cursor number
        cursor: i32,
        /// Adjustment value
        adjustment: i32,
        /// New low value
        low: i32,
    },

    /// This opcode is a prefix opcode to SeekGE. It restricts the range of
    /// values that the subsequent SeekGE will consider.
    SeekScan {
        /// Cursor number
        cursor: i32,
        /// Jump target
        target: i32,
    },

    /// This opcode provides a hint to the cursor. P1 is the cursor number. P4
    /// is a bit vector that describes the cursor columns that are actually
    /// used.
    ColumnsUsed {
        /// Cursor number
        cursor: i32,
    },

    /// Open a new cursor P1 to the same table/index that cursor P2 is currently
    /// pointing to.
    OpenDup {
        /// New cursor number
        cursor: i32,
        /// Existing cursor to duplicate
        orig_cursor: i32,
    },

    /// Open a cursor P1 to a transient table that will be automatically deleted
    /// when the cursor is closed. The cursor is on an automatically created
    /// index.
    OpenAutoindex {
        /// Cursor number
        cursor: i32,
        /// Number of columns
        num_columns: i32,
    },

    /// Open a new cursor that points to a fake table that contains a single row
    /// of data. The content of that one row is the content of memory register
    /// P2.
    OpenPseudo {
        /// Cursor number
        cursor: i32,
        /// Content register
        content: i32,
        /// Number of columns
        num_columns: i32,
    },

    /// Copy the current record from cursor P1 into register P2.
    RowCell {
        /// Cursor number
        cursor: i32,
        /// Destination register
        dest: i32,
    },

    // =========================================================================
    // Sorter Operations
    // =========================================================================
    /// Open a new sorter cursor on a transient index.
    SorterOpen {
        /// Cursor number
        cursor: i32,
        /// Number of columns
        num_columns: i32,
    },

    /// After all records have been inserted into a sorter cursor, invoke this
    /// opcode to actually perform the sort.
    SorterSort {
        /// Cursor number
        cursor: i32,
        /// Jump target if empty
        target: i32,
    },

    /// This opcode does exactly the same thing as SorterSort, except for the
    /// name. It exists because sometimes the Sort opcode appears naturally in
    /// code generated for statements that require a sort.
    Sort {
        /// Cursor number
        cursor: i32,
        /// Jump target if empty
        target: i32,
    },

    /// Advance the sorter cursor P1 to the next entry. Jump to P2 if there are
    /// no more entries.
    SorterNext {
        /// Cursor number
        cursor: i32,
        /// Jump target for next entry
        target: i32,
    },

    /// Write the current sorter key into register P2.
    SorterData {
        /// Cursor number
        cursor: i32,
        /// Destination register
        dest: i32,
    },

    /// Write the P3 value into the sorter at cursor P1.
    SorterInsert {
        /// Cursor number
        cursor: i32,
        /// Key register
        key: i32,
    },

    /// Compare the key in the sorter to the key constructed by the MakeRecord
    /// from register P3.
    SorterCompare {
        /// Cursor number
        cursor: i32,
        /// Jump target
        target: i32,
        /// Key register
        key: i32,
        /// Number of key fields
        num_fields: i32,
    },

    /// Delete all contents from the sorter at cursor P1.
    ResetSorter {
        /// Cursor number
        cursor: i32,
    },

    // =========================================================================
    // Foreign Key Operations
    // =========================================================================
    /// Invoke the foreign key check and return an error if there are any
    /// outstanding foreign key constraint violations.
    FkCheck,

    /// Increment a "constraint counter" by P2 (P2 may be negative or positive).
    /// If P1 is non-zero, the database constraint counter is incremented
    /// (deferred foreign key constraints). Otherwise, if P1 is zero, the
    /// statement counter is incremented (immediate foreign key constraints).
    FkCounter {
        /// Counter type (0=statement, non-zero=database)
        counter_type: i32,
        /// Amount to add
        amount: i32,
    },

    /// This opcode tests if a foreign key constraint-counter is currently zero.
    /// If so, jump to instruction P2. Otherwise, fall through to the next
    /// instruction.
    ///
    /// If P1 is non-zero, then the jump is taken if the database constraint
    /// counter is zero (the one incremented by deferred constraints). If P1 is
    /// zero, the jump is taken if the statement constraint counter is zero.
    FkIfZero {
        /// Counter type (0=statement, non-zero=database)
        counter_type: i32,
        /// Jump target
        target: i32,
    },

    // =========================================================================
    // Transaction and Savepoint Operations
    // =========================================================================
    /// Begin a transaction on database P1 if a transaction is not already
    /// active. If P2 is non-zero, then a write-transaction is started.
    Transaction {
        /// Database index
        db_num: i32,
        /// 0=read, non-zero=write
        write: i32,
    },

    /// Open, release or rollback a savepoint.
    ///
    /// P1 is the savepoint operation: 0=SAVEPOINT, 1=RELEASE, 2=ROLLBACK.
    /// P4 is the name of the savepoint.
    Savepoint {
        /// Operation (0=begin, 1=release, 2=rollback)
        operation: i32,
    },

    /// Set the database auto-commit flag to P1 (1 or 0). If P2 is non-zero,
    /// roll back any currently active transaction.
    AutoCommit {
        /// Auto-commit flag (0 or 1)
        auto_commit: i32,
        /// Rollback flag
        rollback: i32,
    },

    /// Checkpoint database P1.
    Checkpoint {
        /// Database index
        db_num: i32,
        /// Checkpoint mode
        mode: i32,
    },

    /// Query the journal mode of database P1 and store the result in register
    /// P3.
    JournalMode {
        /// Database index
        db_num: i32,
        /// Jump target (unused)
        target: i32,
        /// Destination register
        dest: i32,
    },

    /// Run a complete VACUUM operation on the database.
    Vacuum {
        /// Database index
        db_num: i32,
    },

    // =========================================================================
    // Database Schema Operations
    // =========================================================================
    /// Allocate a new table in the main database if P1==0 or in the auxiliary
    /// database if P1==1 or in the temp database if P1==2.
    CreateBtree {
        /// Database index
        db_num: i32,
        /// Destination register for root page
        dest: i32,
        /// Flags
        flags: i32,
    },

    /// Run the SQL statement or statements specified in P4.
    SqlExec {
        /// Database index
        db_num: i32,
    },

    /// Read and parse all entries from the sqlite_schema table of database P1.
    ParseSchema {
        /// Database index
        db_num: i32,
    },

    /// Load the data for the ANALYZE results for database P1.
    LoadAnalysis {
        /// Database index
        db_num: i32,
    },

    /// Delete all information from the database table or index named P4.
    Destroy {
        /// Root page number
        root_page: i32,
        /// Database index
        db_num: i32,
    },

    /// Delete all records from the table identified by P1.
    Clear {
        /// Root page number
        root_page: i32,
        /// Database index
        db_num: i32,
        /// Reset rowid flag
        reset_rowid: i32,
    },

    /// Remove the internal sqlite_schema entry for a table or index.
    DropTable {
        /// Database index
        db_num: i32,
    },

    /// Remove the internal sqlite_schema entry for an index.
    DropIndex {
        /// Database index
        db_num: i32,
    },

    /// Remove the internal sqlite_schema entry for a trigger.
    DropTrigger {
        /// Database index
        db_num: i32,
    },

    // =========================================================================
    // Cookie Operations
    // =========================================================================
    /// Read cookie number P3 from database P1 and write it into register P2.
    ReadCookie {
        /// Database index
        db_num: i32,
        /// Destination register
        dest: i32,
        /// Cookie number
        cookie: i32,
    },

    /// Write P3 into cookie number P2 of database P1.
    SetCookie {
        /// Database index
        db_num: i32,
        /// Cookie number
        cookie: i32,
        /// Value register
        value: i32,
    },

    // =========================================================================
    // Count and Statistics Operations
    // =========================================================================
    /// Store the number of entries (an integer value) in the table or index
    /// opened by cursor P1 in register P2.
    Count {
        /// Cursor number
        cursor: i32,
        /// Destination register
        dest: i32,
    },

    /// Return the current rowid offset of cursor P1 in register P2.
    Offset {
        /// Cursor number
        cursor: i32,
        /// Destination register
        dest: i32,
    },

    /// Store the maximum number of pages in register P2, or the new max if P3
    /// is positive.
    MaxPgcnt {
        /// Database index
        db_num: i32,
        /// Destination register
        dest: i32,
        /// New maximum (if positive)
        new_max: i32,
    },

    /// Store the total number of pages in database P1 into register P2.
    Pagecount {
        /// Database index
        db_num: i32,
        /// Destination register
        dest: i32,
    },

    // =========================================================================
    // Virtual Table Operations
    // =========================================================================
    /// Call the xBegin method for a virtual table.
    ///
    /// P4 may be a pointer to an sqlite3_vtab structure. If so, call the
    /// xBegin method for that table. Also, whether or not P4 is set, check
    /// that this is not being called from within a callback to a virtual
    /// table xSync() method. If it is, the error code will be set to
    /// SQLITE_LOCKED.
    VBegin,

    /// Call the xCreate method for a virtual table.
    ///
    /// P2 is a register that holds the name of a virtual table in database
    /// P1. Call the xCreate method for that table.
    VCreate {
        /// Database number
        db_num: i32,
        /// Register containing table name
        name_reg: i32,
    },

    /// Call the xDestroy method for a virtual table.
    ///
    /// P4 is the name of a virtual table in database P1. Call the xDestroy
    /// method of that table.
    VDestroy {
        /// Database number
        db_num: i32,
    },

    /// Open a cursor to a virtual table.
    ///
    /// P4 is a pointer to a virtual table object, an sqlite3_vtab structure.
    /// P1 is a cursor number. This opcode opens a cursor to the virtual
    /// table and stores that cursor in P1.
    VOpen {
        /// Cursor number
        cursor: i32,
    },

    /// Run the xIntegrity method for a virtual table.
    ///
    /// P4 is a pointer to a Table object that is a virtual table in schema P1
    /// that supports the xIntegrity() method. This opcode runs the xIntegrity()
    /// method for that virtual table, using P3 as the integer argument. If
    /// an error is reported back, the table name is prepended to the error
    /// message and that message is stored in P2. If no errors are seen,
    /// register P2 is set to NULL.
    VCheck {
        /// Schema number
        schema: i32,
        /// Output register for error message
        dest: i32,
        /// Integer argument for xIntegrity
        arg: i32,
    },

    /// Set up a ValueList for sqlite3_vtab_in_first()/sqlite3_vtab_in_next().
    ///
    /// Set register P2 to be a pointer to a ValueList object for cursor P1
    /// with cache register P3 and output register P3+1. This ValueList object
    /// can be used as the first argument to sqlite3_vtab_in_first() and
    /// sqlite3_vtab_in_next() to extract all of the values stored in the P1
    /// cursor.
    VInitIn {
        /// Cursor number
        cursor: i32,
        /// Output register for ValueList pointer
        dest: i32,
        /// Cache register
        cache_reg: i32,
    },

    /// Filter a virtual table result set.
    ///
    /// P1 is a cursor opened using VOpen. P2 is an address to jump to if
    /// the filtered result set is empty. P4 is either NULL or a string that
    /// was generated by the xBestIndex method of the module. The interpretation
    /// of the P4 string is left to the module implementation.
    ///
    /// This opcode invokes the xFilter method on the virtual table specified
    /// by P1. The integer query plan parameter to xFilter is stored in register
    /// P3. Register P3+1 stores the argc parameter to be passed to the xFilter
    /// method. Registers P3+2..P3+1+argc are the argc additional parameters
    /// which are passed to xFilter as argv.
    VFilter {
        /// Cursor number
        cursor: i32,
        /// Jump target if empty
        target: i32,
        /// Register containing query plan
        args_reg: i32,
    },

    /// Get a column value from a virtual table.
    ///
    /// Store in register P3 the value of the P2-th column of the current row
    /// of the virtual-table of cursor P1.
    ///
    /// If the VColumn opcode is being used to fetch the value of an unchanging
    /// column during an UPDATE operation, then the P5 value is OPFLAG_NOCHNG.
    /// This will cause the sqlite3_vtab_nochange() function to return true
    /// inside the xColumn method of the virtual table implementation.
    VColumn {
        /// Cursor number
        cursor: i32,
        /// Column number
        column: i32,
        /// Destination register
        dest: i32,
        /// Flags (e.g., OPFLAG_NOCHNG)
        flags: u16,
    },

    /// Advance to the next row in a virtual table result set.
    ///
    /// Advance virtual table P1 to the next row in its result set and
    /// jump to instruction P2. Or, if the virtual table has reached
    /// the end of its result set, then fall through to the next instruction.
    VNext {
        /// Cursor number
        cursor: i32,
        /// Jump target if more rows
        target: i32,
    },

    /// Rename a virtual table.
    ///
    /// P4 is a pointer to a virtual table object, an sqlite3_vtab structure.
    /// This opcode invokes the corresponding xRename method. The value
    /// in register P1 is passed as the zName argument to the xRename method.
    VRename {
        /// Register containing new name
        name_reg: i32,
    },

    /// Update a virtual table (INSERT, UPDATE, or DELETE).
    ///
    /// P4 is a pointer to a virtual table object, an sqlite3_vtab structure.
    /// This opcode invokes the corresponding xUpdate method. P2 values
    /// are contiguous memory cells starting at P3 to pass to the xUpdate
    /// invocation. The value in register (P3+P2-1) corresponds to the
    /// p2th element of the argv array passed to xUpdate.
    ///
    /// The xUpdate method will do a DELETE or an INSERT or both.
    /// The argv\[0\] element (which corresponds to memory cell P3)
    /// is the rowid of a row to delete. If argv\[0\] is NULL then no
    /// deletion occurs. The argv\[1\] element is the rowid of the new
    /// row. This can be NULL to have the virtual table select the new
    /// rowid for itself. The subsequent elements in the array are
    /// the values of columns in the new row.
    ///
    /// If P2==1 then no insert is performed. argv\[0\] is the rowid of
    /// a row to delete.
    ///
    /// P1 is a boolean flag. If it is set to true and the xUpdate call
    /// is successful, then the value returned by sqlite3_last_insert_rowid()
    /// is set to the value of the rowid for the row just inserted.
    ///
    /// P5 is the error actions (OE_Replace, OE_Fail, etc) to apply in the case
    /// of a constraint failure on an insert or update.
    VUpdate {
        /// Update rowid flag
        update_rowid: i32,
        /// Number of arguments
        argc: i32,
        /// First argument register
        args_reg: i32,
        /// Error action flags
        on_error: u16,
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
    // Subtype Operations
    // =========================================================================
    /// Clear the subtype from register P1.
    ///
    /// This opcode clears the MEM_Subtype flag from the value in register P1.
    ClrSubtype {
        /// Register to clear subtype from
        src: i32,
    },

    /// Get the subtype from register P1 and store it in register P2.
    ///
    /// Extract the subtype value from register P1 and write that subtype
    /// into register P2. If P1 has no subtype, then P2 gets a NULL.
    GetSubtype {
        /// Source register
        src: i32,
        /// Destination register
        dest: i32,
    },

    /// Set the subtype of register P2 to the integer from register P1.
    ///
    /// Set the subtype value of register P2 to the integer from register P1.
    /// If P1 is NULL, clear the subtype from P2.
    SetSubtype {
        /// Register containing subtype value
        src: i32,
        /// Register to set subtype on
        dest: i32,
    },

    // =========================================================================
    // Cursor Locking
    // =========================================================================
    /// Lock the btree to which cursor P1 is pointing.
    ///
    /// Lock the btree so that it cannot be written by another cursor.
    CursorLock {
        /// Cursor number
        cursor: i32,
    },

    /// Unlock the btree to which cursor P1 is pointing.
    ///
    /// Unlock the btree so that it can be written by other cursors.
    CursorUnlock {
        /// Cursor number
        cursor: i32,
    },

    // =========================================================================
    // Statement Control
    // =========================================================================
    /// Cause precompiled statements to expire.
    ///
    /// When an expired statement is executed using sqlite3_step() it will either
    /// automatically reprepare itself (if it was originally created using
    /// sqlite3_prepare_v2()) or it will fail with SQLITE_SCHEMA.
    ///
    /// If P1 is 0, then all SQL statements become expired. If P1 is non-zero,
    /// then only the currently executing statement is expired.
    ///
    /// If P2 is 0, then SQL statements are expired immediately. If P2 is 1,
    /// then running SQL statements are allowed to continue to run to completion.
    Expire {
        /// If 1, expire only current statement; if 0, expire all
        current_only: i32,
        /// If 1, allow running statements to complete; if 0, expire immediately
        deferred: i32,
    },

    /// Reset the change counter.
    ///
    /// Copy the current change count to the database handle change counter
    /// (returned by sqlite3_changes()) then reset the VDBE's change counter
    /// to zero.
    ResetCount,

    // =========================================================================
    // Vacuum Operations
    // =========================================================================
    /// Perform a single step of the incremental vacuum procedure on database P1.
    ///
    /// If the vacuum has finished, jump to instruction P2. Otherwise, fall
    /// through to the next instruction.
    IncrVacuum {
        /// Database number
        db_num: i32,
        /// Jump target when done
        target: i32,
    },

    // =========================================================================
    // Size Estimation
    // =========================================================================
    /// Estimate the table size and jump if smaller than threshold.
    ///
    /// Check cursor P1 to see if the table has fewer rows than the log estimate
    /// in P3. If so, jump to instruction P2. Otherwise, fall through.
    IfSmaller {
        /// Cursor number
        cursor: i32,
        /// Jump target if table is smaller
        target: i32,
        /// Log estimate threshold
        threshold: i32,
    },

    // =========================================================================
    // Debug/Tracing
    // =========================================================================
    /// Verify that an Abort can happen (debug only).
    ///
    /// Assert if an Abort at this point might cause database corruption.
    /// This opcode only appears in debugging builds.
    Abortable,

    /// Trace opcode for debugging.
    ///
    /// Write a trace message. The P4 argument is a text string that is output
    /// along with other trace information.
    Trace,

    // =========================================================================
    // Memory Operations
    // =========================================================================
    /// Set register P1 to the maximum of P1 and P2.
    ///
    /// Register P1 must contain an integer. Compare the value in P1 with the
    /// value in P2. If P2 is greater, then copy P2 into P1.
    ///
    /// This instruction throws an error if the memory cell is not initially
    /// an integer.
    MemMax {
        /// Accumulator register
        accum: i32,
        /// Value register
        value: i32,
    },

    /// Compute LIMIT + OFFSET.
    ///
    /// If LIMIT (P1) is less than or equal to zero, set P2 to -1 (infinite).
    /// Otherwise compute LIMIT + OFFSET and store in P2. If the sum overflows,
    /// also set P2 to -1.
    OffsetLimit {
        /// LIMIT register
        limit: i32,
        /// Destination register
        dest: i32,
        /// OFFSET register
        offset: i32,
    },

    /// Release registers from service (debug only).
    ///
    /// Release P2 registers starting at P1. Any content in those registers
    /// is unreliable after this opcode completes.
    ///
    /// P3 is a bitmask of registers to preserve (bit i set = preserve P1+i).
    /// P5 flags cause released registers to be set to MEM_Undefined.
    ReleaseReg {
        /// Start register
        start: i32,
        /// Number of registers
        count: i32,
        /// Preserve mask
        mask: i32,
        /// Flags
        flags: u16,
    },

    // =========================================================================
    // RowSet Operations
    // =========================================================================
    /// Add an integer to a RowSet.
    ///
    /// Insert the integer value held in register P2 into a RowSet object
    /// held in register P1. If P1 does not contain a RowSet, create one.
    RowSetAdd {
        /// RowSet register
        rowset: i32,
        /// Integer value register
        value: i32,
    },

    /// Read a value from a RowSet.
    ///
    /// Extract the smallest value from the RowSet in P1 and put it in P3.
    /// If the RowSet is empty, leave P3 unchanged and jump to P2.
    RowSetRead {
        /// RowSet register
        rowset: i32,
        /// Jump target if empty
        target: i32,
        /// Destination register
        dest: i32,
    },

    /// Test if a value is in a RowSet.
    ///
    /// Check if the integer in P3 is in the RowSet in P1. If found, jump to P2.
    /// Otherwise, insert it into the RowSet and continue.
    ///
    /// P4 is an integer (set number) that identifies which set within the
    /// RowSet to test/insert. If P4 is -1, only test; if P4 >= 0, also insert.
    RowSetTest {
        /// RowSet register
        rowset: i32,
        /// Jump target if found
        target: i32,
        /// Value register
        value: i32,
        /// Set number
        set_num: i32,
    },

    // =========================================================================
    // Bloom Filter Operations
    // =========================================================================
    /// Add a key to a bloom filter.
    ///
    /// Compute a hash on the P4 registers starting with r\[P3\] and add that
    /// hash to the bloom filter contained in r\[P1\].
    FilterAdd {
        /// Bloom filter register
        filter: i32,
        /// Start of key registers
        key_start: i32,
        /// Number of key registers
        key_count: i32,
    },

    /// Check if a key might be in a bloom filter.
    ///
    /// Compute a hash on the key in the P4 registers starting with r\[P3\].
    /// Check if that hash is in the bloom filter in P1. If not present,
    /// jump to P2. Otherwise fall through.
    ///
    /// False negatives are harmless - it's always safe to fall through.
    Filter {
        /// Bloom filter register
        filter: i32,
        /// Jump target if not in filter
        target: i32,
        /// Start of key registers
        key_start: i32,
        /// Number of key registers
        key_count: i32,
    },

    // =========================================================================
    // Comparison Operations
    // =========================================================================
    /// Jump if the previous comparison was equal.
    ///
    /// This opcode must immediately follow an OP_Lt or OP_Gt comparison.
    /// If the comparison resulted in equality, jump to P2.
    ElseEq {
        /// Jump target if equal
        target: i32,
    },

    /// Set the permutation used by the next Compare opcode.
    ///
    /// The permutation is stored in P4 as an integer array. The first integer
    /// in the array is the length, and does not become part of the permutation.
    ///
    /// This opcode must immediately precede a Compare opcode that has the
    /// OPFLAG_PERMUTE bit set in P5.
    ///
    /// Note: P4 must be set separately as it requires a P4_INTARRAY pointer.
    Permutation,

    /// Compare two vectors of registers.
    ///
    /// Compare registers in reg(P1)..reg(P1+P3-1) with reg(P2)..reg(P2+P3-1).
    /// Save the comparison result for use by the next Jump instruction.
    ///
    /// If P5 has OPFLAG_PERMUTE set, the comparison order is determined by
    /// the preceding Permutation opcode.
    ///
    /// Note: P4 (KeyInfo) must be set separately for collation sequences.
    Compare {
        /// First register range start
        lhs: i32,
        /// Second register range start
        rhs: i32,
        /// Number of registers to compare
        count: i32,
        /// Flags (OPFLAG_PERMUTE)
        flags: u16,
    },

    // =========================================================================
    // Collation and Sorting
    // =========================================================================
    /// Set the collation sequence for subsequent operations.
    ///
    /// P4 is a pointer to a CollSeq structure. If P1 is non-zero, then
    /// register P1 is set to zero.
    ///
    /// Note: P4 (CollSeq pointer) must be set separately.
    CollSeq {
        /// Register to set to zero (0 if unused)
        dest: i32,
    },

    // =========================================================================
    // Cursor Operations (Advanced)
    // =========================================================================
    /// Reopen an index cursor if it's on a different index.
    ///
    /// If cursor P1 is open on an index with root page P2, clear the cursor
    /// and fall through. Otherwise, close and reopen the cursor on the new
    /// index.
    ///
    /// This is an optimization to avoid closing and reopening cursors
    /// unnecessarily.
    ///
    /// Note: P4 (KeyInfo) must be set separately.
    ReopenIdx {
        /// Cursor number
        cursor: i32,
        /// Root page number
        root: i32,
        /// Database number
        db_num: i32,
        /// Flags (OPFLAG_SEEKEQ)
        flags: u16,
    },

    /// Provide a hint to the cursor about expected access patterns.
    ///
    /// P1 is a cursor. P4 is an expression tree (Expr pointer) that
    /// describes the expected range of keys to be accessed.
    ///
    /// Note: P4 (Expr pointer) must be set separately. Requires
    /// SQLITE_ENABLE_CURSOR_HINTS compile flag.
    CursorHint {
        /// Cursor number
        cursor: i32,
    },

    // =========================================================================
    // Table Locking (Shared Cache)
    // =========================================================================
    /// Obtain a lock on a table.
    ///
    /// P1 is the database index. P2 is the root page of the table.
    /// P3 is 1 for a write lock, 0 for a read lock.
    /// P4 is the table name (for error messages).
    ///
    /// This is only used with shared-cache mode.
    TableLock {
        /// Database index
        db_num: i32,
        /// Root page of the table
        root: i32,
        /// 1 for write lock, 0 for read lock
        write: i32,
    },

    // =========================================================================
    // Integrity Check
    // =========================================================================
    /// Check database integrity.
    ///
    /// Do an analysis of the database to verify integrity. P1 is the register
    /// to store error messages. P2 is the number of root pages to check.
    /// P3 is the register containing the maximum number of errors to report.
    /// P4 is an array of root page numbers. P5 is the database number.
    ///
    /// Note: P4 (int array) must be set separately.
    IntegrityCk {
        /// Register for error message output
        msg_reg: i32,
        /// Number of root pages in P4 array
        count: i32,
        /// Register with max errors to report
        err_reg: i32,
        /// Database number
        db_num: u16,
    },

    // =========================================================================
    // Triggers and Subprograms
    // =========================================================================
    /// Execute a trigger subprogram.
    ///
    /// P4 is a pointer to the SubProgram structure for the trigger.
    /// P2 is the jump target if the trigger execution completes.
    /// P3 is a register to allocate runtime space.
    ///
    /// Note: P4 (SubProgram pointer) must be set separately.
    Program {
        /// Jump target on completion
        target: i32,
        /// Register for runtime space allocation
        runtime_reg: i32,
        /// Flags
        flags: u16,
    },

    /// Copy a trigger parameter to a register.
    ///
    /// This opcode is only valid within a trigger subprogram. It copies
    /// a value from the parent frame to register P2.
    ///
    /// P1 is the parameter offset in the parent frame.
    Param {
        /// Parameter index in parent frame
        index: i32,
        /// Destination register
        dest: i32,
    },

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
            Insn::AggStep1 { .. } => RawOpcode::AggStep1 as u8,
            Insn::AggValue { .. } => RawOpcode::AggValue as u8,
            Insn::AggInverse { .. } => RawOpcode::AggInverse as u8,
            Insn::Function { .. } => RawOpcode::Function as u8,
            Insn::PureFunc { .. } => RawOpcode::PureFunc as u8,

            // Logical
            Insn::And { .. } => RawOpcode::And as u8,
            Insn::Or { .. } => RawOpcode::Or as u8,

            // Type operations
            Insn::Cast { .. } => RawOpcode::Cast as u8,
            Insn::Affinity { .. } => RawOpcode::Affinity as u8,
            Insn::RealAffinity { .. } => RawOpcode::RealAffinity as u8,
            Insn::TypeCheck { .. } => RawOpcode::TypeCheck as u8,
            Insn::IsType { .. } => RawOpcode::IsType as u8,
            Insn::IsTrue { .. } => RawOpcode::IsTrue as u8,

            // Blob/String
            Insn::Blob { .. } => RawOpcode::Blob as u8,
            Insn::String { .. } => RawOpcode::String as u8,
            Insn::Variable { .. } => RawOpcode::Variable as u8,

            // Null operations
            Insn::SoftNull { .. } => RawOpcode::SoftNull as u8,
            Insn::ZeroOrNull { .. } => RawOpcode::ZeroOrNull as u8,
            Insn::NullRow { .. } => RawOpcode::NullRow as u8,

            // Subroutines
            Insn::BeginSubrtn { .. } => RawOpcode::BeginSubrtn as u8,

            // Seek/Search
            Insn::Found { .. } => RawOpcode::Found as u8,
            Insn::NotFound { .. } => RawOpcode::NotFound as u8,
            Insn::NotExists { .. } => RawOpcode::NotExists as u8,
            Insn::NoConflict { .. } => RawOpcode::NoConflict as u8,
            Insn::IfNoHope { .. } => RawOpcode::IfNoHope as u8,
            Insn::IfNotOpen { .. } => RawOpcode::IfNotOpen as u8,
            Insn::IfNullRow { .. } => RawOpcode::IfNullRow as u8,

            // Index comparisons
            Insn::IdxGE { .. } => RawOpcode::IdxGE as u8,
            Insn::IdxGT { .. } => RawOpcode::IdxGT as u8,
            Insn::IdxLE { .. } => RawOpcode::IdxLE as u8,
            Insn::IdxLT { .. } => RawOpcode::IdxLT as u8,

            // Advanced cursor
            Insn::Sequence { .. } => RawOpcode::Sequence as u8,
            Insn::SequenceTest { .. } => RawOpcode::SequenceTest as u8,
            Insn::RowData { .. } => RawOpcode::RowData as u8,
            Insn::DeferredSeek { .. } => RawOpcode::DeferredSeek as u8,
            Insn::FinishSeek { .. } => RawOpcode::FinishSeek as u8,
            Insn::SeekEnd { .. } => RawOpcode::SeekEnd as u8,
            Insn::SeekHit { .. } => RawOpcode::SeekHit as u8,
            Insn::SeekScan { .. } => RawOpcode::SeekScan as u8,
            Insn::ColumnsUsed { .. } => RawOpcode::ColumnsUsed as u8,
            Insn::OpenDup { .. } => RawOpcode::OpenDup as u8,
            Insn::OpenAutoindex { .. } => RawOpcode::OpenAutoindex as u8,
            Insn::OpenPseudo { .. } => RawOpcode::OpenPseudo as u8,
            Insn::RowCell { .. } => RawOpcode::RowCell as u8,

            // Sorter
            Insn::SorterOpen { .. } => RawOpcode::SorterOpen as u8,
            Insn::SorterSort { .. } => RawOpcode::SorterSort as u8,
            Insn::Sort { .. } => RawOpcode::Sort as u8,
            Insn::SorterNext { .. } => RawOpcode::SorterNext as u8,
            Insn::SorterData { .. } => RawOpcode::SorterData as u8,
            Insn::SorterInsert { .. } => RawOpcode::SorterInsert as u8,
            Insn::SorterCompare { .. } => RawOpcode::SorterCompare as u8,
            Insn::ResetSorter { .. } => RawOpcode::ResetSorter as u8,

            // Foreign keys
            Insn::FkCheck => RawOpcode::FkCheck as u8,
            Insn::FkCounter { .. } => RawOpcode::FkCounter as u8,
            Insn::FkIfZero { .. } => RawOpcode::FkIfZero as u8,

            // Transactions
            Insn::Transaction { .. } => RawOpcode::Transaction as u8,
            Insn::Savepoint { .. } => RawOpcode::Savepoint as u8,
            Insn::AutoCommit { .. } => RawOpcode::AutoCommit as u8,
            Insn::Checkpoint { .. } => RawOpcode::Checkpoint as u8,
            Insn::JournalMode { .. } => RawOpcode::JournalMode as u8,
            Insn::Vacuum { .. } => RawOpcode::Vacuum as u8,

            // Schema
            Insn::CreateBtree { .. } => RawOpcode::CreateBtree as u8,
            Insn::SqlExec { .. } => RawOpcode::SqlExec as u8,
            Insn::ParseSchema { .. } => RawOpcode::ParseSchema as u8,
            Insn::LoadAnalysis { .. } => RawOpcode::LoadAnalysis as u8,
            Insn::Destroy { .. } => RawOpcode::Destroy as u8,
            Insn::Clear { .. } => RawOpcode::Clear as u8,
            Insn::DropTable { .. } => RawOpcode::DropTable as u8,
            Insn::DropIndex { .. } => RawOpcode::DropIndex as u8,
            Insn::DropTrigger { .. } => RawOpcode::DropTrigger as u8,

            // Cookies
            Insn::ReadCookie { .. } => RawOpcode::ReadCookie as u8,
            Insn::SetCookie { .. } => RawOpcode::SetCookie as u8,

            // Statistics
            Insn::Count { .. } => RawOpcode::Count as u8,
            Insn::Offset { .. } => RawOpcode::Offset as u8,
            Insn::MaxPgcnt { .. } => RawOpcode::MaxPgcnt as u8,
            Insn::Pagecount { .. } => RawOpcode::Pagecount as u8,

            // Virtual tables
            Insn::VBegin => RawOpcode::VBegin as u8,
            Insn::VCreate { .. } => RawOpcode::VCreate as u8,
            Insn::VDestroy { .. } => RawOpcode::VDestroy as u8,
            Insn::VOpen { .. } => RawOpcode::VOpen as u8,
            Insn::VCheck { .. } => RawOpcode::VCheck as u8,
            Insn::VInitIn { .. } => RawOpcode::VInitIn as u8,
            Insn::VFilter { .. } => RawOpcode::VFilter as u8,
            Insn::VColumn { .. } => RawOpcode::VColumn as u8,
            Insn::VNext { .. } => RawOpcode::VNext as u8,
            Insn::VRename { .. } => RawOpcode::VRename as u8,
            Insn::VUpdate { .. } => RawOpcode::VUpdate as u8,

            Insn::Noop => RawOpcode::Noop as u8,
            Insn::Explain => RawOpcode::Explain as u8,

            // Subtype operations
            Insn::ClrSubtype { .. } => RawOpcode::ClrSubtype as u8,
            Insn::GetSubtype { .. } => RawOpcode::GetSubtype as u8,
            Insn::SetSubtype { .. } => RawOpcode::SetSubtype as u8,

            // Cursor locking
            Insn::CursorLock { .. } => RawOpcode::CursorLock as u8,
            Insn::CursorUnlock { .. } => RawOpcode::CursorUnlock as u8,

            // Statement control
            Insn::Expire { .. } => RawOpcode::Expire as u8,
            Insn::ResetCount => RawOpcode::ResetCount as u8,

            // Vacuum
            Insn::IncrVacuum { .. } => RawOpcode::IncrVacuum as u8,

            // Size estimation
            Insn::IfSmaller { .. } => RawOpcode::IfSmaller as u8,

            // Debug/tracing
            Insn::Abortable => RawOpcode::Abortable as u8,
            Insn::Trace => RawOpcode::Trace as u8,

            // Memory operations
            Insn::MemMax { .. } => RawOpcode::MemMax as u8,
            Insn::OffsetLimit { .. } => RawOpcode::OffsetLimit as u8,
            Insn::ReleaseReg { .. } => RawOpcode::ReleaseReg as u8,

            // RowSet operations
            Insn::RowSetAdd { .. } => RawOpcode::RowSetAdd as u8,
            Insn::RowSetRead { .. } => RawOpcode::RowSetRead as u8,
            Insn::RowSetTest { .. } => RawOpcode::RowSetTest as u8,

            // Bloom filter operations
            Insn::FilterAdd { .. } => RawOpcode::FilterAdd as u8,
            Insn::Filter { .. } => RawOpcode::Filter as u8,

            // Comparison
            Insn::ElseEq { .. } => RawOpcode::ElseEq as u8,

            // Advanced comparison
            Insn::Permutation => RawOpcode::Permutation as u8,
            Insn::Compare { .. } => RawOpcode::Compare as u8,

            // Collation
            Insn::CollSeq { .. } => RawOpcode::CollSeq as u8,

            // Advanced cursor
            Insn::ReopenIdx { .. } => RawOpcode::ReopenIdx as u8,
            Insn::CursorHint { .. } => RawOpcode::CursorHint as u8,

            // Table locking
            Insn::TableLock { .. } => RawOpcode::TableLock as u8,

            // Integrity check
            Insn::IntegrityCk { .. } => RawOpcode::IntegrityCk as u8,

            // Triggers
            Insn::Program { .. } => RawOpcode::Program as u8,
            Insn::Param { .. } => RawOpcode::Param as u8,

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

            // Aggregation/Functions
            Insn::AggStep {
                args,
                accum,
                num_args,
                ..
            } => (*args, 0, *accum, *num_args as u16),
            Insn::AggStep1 {
                is_inverse,
                args,
                accum,
                num_args,
            } => (*is_inverse, *args, *accum, *num_args),
            Insn::AggValue { num_args, dest } => (0, *num_args, *dest, 0),
            Insn::AggInverse {
                args,
                accum,
                num_args,
            } => (0, *args, *accum, *num_args),
            Insn::Function {
                const_mask,
                args,
                dest,
            } => (*const_mask, *args, *dest, 0),
            Insn::PureFunc {
                const_mask,
                args,
                dest,
            } => (*const_mask, *args, *dest, 0),
            Insn::AggFinal { accum, num_args } => (*accum, *num_args, 0, 0),

            // Logical
            Insn::And { lhs, rhs, dest } => (*lhs, *rhs, *dest, 0),
            Insn::Or { lhs, rhs, dest } => (*lhs, *rhs, *dest, 0),

            // Type operations
            Insn::Cast { src, affinity } => (*src, *affinity, 0, 0),
            Insn::Affinity { start, count } => (*start, *count, 0, 0),
            Insn::RealAffinity { src } => (*src, 0, 0, 0),
            Insn::TypeCheck { src, type_mask } => (*src, 0, 0, *type_mask),
            Insn::IsType {
                cursor,
                target,
                column,
                type_mask,
            } => (*cursor, *target, *column, *type_mask),
            Insn::IsTrue {
                src,
                dest,
                null_value,
            } => (*src, *dest, *null_value, 0),

            // Blob/String
            Insn::Blob { len, dest } => (*len, *dest, 0, 0),
            Insn::String {
                len,
                dest,
                blob_reg,
            } => (*len, *dest, *blob_reg, 0),
            Insn::Variable { param, dest } => (*param, *dest, 0, 0),

            // Null operations
            Insn::SoftNull { dest } => (*dest, 0, 0, 0),
            Insn::ZeroOrNull {
                src,
                dest,
                null_check,
            } => (*src, *dest, *null_check, 0),
            Insn::NullRow { cursor } => (*cursor, 0, 0, 0),

            // Subroutines
            Insn::BeginSubrtn { return_reg, target } => (*return_reg, *target, 0, 0),

            // Seek/Search
            Insn::Found {
                cursor,
                target,
                key,
                num_fields,
            } => (*cursor, *target, *key, *num_fields as u16),
            Insn::NotFound {
                cursor,
                target,
                key,
                num_fields,
            } => (*cursor, *target, *key, *num_fields as u16),
            Insn::NotExists {
                cursor,
                target,
                rowid,
            } => (*cursor, *target, *rowid, 0),
            Insn::NoConflict {
                cursor,
                target,
                key,
                num_fields,
            } => (*cursor, *target, *key, *num_fields as u16),
            Insn::IfNoHope {
                cursor,
                target,
                key,
                num_fields,
            } => (*cursor, *target, *key, *num_fields as u16),
            Insn::IfNotOpen { cursor, target } => (*cursor, *target, 0, 0),
            Insn::IfNullRow {
                cursor,
                target,
                dest,
            } => (*cursor, *target, *dest, 0),

            // Index comparisons
            Insn::IdxGE {
                cursor,
                target,
                key,
                num_fields,
            } => (*cursor, *target, *key, *num_fields as u16),
            Insn::IdxGT {
                cursor,
                target,
                key,
                num_fields,
            } => (*cursor, *target, *key, *num_fields as u16),
            Insn::IdxLE {
                cursor,
                target,
                key,
                num_fields,
            } => (*cursor, *target, *key, *num_fields as u16),
            Insn::IdxLT {
                cursor,
                target,
                key,
                num_fields,
            } => (*cursor, *target, *key, *num_fields as u16),

            // Advanced cursor
            Insn::Sequence { cursor, dest } => (*cursor, *dest, 0, 0),
            Insn::SequenceTest { cursor, target } => (*cursor, *target, 0, 0),
            Insn::RowData { cursor, dest } => (*cursor, *dest, 0, 0),
            Insn::DeferredSeek {
                cursor,
                target,
                table_cursor,
            } => (*cursor, *target, *table_cursor, 0),
            Insn::FinishSeek { cursor } => (*cursor, 0, 0, 0),
            Insn::SeekEnd { cursor } => (*cursor, 0, 0, 0),
            Insn::SeekHit {
                cursor,
                adjustment,
                low,
            } => (*cursor, *adjustment, *low, 0),
            Insn::SeekScan { cursor, target } => (*cursor, *target, 0, 0),
            Insn::ColumnsUsed { cursor } => (*cursor, 0, 0, 0),
            Insn::OpenDup {
                cursor,
                orig_cursor,
            } => (*cursor, *orig_cursor, 0, 0),
            Insn::OpenAutoindex {
                cursor,
                num_columns,
            } => (*cursor, *num_columns, 0, 0),
            Insn::OpenPseudo {
                cursor,
                content,
                num_columns,
            } => (*cursor, *content, *num_columns, 0),
            Insn::RowCell { cursor, dest } => (*cursor, *dest, 0, 0),

            // Sorter
            Insn::SorterOpen {
                cursor,
                num_columns,
            } => (*cursor, *num_columns, 0, 0),
            Insn::SorterSort { cursor, target } => (*cursor, *target, 0, 0),
            Insn::Sort { cursor, target } => (*cursor, *target, 0, 0),
            Insn::SorterNext { cursor, target } => (*cursor, *target, 0, 0),
            Insn::SorterData { cursor, dest } => (*cursor, *dest, 0, 0),
            Insn::SorterInsert { cursor, key } => (*cursor, *key, 0, 0),
            Insn::SorterCompare {
                cursor,
                target,
                key,
                num_fields,
            } => (*cursor, *target, *key, *num_fields as u16),
            Insn::ResetSorter { cursor } => (*cursor, 0, 0, 0),

            // Foreign keys
            Insn::FkCheck => (0, 0, 0, 0),
            Insn::FkCounter {
                counter_type,
                amount,
            } => (*counter_type, *amount, 0, 0),
            Insn::FkIfZero {
                counter_type,
                target,
            } => (*counter_type, *target, 0, 0),

            // Transactions
            Insn::Transaction { db_num, write } => (*db_num, *write, 0, 0),
            Insn::Savepoint { operation } => (*operation, 0, 0, 0),
            Insn::AutoCommit {
                auto_commit,
                rollback,
            } => (*auto_commit, *rollback, 0, 0),
            Insn::Checkpoint { db_num, mode } => (*db_num, *mode, 0, 0),
            Insn::JournalMode {
                db_num,
                target,
                dest,
            } => (*db_num, *target, *dest, 0),
            Insn::Vacuum { db_num } => (*db_num, 0, 0, 0),

            // Schema
            Insn::CreateBtree {
                db_num,
                dest,
                flags,
            } => (*db_num, *dest, *flags, 0),
            Insn::SqlExec { db_num } => (*db_num, 0, 0, 0),
            Insn::ParseSchema { db_num } => (*db_num, 0, 0, 0),
            Insn::LoadAnalysis { db_num } => (*db_num, 0, 0, 0),
            Insn::Destroy { root_page, db_num } => (*root_page, *db_num, 0, 0),
            Insn::Clear {
                root_page,
                db_num,
                reset_rowid,
            } => (*root_page, *db_num, *reset_rowid, 0),
            Insn::DropTable { db_num } => (*db_num, 0, 0, 0),
            Insn::DropIndex { db_num } => (*db_num, 0, 0, 0),
            Insn::DropTrigger { db_num } => (*db_num, 0, 0, 0),

            // Cookies
            Insn::ReadCookie {
                db_num,
                dest,
                cookie,
            } => (*db_num, *dest, *cookie, 0),
            Insn::SetCookie {
                db_num,
                cookie,
                value,
            } => (*db_num, *cookie, *value, 0),

            // Statistics
            Insn::Count { cursor, dest } => (*cursor, *dest, 0, 0),
            Insn::Offset { cursor, dest } => (*cursor, *dest, 0, 0),
            Insn::MaxPgcnt {
                db_num,
                dest,
                new_max,
            } => (*db_num, *dest, *new_max, 0),
            Insn::Pagecount { db_num, dest } => (*db_num, *dest, 0, 0),

            // Virtual tables
            Insn::VBegin => (0, 0, 0, 0),
            Insn::VCreate { db_num, name_reg } => (*db_num, *name_reg, 0, 0),
            Insn::VDestroy { db_num } => (*db_num, 0, 0, 0),
            Insn::VOpen { cursor } => (*cursor, 0, 0, 0),
            Insn::VCheck { schema, dest, arg } => (*schema, *dest, *arg, 0),
            Insn::VInitIn {
                cursor,
                dest,
                cache_reg,
            } => (*cursor, *dest, *cache_reg, 0),
            Insn::VFilter {
                cursor,
                target,
                args_reg,
            } => (*cursor, *target, *args_reg, 0),
            Insn::VColumn {
                cursor,
                column,
                dest,
                flags,
            } => (*cursor, *column, *dest, *flags),
            Insn::VNext { cursor, target } => (*cursor, *target, 0, 0),
            Insn::VRename { name_reg } => (*name_reg, 0, 0, 0),
            Insn::VUpdate {
                update_rowid,
                argc,
                args_reg,
                on_error,
            } => (*update_rowid, *argc, *args_reg, *on_error),

            // Misc
            Insn::Noop => (0, 0, 0, 0),
            Insn::Explain => (0, 0, 0, 0),

            // Subtype operations
            Insn::ClrSubtype { src } => (*src, 0, 0, 0),
            Insn::GetSubtype { src, dest } => (*src, *dest, 0, 0),
            Insn::SetSubtype { src, dest } => (*src, *dest, 0, 0),

            // Cursor locking
            Insn::CursorLock { cursor } => (*cursor, 0, 0, 0),
            Insn::CursorUnlock { cursor } => (*cursor, 0, 0, 0),

            // Statement control
            Insn::Expire {
                current_only,
                deferred,
            } => (*current_only, *deferred, 0, 0),
            Insn::ResetCount => (0, 0, 0, 0),

            // Vacuum
            Insn::IncrVacuum { db_num, target } => (*db_num, *target, 0, 0),

            // Size estimation
            Insn::IfSmaller {
                cursor,
                target,
                threshold,
            } => (*cursor, *target, *threshold, 0),

            // Debug/tracing
            Insn::Abortable => (0, 0, 0, 0),
            Insn::Trace => (0, 0, 0, 0),

            // Memory operations
            Insn::MemMax { accum, value } => (*accum, *value, 0, 0),
            Insn::OffsetLimit {
                limit,
                dest,
                offset,
            } => (*limit, *dest, *offset, 0),
            Insn::ReleaseReg {
                start,
                count,
                mask,
                flags,
            } => (*start, *count, *mask, *flags),

            // RowSet operations
            Insn::RowSetAdd { rowset, value } => (*rowset, *value, 0, 0),
            Insn::RowSetRead {
                rowset,
                target,
                dest,
            } => (*rowset, *target, *dest, 0),
            Insn::RowSetTest {
                rowset,
                target,
                value,
                set_num,
            } => (*rowset, *target, *value, *set_num as u16),

            // Bloom filter operations
            Insn::FilterAdd {
                filter,
                key_start,
                key_count,
            } => (*filter, 0, *key_start, *key_count as u16),
            Insn::Filter {
                filter,
                target,
                key_start,
                key_count,
            } => (*filter, *target, *key_start, *key_count as u16),

            // Comparison
            Insn::ElseEq { target } => (0, *target, 0, 0),

            // Advanced comparison
            Insn::Permutation => (0, 0, 0, 0),
            Insn::Compare {
                lhs,
                rhs,
                count,
                flags,
            } => (*lhs, *rhs, *count, *flags),

            // Collation
            Insn::CollSeq { dest } => (*dest, 0, 0, 0),

            // Advanced cursor
            Insn::ReopenIdx {
                cursor,
                root,
                db_num,
                flags,
            } => (*cursor, *root, *db_num, *flags),
            Insn::CursorHint { cursor } => (*cursor, 0, 0, 0),

            // Table locking
            Insn::TableLock {
                db_num,
                root,
                write,
            } => (*db_num, *root, *write, 0),

            // Integrity check
            Insn::IntegrityCk {
                msg_reg,
                count,
                err_reg,
                db_num,
            } => (*msg_reg, *count, *err_reg, *db_num),

            // Triggers
            Insn::Program {
                target,
                runtime_reg,
                flags,
            } => (0, *target, *runtime_reg, *flags),
            Insn::Param { index, dest } => (*index, *dest, 0, 0),

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
            Insn::AggStep1 { .. } => "AggStep1",
            Insn::AggValue { .. } => "AggValue",
            Insn::AggInverse { .. } => "AggInverse",
            Insn::Function { .. } => "Function",
            Insn::PureFunc { .. } => "PureFunc",

            // Logical
            Insn::And { .. } => "And",
            Insn::Or { .. } => "Or",

            // Type operations
            Insn::Cast { .. } => "Cast",
            Insn::Affinity { .. } => "Affinity",
            Insn::RealAffinity { .. } => "RealAffinity",
            Insn::TypeCheck { .. } => "TypeCheck",
            Insn::IsType { .. } => "IsType",
            Insn::IsTrue { .. } => "IsTrue",

            // Blob/String
            Insn::Blob { .. } => "Blob",
            Insn::String { .. } => "String",
            Insn::Variable { .. } => "Variable",

            // Null operations
            Insn::SoftNull { .. } => "SoftNull",
            Insn::ZeroOrNull { .. } => "ZeroOrNull",
            Insn::NullRow { .. } => "NullRow",

            // Subroutines
            Insn::BeginSubrtn { .. } => "BeginSubrtn",

            // Seek/Search
            Insn::Found { .. } => "Found",
            Insn::NotFound { .. } => "NotFound",
            Insn::NotExists { .. } => "NotExists",
            Insn::NoConflict { .. } => "NoConflict",
            Insn::IfNoHope { .. } => "IfNoHope",
            Insn::IfNotOpen { .. } => "IfNotOpen",
            Insn::IfNullRow { .. } => "IfNullRow",

            // Index comparisons
            Insn::IdxGE { .. } => "IdxGE",
            Insn::IdxGT { .. } => "IdxGT",
            Insn::IdxLE { .. } => "IdxLE",
            Insn::IdxLT { .. } => "IdxLT",

            // Advanced cursor
            Insn::Sequence { .. } => "Sequence",
            Insn::SequenceTest { .. } => "SequenceTest",
            Insn::RowData { .. } => "RowData",
            Insn::DeferredSeek { .. } => "DeferredSeek",
            Insn::FinishSeek { .. } => "FinishSeek",
            Insn::SeekEnd { .. } => "SeekEnd",
            Insn::SeekHit { .. } => "SeekHit",
            Insn::SeekScan { .. } => "SeekScan",
            Insn::ColumnsUsed { .. } => "ColumnsUsed",
            Insn::OpenDup { .. } => "OpenDup",
            Insn::OpenAutoindex { .. } => "OpenAutoindex",
            Insn::OpenPseudo { .. } => "OpenPseudo",
            Insn::RowCell { .. } => "RowCell",

            // Sorter
            Insn::SorterOpen { .. } => "SorterOpen",
            Insn::SorterSort { .. } => "SorterSort",
            Insn::Sort { .. } => "Sort",
            Insn::SorterNext { .. } => "SorterNext",
            Insn::SorterData { .. } => "SorterData",
            Insn::SorterInsert { .. } => "SorterInsert",
            Insn::SorterCompare { .. } => "SorterCompare",
            Insn::ResetSorter { .. } => "ResetSorter",

            // Foreign keys
            Insn::FkCheck => "FkCheck",
            Insn::FkCounter { .. } => "FkCounter",
            Insn::FkIfZero { .. } => "FkIfZero",

            // Transactions
            Insn::Transaction { .. } => "Transaction",
            Insn::Savepoint { .. } => "Savepoint",
            Insn::AutoCommit { .. } => "AutoCommit",
            Insn::Checkpoint { .. } => "Checkpoint",
            Insn::JournalMode { .. } => "JournalMode",
            Insn::Vacuum { .. } => "Vacuum",

            // Schema
            Insn::CreateBtree { .. } => "CreateBtree",
            Insn::SqlExec { .. } => "SqlExec",
            Insn::ParseSchema { .. } => "ParseSchema",
            Insn::LoadAnalysis { .. } => "LoadAnalysis",
            Insn::Destroy { .. } => "Destroy",
            Insn::Clear { .. } => "Clear",
            Insn::DropTable { .. } => "DropTable",
            Insn::DropIndex { .. } => "DropIndex",
            Insn::DropTrigger { .. } => "DropTrigger",

            // Cookies
            Insn::ReadCookie { .. } => "ReadCookie",
            Insn::SetCookie { .. } => "SetCookie",

            // Statistics
            Insn::Count { .. } => "Count",
            Insn::Offset { .. } => "Offset",
            Insn::MaxPgcnt { .. } => "MaxPgcnt",
            Insn::Pagecount { .. } => "Pagecount",

            // Virtual tables
            Insn::VBegin => "VBegin",
            Insn::VCreate { .. } => "VCreate",
            Insn::VDestroy { .. } => "VDestroy",
            Insn::VOpen { .. } => "VOpen",
            Insn::VCheck { .. } => "VCheck",
            Insn::VInitIn { .. } => "VInitIn",
            Insn::VFilter { .. } => "VFilter",
            Insn::VColumn { .. } => "VColumn",
            Insn::VNext { .. } => "VNext",
            Insn::VRename { .. } => "VRename",
            Insn::VUpdate { .. } => "VUpdate",

            Insn::Noop => "Noop",
            Insn::Explain => "Explain",

            // Subtype operations
            Insn::ClrSubtype { .. } => "ClrSubtype",
            Insn::GetSubtype { .. } => "GetSubtype",
            Insn::SetSubtype { .. } => "SetSubtype",

            // Cursor locking
            Insn::CursorLock { .. } => "CursorLock",
            Insn::CursorUnlock { .. } => "CursorUnlock",

            // Statement control
            Insn::Expire { .. } => "Expire",
            Insn::ResetCount => "ResetCount",

            // Vacuum
            Insn::IncrVacuum { .. } => "IncrVacuum",

            // Size estimation
            Insn::IfSmaller { .. } => "IfSmaller",

            // Debug/tracing
            Insn::Abortable => "Abortable",
            Insn::Trace => "Trace",

            // Memory operations
            Insn::MemMax { .. } => "MemMax",
            Insn::OffsetLimit { .. } => "OffsetLimit",
            Insn::ReleaseReg { .. } => "ReleaseReg",

            // RowSet operations
            Insn::RowSetAdd { .. } => "RowSetAdd",
            Insn::RowSetRead { .. } => "RowSetRead",
            Insn::RowSetTest { .. } => "RowSetTest",

            // Bloom filter operations
            Insn::FilterAdd { .. } => "FilterAdd",
            Insn::Filter { .. } => "Filter",

            // Comparison
            Insn::ElseEq { .. } => "ElseEq",

            // Advanced comparison
            Insn::Permutation => "Permutation",
            Insn::Compare { .. } => "Compare",

            // Collation
            Insn::CollSeq { .. } => "CollSeq",

            // Advanced cursor
            Insn::ReopenIdx { .. } => "ReopenIdx",
            Insn::CursorHint { .. } => "CursorHint",

            // Table locking
            Insn::TableLock { .. } => "TableLock",

            // Integrity check
            Insn::IntegrityCk { .. } => "IntegrityCk",

            // Triggers
            Insn::Program { .. } => "Program",
            Insn::Param { .. } => "Param",

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
