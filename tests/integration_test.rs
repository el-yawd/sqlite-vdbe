//! Integration tests for sqlite-vdbe
//!
//! Comprehensive test coverage including:
//! - All instruction types
//! - Edge cases and boundary conditions
//! - Error handling
//! - Fuzzing with random programs

use sqlite_vdbe::{Connection, Insn, StepResult, Value, ffi};

// ============================================================================
// Basic Instruction Tests
// ============================================================================

#[test]
fn test_simple_integer_program() {
    let mut conn = Connection::open_in_memory().expect("Failed to open connection");
    let mut builder = conn.new_program().expect("Failed to create program");

    let r1 = builder.alloc_register();

    builder.add(Insn::Integer { value: 42, dest: r1 });
    builder.add(Insn::ResultRow { start: r1, count: 1 });
    builder.add(Insn::Halt);

    let mut program = builder.finish(1).expect("Failed to finish program");

    let result = program.step().expect("Failed to step");
    assert_eq!(result, StepResult::Row);
    assert_eq!(program.column_int(0), 42);

    let result = program.step().expect("Failed to step");
    assert_eq!(result, StepResult::Done);
}

#[test]
fn test_c_simple_vdbe() {
    let conn = Connection::open_in_memory().expect("Failed to open connection");
    let result = unsafe { ffi::sqlite3_vdbe_test_simple(conn.raw_ptr()) };
    assert_eq!(result, 42);
}

#[test]
fn test_addition_program() {
    let mut conn = Connection::open_in_memory().expect("Failed to open connection");
    let mut builder = conn.new_program().expect("Failed to create program");

    let r1 = builder.alloc_register();
    let r2 = builder.alloc_register();
    let r3 = builder.alloc_register();

    builder.add(Insn::Integer { value: 10, dest: r1 });
    builder.add(Insn::Integer { value: 32, dest: r2 });
    builder.add(Insn::Add { lhs: r1, rhs: r2, dest: r3 });
    builder.add(Insn::ResultRow { start: r3, count: 1 });
    builder.add(Insn::Halt);

    let mut program = builder.finish(1).expect("Failed to finish program");

    let result = program.step().expect("Failed to step");
    assert_eq!(result, StepResult::Row);
    assert_eq!(program.column_int(0), 42);
}

#[test]
fn test_multiple_rows() {
    let mut conn = Connection::open_in_memory().expect("Failed to open connection");
    let mut builder = conn.new_program().expect("Failed to create program");

    let r1 = builder.alloc_register();

    builder.add(Insn::Integer { value: 1, dest: r1 });
    builder.add(Insn::ResultRow { start: r1, count: 1 });
    builder.add(Insn::Integer { value: 2, dest: r1 });
    builder.add(Insn::ResultRow { start: r1, count: 1 });
    builder.add(Insn::Integer { value: 3, dest: r1 });
    builder.add(Insn::ResultRow { start: r1, count: 1 });
    builder.add(Insn::Halt);

    let mut program = builder.finish(1).expect("Failed to finish program");

    let mut results = Vec::new();
    loop {
        match program.step().expect("Failed to step") {
            StepResult::Row => results.push(program.column_int(0)),
            StepResult::Done => break,
        }
    }

    assert_eq!(results, vec![1, 2, 3]);
}

#[test]
fn test_multiple_columns() {
    let mut conn = Connection::open_in_memory().expect("Failed to open connection");
    let mut builder = conn.new_program().expect("Failed to create program");

    let r1 = builder.alloc_register();
    let r2 = builder.alloc_register();
    let r3 = builder.alloc_register();

    builder.add(Insn::Integer { value: 100, dest: r1 });
    builder.add(Insn::Integer { value: 200, dest: r2 });
    builder.add(Insn::Integer { value: 300, dest: r3 });
    builder.add(Insn::ResultRow { start: r1, count: 3 });
    builder.add(Insn::Halt);

    let mut program = builder.finish(3).expect("Failed to finish program");

    let result = program.step().expect("Failed to step");
    assert_eq!(result, StepResult::Row);

    assert_eq!(program.column_count(), 3);
    assert_eq!(program.column_int(0), 100);
    assert_eq!(program.column_int(1), 200);
    assert_eq!(program.column_int(2), 300);
}

#[test]
fn test_arithmetic_operations() {
    let mut conn = Connection::open_in_memory().expect("Failed to open connection");
    let mut builder = conn.new_program().expect("Failed to create program");

    let r1 = builder.alloc_register();
    let r2 = builder.alloc_register();
    let r_add = builder.alloc_register();
    let r_sub = builder.alloc_register();
    let r_mul = builder.alloc_register();
    let r_div = builder.alloc_register();
    let r_rem = builder.alloc_register();

    builder.add(Insn::Integer { value: 17, dest: r1 });
    builder.add(Insn::Integer { value: 5, dest: r2 });

    builder.add(Insn::Add { lhs: r1, rhs: r2, dest: r_add });
    builder.add(Insn::Subtract { lhs: r1, rhs: r2, dest: r_sub });
    builder.add(Insn::Multiply { lhs: r1, rhs: r2, dest: r_mul });
    builder.add(Insn::Divide { lhs: r1, rhs: r2, dest: r_div });
    builder.add(Insn::Remainder { lhs: r1, rhs: r2, dest: r_rem });

    builder.add(Insn::ResultRow { start: r_add, count: 5 });
    builder.add(Insn::Halt);

    let mut program = builder.finish(5).expect("Failed to finish program");

    let result = program.step().expect("Failed to step");
    assert_eq!(result, StepResult::Row);

    assert_eq!(program.column_int(0), 22);  // 17 + 5
    assert_eq!(program.column_int(1), 12);  // 17 - 5
    assert_eq!(program.column_int(2), 85);  // 17 * 5
    assert_eq!(program.column_int(3), 3);   // 17 / 5
    assert_eq!(program.column_int(4), 2);   // 17 % 5
}

#[test]
fn test_program_reset() {
    let mut conn = Connection::open_in_memory().expect("Failed to open connection");
    let mut builder = conn.new_program().expect("Failed to create program");

    let r1 = builder.alloc_register();
    builder.add(Insn::Integer { value: 123, dest: r1 });
    builder.add(Insn::ResultRow { start: r1, count: 1 });
    builder.add(Insn::Halt);

    let mut program = builder.finish(1).expect("Failed to finish program");

    // First execution
    assert_eq!(program.step().unwrap(), StepResult::Row);
    assert_eq!(program.column_int(0), 123);
    assert_eq!(program.step().unwrap(), StepResult::Done);

    // Reset and execute again
    program.reset();

    assert_eq!(program.step().unwrap(), StepResult::Row);
    assert_eq!(program.column_int(0), 123);
    assert_eq!(program.step().unwrap(), StepResult::Done);
}

#[test]
fn test_null_value() {
    let mut conn = Connection::open_in_memory().expect("Failed to open connection");
    let mut builder = conn.new_program().expect("Failed to create program");

    let r1 = builder.alloc_register();

    builder.add(Insn::Null { dest: r1, count: 1 });
    builder.add(Insn::ResultRow { start: r1, count: 1 });
    builder.add(Insn::Halt);

    let mut program = builder.finish(1).expect("Failed to finish program");

    assert_eq!(program.step().unwrap(), StepResult::Row);
    let value = program.column_value(0);
    assert!(value.is_null());
}

#[test]
fn test_jump_here() {
    let mut conn = Connection::open_in_memory().expect("Failed to open connection");
    let mut builder = conn.new_program().expect("Failed to create program");

    let r1 = builder.alloc_register();

    let jump_addr = builder.add(Insn::Goto { target: 0 });
    builder.add(Insn::Integer { value: 999, dest: r1 }); // Skipped
    builder.jump_here(jump_addr);
    builder.add(Insn::Integer { value: 42, dest: r1 });
    builder.add(Insn::ResultRow { start: r1, count: 1 });
    builder.add(Insn::Halt);

    let mut program = builder.finish(1).expect("Failed to finish program");

    assert_eq!(program.step().unwrap(), StepResult::Row);
    assert_eq!(program.column_int(0), 42);
}

#[test]
fn test_comparison_eq() {
    let mut conn = Connection::open_in_memory().expect("Failed to open connection");
    let mut builder = conn.new_program().expect("Failed to create program");

    let r1 = builder.alloc_register();
    let r2 = builder.alloc_register();
    let r_result = builder.alloc_register();

    builder.add(Insn::Integer { value: 5, dest: r1 });
    builder.add(Insn::Integer { value: 5, dest: r2 });
    builder.add(Insn::Integer { value: 0, dest: r_result });

    let eq_addr = builder.add(Insn::Eq { lhs: r1, rhs: r2, target: 0 });
    let goto_addr = builder.add(Insn::Goto { target: 0 });

    builder.jump_here(eq_addr);
    builder.add(Insn::Integer { value: 1, dest: r_result });

    builder.jump_here(goto_addr);
    builder.add(Insn::ResultRow { start: r_result, count: 1 });
    builder.add(Insn::Halt);

    let mut program = builder.finish(1).expect("Failed to finish program");

    assert_eq!(program.step().unwrap(), StepResult::Row);
    assert_eq!(program.column_int(0), 1);
}

#[test]
fn test_copy_instruction() {
    let mut conn = Connection::open_in_memory().expect("Failed to open connection");
    let mut builder = conn.new_program().expect("Failed to create program");

    let r1 = builder.alloc_register();
    let r2 = builder.alloc_register();

    builder.add(Insn::Integer { value: 42, dest: r1 });
    builder.add(Insn::SCopy { src: r1, dest: r2 });
    builder.add(Insn::ResultRow { start: r2, count: 1 });
    builder.add(Insn::Halt);

    let mut program = builder.finish(1).expect("Failed to finish program");

    assert_eq!(program.step().unwrap(), StepResult::Row);
    assert_eq!(program.column_int(0), 42);
}

