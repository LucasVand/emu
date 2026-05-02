# LUC-8 Assembly Language Guide

This guide covers the fundamentals of writing LUC-8 assembly programs, all core instructions, data definitions, and basic concepts.

## Table of Contents

1. [Overview](#overview)
2. [Getting Started](#getting-started)
3. [Registers](#registers)
4. [Memory Layout](#memory-layout)
5. [Core Instructions](#core-instructions)
6. [Data Definitions](#data-definitions)
7. [Basic Syntax](#basic-syntax)
8. [Preprocessor Directives](#preprocessor-directives)
9. [Examples](#examples)

---

## Overview

LUC-8 is an 8-bit processor architecture with:

- **8-bit data width**: All operations work with 8-bit values
- **16-bit address bus**: 64 KiB of directly addressable memory
- **Memory banking**: MB register provides access to 256 memory banks
- **7 general purpose registers** plus a flags register
- **Stack support** for function calls and temporary storage
- **Graphics capabilities** with raw pixel and tile modes

---

## Getting Started

Every program starts with a `main:` label:

```asm
main:
  mov a, 5      ; Load 5 into register A
  HALT          ; Stop execution
```

The assembler automatically injects entry point code when assembling:

```asm
lda [main]        ; Load main address
jnz 1             ; Jump to main
@include <always.asm>  ; Include standard library
```

Comments use semicolons:

```asm
mov a, 10  ; This is a comment
; Entire line comment
```

---

## Registers

LUC-8 has 8 registers:

| Name  | ID  | Purpose                              |
| ----- | --- | ------------------------------------ |
| **A** | 0   | General Purpose                      |
| **B** | 1   | General Purpose                      |
| **C** | 2   | General Purpose                      |
| **D** | 3   | General Purpose                      |
| **L** | 4   | Low Index Address Register           |
| **H** | 5   | High Index Address Register          |
| **Z** | 6   | General Purpose / Function Returns   |
| **F** | 7   | Flags Register (instruction results) |

### Register Pairs

For 16-bit operations, registers are paired, all pairings of registers are valid:

- **AB**: A (high byte) + B (low byte)
- **CD**: C (high byte) + D (low byte)
- **HL**: H (high byte) + L (low byte)
- **HA**: H (high byte) + A (low byte)

### Flags Register (F)

The flags register stores status bits from arithmetic operations:

| Bit | Name         | Set When                                |
| --- | ------------ | --------------------------------------- |
| 7   | L (LESS)     | Result is negative (in comparisons)     |
| 6   | Z (ZERO)     | Result is zero                          |
| 5   | C (CARRY)    | Addition caused carry or shift overflow |
| 4   | B (BORROW)   | Subtraction caused borrow               |
| 3   | O (OVERFLOW) | Signed arithmetic overflowed            |
| 2-0 | Unused       | Reserved                                |

layout:
LZCBONNN

---

## Memory Layout

| Address       | Size  | Purpose                             |
| ------------- | ----- | ----------------------------------- |
| 0x0000-0x7FFF | 32 KB | RAM                                 |
| 0x8000-0xBFFF | 16 KB | Banked RAM (switchable via MB)      |
| 0xC000-0xFDFF | 20 KB | General RAM                         |
| 0xFC00-0xFEFF | 512 B | Stack (recommended location)        |
| 0xFF00-0xFFF7 | 248 B | General RAM                         |
| 0xFFF8        | 1 B   | Graphics Mode                       |
| 0xFFF9        | 1 B   | Additional Flags (Print regs, Halt) |
| 0xFFFA        | 1 B   | Memory Bank (MB) Register           |
| 0xFFFB        | 1 B   | Controller Input                    |
| 0xFFFC-0xFFFD | 2 B   | Stack Pointer (SP)                  |
| 0xFFFE-0xFFFF | 2 B   | Program Counter (PC)                |

### Memory Banking

Set the MB register to switch which bank is visible at 0x8000-0xBFFF:
(not really a register just a location in memory)

```asm
mov a, 0
str a, [0xFFFA]     ; Set MB=0, use RAM

mov a, 1
str a, [0xFFFA]     ; Set MB=1, use VRAM (video memory)
```

---

## Core Instructions

All instructions operate on 8-bit data. Instructions are 1-3 bytes long.

For detailed instruction documentation including complete operand information, flag behavior, and examples, see **[ASM_INSTRUCTIONS_REFERENCE.md](ASM_INSTRUCTIONS_REFERENCE.md)**.

## Instruction Summary Table

| Mnemonic | Opcode | Operands      | Description          | Flags |
| -------- | ------ | ------------- | -------------------- | ----- |
| MOV      | 0x0    | reg, imm8/reg | Move value           | -     |
| LDR      | 0x1    | reg, [addr]   | Load from memory     | -     |
| STR      | 0x2    | reg, [addr]   | Store to memory      | -     |
| PUSH     | 0x3    | imm8/reg      | Push to stack        | -     |
| POP      | 0x4    | reg           | Pop from stack       | -     |
| LDA      | 0x5    | [imm16]       | Load address to HL   | -     |
| JNZ      | 0x6    | imm8/reg      | Jump if not zero     | -     |
| LSL      | 0x7    | reg, imm8/reg | Shift left           | C     |
| SUB      | 0x8    | reg, imm8/reg | Subtract             | C,O   |
| ADD      | 0x9    | reg, imm8/reg | Add                  | C,O   |
| ADC      | 0xA    | reg, imm8/reg | Add with carry       | C,O   |
| AND      | 0xB    | reg, imm8/reg | Bitwise AND          | Z     |
| OR       | 0xC    | reg, imm8/reg | Bitwise OR           | Z     |
| NOR      | 0xD    | reg, imm8/reg | Bitwise NOR          | Z     |
| CMP      | 0xE    | reg, imm8/reg | Compare              | Z,L   |
| SBB      | 0xF    | reg, imm8/reg | Subtract with borrow | C,O   |

---

## Data Definitions

Data is stored in memory using data definition keywords. For detailed documentation and examples, see **[ASM_INSTRUCTIONS_REFERENCE.md](ASM_INSTRUCTIONS_REFERENCE.md)**.

### Data Definition Quick Reference

| Directive | Purpose                | Syntax                                       | Bytes per Value |
| --------- | ---------------------- | -------------------------------------------- | --------------- |
| @db       | Define Bytes           | `label: @db val1, val2, ...`                 | 1 byte each     |
| @dd       | Define Double Words    | `label: @dd val1, val2, ...`                 | 2 bytes each    |
| @dw       | Define Words (Reserve) | `label: @dw size` or `@dw size1, size2, ...` | As specified    |
| @ds       | Define String          | `label: @ds "text"` or `@ds val1, val2, ...` | String length   |

**Quick Examples**:

```asm
; Single values
my_byte: @db 42
my_word: @dd 0x1000

; Multiple values
flags: @db 0xFF, 0x00, 0b10101010
data: @dd 300, 400, 500

; Strings
message: @ds "Hello"

; Reserve space
buffer: @dw 40
```

---

## Basic Syntax

### Labels

Labels mark positions in code and data. They end with a colon:

```asm
main:
  mov a, 5

loop:
  add a, 1

data:
  @db 42

another_label:
  @ds "text"
```

Labels can be used as jump targets and with `lda`:

```asm
lda [loop]      ; Load address of loop label
jnz 1           ; Jump to it
```

### Constants in Instructions

Use numeric formats and expressions:

```asm
mov a, 42           ; Decimal
mov b, 0xFF         ; Hex
mov c, 0b1010       ; Binary
mov d, 'A'          ; Character
mov e, (10 + 5)     ; Expression
ldr a, [0x1000]     ; Hex address
```

### Address Modes

- **Immediate**: `mov a, 42`
- **Register**: `mov a, b`
- **Memory (16-bit)**: `ldr a, [0x1000]`
- **Memory (HL pair)**: `ldr a, [hl]`

---

## Preprocessor Directives

### @include - Include Files

Include other assembly files:

```asm
@include <debug.asm>        ; From std
@include "helper.asm"       ; From current directory
```

### @define - Define Constants

Define named values:

```asm
@define WIDTH 128
@define HEIGHT 128
@define BUFFER_ADDR 0x1000

; Later:
mov a, WIDTH        ; Substitutes to: mov a, 128
lda [BUFFER_ADDR]   ; Substitutes to: lda [0x1000]
```

### @undefine - Remove Definition

Remove a previously defined constant:

```asm
@define DEBUG 1
; ... code using DEBUG ...
@undefine DEBUG
```

### Expression Solver

Any value wrapped in parentheses `()` is evaluated at compile time. This allows for compile-time math and address calculations using definitions and labels.

**Valid Operations**:

- **Addition**: `+`
- **Subtraction**: `-`
- **Multiplication**: `*`
- **Division**: `/`
- **Left Shift**: `<<` (multiply by powers of 2)
- **Right Shift**: `>>` (divide by powers of 2)
- **Bitwise NOT**: `~` (invert all bits)
- **Unary Minus**: `-value`
- **Grouping**: Parentheses for precedence

**Operands**:

- Numeric literals: decimal, hex (`0x`), binary (`0b`)
- `@define` constants
- Label addresses
- Combinations of the above

**Examples**:

```asm
@define TILE_SIZE 8
@define NUM_TILES 192
@define TILE_TABLE_START 0xA000

; Calculate total tile data size at compile time
total_tile_bytes: @dw (TILE_SIZE * TILE_SIZE * NUM_TILES)

; Calculate offsets within tile table
tile_5_offset: @dd (TILE_TABLE_START + (5 * 64))

; Bit operations
mask: @db (0xFF << 2)           ; 0xFF shifted left 2 bits
shift_result: @db (200 >> 3)    ; Divide by 8

; Address calculations with labels
main_addr: @dd (main)           ; Address of main label
offset_math: @dd (main + 10)    ; Address plus offset

; Complex expressions
game_data:
  @dw ((BUFFER_WIDTH * BUFFER_HEIGHT) + 100)

; Unary operations
inverted: @db (~0x00)           ; All bits set
negative: @db (-5)              ; Two's complement -5
```

**Compile-Time Evaluation**:

- Expressions are solved during assembly, before code generation
- Only compile-time known values are allowed (definitions, label addresses, literals)
- Results are baked into the assembled code
- Useful for address math, memory layout calculations, and bit manipulations

---

## Examples

### Example 1: Simple Loop

```asm
main:
  mov b, 5        ; Counter

loop:
  sub b, 1        ; Decrement counter
  lda [loop]      ; Load loop address
  jnz b           ; Jump if B is non-zero
```

### Example 2: Memory Operations

```asm
main:
  @define ADDR 0x1000

  mov a, 42
  str a, [ADDR]       ; Store 42 at 0x1000

  ldr b, [ADDR]       ; Load from 0x1000 into B

  add b, 10
  str b, [ADDR]       ; Store modified value back
```

### Example 3: Stack Operations

```asm
main:
  mov a, 10
  mov b, 20

  push a          ; Push 10
  push b          ; Push 20

  pop c           ; C = 20
  pop d           ; D = 10
```

### Example 4: Comparison

```asm
main:
  mov a, 50

  cmp a, 50
  lda [equal]
  and f, 0b01000000   ; Check zero flag
  jnz f               ; Jump if equal

  ; Not equal
  lda [done]
  jnz 1

equal:
  ; Equal - do something

done:
```

### Example 5: 16-bit Address

```asm
main:
  lda [0x2000]    ; Load address 0x2000 into HL

  ldr a, [hl]     ; Load value from address in HL
  add a, 1        ; Increment
  str a, [hl]     ; Store back
```

### Example 6: Data Definitions

```asm
main:
  lda [scores]
  mov c, 5        ; 5 scores to process

loop:
  ldr a, [hl]
  ; Process score in A
  add l, 1        ; Move to next score
  sub c, 1
  lda [loop]
  jnz c

scores:
  @db 85, 90, 78, 92, 88
```

---

## Tips

1. **Label addresses**: Use `lda [label]` to load a label's address, then `jnz 1` to jump
2. **16-bit values**: Use `ADD16`, `SUB16` macros from standard library for multi-byte math
3. **Registers**: A, B, C, D are general purpose. L, H form HL pair for addressing. Z is for returns.
4. **Memory**: Check your code isn't overwriting the stack (0xFC00-0xFEFF) or other important areas
5. **Conditional logic**: Use `cmp` followed by flag checking to make decisions

---

## Related Files

- **outline.txt** - Hardware specification
- **asm/std/** - Standard library with useful macros
- **asm/\*.asm** - Example programs
