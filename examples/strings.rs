//! String operations with VDBE bytecode

use sqlite_vdbe::{Connection, Insn, StepResult};

fn main() -> sqlite_vdbe::Result<()> {
    let mut conn = Connection::open_in_memory()?;
    let mut builder = conn.new_program()?;

    // Registers
    let r1 = builder.alloc_register();
    let r2 = builder.alloc_register();
    let r3 = builder.alloc_register();

    // Load strings
    builder.add(Insn::String8 {
        value: "Hello, ".to_string(),
        dest: r1,
    });
    builder.add(Insn::String8 {
        value: "World!".to_string(),
        dest: r2,
    });

    // Concatenate
    builder.add(Insn::Concat {
        lhs: r1,
        rhs: r2,
        dest: r3,
    });

    // Output result
    builder.add(Insn::ResultRow { start: r3, count: 1 });
    builder.add(Insn::Halt);

    // Execute
    let mut program = builder.finish(1)?;

    if let StepResult::Row = program.step()? {
        if let Some(text) = program.column_text(0) {
            println!("Result: {}", text);
        }
    }

    Ok(())
}