// ============================================================================
// Integer Boundary Tests
// ============================================================================

#[test]
fn test_integer_max_value() {
    let mut conn = Connection::open_in_memory().expect("Failed to open connection");
    let mut builder = conn.new_program().expect("Failed to create program");

    let r1 = builder.alloc_register();

    // i32::MAX
    builder.add(Insn::Integer { value: i32::MAX, dest: r1 });
    builder.add(Insn::ResultRow { start: r1, count: 1 });
    builder.add(Insn::Halt);

    let mut program = builder.finish(1).expect("Failed to finish program");

    assert_eq!(program.step().unwrap(), StepResult::Row);
    assert_eq!(program.column_int(0), i32::MAX);
}

#[test]
fn test_integer_min_value() {
    let mut conn = Connection::open_in_memory().expect("Failed to open connection");
    let mut builder = conn.new_program().expect("Failed to create program");

    let r1 = builder.alloc_register();

    builder.add(Insn::Integer { value: i32::MIN, dest: r1 });
    builder.add(Insn::ResultRow { start: r1, count: 1 });
    builder.add(Insn::Halt);

    let mut program = builder.finish(1).expect("Failed to finish program");

    assert_eq!(program.step().unwrap(), StepResult::Row);
    assert_eq!(program.column_int(0), i32::MIN);
}

// NOTE: Int64/Real P4 handling needs fixing (memory allocation mismatch with SQLite)
// For now, skip this test
#[test]
fn test_int64_large_values() {
    let mut conn = Connection::open_in_memory().expect("Failed to open connection");
    let mut builder = conn.new_program().expect("Failed to create program");

    let r1 = builder.alloc_register();
    let r2 = builder.alloc_register();

    builder.add(Insn::Int64 { value: i64::MAX, dest: r1 });
    builder.add(Insn::Int64 { value: i64::MIN, dest: r2 });
    builder.add(Insn::ResultRow { start: r1, count: 2 });
    builder.add(Insn::Halt);

    let mut program = builder.finish(2).expect("Failed to finish program");

    assert_eq!(program.step().unwrap(), StepResult::Row);
    assert_eq!(program.column_int64(0), i64::MAX);
    assert_eq!(program.column_int64(1), i64::MIN);
}

#[test]
fn test_integer_zero() {
    let mut conn = Connection::open_in_memory().expect("Failed to open connection");
    let mut builder = conn.new_program().expect("Failed to create program");

    let r1 = builder.alloc_register();

    builder.add(Insn::Integer { value: 0, dest: r1 });
    builder.add(Insn::ResultRow { start: r1, count: 1 });
    builder.add(Insn::Halt);

    let mut program = builder.finish(1).expect("Failed to finish program");

    assert_eq!(program.step().unwrap(), StepResult::Row);
    assert_eq!(program.column_int(0), 0);
}

#[test]
fn test_negative_integers() {
    let mut conn = Connection::open_in_memory().expect("Failed to open connection");
    let mut builder = conn.new_program().expect("Failed to create program");

    let r1 = builder.alloc_register();
    let r2 = builder.alloc_register();
    let r3 = builder.alloc_register();

    builder.add(Insn::Integer { value: -1, dest: r1 });
    builder.add(Insn::Integer { value: -100, dest: r2 });
    builder.add(Insn::Integer { value: -999999, dest: r3 });
    builder.add(Insn::ResultRow { start: r1, count: 3 });
    builder.add(Insn::Halt);

    let mut program = builder.finish(3).expect("Failed to finish program");

    assert_eq!(program.step().unwrap(), StepResult::Row);
    assert_eq!(program.column_int(0), -1);
    assert_eq!(program.column_int(1), -100);
    assert_eq!(program.column_int(2), -999999);
}

// ============================================================================
// Arithmetic Edge Cases
// ============================================================================

#[test]
fn test_add_negative_numbers() {
    let mut conn = Connection::open_in_memory().expect("Failed to open connection");
    let mut builder = conn.new_program().expect("Failed to create program");

    let r1 = builder.alloc_register();
    let r2 = builder.alloc_register();
    let r3 = builder.alloc_register();

    builder.add(Insn::Integer { value: -10, dest: r1 });
    builder.add(Insn::Integer { value: -20, dest: r2 });
    builder.add(Insn::Add { lhs: r1, rhs: r2, dest: r3 });
    builder.add(Insn::ResultRow { start: r3, count: 1 });
    builder.add(Insn::Halt);

    let mut program = builder.finish(1).expect("Failed to finish program");

    assert_eq!(program.step().unwrap(), StepResult::Row);
    assert_eq!(program.column_int(0), -30);
}

#[test]
fn test_subtract_to_negative() {
    let mut conn = Connection::open_in_memory().expect("Failed to open connection");
    let mut builder = conn.new_program().expect("Failed to create program");

    let r1 = builder.alloc_register();
    let r2 = builder.alloc_register();
    let r3 = builder.alloc_register();

    builder.add(Insn::Integer { value: 5, dest: r1 });
    builder.add(Insn::Integer { value: 10, dest: r2 });
    builder.add(Insn::Subtract { lhs: r1, rhs: r2, dest: r3 });
    builder.add(Insn::ResultRow { start: r3, count: 1 });
    builder.add(Insn::Halt);

    let mut program = builder.finish(1).expect("Failed to finish program");

    assert_eq!(program.step().unwrap(), StepResult::Row);
    assert_eq!(program.column_int(0), -5);
}

#[test]
fn test_multiply_by_zero() {
    let mut conn = Connection::open_in_memory().expect("Failed to open connection");
    let mut builder = conn.new_program().expect("Failed to create program");

    let r1 = builder.alloc_register();
    let r2 = builder.alloc_register();
    let r3 = builder.alloc_register();

    builder.add(Insn::Integer { value: 1000000, dest: r1 });
    builder.add(Insn::Integer { value: 0, dest: r2 });
    builder.add(Insn::Multiply { lhs: r1, rhs: r2, dest: r3 });
    builder.add(Insn::ResultRow { start: r3, count: 1 });
    builder.add(Insn::Halt);

    let mut program = builder.finish(1).expect("Failed to finish program");

    assert_eq!(program.step().unwrap(), StepResult::Row);
    assert_eq!(program.column_int(0), 0);
}

#[test]
fn test_multiply_negative_numbers() {
    let mut conn = Connection::open_in_memory().expect("Failed to open connection");
    let mut builder = conn.new_program().expect("Failed to create program");

    let r1 = builder.alloc_register();
    let r2 = builder.alloc_register();
    let r3 = builder.alloc_register();
    let r4 = builder.alloc_register();

    builder.add(Insn::Integer { value: -5, dest: r1 });
    builder.add(Insn::Integer { value: 3, dest: r2 });
    builder.add(Insn::Integer { value: -4, dest: r3 });

    // -5 * 3 = -15
    builder.add(Insn::Multiply { lhs: r1, rhs: r2, dest: r4 });
    builder.add(Insn::ResultRow { start: r4, count: 1 });

    // -5 * -4 = 20
    builder.add(Insn::Multiply { lhs: r1, rhs: r3, dest: r4 });
    builder.add(Insn::ResultRow { start: r4, count: 1 });

    builder.add(Insn::Halt);

    let mut program = builder.finish(1).expect("Failed to finish program");

    assert_eq!(program.step().unwrap(), StepResult::Row);
    assert_eq!(program.column_int(0), -15);

    assert_eq!(program.step().unwrap(), StepResult::Row);
    assert_eq!(program.column_int(0), 20);
}

#[test]
fn test_divide_by_zero_returns_null() {
    let mut conn = Connection::open_in_memory().expect("Failed to open connection");
    let mut builder = conn.new_program().expect("Failed to create program");

    let r1 = builder.alloc_register();
    let r2 = builder.alloc_register();
    let r3 = builder.alloc_register();

    builder.add(Insn::Integer { value: 42, dest: r1 });
    builder.add(Insn::Integer { value: 0, dest: r2 });
    builder.add(Insn::Divide { lhs: r1, rhs: r2, dest: r3 });
    builder.add(Insn::ResultRow { start: r3, count: 1 });
    builder.add(Insn::Halt);

    let mut program = builder.finish(1).expect("Failed to finish program");

    assert_eq!(program.step().unwrap(), StepResult::Row);
    // Division by zero returns NULL in SQLite
    assert!(program.column_value(0).is_null());
}

#[test]
fn test_remainder_by_zero_returns_null() {
    let mut conn = Connection::open_in_memory().expect("Failed to open connection");
    let mut builder = conn.new_program().expect("Failed to create program");

    let r1 = builder.alloc_register();
    let r2 = builder.alloc_register();
    let r3 = builder.alloc_register();

    builder.add(Insn::Integer { value: 42, dest: r1 });
    builder.add(Insn::Integer { value: 0, dest: r2 });
    builder.add(Insn::Remainder { lhs: r1, rhs: r2, dest: r3 });
    builder.add(Insn::ResultRow { start: r3, count: 1 });
    builder.add(Insn::Halt);

    let mut program = builder.finish(1).expect("Failed to finish program");

    assert_eq!(program.step().unwrap(), StepResult::Row);
    assert!(program.column_value(0).is_null());
}

