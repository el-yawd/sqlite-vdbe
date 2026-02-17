//! EXPLAIN example: display VDBE programs in SQLite's EXPLAIN format
//!
//! This example demonstrates how to use the `Display` implementation for `Program`
//! to output bytecode in the same format as SQLite's `EXPLAIN` statement.
//!
//! Run with: cargo run --example explain

use sqlite_vdbe::{Connection, Insn, StepResult};

fn main() -> sqlite_vdbe::Result<()> {
    // Open an in-memory database
    let mut conn = Connection::open_in_memory()?;

    // Create a VDBE program that computes 1 + 1
    let mut builder = conn.new_program()?;

    // Allocate registers
    let r1 = builder.alloc_register(); // Will hold operand (1)
    let r2 = builder.alloc_register(); // Will hold operand (1)
    let r3 = builder.alloc_register(); // Will hold result (2)

    // Build the program with comments (like SQLite's EXPLAIN output)
    builder.add_with_comment(
        Insn::Integer { value: 1, dest: r1 },
        &format!("r[{}]=1", r1),
    );
    builder.add_with_comment(
        Insn::Integer { value: 1, dest: r2 },
        &format!("r[{}]=1", r2),
    );
    builder.add_with_comment(
        Insn::Add {
            lhs: r1,
            rhs: r2,
            dest: r3,
        },
        &format!("r[{}]=r[{}]+r[{}]", r3, r1, r2),
    );
    builder.add_with_comment(
        Insn::ResultRow {
            start: r3,
            count: 1,
        },
        &format!("output=r[{}]", r3),
    );
    builder.add(Insn::Halt);

    // Finish building the program
    let mut program = builder.finish(1)?;

    // Display the program in EXPLAIN format (like SQLite's EXPLAIN command)
    println!("Program bytecode (EXPLAIN format):\n");
    println!("{}", program);

    // Execute the program
    println!("Executing...");
    loop {
        match program.step()? {
            StepResult::Row => {
                println!("Result: 1 + 1 = {}", program.column_int(0));
            }
            StepResult::Done => break,
        }
    }

    // You can also access individual instructions programmatically
    println!("\nProgrammatic access to instructions:");
    for (addr, insn) in program.instructions().iter().enumerate() {
        if !insn.comment.is_empty() {
            println!("  @{}: {} -- {}", addr, insn.opcode, insn.comment);
        } else {
            println!("  @{}: {}", addr, insn.opcode);
        }
    }

    Ok(())
}
