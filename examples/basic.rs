//! Basic example: compute 10 + 32 using VDBE bytecode

use sqlite_vdbe::{Connection, Insn, StepResult};

fn main() -> sqlite_vdbe::Result<()> {
    // Open an in-memory database
    let mut conn = Connection::open_in_memory()?;

    // Create a VDBE program
    let mut builder = conn.new_program()?;

    // Allocate registers
    let r1 = builder.alloc_register();
    let r2 = builder.alloc_register();
    let r3 = builder.alloc_register();

    // Build the program: compute 10 + 32
    builder.add(Insn::Integer {
        value: 10,
        dest: r1,
    });
    builder.add(Insn::Integer {
        value: 32,
        dest: r2,
    });
    builder.add(Insn::Add {
        lhs: r1,
        rhs: r2,
        dest: r3,
    });
    builder.add(Insn::ResultRow {
        start: r3,
        count: 1,
    });
    builder.add(Insn::Halt);

    // Execute
    let mut program = builder.finish(1)?;

    match program.step()? {
        StepResult::Row => {
            println!("10 + 32 = {}", program.column_int(0));
        }
        StepResult::Done => {
            println!("No results");
        }
    }

    Ok(())
}