#[test]
fn test_remainder_with_negatives() {
    let mut conn = Connection::open_in_memory().expect("Failed to open connection");
    let mut builder = conn.new_program().expect("Failed to create program");

    let r1 = builder.alloc_register();
    let r2 = builder.alloc_register();
    let r3 = builder.alloc_register();

    // -17 % 5
    builder.add(Insn::Integer { value: -17, dest: r1 });
    builder.add(Insn::Integer { value: 5, dest: r2 });
    builder.add(Insn::Remainder { lhs: r1, rhs: r2, dest: r3 });
    builder.add(Insn::ResultRow { start: r3, count: 1 });
    builder.add(Insn::Halt);

    let mut program = builder.finish(1).expect("Failed to finish program");

    assert_eq!(program.step().unwrap(), StepResult::Row);
    // SQLite uses truncated division, so -17 % 5 = -2
    assert_eq!(program.column_int(0), -2);
}

#[test]
fn test_integer_division_truncates() {
    let mut conn = Connection::open_in_memory().expect("Failed to open connection");
    let mut builder = conn.new_program().expect("Failed to create program");

    let r1 = builder.alloc_register();
    let r2 = builder.alloc_register();
    let r3 = builder.alloc_register();

    builder.add(Insn::Integer { value: 7, dest: r1 });
    builder.add(Insn::Integer { value: 3, dest: r2 });
    builder.add(Insn::Divide { lhs: r1, rhs: r2, dest: r3 });
    builder.add(Insn::ResultRow { start: r3, count: 1 });
    builder.add(Insn::Halt);

    let mut program = builder.finish(1).expect("Failed to finish program");

    assert_eq!(program.step().unwrap(), StepResult::Row);
    assert_eq!(program.column_int(0), 2); // 7/3 = 2 (truncated)
}

// ============================================================================
// Bitwise Operations
// ============================================================================

#[test]
fn test_bitwise_and() {
    let mut conn = Connection::open_in_memory().expect("Failed to open connection");
    let mut builder = conn.new_program().expect("Failed to create program");

    let r1 = builder.alloc_register();
    let r2 = builder.alloc_register();
    let r3 = builder.alloc_register();

    builder.add(Insn::Integer { value: 0b1100, dest: r1 }); // 12
    builder.add(Insn::Integer { value: 0b1010, dest: r2 }); // 10
    builder.add(Insn::BitAnd { lhs: r1, rhs: r2, dest: r3 });
    builder.add(Insn::ResultRow { start: r3, count: 1 });
    builder.add(Insn::Halt);

    let mut program = builder.finish(1).expect("Failed to finish program");

    assert_eq!(program.step().unwrap(), StepResult::Row);
    assert_eq!(program.column_int(0), 0b1000); // 8
}

#[test]
fn test_bitwise_or() {
    let mut conn = Connection::open_in_memory().expect("Failed to open connection");
    let mut builder = conn.new_program().expect("Failed to create program");

    let r1 = builder.alloc_register();
    let r2 = builder.alloc_register();
    let r3 = builder.alloc_register();

    builder.add(Insn::Integer { value: 0b1100, dest: r1 }); // 12
    builder.add(Insn::Integer { value: 0b1010, dest: r2 }); // 10
    builder.add(Insn::BitOr { lhs: r1, rhs: r2, dest: r3 });
    builder.add(Insn::ResultRow { start: r3, count: 1 });
    builder.add(Insn::Halt);

    let mut program = builder.finish(1).expect("Failed to finish program");

    assert_eq!(program.step().unwrap(), StepResult::Row);
    assert_eq!(program.column_int(0), 0b1110); // 14
}

#[test]
fn test_bitwise_not() {
    let mut conn = Connection::open_in_memory().expect("Failed to open connection");
    let mut builder = conn.new_program().expect("Failed to create program");

    let r1 = builder.alloc_register();
    let r2 = builder.alloc_register();

    builder.add(Insn::Integer { value: 0, dest: r1 });
    builder.add(Insn::BitNot { src: r1, dest: r2 });
    builder.add(Insn::ResultRow { start: r2, count: 1 });
    builder.add(Insn::Halt);

    let mut program = builder.finish(1).expect("Failed to finish program");

    assert_eq!(program.step().unwrap(), StepResult::Row);
    assert_eq!(program.column_int(0), -1); // ~0 = -1 in two's complement
}

#[test]
fn test_shift_left() {
    let mut conn = Connection::open_in_memory().expect("Failed to open connection");
    let mut builder = conn.new_program().expect("Failed to create program");

    let r1 = builder.alloc_register();
    let r2 = builder.alloc_register();
    let r3 = builder.alloc_register();

    builder.add(Insn::Integer { value: 1, dest: r1 });
    builder.add(Insn::Integer { value: 4, dest: r2 });
    builder.add(Insn::ShiftLeft { lhs: r1, rhs: r2, dest: r3 });
    builder.add(Insn::ResultRow { start: r3, count: 1 });
    builder.add(Insn::Halt);

    let mut program = builder.finish(1).expect("Failed to finish program");

    assert_eq!(program.step().unwrap(), StepResult::Row);
    assert_eq!(program.column_int(0), 16); // 1 << 4 = 16
}

#[test]
fn test_shift_right() {
    let mut conn = Connection::open_in_memory().expect("Failed to open connection");
    let mut builder = conn.new_program().expect("Failed to create program");

    let r1 = builder.alloc_register();
    let r2 = builder.alloc_register();
    let r3 = builder.alloc_register();

    builder.add(Insn::Integer { value: 32, dest: r1 });
    builder.add(Insn::Integer { value: 2, dest: r2 });
    builder.add(Insn::ShiftRight { lhs: r1, rhs: r2, dest: r3 });
    builder.add(Insn::ResultRow { start: r3, count: 1 });
    builder.add(Insn::Halt);

    let mut program = builder.finish(1).expect("Failed to finish program");

    assert_eq!(program.step().unwrap(), StepResult::Row);
    assert_eq!(program.column_int(0), 8); // 32 >> 2 = 8
}

// ============================================================================
// NULL Handling Tests
// ============================================================================

#[test]
fn test_null_multiple_registers() {
    let mut conn = Connection::open_in_memory().expect("Failed to open connection");
    let mut builder = conn.new_program().expect("Failed to create program");

    let r1 = builder.alloc_register();
    let _r2 = builder.alloc_register();
    let _r3 = builder.alloc_register();

    // Set 3 consecutive registers to NULL
    builder.add(Insn::Null { dest: r1, count: 3 });
    builder.add(Insn::ResultRow { start: r1, count: 3 });
    builder.add(Insn::Halt);

    let mut program = builder.finish(3).expect("Failed to finish program");

    assert_eq!(program.step().unwrap(), StepResult::Row);
    assert!(program.column_value(0).is_null());
    assert!(program.column_value(1).is_null());
    assert!(program.column_value(2).is_null());
}

#[test]
fn test_add_with_null() {
    let mut conn = Connection::open_in_memory().expect("Failed to open connection");
    let mut builder = conn.new_program().expect("Failed to create program");

    let r1 = builder.alloc_register();
    let r2 = builder.alloc_register();
    let r3 = builder.alloc_register();

    builder.add(Insn::Integer { value: 42, dest: r1 });
    builder.add(Insn::Null { dest: r2, count: 1 });
    builder.add(Insn::Add { lhs: r1, rhs: r2, dest: r3 });
    builder.add(Insn::ResultRow { start: r3, count: 1 });
    builder.add(Insn::Halt);

    let mut program = builder.finish(1).expect("Failed to finish program");

    assert_eq!(program.step().unwrap(), StepResult::Row);
    // Arithmetic with NULL returns NULL
    assert!(program.column_value(0).is_null());
}

#[test]
fn test_is_null_instruction() {
    let mut conn = Connection::open_in_memory().expect("Failed to open connection");
    let mut builder = conn.new_program().expect("Failed to create program");

    let r1 = builder.alloc_register();
    let r_result = builder.alloc_register();

    builder.add(Insn::Null { dest: r1, count: 1 });
    builder.add(Insn::Integer { value: 0, dest: r_result });

    let is_null_addr = builder.add(Insn::IsNull { src: r1, target: 0 });
    let goto_addr = builder.add(Insn::Goto { target: 0 });

    builder.jump_here(is_null_addr);
    builder.add(Insn::Integer { value: 1, dest: r_result });

    builder.jump_here(goto_addr);
    builder.add(Insn::ResultRow { start: r_result, count: 1 });
    builder.add(Insn::Halt);

    let mut program = builder.finish(1).expect("Failed to finish program");

    assert_eq!(program.step().unwrap(), StepResult::Row);
    assert_eq!(program.column_int(0), 1); // NULL check succeeded
}

#[test]
fn test_not_null_instruction() {
    let mut conn = Connection::open_in_memory().expect("Failed to open connection");
    let mut builder = conn.new_program().expect("Failed to create program");

    let r1 = builder.alloc_register();
    let r_result = builder.alloc_register();

    builder.add(Insn::Integer { value: 42, dest: r1 });
    builder.add(Insn::Integer { value: 0, dest: r_result });

    let not_null_addr = builder.add(Insn::NotNull { src: r1, target: 0 });
    let goto_addr = builder.add(Insn::Goto { target: 0 });

    builder.jump_here(not_null_addr);
    builder.add(Insn::Integer { value: 1, dest: r_result });

    builder.jump_here(goto_addr);
    builder.add(Insn::ResultRow { start: r_result, count: 1 });
    builder.add(Insn::Halt);

    let mut program = builder.finish(1).expect("Failed to finish program");

    assert_eq!(program.step().unwrap(), StepResult::Row);
    assert_eq!(program.column_int(0), 1); // Not NULL check succeeded
}

// ============================================================================
// Control Flow Tests
// ============================================================================

