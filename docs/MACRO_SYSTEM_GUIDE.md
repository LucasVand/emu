# MINI-8 Macro System Guide

Complete reference for creating and using macros in MINI-8 assembly, including syntax, parameter types, macro overloading, and practical examples.

## Table of Contents

1. [Overview](#overview)
2. [Creating Macros](#creating-macros)
3. [Parameter Types](#parameter-types)
4. [Using Macros](#using-macros)
5. [Macro Overloading](#macro-overloading)
6. [How Macros Work](#how-macros-work)
7. [Advanced Patterns](#advanced-patterns)
8. [Standard Library](#standard-library)
9. [Best Practices](#best-practices)

---

## Overview

Macros are compile-time code templates that allow you to:

- **Reuse code**: Define once, use many times
- **Abstract complexity**: Hide instruction sequences behind readable names
- **Implement 16-bit operations**: Emulate 16-bit math on 8-bit CPU
- **Build higher-level abstractions**: Create conditional jumps, function calls, etc.

Macros are **expanded inline** at compile time with zero runtime overhead.

---

## Creating Macros

### Basic Syntax

```asm
@macro [optional comment]
MACRO_NAME %param1, %param2, %param3:
  ; Macro body - instructions to repeat
  ; Parameters prefixed with % get substituted
@end
```

### Components Explained

| Component  | Example      | Purpose                                      |
| ---------- | ------------ | -------------------------------------------- |
| `@macro`   | `@macro`     | Keyword starting macro definition            |
| Macro name | `INC16`      | Called name (uppercase by convention)        |
| Parameters | `%r0, %r1`   | Placeholders for arguments (prefixed with %) |
| Colon      | `:`          | Separates parameters from body               |
| Body       | `add %r1, 1` | Instructions to insert when called           |
| `@end`     | `@end`       | Ends the macro                               |

### Simple Example

```asm
@macro ; double a register value
DOUBLE %r0:
  add %r0, %r0      ; Add register to itself
@end

; Usage:
DOUBLE a            ; Expands to: add a, a
DOUBLE b            ; Expands to: add b, b
```

### Example with Multiple Parameters

```asm
@macro ; add two registers
ADD_REGS %r0, %r1:
  add %r0, %r1
@end

; Usage:
ADD_REGS a, b       ; Expands to: add a, b
ADD_REGS d, c       ; Expands to: add d, c
```

---

## Parameter Types

### Register Parameters (`%r`)

Replaced with actual register names. Use for operations on registers.

```asm
@macro
NOT %r0:
  nor %r0, %r0
@end

; Usage:
NOT a               ; Expands to: nor a, a
NOT h               ; Expands to: nor h, h
NOT b               ; Expands to: nor b, b
```

**When to use**: Operations that work on any register

### Immediate Parameters (`%i`)

Replaced with numeric values or expressions. Use for constants.

```asm
@macro
LOAD_ADDR %i0:
  lda [%i0]
@end

; Usage:
LOAD_ADDR 0x1000    ; Expands to: lda [0x1000]
LOAD_ADDR my_label  ; Expands to: lda [my_label]
```

Immediates can include expressions that are evaluated at compile time:

```asm
@macro
ADD16_IMM %r0, %r1, %i2:
  add %r1, (%i2)           ; Low byte
  adc %r0, (%i2 >> 8)      ; High byte (right-shifted)
@end

; Usage:
ADD16_IMM h, l, 100
; Expands to:
;   add l, (100)
;   adc h, (100 >> 8)
```

**When to use**: Constants, addresses, and compile-time values

### Generic Parameters (`%x`)

Replaced with either register or immediate. Use when either makes sense.

```asm
@macro
COMPARE %r0, %x1:
  cmp %r0, %x1
@end

; Usage:
COMPARE a, 42       ; Expands to: cmp a, 42
COMPARE a, b        ; Expands to: cmp a, b
COMPARE a, 0xFF     ; Expands to: cmp a, 0xFF
```

**When to use**: Flexible operations that work with any argument type

---

## Using Macros

### Macro Invocation

Call a macro by writing its name followed by arguments:

```asm
MACRO_NAME arg1, arg2, arg3
```

Arguments are substituted for parameters in the macro body.

### Examples

```asm
@macro
PUSH_ALL %r0, %r1, %r2:
  push %r0
  push %r1
  push %r2
@end

main:
  ; Call macro three times with different arguments
  PUSH_ALL a, b, c    ; Push A, B, C
  PUSH_ALL d, h, l    ; Push D, H, L
```

### Macro Expansion

When you write:

```asm
PUSH_ALL a, b, c
```

The assembler expands it to:

```asm
push a
push b
push c
```

---

## Macro Overloading

Define the same macro name multiple times with different parameter counts or types. The assembler automatically chooses the right version based on your call.

### Example: ADD16 with Three Overloads

```asm
; Version 1: Add immediate to 16-bit registers
@macro
ADD16 %r0, %r1, %i2:
  add %r1, (%i2)
  adc %r0, (%i2 >> 8)
@end

; Version 2: Add single register to 16-bit registers
@macro
ADD16 %r0, %r1, %r2:
  add %r1, %r2
  adc %r0, 0
@end

; Version 3: Add two 16-bit register pairs
@macro
ADD16 %r0, %r1, %r2, %r3:
  add %r1, %r3
  adc %r0, %r2
@end
```

When you call:

```asm
ADD16 h, l, 100           ; Calls Version 1 (immediate)
ADD16 h, l, c             ; Calls Version 2 (single register)
ADD16 h, l, b, c          ; Calls Version 3 (register pair)
```

### Example: PUSHM with Multiple Versions

```asm
@macro
PUSHM %r0, %r1:
  push %r0
  push %r1
@end

@macro
PUSHM %r0, %r1, %r2:
  push %r0
  push %r1
  push %r2
@end

@macro
PUSHM %r0, %r1, %r2, %r3:
  push %r0
  push %r1
  push %r2
  push %r3
@end
```

Usage:

```asm
PUSHM a, b              ; Calls 2-register version
PUSHM a, b, c           ; Calls 3-register version
PUSHM a, b, c, d        ; Calls 4-register version
```

**Overloading resolution**: The assembler matches based on:

1. Number of arguments
2. Types of arguments (register vs immediate)
3. Order of macro definitions

---

## How Macros Work

### Step-by-Step Expansion

1. **Parser sees macro invocation**: `INC16 h, l`
2. **Find matching definition**: Look for `INC16` with 2 parameters
3. **Map arguments to parameters**: `%r0=h`, `%r1=l`
4. **Substitute in body**: Replace all `%r0` with `h`, `%r1` with `l`
5. **Insert expanded code**: Replace the call with expanded body

### Example: Tracing Expansion

Original code:

```asm
@macro
INC16 %r0, %r1:
  add %r1, 1
  adc %r0, 0
@end

main:
  INC16 h, l
```

Expansion process:

```
Macro call: INC16 h, l
Parameters: %r0=h, %r1=l
Body:
  add h, 1        ; %r1 replaced with h
  adc h, 0        ; %r0 replaced with h
Result (inserted at call site):
main:
  add l, 1
  adc h, 0
```

### Expressions in Parameters

Immediate parameters can contain expressions that are evaluated at compile time:

```asm
@macro
OFFSET_ADDR %i0, %i1:
  lda [(%i0 + %i1)]
@end

; Call:
OFFSET_ADDR 0x1000, 0x100

; Expands to:
lda [(0x1000 + 0x100)]    ; Evaluated to: lda [0x1100]
```

---

## Advanced Patterns

### Pattern: 16-bit Emulation

Since CPU is 8-bit, use register pairs for 16-bit values:

```asm
@macro ; Increment 16-bit number
INC16 %r0, %r1:
  add %r1, 1        ; Increment low byte
  adc %r0, 0        ; Add carry to high byte
@end

@macro ; Decrement 16-bit number
DEC16 %r0, %r1:
  sub %r1, 1        ; Decrement low byte
  sbb %r0, 0        ; Subtract borrow from high byte
@end

; Usage (HL as 16-bit value):
INC16 h, l          ; HL++
DEC16 h, l          ; HL--
```

### Pattern: Conditional Jumps

Implement conditional control flow using compare and flags:

```asm
@macro ; Jump if equal
JEQ %r0, %x1:
  cmp %r0, %x1
  and f, 0b01000000 ; Mask zero flag (bit 6)
  jnz f             ; Jump if masked flag set
@end

; Usage:
JEQ a, 42           ; Jump if A == 42
```

**Flag bits in 0xFFF9**:

- Bit 6 (0b01000000): Zero flag

### Pattern: Multiple Register Operations

Push or pop multiple registers efficiently:

```asm
@macro
PUSHM %r0, %r1, %r2, %r3:
  push %r0
  push %r1
  push %r2
  push %r3
@end

@macro
POPM %r0, %r1, %r2, %r3:
  pop %r3           ; Reverse order for stack
  pop %r2
  pop %r1
  pop %r0
@end

; Usage:
PUSHM a, b, c, d    ; Save all general registers
POPM a, b, c, d     ; Restore all general registers
```

### Pattern: Macro Nesting

Macros can call other macros:

```asm
@macro
ADD16 %r0, %r1, %r2:
  add %r1, %r2
  adc %r0, 0
@end

@macro
DOUBLE16 %r0, %r1:
  ADD16 %r0, %r1, %r0, %r1  ; Call ADD16 inside DOUBLE16
@end

; Usage:
DOUBLE16 h, l       ; Double the 16-bit HL value
```

### Pattern: Special Variables in Macros

The `$` symbol represents the current address during macro expansion:

```asm
@macro
CALL [%i0]:
  push (($ + 9) >> 8)   ; Push return address high byte
  push (($ + 7))        ; Push return address low byte
  lda [%i0]
  jnz 1
@end

; The $ value changes based on instruction sizes
```

---

## Standard Library

The MINI-8 standard library provides pre-made macros for common operations.

### Including the Library

always.asm is included by defualt at assemble time, it contains most of the common modules
there are other useful modules that can be included

```asm
; Include everything
@include <always.asm>

; always.asm includes everything below
@include <logical_operators.asm>
@include <jumps.asm>
@include <interupts.asm>
@include <stack.asm>
@include <functions.asm>
@include <arithmetic16.asm>
```

### Library Modules

| Module                  | Purpose            | Common Macros                                 |
| ----------------------- | ------------------ | --------------------------------------------- |
| `arithmetic16.asm`      | 16-bit math        | `ADD16`, `SUB16`, `INC16`, `DEC16`, `NOT16`   |
| `jumps.asm`             | Conditional jumps  | `JEQ`, `JNE`, `JLT`, `JLE`, `JGT`, `JZE`      |
| `logical_operators.asm` | Bit operations     | `NOT`, `XOR`, `NAND`                          |
| `stack.asm`             | Stack ops          | `PUSHM`, `POPM`, `DEC_SP`, `INC_SP`, `SET_FP` |
| `functions.asm`         | Function calls     | `CALL`, `RET`, `LDR_FP`, `STR_FP`             |
| `debug.asm`             | Debugging          | `PRINT`, `HALT`                               |
| `testing.asm`           | Unit tests         | `ASSERT`, `ASSERT16`, `COMPLETE_TESTS`        |
| `multiply.asm`          | 8-bit multiply     | `MULTIPLY`, `multiply` function               |
| `multiply16.asm`        | 16-bit multiply    | `multiply16` function                         |
| `remainder.asm`         | Modulo             | `remainder` function                          |
| `random.asm`            | Random numbers     | `random` function                             |
| `interupts.asm`         | Interrupts         | `HALT`                                        |
| `always.asm`            | All standard flies | Standard Functions and macros                 |

### Quick Example: Using Standard Library

```asm
; always.asm is always included

main:
  mov h, 0x10
  mov l, 0x20

  ; Use library macros
  INC16 h, l          ; Increment 16-bit HL
  ADD16 h, l, 100     ; Add 100 to HL
  DEC16 h, l          ; Decrement HL

  JEQ h, 0x10         ; Jump if H == 0x10

  PUSHM a, b, c, d    ; Push multiple registers
  POPM a, b, c, d     ; Pop multiple registers

  HALT                ; Stop execution
```

---

## Best Practices

### Design Principles

1. **Use uppercase names**: `INC16` not `inc16` or `Inc16`
2. **Document behavior**: Add comments explaining what the macro does
3. **Note side effects**: Document which registers are "trashed" (modified)
4. **Keep simple**: Complex macros are hard to debug
5. **Use descriptive names**: `PUSH_ALL_REGS` not `PAR`

### Documentation Template

```asm
@macro
MACRO_NAME %param1, %param2:
  ; Description: What this macro does
  ; Parameters: Explanation of each parameter
  ; Trashes: Which registers are modified
  ; Example: MACRO_NAME a, b

  ; Macro body here
@end
```

---

## Common Mistakes

### Mistake 1: Forgetting Parameter Prefix

```asm
@macro
WRONG_MACRO a, b:     ; WRONG - no % prefix
  add a, b
@end
```

**Fix:**

```asm
@macro
RIGHT_MACRO %r0, %r1: ; Correct - % prefix on parameters
  add %r0, %r1
@end
```

### Mistake 2: Missing @end

```asm
@macro
INCOMPLETE %r0:
  add %r0, 1
; WRONG - missing @end
```

**Fix:**

```asm
@macro
COMPLETE %r0:
  add %r0, 1
@end
```

### Mistake 3: Wrong Parameter Type

```asm
@macro
ADD_IMM %r0, %i1:     ; Expects immediate as second param
  add %r0, (%i1)
@end

; Usage:
ADD_IMM a, b          ; WRONG - b is register, not immediate
ADD_IMM a, 42         ; Correct - 42 is immediate
```

### Mistake 4: Not Accounting for Carry/Borrow

```asm
@macro
WRONG_ADD16 %r0, %r1, %r2:
  add %r1, %r2        ; Wrong - doesn't handle carry
  add %r0, 0          ; Should be 'adc' not 'add'
@end

@macro
RIGHT_ADD16 %r0, %r1, %r2:
  add %r1, %r2
  adc %r0, 0          ; Correct - uses adc for carry
@end
```

---

## Complete Example: Creating a Macro Library

```asm
; my_math.asm - Custom math macros

@macro ; add 1 to a register
INC %r0:
  add %r0, 1
@end

@macro ; subtract 1 from a register
DEC %r0:
  sub %r0, 1
@end

@macro ; multiply a register by 2
MUL2 %r0:
  add %r0, %r0
@end

@macro ; divide a register by 2 (unsigned right shift)
DIV2 %r0:
  lsl %r0, 7          ; Move MSB out
  and %r0, 0x7F       ; Mask off high bit
@end

@macro ; check if register is zero, jump if so
JZ %r0:
  cmp %r0, 0
  and f, 0b01000000
  jnz f
@end
```

Usage in another file:

```asm
@include "my_math.asm"

main:
  mov a, 10
  INC a               ; A = 11
  MUL2 a              ; A = 22
  DIV2 a              ; A = 11

  JZ a                ; Jump if A == 0 (it's not)

  HALT
```

---

## Summary

| Concept          | Key Point                                            |
| ---------------- | ---------------------------------------------------- |
| **Definition**   | Use `@macro ... @end` syntax                         |
| **Parameters**   | Prefix with `%` (like `%r0`, `%i1`)                  |
| **Expansion**    | Macro is replaced with body at compile time          |
| **Substitution** | All `%param` references get replaced                 |
| **Overloading**  | Same name, different parameters = different versions |
| **Performance**  | Zero runtime overhead (compile-time substitution)    |
| **Debugging**    | Understand what code your macro generates            |
