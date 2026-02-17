/*
** vdbe_rust.c - Helper functions for Rust VDBE API
**
** These functions provide a stable C interface for Rust FFI
** to interact with SQLite's internal VDBE structures.
*/

#ifndef VDBE_RUST_AMALGAMATION
/* When compiled standalone, include the internal headers */
#include "sqliteInt.h"
#include "vdbeInt.h"
#endif
/* When VDBE_RUST_AMALGAMATION is defined, we're appended to sqlite3.c
** and all internal structures are already available */

/*
** Create a new VDBE program directly, without SQL parsing.
** This creates a minimal Parse context just for VDBE construction.
**
** Returns a new Vdbe pointer on success, or NULL on allocation failure.
*/
SQLITE_API Vdbe *sqlite3_vdbe_create(sqlite3 *db){
  Parse sParse;
  Vdbe *p;

  if( db==0 ) return 0;

  memset(&sParse, 0, sizeof(sParse));
  sParse.db = db;

  p = sqlite3VdbeCreate(&sParse);
  if( p ){
    /* Detach from Parse since we're managing directly */
    p->pParse = 0;
  }
  return p;
}

/*
** Prepare a VDBE program for execution.
**
** nMem = number of memory registers needed (pass the highest register + 1)
** nCursor = number of cursors needed
**
** This must be called after all opcodes have been added and before
** sqlite3_step() is called.
*/
SQLITE_API void sqlite3_vdbe_make_ready(Vdbe *p, int nMem, int nCursor){
  Parse sParse;

  if( p==0 ) return;

  memset(&sParse, 0, sizeof(sParse));
  sParse.db = p->db;
  sParse.pVdbe = p;
  sParse.nMem = nMem;
  sParse.nTab = nCursor;
  sParse.nMaxArg = 0;

  p->pParse = &sParse;
  sqlite3VdbeMakeReady(p, &sParse);
  p->pParse = 0;
}

/*
** Get the number of opcodes currently in the program.
*/
SQLITE_API int sqlite3_vdbe_op_count(Vdbe *p){
  if( p==0 ) return 0;
  return p->nOp;
}

/*
** Get the current VDBE state.
** Returns: 0=INIT, 1=READY, 2=RUN, 3=HALT
*/
SQLITE_API int sqlite3_vdbe_state(Vdbe *p){
  if( p==0 ) return -1;
  return p->eVdbeState;
}

/*
** Set a register to an integer value.
** Returns SQLITE_OK on success, SQLITE_ERROR if register is out of bounds.
**
** Note: Can only be called after sqlite3_vdbe_make_ready().
*/
SQLITE_API int sqlite3_vdbe_set_int(Vdbe *p, int reg, sqlite3_int64 value){
  if( p==0 ) return SQLITE_ERROR;
  if( reg < 1 || reg > p->nMem ) return SQLITE_ERROR;
  sqlite3VdbeMemSetInt64(&p->aMem[reg], value);
  return SQLITE_OK;
}

/*
** Get an integer value from a register.
** Returns the integer value, or 0 if register is out of bounds or not an integer.
**
** Note: Can only be called after sqlite3_vdbe_make_ready().
*/
SQLITE_API sqlite3_int64 sqlite3_vdbe_get_int(Vdbe *p, int reg){
  if( p==0 ) return 0;
  if( reg < 1 || reg > p->nMem ) return 0;
  return sqlite3VdbeIntValue(&p->aMem[reg]);
}

/*
** Set a register to a double (real) value.
*/
SQLITE_API int sqlite3_vdbe_set_double(Vdbe *p, int reg, double value){
  if( p==0 ) return SQLITE_ERROR;
  if( reg < 1 || reg > p->nMem ) return SQLITE_ERROR;
  sqlite3VdbeMemSetDouble(&p->aMem[reg], value);
  return SQLITE_OK;
}

/*
** Get a double value from a register.
*/
SQLITE_API double sqlite3_vdbe_get_double(Vdbe *p, int reg){
  if( p==0 ) return 0.0;
  if( reg < 1 || reg > p->nMem ) return 0.0;
  return sqlite3VdbeRealValue(&p->aMem[reg]);
}

/*
** Set a register to NULL.
*/
SQLITE_API int sqlite3_vdbe_set_null(Vdbe *p, int reg){
  if( p==0 ) return SQLITE_ERROR;
  if( reg < 1 || reg > p->nMem ) return SQLITE_ERROR;
  sqlite3VdbeMemSetNull(&p->aMem[reg]);
  return SQLITE_OK;
}

/*
** Check if a register value is NULL.
*/
SQLITE_API int sqlite3_vdbe_is_null(Vdbe *p, int reg){
  if( p==0 ) return 1;
  if( reg < 1 || reg > p->nMem ) return 1;
  return (p->aMem[reg].flags & MEM_Null) != 0;
}

/*
** Get the number of memory registers allocated.
*/
SQLITE_API int sqlite3_vdbe_mem_count(Vdbe *p){
  if( p==0 ) return 0;
  return p->nMem;
}

/*
** Get the number of cursors allocated.
*/
SQLITE_API int sqlite3_vdbe_cursor_count(Vdbe *p){
  if( p==0 ) return 0;
  return p->nCursor;
}

/*
** Create a label for forward jumps.
** Labels are negative numbers that get resolved later.
**
** Note: This requires a Parse context, so we create a temporary one.
*/
SQLITE_API int sqlite3_vdbe_make_label(Vdbe *p){
  Parse sParse;
  int label;

  if( p==0 ) return 0;

  memset(&sParse, 0, sizeof(sParse));
  sParse.db = p->db;
  sParse.pVdbe = p;

  p->pParse = &sParse;
  label = sqlite3VdbeMakeLabel(&sParse);
  p->pParse = 0;

  return label;
}

/*
** Resolve a label to a specific address.
*/
SQLITE_API void sqlite3_vdbe_resolve_label(Vdbe *p, int label){
  if( p==0 ) return;
  sqlite3VdbeResolveLabel(p, label);
}

/*
** Test function that creates and runs a simple VDBE program.
** Returns 42 if successful, or a negative error code.
*/
SQLITE_API int sqlite3_vdbe_test_simple(sqlite3 *db){
  Vdbe *p;
  int rc;
  int result = -1;

  /* Create a new VDBE */
  p = sqlite3_vdbe_create(db);
  if( p==0 ) return -2;

  /* Add opcodes: Integer 42 into register 1, then Halt */
  /* Note: address 0 already has OP_Init(0,1) from sqlite3VdbeCreate */
  sqlite3VdbeAddOp2(p, OP_Integer, 42, 1);  /* address 1: r[1] = 42 */
  sqlite3VdbeAddOp2(p, OP_ResultRow, 1, 1); /* address 2: output r[1] */
  sqlite3VdbeAddOp0(p, OP_Halt);            /* address 3: halt */

  /* Prepare for execution */
  sqlite3VdbeSetNumCols(p, 1);
  sqlite3_vdbe_make_ready(p, 2, 0);  /* 2 registers (0 reserved, 1 used), 0 cursors */

  /* Step through */
  rc = sqlite3_step((sqlite3_stmt*)p);
  if( rc==SQLITE_ROW ){
    result = sqlite3_column_int((sqlite3_stmt*)p, 0);
  }else{
    result = -rc;
  }

  /* Cleanup */
  sqlite3_finalize((sqlite3_stmt*)p);

  return result;
}