#[test]
fn test_if_true() {
    let mut conn = Connection::open_in_memory().expect("Failed to open connection");
    let mut builder = conn.new_program().expect("Failed to create program");

    let r1 = builder.alloc_register();
    let r_result = builder.alloc_register();

    builder.add(Insn::Integer { value: 1, dest: r1 }); // Non-zero = true
    builder.add(Insn::Integer { value: 0, dest: r_result });

    let if_addr = builder.add(Insn::If { src: r1, target: 0, jump_if_null: false });
    let goto_addr = builder.add(Insn::Goto { target: 0 });

    builder.jump_here(if_addr);
    builder.add(Insn::Integer { value: 1, dest: r_result });

    builder.jump_here(goto_addr);
    builder.add(Insn::ResultRow { start: r_result, count: 1 });
    builder.add(Insn::Halt);

    let mut program = builder.finish(1).expect("Failed to finish program");

    assert_eq!(program.step().unwrap(), StepResult::Row);
    assert_eq!(program.column_int(0), 1);
}

#[test]
fn test_if_false() {
    let mut conn = Connection::open_in_memory().expect("Failed to open connection");
    let mut builder = conn.new_program().expect("Failed to create program");

    let r1 = builder.alloc_register();
    let r_result = builder.alloc_register();

    builder.add(Insn::Integer { value: 0, dest: r1 }); // Zero = false
    builder.add(Insn::Integer { value: 0, dest: r_result });

    let if_addr = builder.add(Insn::If { src: r1, target: 0, jump_if_null: false });
    let goto_addr = builder.add(Insn::Goto { target: 0 });

    builder.jump_here(if_addr);
    builder.add(Insn::Integer { value: 1, dest: r_result });

    builder.jump_here(goto_addr);
    builder.add(Insn::ResultRow { start: r_result, count: 1 });
    builder.add(Insn::Halt);

    let mut program = builder.finish(1).expect("Failed to finish program");

    assert_eq!(program.step().unwrap(), StepResult::Row);
    assert_eq!(program.column_int(0), 0); // Jump not taken
}

#[test]
fn test_ifnot_instruction() {
    let mut conn = Connection::open_in_memory().expect("Failed to open connection");
    let mut builder = conn.new_program().expect("Failed to create program");

    let r1 = builder.alloc_register();
    let r_result = builder.alloc_register();

    builder.add(Insn::Integer { value: 0, dest: r1 }); // Zero
    builder.add(Insn::Integer { value: 0, dest: r_result });

    let ifnot_addr = builder.add(Insn::IfNot { src: r1, target: 0, jump_if_null: false });
    let goto_addr = builder.add(Insn::Goto { target: 0 });

    builder.jump_here(ifnot_addr);
    builder.add(Insn::Integer { value: 1, dest: r_result });

    builder.jump_here(goto_addr);
    builder.add(Insn::ResultRow { start: r_result, count: 1 });
    builder.add(Insn::Halt);

    let mut program = builder.finish(1).expect("Failed to finish program");

    assert_eq!(program.step().unwrap(), StepResult::Row);
    assert_eq!(program.column_int(0), 1); // Jump taken on zero
}

#[test]
fn test_loop_with_ifnotzero() {
    // Use IfNotZero for loops: it decrements and jumps if the value WAS not zero
    let mut conn = Connection::open_in_memory().expect("Failed to open connection");
    let mut builder = conn.new_program().expect("Failed to create program");

    let r_counter = builder.alloc_register();
    let r_sum = builder.alloc_register();

    // Initialize: counter = 5, sum = 0
    builder.add(Insn::Integer { value: 5, dest: r_counter });
    builder.add(Insn::Integer { value: 0, dest: r_sum });

    // Loop: add counter to sum, then decrement and loop back while not zero
    let loop_start = builder.current_addr();
    builder.add(Insn::Add { lhs: r_sum, rhs: r_counter, dest: r_sum });
    builder.add(Insn::IfNotZero { src: r_counter, target: loop_start.raw() });

    builder.add(Insn::ResultRow { start: r_sum, count: 1 });
    builder.add(Insn::Halt);

    let mut program = builder.finish(1).expect("Failed to finish program");

    assert_eq!(program.step().unwrap(), StepResult::Row);
    // Trace with IfNotZero (checks before decrement):
    // iter1: sum=0+5=5, check(5)!=0, decr to 4, jump
    // iter2: sum=5+4=9, check(4)!=0, decr to 3, jump
    // iter3: sum=9+3=12, check(3)!=0, decr to 2, jump
    // iter4: sum=12+2=14, check(2)!=0, decr to 1, jump
    // iter5: sum=14+1=15, check(1)!=0, decr to 0, jump
    // iter6: sum=15+0=15, check(0)==0, decr to -1, no jump
    // Final: sum = 15
    assert_eq!(program.column_int(0), 15);
}

#[test]
fn test_ifpos_instruction() {
    let mut conn = Connection::open_in_memory().expect("Failed to open connection");
    let mut builder = conn.new_program().expect("Failed to create program");

    let r1 = builder.alloc_register();
    let r_result = builder.alloc_register();

    builder.add(Insn::Integer { value: 5, dest: r1 });
    builder.add(Insn::Integer { value: 0, dest: r_result });

    let ifpos_addr = builder.add(Insn::IfPos { src: r1, target: 0, decrement: 0 });
    let goto_addr = builder.add(Insn::Goto { target: 0 });

    builder.jump_here(ifpos_addr);
    builder.add(Insn::Integer { value: 1, dest: r_result });

    builder.jump_here(goto_addr);
    builder.add(Insn::ResultRow { start: r_result, count: 1 });
    builder.add(Insn::Halt);

    let mut program = builder.finish(1).expect("Failed to finish program");

    assert_eq!(program.step().unwrap(), StepResult::Row);
    assert_eq!(program.column_int(0), 1); // 5 is positive
}

#[test]
fn test_ifnotzero_instruction() {
    // Note: IfNotZero decrements P1 internally, so don't use AddImm too
    let mut conn = Connection::open_in_memory().expect("Failed to open connection");
    let mut builder = conn.new_program().expect("Failed to create program");

    let r_counter = builder.alloc_register();
    let r_sum = builder.alloc_register();

    // Initialize counter=3, sum=0
    builder.add(Insn::Integer { value: 3, dest: r_counter });
    builder.add(Insn::Integer { value: 0, dest: r_sum });

    // Loop: IfNotZero decrements counter and jumps if not zero
    // So we add BEFORE the decrement happens
    let loop_start = builder.current_addr();
    builder.add(Insn::Add { lhs: r_sum, rhs: r_counter, dest: r_sum });
    // IfNotZero checks if counter != 0, decrements, and jumps if it was not zero
    builder.add(Insn::IfNotZero { src: r_counter, target: loop_start.raw() });

    builder.add(Insn::ResultRow { start: r_sum, count: 1 });
    builder.add(Insn::Halt);

    let mut program = builder.finish(1).expect("Failed to finish program");

    assert_eq!(program.step().unwrap(), StepResult::Row);
    // Trace: counter starts at 3
    // iter1: sum=0+3=3, counter=3->2 (decr), jump (3 was not 0)
    // iter2: sum=3+2=5, counter=2->1 (decr), jump (2 was not 0)
    // iter3: sum=5+1=6, counter=1->0 (decr), jump (1 was not 0)
    // iter4: sum=6+0=6, counter=0->-1 (decr), no jump (0 was 0)
    // Actually that's 4 iterations. Let me trace more carefully.
    // The jump happens AFTER decrement but based on value BEFORE decrement.
    // So: if value != 0 { decr; jump } else { decr }
    // iter1: check(3)!=0, decr 3->2, jump, sum=0+3=3
    // Hmm, the add happens before the check...
    //
    // Actually sum = 3 + 2 + 1 + 0 = 6 with the trace:
    // - sum=0+3=3, check(3), decr to 2, jump
    // - sum=3+2=5, check(2), decr to 1, jump
    // - sum=5+1=6, check(1), decr to 0, jump
    // - sum=6+0=6, check(0), decr to -1, no jump
    assert_eq!(program.column_int(0), 6);
}

// ============================================================================
// Comparison Tests
// ============================================================================

