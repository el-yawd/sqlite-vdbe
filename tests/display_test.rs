//! Tests for Program display functionality (EXPLAIN format)

use sqlite_vdbe::{Connection, Insn};

#[test]
fn test_program_explain_display() {
    let mut conn = Connection::open_in_memory().unwrap();
    let mut builder = conn.new_program().unwrap();

    let r1 = builder.alloc_register();
    let r2 = builder.alloc_register();

    // Simulating: explain select 1 + 1;
    builder.add_with_comment(Insn::Init { target: 4 }, "Start at 4");
    builder.add_with_comment(
        Insn::Add {
            lhs: r2,
            rhs: r2,
            dest: r1,
        },
        "r[1]=r[2]+r[2]",
    );
    builder.add_with_comment(
        Insn::ResultRow {
            start: r1,
            count: 1,
        },
        "output=r[1]",
    );
    builder.add(Insn::Halt);
    builder.add_with_comment(Insn::Integer { value: 1, dest: r2 }, "r[2]=1");
    builder.add_with_comment(Insn::Goto { target: 1 }, "select 1 + 1;");

    let program = builder.finish(1).unwrap();
    insta::assert_snapshot!(program.to_string());
}

#[test]
fn test_program_instructions_accessor() {
    let mut conn = Connection::open_in_memory().unwrap();
    let mut builder = conn.new_program().unwrap();

    let r1 = builder.alloc_register();

    builder.add_with_comment(
        Insn::Integer {
            value: 42,
            dest: r1,
        },
        "the answer",
    );
    builder.add(Insn::ResultRow {
        start: r1,
        count: 1,
    });
    builder.add(Insn::Halt);

    let program = builder.finish(1).unwrap();
    let instructions = program.instructions();

    assert_eq!(instructions.len(), 3);
    assert_eq!(instructions[0].opcode, "Integer");
    assert_eq!(instructions[0].p1, 42);
    assert_eq!(instructions[0].p2, r1);
    assert_eq!(instructions[0].comment, "the answer");

    assert_eq!(instructions[1].opcode, "ResultRow");
    assert_eq!(instructions[2].opcode, "Halt");
}
