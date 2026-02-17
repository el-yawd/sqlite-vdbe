# sqlite-vdbe

Low-level access to SQLite's VDBE (Virtual Database Engine) bytecode.

This crate allows you to create and execute VDBE programs directly, bypassing SQL parsing. This is useful for:

- Building custom query engines on top of SQLite's storage
- Testing VDBE behavior
- Learning how SQLite works internally
- Implementing specialized database operations

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
sqlite-vdbe = "0.0.2"
```

SQLite 3.51.2 is bundled and compiled automatically during build.

## Quick Start

```rust
use sqlite_vdbe::{Connection, Insn, StepResult};

fn main() -> sqlite_vdbe::Result<()> {
    // Open an in-memory database
    let mut conn = Connection::open_in_memory()?;

    // Create a VDBE program that computes 10 + 32
    let mut builder = conn.new_program()?;

    // Allocate registers
    let r1 = builder.alloc_register();
    let r2 = builder.alloc_register();
    let r3 = builder.alloc_register();

    // Build the program with named instruction fields
    builder.add(Insn::Integer { value: 10, dest: r1 });
    builder.add(Insn::Integer { value: 32, dest: r2 });
    builder.add(Insn::Add { lhs: r1, rhs: r2, dest: r3 });
    builder.add(Insn::ResultRow { start: r3, count: 1 });
    builder.add(Insn::Halt);

    // Execute
    let mut program = builder.finish(1)?;

    match program.step()? {
        StepResult::Row => {
            println!("Result: {}", program.column_int(0)); // Prints: Result: 42
        }
        StepResult::Done => unreachable!(),
    }

    Ok(())
}
```

## API Overview

### Instructions

The `Insn` enum provides a type-safe way to build VDBE programs:

```rust
use sqlite_vdbe::Insn;

// Load constants
Insn::Integer { value: 42, dest: r1 }
Insn::Null { dest: r2, count: 1 }

// Arithmetic
Insn::Add { lhs: r1, rhs: r2, dest: r3 }
Insn::Subtract { lhs: r1, rhs: r2, dest: r3 }
Insn::Multiply { lhs: r1, rhs: r2, dest: r3 }

// Control flow
Insn::Goto { target: addr }
Insn::If { src: r1, target: addr, jump_if_null: false }
Insn::Halt

// Results
Insn::ResultRow { start: r1, count: 3 }

// Comparisons
Insn::Eq { lhs: r1, rhs: r2, target: addr }
Insn::Lt { lhs: r1, rhs: r2, target: addr }
```

### Forward Jumps

Use `jump_here()` to patch forward jumps:

```rust
let jump_addr = builder.add(Insn::Goto { target: 0 });
builder.add(Insn::Integer { value: 999, dest: r1 }); // Skipped
builder.jump_here(jump_addr); // Patch jump to here
builder.add(Insn::Integer { value: 42, dest: r1 });  // Executed
```

## Thread Safety

This crate compiles SQLite with `SQLITE_THREADSAFE=0` for simplicity. All types are `!Send` and `!Sync`. Use one connection per thread.

## Features

- `bundled` (default): Compiles the bundled SQLite 3.51.2 amalgamation

## Requirements

- A C compiler (cc)