#[test]
fn test_all_comparisons() {
    // Test each comparison individually to avoid register ordering issues
    fn test_cmp(a: i32, b: i32, expected_eq: bool, expected_ne: bool, expected_lt: bool, expected_le: bool, expected_gt: bool, expected_ge: bool) {
        // Test Eq
        {
            let mut conn = Connection::open_in_memory().expect("Failed to open connection");
            let mut builder = conn.new_program().expect("Failed to create program");
            let r1 = builder.alloc_register();
            let r2 = builder.alloc_register();
            let r_result = builder.alloc_register();

            builder.add(Insn::Integer { value: a, dest: r1 });
            builder.add(Insn::Integer { value: b, dest: r2 });
            builder.add(Insn::Integer { value: 0, dest: r_result });

            let eq_addr = builder.add(Insn::Eq { lhs: r1, rhs: r2, target: 0 });
            let skip = builder.add(Insn::Goto { target: 0 });
            builder.jump_here(eq_addr);
            builder.add(Insn::Integer { value: 1, dest: r_result });
            builder.jump_here(skip);
            builder.add(Insn::ResultRow { start: r_result, count: 1 });
            builder.add(Insn::Halt);

            let mut program = builder.finish(1).expect("Failed to finish program");
            assert_eq!(program.step().unwrap(), StepResult::Row);
            assert_eq!(program.column_int(0) != 0, expected_eq, "Eq failed for {} == {}", a, b);
        }

        // Test Ne
        {
            let mut conn = Connection::open_in_memory().expect("Failed to open connection");
            let mut builder = conn.new_program().expect("Failed to create program");
            let r1 = builder.alloc_register();
            let r2 = builder.alloc_register();
            let r_result = builder.alloc_register();

            builder.add(Insn::Integer { value: a, dest: r1 });
            builder.add(Insn::Integer { value: b, dest: r2 });
            builder.add(Insn::Integer { value: 0, dest: r_result });

            let ne_addr = builder.add(Insn::Ne { lhs: r1, rhs: r2, target: 0 });
            let skip = builder.add(Insn::Goto { target: 0 });
            builder.jump_here(ne_addr);
            builder.add(Insn::Integer { value: 1, dest: r_result });
            builder.jump_here(skip);
            builder.add(Insn::ResultRow { start: r_result, count: 1 });
            builder.add(Insn::Halt);

            let mut program = builder.finish(1).expect("Failed to finish program");
            assert_eq!(program.step().unwrap(), StepResult::Row);
            assert_eq!(program.column_int(0) != 0, expected_ne, "Ne failed for {} != {}", a, b);
        }

        // Test Lt
        {
            let mut conn = Connection::open_in_memory().expect("Failed to open connection");
            let mut builder = conn.new_program().expect("Failed to create program");
            let r1 = builder.alloc_register();
            let r2 = builder.alloc_register();
            let r_result = builder.alloc_register();

            builder.add(Insn::Integer { value: a, dest: r1 });
            builder.add(Insn::Integer { value: b, dest: r2 });
            builder.add(Insn::Integer { value: 0, dest: r_result });

            let lt_addr = builder.add(Insn::Lt { lhs: r1, rhs: r2, target: 0 });
            let skip = builder.add(Insn::Goto { target: 0 });
            builder.jump_here(lt_addr);
            builder.add(Insn::Integer { value: 1, dest: r_result });
            builder.jump_here(skip);
            builder.add(Insn::ResultRow { start: r_result, count: 1 });
            builder.add(Insn::Halt);

            let mut program = builder.finish(1).expect("Failed to finish program");
            assert_eq!(program.step().unwrap(), StepResult::Row);
            assert_eq!(program.column_int(0) != 0, expected_lt, "Lt failed for {} < {}", a, b);
        }

        // Test Le
        {
            let mut conn = Connection::open_in_memory().expect("Failed to open connection");
            let mut builder = conn.new_program().expect("Failed to create program");
            let r1 = builder.alloc_register();
            let r2 = builder.alloc_register();
            let r_result = builder.alloc_register();

            builder.add(Insn::Integer { value: a, dest: r1 });
            builder.add(Insn::Integer { value: b, dest: r2 });
            builder.add(Insn::Integer { value: 0, dest: r_result });

            let le_addr = builder.add(Insn::Le { lhs: r1, rhs: r2, target: 0 });
            let skip = builder.add(Insn::Goto { target: 0 });
            builder.jump_here(le_addr);
            builder.add(Insn::Integer { value: 1, dest: r_result });
            builder.jump_here(skip);
            builder.add(Insn::ResultRow { start: r_result, count: 1 });
            builder.add(Insn::Halt);

            let mut program = builder.finish(1).expect("Failed to finish program");
            assert_eq!(program.step().unwrap(), StepResult::Row);
            assert_eq!(program.column_int(0) != 0, expected_le, "Le failed for {} <= {}", a, b);
        }

        // Test Gt
        {
            let mut conn = Connection::open_in_memory().expect("Failed to open connection");
            let mut builder = conn.new_program().expect("Failed to create program");
            let r1 = builder.alloc_register();
            let r2 = builder.alloc_register();
            let r_result = builder.alloc_register();

            builder.add(Insn::Integer { value: a, dest: r1 });
            builder.add(Insn::Integer { value: b, dest: r2 });
            builder.add(Insn::Integer { value: 0, dest: r_result });

            let gt_addr = builder.add(Insn::Gt { lhs: r1, rhs: r2, target: 0 });
            let skip = builder.add(Insn::Goto { target: 0 });
            builder.jump_here(gt_addr);
            builder.add(Insn::Integer { value: 1, dest: r_result });
            builder.jump_here(skip);
            builder.add(Insn::ResultRow { start: r_result, count: 1 });
            builder.add(Insn::Halt);

            let mut program = builder.finish(1).expect("Failed to finish program");
            assert_eq!(program.step().unwrap(), StepResult::Row);
            assert_eq!(program.column_int(0) != 0, expected_gt, "Gt failed for {} > {}", a, b);
        }

        // Test Ge
        {
            let mut conn = Connection::open_in_memory().expect("Failed to open connection");
            let mut builder = conn.new_program().expect("Failed to create program");
            let r1 = builder.alloc_register();
            let r2 = builder.alloc_register();
            let r_result = builder.alloc_register();

            builder.add(Insn::Integer { value: a, dest: r1 });
            builder.add(Insn::Integer { value: b, dest: r2 });
            builder.add(Insn::Integer { value: 0, dest: r_result });

            let ge_addr = builder.add(Insn::Ge { lhs: r1, rhs: r2, target: 0 });
            let skip = builder.add(Insn::Goto { target: 0 });
            builder.jump_here(ge_addr);
            builder.add(Insn::Integer { value: 1, dest: r_result });
            builder.jump_here(skip);
            builder.add(Insn::ResultRow { start: r_result, count: 1 });
            builder.add(Insn::Halt);

            let mut program = builder.finish(1).expect("Failed to finish program");
            assert_eq!(program.step().unwrap(), StepResult::Row);
            assert_eq!(program.column_int(0) != 0, expected_ge, "Ge failed for {} >= {}", a, b);
        }
    }

    // Test 5 vs 10
    test_cmp(5, 10, false, true, true, true, false, false);

    // Test 10 vs 5
    test_cmp(10, 5, false, true, false, false, true, true);

    // Test 7 vs 7
    test_cmp(7, 7, true, false, false, true, false, true);
}

#[test]
fn test_comparison_equal_values() {
    let mut conn = Connection::open_in_memory().expect("Failed to open connection");
    let mut builder = conn.new_program().expect("Failed to create program");

    let r1 = builder.alloc_register();
    let r2 = builder.alloc_register();
    let r_eq = builder.alloc_register();
    let r_le = builder.alloc_register();
    let r_ge = builder.alloc_register();

    builder.add(Insn::Integer { value: 7, dest: r1 });
    builder.add(Insn::Integer { value: 7, dest: r2 });

    builder.add(Insn::Integer { value: 0, dest: r_eq });
    builder.add(Insn::Integer { value: 0, dest: r_le });
    builder.add(Insn::Integer { value: 0, dest: r_ge });

    // 7 == 7 -> true
    let eq_addr = builder.add(Insn::Eq { lhs: r1, rhs: r2, target: 0 });
    let eq_skip = builder.add(Insn::Goto { target: 0 });
    builder.jump_here(eq_addr);
    builder.add(Insn::Integer { value: 1, dest: r_eq });
    builder.jump_here(eq_skip);

    // 7 <= 7 -> true
    let le_addr = builder.add(Insn::Le { lhs: r1, rhs: r2, target: 0 });
    let le_skip = builder.add(Insn::Goto { target: 0 });
    builder.jump_here(le_addr);
    builder.add(Insn::Integer { value: 1, dest: r_le });
    builder.jump_here(le_skip);

    // 7 >= 7 -> true
    let ge_addr = builder.add(Insn::Ge { lhs: r1, rhs: r2, target: 0 });
    let ge_skip = builder.add(Insn::Goto { target: 0 });
    builder.jump_here(ge_addr);
    builder.add(Insn::Integer { value: 1, dest: r_ge });
    builder.jump_here(ge_skip);

    builder.add(Insn::ResultRow { start: r_eq, count: 3 });
    builder.add(Insn::Halt);

    let mut program = builder.finish(3).expect("Failed to finish program");

    assert_eq!(program.step().unwrap(), StepResult::Row);
    assert_eq!(program.column_int(0), 1); // eq
    assert_eq!(program.column_int(1), 1); // le
    assert_eq!(program.column_int(2), 1); // ge
}

// ============================================================================
// Register Operations Tests
// ============================================================================

#[test]
fn test_copy_range() {
    let mut conn = Connection::open_in_memory().expect("Failed to open connection");
    let mut builder = conn.new_program().expect("Failed to create program");

    let r1 = builder.alloc_register();
    let r2 = builder.alloc_register();
    let r3 = builder.alloc_register();
    let r4 = builder.alloc_register();
    let _r5 = builder.alloc_register();
    let _r6 = builder.alloc_register();

    builder.add(Insn::Integer { value: 10, dest: r1 });
    builder.add(Insn::Integer { value: 20, dest: r2 });
    builder.add(Insn::Integer { value: 30, dest: r3 });

    // Copy 3 registers from r1 to r4
    builder.add(Insn::Copy { src: r1, dest: r4, count: 3 });

    builder.add(Insn::ResultRow { start: r4, count: 3 });
    builder.add(Insn::Halt);

    let mut program = builder.finish(3).expect("Failed to finish program");

    assert_eq!(program.step().unwrap(), StepResult::Row);
    assert_eq!(program.column_int(0), 10);
    assert_eq!(program.column_int(1), 20);
    assert_eq!(program.column_int(2), 30);
}

