//! Fibonacci sequence using VDBE control flow

use sqlite_vdbe::{Connection, Insn, StepResult};

fn main() -> sqlite_vdbe::Result<()> {
    let mut conn = Connection::open_in_memory()?;
    let mut builder = conn.new_program()?;

    // Registers
    let r_a = builder.alloc_register(); // fib(n-2)
    let r_b = builder.alloc_register(); // fib(n-1)
    let r_tmp = builder.alloc_register(); // temp for swap
    let r_count = builder.alloc_register(); // iteration counter

    // Initialize: a=0, b=1, count=10
    builder.add(Insn::Integer {
        value: 0,
        dest: r_a,
    });
    builder.add(Insn::Integer {
        value: 1,
        dest: r_b,
    });
    builder.add(Insn::Integer {
        value: 10,
        dest: r_count,
    });

    // Loop start - output current value
    let loop_start = builder.current_addr();
    builder.add(Insn::ResultRow {
        start: r_a,
        count: 1,
    });

    // tmp = a + b
    builder.add(Insn::Add {
        lhs: r_a,
        rhs: r_b,
        dest: r_tmp,
    });

    // a = b
    builder.add(Insn::SCopy {
        src: r_b,
        dest: r_a,
    });

    // b = tmp
    builder.add(Insn::SCopy {
        src: r_tmp,
        dest: r_b,
    });

    // Decrement counter and loop if not zero
    builder.add(Insn::IfNotZero {
        src: r_count,
        target: loop_start.raw(),
    });

    builder.add(Insn::Halt);

    // Execute
    let mut program = builder.finish(1)?;

    println!("First 10 Fibonacci numbers:");
    while let StepResult::Row = program.step()? {
        print!("{} ", program.column_int(0));
    }
    println!();

    Ok(())
}