#[test]
fn test_move_clears_source() {
    let mut conn = Connection::open_in_memory().expect("Failed to open connection");
    let mut builder = conn.new_program().expect("Failed to create program");

    let r1 = builder.alloc_register();
    let r2 = builder.alloc_register();

    builder.add(Insn::Integer { value: 42, dest: r1 });
    builder.add(Insn::Move { src: r1, dest: r2, count: 1 });

    // After move, r1 should be NULL
    builder.add(Insn::ResultRow { start: r1, count: 2 });
    builder.add(Insn::Halt);

    let mut program = builder.finish(2).expect("Failed to finish program");

    assert_eq!(program.step().unwrap(), StepResult::Row);
    assert!(program.column_value(0).is_null()); // Source cleared
    assert_eq!(program.column_int(1), 42); // Dest has value
}

#[test]
fn test_addimm() {
    let mut conn = Connection::open_in_memory().expect("Failed to open connection");
    let mut builder = conn.new_program().expect("Failed to create program");

    let r1 = builder.alloc_register();

    builder.add(Insn::Integer { value: 100, dest: r1 });
    builder.add(Insn::AddImm { dest: r1, value: 50 });
    builder.add(Insn::ResultRow { start: r1, count: 1 });
    builder.add(Insn::Halt);

    let mut program = builder.finish(1).expect("Failed to finish program");

    assert_eq!(program.step().unwrap(), StepResult::Row);
    assert_eq!(program.column_int(0), 150);
}

#[test]
fn test_addimm_negative() {
    let mut conn = Connection::open_in_memory().expect("Failed to open connection");
    let mut builder = conn.new_program().expect("Failed to create program");

    let r1 = builder.alloc_register();

    builder.add(Insn::Integer { value: 100, dest: r1 });
    builder.add(Insn::AddImm { dest: r1, value: -30 });
    builder.add(Insn::ResultRow { start: r1, count: 1 });
    builder.add(Insn::Halt);

    let mut program = builder.finish(1).expect("Failed to finish program");

    assert_eq!(program.step().unwrap(), StepResult::Row);
    assert_eq!(program.column_int(0), 70);
}

// ============================================================================
// Floating Point Tests
// NOTE: Real/Int64 P4 handling needs fixing (memory allocation mismatch with SQLite)
// ============================================================================

#[test]
fn test_real_values() {
    let mut conn = Connection::open_in_memory().expect("Failed to open connection");
    let mut builder = conn.new_program().expect("Failed to create program");

    let r1 = builder.alloc_register();
    let r2 = builder.alloc_register();

    builder.add(Insn::Real { value: 3.14159, dest: r1 });
    builder.add(Insn::Real { value: -2.5, dest: r2 });
    builder.add(Insn::ResultRow { start: r1, count: 2 });
    builder.add(Insn::Halt);

    let mut program = builder.finish(2).expect("Failed to finish program");

    assert_eq!(program.step().unwrap(), StepResult::Row);
    let v1 = program.column_value(0);
    let v2 = program.column_value(1);
    assert!((v1.as_real().unwrap() - 3.14159).abs() < 0.00001);
    assert!((v2.as_real().unwrap() - (-2.5)).abs() < 0.00001);
}

#[test]
fn test_real_arithmetic() {
    let mut conn = Connection::open_in_memory().expect("Failed to open connection");
    let mut builder = conn.new_program().expect("Failed to create program");

    let r1 = builder.alloc_register();
    let r2 = builder.alloc_register();
    let r3 = builder.alloc_register();

    builder.add(Insn::Real { value: 10.5, dest: r1 });
    builder.add(Insn::Real { value: 3.0, dest: r2 });
    builder.add(Insn::Divide { lhs: r1, rhs: r2, dest: r3 });
    builder.add(Insn::ResultRow { start: r3, count: 1 });
    builder.add(Insn::Halt);

    let mut program = builder.finish(1).expect("Failed to finish program");

    assert_eq!(program.step().unwrap(), StepResult::Row);
    let v = program.column_value(0);
    assert!((v.as_real().unwrap() - 3.5).abs() < 0.00001);
}

// ============================================================================
// String Tests
// ============================================================================

#[test]
fn test_string_values() {
    let mut conn = Connection::open_in_memory().expect("Failed to open connection");
    let mut builder = conn.new_program().expect("Failed to create program");

    let r1 = builder.alloc_register();

    builder.add(Insn::String8 { value: "Hello, World!".to_string(), dest: r1 });
    builder.add(Insn::ResultRow { start: r1, count: 1 });
    builder.add(Insn::Halt);

    let mut program = builder.finish(1).expect("Failed to finish program");

    assert_eq!(program.step().unwrap(), StepResult::Row);
    let v = program.column_value(0);
    assert_eq!(v.as_text(), Some("Hello, World!"));
}

#[test]
fn test_string_concat() {
    let mut conn = Connection::open_in_memory().expect("Failed to open connection");
    let mut builder = conn.new_program().expect("Failed to create program");

    let r1 = builder.alloc_register();
    let r2 = builder.alloc_register();
    let r3 = builder.alloc_register();

    builder.add(Insn::String8 { value: "Hello, ".to_string(), dest: r1 });
    builder.add(Insn::String8 { value: "World!".to_string(), dest: r2 });
    builder.add(Insn::Concat { lhs: r1, rhs: r2, dest: r3 });
    builder.add(Insn::ResultRow { start: r3, count: 1 });
    builder.add(Insn::Halt);

    let mut program = builder.finish(1).expect("Failed to finish program");

    assert_eq!(program.step().unwrap(), StepResult::Row);
    let v = program.column_value(0);
    assert_eq!(v.as_text(), Some("Hello, World!"));
}

#[test]
fn test_empty_string() {
    let mut conn = Connection::open_in_memory().expect("Failed to open connection");
    let mut builder = conn.new_program().expect("Failed to create program");

    let r1 = builder.alloc_register();

    builder.add(Insn::String8 { value: "".to_string(), dest: r1 });
    builder.add(Insn::ResultRow { start: r1, count: 1 });
    builder.add(Insn::Halt);

    let mut program = builder.finish(1).expect("Failed to finish program");

    assert_eq!(program.step().unwrap(), StepResult::Row);
    let v = program.column_value(0);
    assert_eq!(v.as_text(), Some(""));
}

#[test]
fn test_string_with_special_chars() {
    let mut conn = Connection::open_in_memory().expect("Failed to open connection");
    let mut builder = conn.new_program().expect("Failed to create program");

    let r1 = builder.alloc_register();

    builder.add(Insn::String8 { value: "Line1\nLine2\tTab".to_string(), dest: r1 });
    builder.add(Insn::ResultRow { start: r1, count: 1 });
    builder.add(Insn::Halt);

    let mut program = builder.finish(1).expect("Failed to finish program");

    assert_eq!(program.step().unwrap(), StepResult::Row);
    let v = program.column_value(0);
    assert_eq!(v.as_text(), Some("Line1\nLine2\tTab"));
}

#[test]
fn test_string_with_unicode() {
    let mut conn = Connection::open_in_memory().expect("Failed to open connection");
    let mut builder = conn.new_program().expect("Failed to create program");

    let r1 = builder.alloc_register();

    builder.add(Insn::String8 { value: "Hello, 世界! 🎉".to_string(), dest: r1 });
    builder.add(Insn::ResultRow { start: r1, count: 1 });
    builder.add(Insn::Halt);

    let mut program = builder.finish(1).expect("Failed to finish program");

    assert_eq!(program.step().unwrap(), StepResult::Row);
    let v = program.column_value(0);
    assert_eq!(v.as_text(), Some("Hello, 世界! 🎉"));
}

// ============================================================================
// Subroutine Tests
// ============================================================================

#[test]
fn test_gosub_return() {
    let mut conn = Connection::open_in_memory().expect("Failed to open connection");
    let mut builder = conn.new_program().expect("Failed to create program");

    let r_return = builder.alloc_register();
    let r_result = builder.alloc_register();

    // Main: Set result to 1, call subroutine, then set result to 3
    builder.add(Insn::Integer { value: 1, dest: r_result });
    let gosub_addr = builder.add(Insn::Gosub { return_reg: r_return, target: 0 });
    builder.add(Insn::Integer { value: 3, dest: r_result });
    builder.add(Insn::ResultRow { start: r_result, count: 1 });
    builder.add(Insn::Halt);

    // Subroutine: Set result to 2 and return
    builder.jump_here(gosub_addr);
    builder.add(Insn::Integer { value: 2, dest: r_result });
    builder.add(Insn::Return { return_reg: r_return });

    let mut program = builder.finish(1).expect("Failed to finish program");

    assert_eq!(program.step().unwrap(), StepResult::Row);
    // Subroutine set it to 2, then after return we set it to 3
    assert_eq!(program.column_int(0), 3);
}

// ============================================================================
// Logical Operations Tests
// ============================================================================

#[test]
fn test_not_instruction() {
    let mut conn = Connection::open_in_memory().expect("Failed to open connection");
    let mut builder = conn.new_program().expect("Failed to create program");

    let r1 = builder.alloc_register();
    let r2 = builder.alloc_register();
    let r3 = builder.alloc_register();
    let r4 = builder.alloc_register();

    builder.add(Insn::Integer { value: 0, dest: r1 }); // false
    builder.add(Insn::Integer { value: 1, dest: r2 }); // true
    builder.add(Insn::Not { src: r1, dest: r3 }); // NOT 0 = 1
    builder.add(Insn::Not { src: r2, dest: r4 }); // NOT 1 = 0
    builder.add(Insn::ResultRow { start: r3, count: 2 });
    builder.add(Insn::Halt);

    let mut program = builder.finish(2).expect("Failed to finish program");

    assert_eq!(program.step().unwrap(), StepResult::Row);
    assert_eq!(program.column_int(0), 1);
    assert_eq!(program.column_int(0), 1);
}

// ============================================================================
// Many Registers Test
// ============================================================================

#[test]
fn test_many_registers() {
    let mut conn = Connection::open_in_memory().expect("Failed to open connection");
    let mut builder = conn.new_program().expect("Failed to create program");

    // Allocate 10 registers and set them to their index
    let first_reg = builder.alloc_register();
    for i in 1..10 {
        let _ = builder.alloc_register();
        builder.add(Insn::Integer { value: i as i32, dest: first_reg + i });
    }
    builder.add(Insn::Integer { value: 0, dest: first_reg });

    builder.add(Insn::ResultRow { start: first_reg, count: 10 });
    builder.add(Insn::Halt);

    let mut program = builder.finish(10).expect("Failed to finish program");

    assert_eq!(program.step().unwrap(), StepResult::Row);
    assert_eq!(program.column_count(), 10);
    for i in 0..10 {
        assert_eq!(program.column_int(i), i);
    }
}

// ============================================================================
// Noop Test
// ============================================================================

#[test]
fn test_noop_instruction() {
    let mut conn = Connection::open_in_memory().expect("Failed to open connection");
    let mut builder = conn.new_program().expect("Failed to create program");

    let r1 = builder.alloc_register();

    builder.add(Insn::Integer { value: 42, dest: r1 });
    builder.add(Insn::Noop);
    builder.add(Insn::Noop);
    builder.add(Insn::Noop);
    builder.add(Insn::ResultRow { start: r1, count: 1 });
    builder.add(Insn::Halt);

    let mut program = builder.finish(1).expect("Failed to finish program");

    assert_eq!(program.step().unwrap(), StepResult::Row);
    assert_eq!(program.column_int(0), 42);
}

// ============================================================================
// Value Type Tests
// ============================================================================

#[test]
fn test_value_type_conversions() {
    // Test Value type conversions
    let v: Value = 42i64.into();
    assert_eq!(v, Value::Integer(42));
    assert_eq!(v.as_integer(), Some(42));
    assert!((v.as_real().unwrap() - 42.0).abs() < 0.001);

    let v: Value = 3.14f64.into();
    assert_eq!(v.as_integer(), Some(3)); // Truncates

    let v: Value = "123".into();
    assert_eq!(v.as_integer(), Some(123));

    let v: Value = "not a number".into();
    assert_eq!(v.as_integer(), None);

    let v: Value = None::<i64>.into();
    assert!(v.is_null());
}

#[test]
fn test_value_to_string_lossy() {
    assert_eq!(Value::Null.to_string_lossy(), "NULL");
    assert_eq!(Value::Integer(42).to_string_lossy(), "42");
    assert_eq!(Value::Real(3.14).to_string_lossy(), "3.14");
    assert_eq!(Value::Text("hello".to_string()).to_string_lossy(), "hello");
    assert_eq!(Value::Blob(vec![0xDE, 0xAD]).to_string_lossy(), "X'DEAD'");
}

// ============================================================================
// Complex Program Tests
// ============================================================================

#[test]
fn test_factorial_5() {
    // Compute 5! = 120 using a loop
    let mut conn = Connection::open_in_memory().expect("Failed to open connection");
    let mut builder = conn.new_program().expect("Failed to create program");

    let r_n = builder.alloc_register();      // Current n
    let r_result = builder.alloc_register(); // Accumulated result
    let r_one = builder.alloc_register();    // Constant 1

    // Initialize: n = 5, result = 1, one = 1
    builder.add(Insn::Integer { value: 5, dest: r_n });
    builder.add(Insn::Integer { value: 1, dest: r_result });
    builder.add(Insn::Integer { value: 1, dest: r_one });

    // Loop: while n > 1
    let loop_start = builder.current_addr();

    // result = result * n
    builder.add(Insn::Multiply { lhs: r_result, rhs: r_n, dest: r_result });

    // n = n - 1
    builder.add(Insn::Subtract { lhs: r_n, rhs: r_one, dest: r_n });

    // if n > 1, loop
    builder.add(Insn::Gt { lhs: r_n, rhs: r_one, target: loop_start.raw() });

    builder.add(Insn::ResultRow { start: r_result, count: 1 });
    builder.add(Insn::Halt);

    let mut program = builder.finish(1).expect("Failed to finish program");

    assert_eq!(program.step().unwrap(), StepResult::Row);
    assert_eq!(program.column_int(0), 120); // 5! = 120
}

#[test]
fn test_fibonacci_sequence() {
    // Generate first 10 Fibonacci numbers
    let mut conn = Connection::open_in_memory().expect("Failed to open connection");
    let mut builder = conn.new_program().expect("Failed to create program");

    let r_a = builder.alloc_register();       // fib(n-2)
    let r_b = builder.alloc_register();       // fib(n-1)
    let r_next = builder.alloc_register();    // fib(n)
    let r_count = builder.alloc_register();   // Counter

    // Initialize: a = 0, b = 1, count = 10
    builder.add(Insn::Integer { value: 0, dest: r_a });
    builder.add(Insn::Integer { value: 1, dest: r_b });
    builder.add(Insn::Integer { value: 10, dest: r_count });

    // Output first number
    builder.add(Insn::ResultRow { start: r_a, count: 1 });
    builder.add(Insn::AddImm { dest: r_count, value: -1 });

    // Loop
    let loop_start = builder.current_addr();

    // Output b
    builder.add(Insn::ResultRow { start: r_b, count: 1 });

    // next = a + b
    builder.add(Insn::Add { lhs: r_a, rhs: r_b, dest: r_next });

    // a = b
    builder.add(Insn::SCopy { src: r_b, dest: r_a });

    // b = next
    builder.add(Insn::SCopy { src: r_next, dest: r_b });

    // count--; if count > 0, loop
    builder.add(Insn::AddImm { dest: r_count, value: -1 });
    builder.add(Insn::IfPos { src: r_count, target: loop_start.raw(), decrement: 0 });

    builder.add(Insn::Halt);

    let mut program = builder.finish(1).expect("Failed to finish program");

    let expected = vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34];
    let mut results = Vec::new();

    loop {
        match program.step().unwrap() {
            StepResult::Row => results.push(program.column_int(0)),
            StepResult::Done => break,
        }
    }

    assert_eq!(results, expected);
}

// ============================================================================
// Fuzzing Tests
// ============================================================================

/// Simple pseudo-random number generator for deterministic fuzzing
struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next(&mut self) -> u64 {
        // xorshift64
        self.state ^= self.state << 13;
        self.state ^= self.state >> 7;
        self.state ^= self.state << 17;
        self.state
    }

    fn next_i32(&mut self) -> i32 {
        self.next() as i32
    }

    fn range(&mut self, min: i32, max: i32) -> i32 {
        let range = (max - min) as u64;
        (self.next() % range) as i32 + min
    }
}

#[test]
fn test_fuzz_arithmetic_programs() {
    let mut rng = SimpleRng::new(12345);

    for _ in 0..100 {
        let mut conn = Connection::open_in_memory().expect("Failed to open connection");
        let mut builder = conn.new_program().expect("Failed to create program");

        let r1 = builder.alloc_register();
        let r2 = builder.alloc_register();
        let r3 = builder.alloc_register();

        let a = rng.next_i32() % 10000;
        let b = rng.range(1, 10000); // Avoid zero for division

        builder.add(Insn::Integer { value: a, dest: r1 });
        builder.add(Insn::Integer { value: b, dest: r2 });

        // Random operation
        match rng.range(0, 5) {
            0 => {
                builder.add(Insn::Add { lhs: r1, rhs: r2, dest: r3 });
            }
            1 => {
                builder.add(Insn::Subtract { lhs: r1, rhs: r2, dest: r3 });
            }
            2 => {
                builder.add(Insn::Multiply { lhs: r1, rhs: r2, dest: r3 });
            }
            3 => {
                builder.add(Insn::Divide { lhs: r1, rhs: r2, dest: r3 });
            }
            _ => {
                builder.add(Insn::Remainder { lhs: r1, rhs: r2, dest: r3 });
            }
        }

        builder.add(Insn::ResultRow { start: r3, count: 1 });
        builder.add(Insn::Halt);

        let mut program = builder.finish(1).expect("Failed to finish program");

        // Just verify it executes without crashing
        let result = program.step();
        assert!(result.is_ok());
    }
}

#[test]
fn test_fuzz_random_integer_values() {
    let mut rng = SimpleRng::new(67890);

    for _ in 0..100 {
        let mut conn = Connection::open_in_memory().expect("Failed to open connection");
        let mut builder = conn.new_program().expect("Failed to create program");

        let r1 = builder.alloc_register();

        // Test various integer values including edge cases
        let value = match rng.range(0, 5) {
            0 => i32::MAX,
            1 => i32::MIN,
            2 => 0,
            3 => -1,
            _ => rng.next_i32(),
        };

        builder.add(Insn::Integer { value, dest: r1 });
        builder.add(Insn::ResultRow { start: r1, count: 1 });
        builder.add(Insn::Halt);

        let mut program = builder.finish(1).expect("Failed to finish program");

        assert_eq!(program.step().unwrap(), StepResult::Row);
        assert_eq!(program.column_int(0), value);
    }
}

#[test]
fn test_fuzz_control_flow() {
    let mut rng = SimpleRng::new(11111);

    for _ in 0..50 {
        let mut conn = Connection::open_in_memory().expect("Failed to open connection");
        let mut builder = conn.new_program().expect("Failed to create program");

        let r1 = builder.alloc_register();
        let r_result = builder.alloc_register();

        let value = rng.range(-100, 100);
        builder.add(Insn::Integer { value, dest: r1 });
        builder.add(Insn::Integer { value: 0, dest: r_result });

        // Test random comparison
        let cmp_type = rng.range(0, 6);
        let cmp_value = rng.range(-100, 100);
        let r_cmp = builder.alloc_register();
        builder.add(Insn::Integer { value: cmp_value, dest: r_cmp });

        let jump_addr = match cmp_type {
            0 => builder.add(Insn::Eq { lhs: r1, rhs: r_cmp, target: 0 }),
            1 => builder.add(Insn::Ne { lhs: r1, rhs: r_cmp, target: 0 }),
            2 => builder.add(Insn::Lt { lhs: r1, rhs: r_cmp, target: 0 }),
            3 => builder.add(Insn::Le { lhs: r1, rhs: r_cmp, target: 0 }),
            4 => builder.add(Insn::Gt { lhs: r1, rhs: r_cmp, target: 0 }),
            _ => builder.add(Insn::Ge { lhs: r1, rhs: r_cmp, target: 0 }),
        };

        let skip_addr = builder.add(Insn::Goto { target: 0 });
        builder.jump_here(jump_addr);
        builder.add(Insn::Integer { value: 1, dest: r_result });
        builder.jump_here(skip_addr);

        builder.add(Insn::ResultRow { start: r_result, count: 1 });
        builder.add(Insn::Halt);

        let mut program = builder.finish(1).expect("Failed to finish program");

        // Verify expected behavior
        let result = program.step();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), StepResult::Row);

        let expected = match cmp_type {
            0 => if value == cmp_value { 1 } else { 0 },
            1 => if value != cmp_value { 1 } else { 0 },
            2 => if value < cmp_value { 1 } else { 0 },
            3 => if value <= cmp_value { 1 } else { 0 },
            4 => if value > cmp_value { 1 } else { 0 },
            _ => if value >= cmp_value { 1 } else { 0 },
        };

        assert_eq!(program.column_int(0), expected);
    }
}

#[test]
fn test_fuzz_long_programs() {
    let mut rng = SimpleRng::new(99999);

    // Reduced iterations to avoid memory issues
    for _ in 0..3 {
        let mut conn = Connection::open_in_memory().expect("Failed to open connection");
        let mut builder = conn.new_program().expect("Failed to create program");

        // Allocate some registers
        let num_regs = rng.range(5, 20) as usize;
        let regs: Vec<i32> = (0..num_regs).map(|_| builder.alloc_register()).collect();

        // Initialize registers with random values
        for (i, &r) in regs.iter().enumerate() {
            builder.add(Insn::Integer { value: (i * 10) as i32, dest: r });
        }

        // Add random arithmetic operations
        let num_ops = rng.range(5, 15);
        for _ in 0..num_ops {
            let src1 = regs[rng.range(0, num_regs as i32) as usize];
            let src2 = regs[rng.range(0, num_regs as i32) as usize];
            let dest = regs[rng.range(0, num_regs as i32) as usize];

            match rng.range(0, 4) {
                0 => builder.add(Insn::Add { lhs: src1, rhs: src2, dest }),
                1 => builder.add(Insn::Subtract { lhs: src1, rhs: src2, dest }),
                2 => builder.add(Insn::Multiply { lhs: src1, rhs: src2, dest }),
                _ => builder.add(Insn::SCopy { src: src1, dest }),
            };
        }

        // Output first register
        builder.add(Insn::ResultRow { start: regs[0], count: 1 });
        builder.add(Insn::Halt);

        let mut program = builder.finish(1).expect("Failed to finish program");

        // Verify it runs without crashing
        let result = program.step();
        assert!(result.is_ok());
    }
}

#[test]
fn test_fuzz_nested_loops() {
    // Simple test with IfNotZero for proper looping
    // (DecrJumpZero jumps when zero, not when not zero)
    let mut rng = SimpleRng::new(55555);

    for _ in 0..10 {
        let mut conn = Connection::open_in_memory().expect("Failed to open connection");
        let mut builder = conn.new_program().expect("Failed to create program");

        let r_counter = builder.alloc_register();
        let r_sum = builder.alloc_register();

        let limit = rng.range(2, 10);

        builder.add(Insn::Integer { value: limit, dest: r_counter });
        builder.add(Insn::Integer { value: 0, dest: r_sum });

        // Loop using IfNotZero (decrement and jump if was not zero)
        let loop_start = builder.current_addr();
        builder.add(Insn::AddImm { dest: r_sum, value: 1 });
        builder.add(Insn::IfNotZero { src: r_counter, target: loop_start.raw() });

        builder.add(Insn::ResultRow { start: r_sum, count: 1 });
        builder.add(Insn::Halt);

        let mut program = builder.finish(1).expect("Failed to finish program");

        assert_eq!(program.step().unwrap(), StepResult::Row);
        // IfNotZero runs limit times (decrementing each iteration)
        // Then one more when counter=0 before exiting
        assert_eq!(program.column_int(0), limit + 1);
    }
}

// ============================================================================
// Stress Tests
// ============================================================================

#[test]
fn test_many_result_rows() {
    let mut conn = Connection::open_in_memory().expect("Failed to open connection");
    let mut builder = conn.new_program().expect("Failed to create program");

    let r_counter = builder.alloc_register();
    let r_limit = builder.alloc_register();

    let num_rows = 1000;
    builder.add(Insn::Integer { value: 0, dest: r_counter });
    builder.add(Insn::Integer { value: num_rows, dest: r_limit });

    let loop_start = builder.current_addr();
    builder.add(Insn::ResultRow { start: r_counter, count: 1 });
    builder.add(Insn::AddImm { dest: r_counter, value: 1 });
    builder.add(Insn::Lt { lhs: r_counter, rhs: r_limit, target: loop_start.raw() });

    builder.add(Insn::Halt);

    let mut program = builder.finish(1).expect("Failed to finish program");

    let mut count = 0;
    loop {
        match program.step().unwrap() {
            StepResult::Row => {
                assert_eq!(program.column_int(0), count);
                count += 1;
            }
            StepResult::Done => break,
        }
    }

    assert_eq!(count, num_rows);
}

#[test]
fn test_program_reuse_many_times() {
    let mut conn = Connection::open_in_memory().expect("Failed to open connection");
    let mut builder = conn.new_program().expect("Failed to create program");

    let r1 = builder.alloc_register();

    builder.add(Insn::Integer { value: 42, dest: r1 });
    builder.add(Insn::ResultRow { start: r1, count: 1 });
    builder.add(Insn::Halt);

    let mut program = builder.finish(1).expect("Failed to finish program");

    // Execute and reset many times
    for _ in 0..100 {
        assert_eq!(program.step().unwrap(), StepResult::Row);
        assert_eq!(program.column_int(0), 42);
        assert_eq!(program.step().unwrap(), StepResult::Done);
        program.reset();
    }
}

// ============================================================================
// Edge Case Tests
// ============================================================================

#[test]
fn test_single_instruction_program() {
    let mut conn = Connection::open_in_memory().expect("Failed to open connection");
    let mut builder = conn.new_program().expect("Failed to create program");

    builder.add(Insn::Halt);

    let mut program = builder.finish(0).expect("Failed to finish program");

    assert_eq!(program.step().unwrap(), StepResult::Done);
}

#[test]
fn test_result_row_zero_columns() {
    let mut conn = Connection::open_in_memory().expect("Failed to open connection");
    let mut builder = conn.new_program().expect("Failed to create program");

    let r1 = builder.alloc_register();

    builder.add(Insn::Integer { value: 42, dest: r1 });
    builder.add(Insn::ResultRow { start: r1, count: 0 }); // Zero columns
    builder.add(Insn::Halt);

    let mut program = builder.finish(0).expect("Failed to finish program");

    // Zero columns should still produce a row
    assert_eq!(program.step().unwrap(), StepResult::Row);
    assert_eq!(program.column_count(), 0);
}

#[test]
fn test_many_consecutive_jumps() {
    let mut conn = Connection::open_in_memory().expect("Failed to open connection");
    let mut builder = conn.new_program().expect("Failed to create program");

    let r1 = builder.alloc_register();

    builder.add(Insn::Integer { value: 0, dest: r1 });

    // Chain of 10 jumps
    let mut addrs = Vec::new();
    for _ in 0..10 {
        addrs.push(builder.add(Insn::Goto { target: 0 }));
        builder.add(Insn::AddImm { dest: r1, value: 100 }); // Should be skipped
    }

    // Patch all jumps to final destination
    for addr in addrs {
        builder.jump_here(addr);
    }

    builder.add(Insn::Integer { value: 42, dest: r1 });
    builder.add(Insn::ResultRow { start: r1, count: 1 });
    builder.add(Insn::Halt);

    let mut program = builder.finish(1).expect("Failed to finish program");

    assert_eq!(program.step().unwrap(), StepResult::Row);
    assert_eq!(program.column_int(0), 42);
}

#[test]
fn test_self_referential_computation() {
    // r1 = r1 + r1 (doubling)
    let mut conn = Connection::open_in_memory().expect("Failed to open connection");
    let mut builder = conn.new_program().expect("Failed to create program");

    let r1 = builder.alloc_register();

    builder.add(Insn::Integer { value: 1, dest: r1 });

    // Double 5 times: 1 -> 2 -> 4 -> 8 -> 16 -> 32
    for _ in 0..5 {
        builder.add(Insn::Add { lhs: r1, rhs: r1, dest: r1 });
    }

    builder.add(Insn::ResultRow { start: r1, count: 1 });
    builder.add(Insn::Halt);

    let mut program = builder.finish(1).expect("Failed to finish program");

    assert_eq!(program.step().unwrap(), StepResult::Row);
    assert_eq!(program.column_int(0), 32);
}
