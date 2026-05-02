# MINI-8 Assembler & Emulator

A complete assembler and emulator toolchain for the **MINI-8 processor** - an 8-bit, 16-bit addressable architecture designed for educational purposes and retro computing projects.

This project provides:

- **Assembler**: Compile MINI-8 assembly code to binary
- **Emulator**: Execute compiled binaries with graphics and debugging support
- **Disassembler**: Reverse engineer binary files back to assembly
- **Standard Library**: Pre-built macros for common operations (16-bit math, functions, I/O)

---

## Quick Start

### Installation

1. Build the project:

```bash
cargo build --release
```

2. The binary is now available at `./target/release/mini8` or `cargo run` does the same

### First Program

Create a simple assembly file (`hello.asm`):

```asm

main:
  mov a, 42         ; Load 42 into register A
  mov b, 10         ; Load 10 into register B
  add a, b          ; A = A + B (52)
  HALT              ; Stop execution
```

Assemble and run:

```bash
mini8 run hello.asm
```

---

## Usage

The MINI-8 CLI has four main commands: `assemble`, `disassemble`, `run`, and `execute`.

### 1. Assemble

Compile assembly code to binary.

```bash
mini8 assemble <input.asm> [output.bin]
```

**Examples:**

```bash
# Compile to default output
mini8 assemble program.asm

# Compile to custom output
mini8 assemble program.asm ./output/program.bin

```

### 2. Disassemble

Reverse-engineer a binary file back to readable assembly.

```bash
mini8 disassemble <input.bin>
```

**Examples:**

```bash
# Basic disassembly
mini8 disassemble program.bin

```

### 3. Run

Assemble and immediately execute an assembly file.

```bash
mini8 run <input.asm>
```

**Examples:**

```bash
# Run with default settings
mini8 run program.asm

# Run with register printing
mini8 run program.asm --print-regs

# Run with graphics window
mini8 run program.asm --graphics

# Run with 1ms delay per instruction
mini8 run program.asm --speed 1000

# All options
mini8 run program.asm --print-regs --graphics --speed 500
```

### 4. Execute

Execute a pre-compiled binary file in the emulator.

```bash
mini8 execute <input.bin>
```

**Examples:**

```bash
# Execute precompiled binary
mini8 execute program.bin

# Execute with debugging
mini8 execute program.bin --print-regs

# Execute with graphics
mini8 execute program.bin --graphics

# Slow execution for debugging
mini8 execute program.bin --speed 5000 --print-regs
```

---

## Assembly Language

The MINI-8 assembly language is fully documented in the **[ASM_LANGUAGE_GUIDE.md](ASM_LANGUAGE_GUIDE.md)**.

### Quick Overview

**Registers (8-bit):**

- `A, B, C, D` - General purpose
- `L, H` - Low/High byte pair (combined as HL for 16-bit)
- `Z` - Return register
- `F` - Flags register

**Instructions:**

- Data movement: `MOV`, `LDR`, `STR`, `LDA`
- Arithmetic: `ADD`, `SUB`, `ADC`, `SBB`
- Bitwise: `AND`, `OR`, `NOR`, `LSL`
- Control: `JNZ`, `CMP`
- Stack: `PUSH`, `POP`

**Data Types:**

- `@db` - Define bytes
- `@dd` - Define double words (16-bit)
- `@dw` - Define words (reserve space)
- `@ds` - Define strings

**Example Program:**

```asm

@define MAX 100
@define ADDR 0x1000

main:
  mov a, 10
  ADD16 h, l, MAX          ; Use 16-bit macro from library

  cmp a, 50
  JEQ a, handle_equal       ; Jump if equal (library macro)

  HALT

handle_equal:
  mov b, 99
  HALT
```

---

## Graphics

The MINI-8 has an integrated graphics system with two modes. See **[GRAPHICS_GUIDE.md](GRAPHICS_GUIDE.md)** for complete documentation.

### Quick Overview

**Mode 0: Immediate Mode**

- Direct pixel writing
- 128 × 128 pixels
- Each pixel is one byte (RRRGGGBB color encoding)

**Mode 1: Tile Mode**

- 64 × 64 tile grid
- 192 unique 8×8 pixel tiles
- Memory-efficient for tile-based graphics

**Enable Graphics:**

```bash
mini8 run program.asm --graphics
```

### Graphics Example

```asm
@define MB_REGISTER 0xFFFA
@define GRAPHICS_MODE 0xFFF8
@define VRAM_START 0x8000

main:
  ; Switch to VRAM bank
  mov a, 1
  str a, [MB_REGISTER]

  ; Set immediate mode
  mov a, 0
  str a, [GRAPHICS_MODE]

  ; Draw red pixel at (10, 5)
  mov a, 0xE0               ; Red color
  lda [(VRAM_START + 10 + 128 * 5)]
  str a, [hl]

  HALT
```

---

## Macros & Standard Library

The standard library provides pre-built macros for common operations. See **[MACRO_SYSTEM_GUIDE.md](MACRO_SYSTEM_GUIDE.md)** for complete documentation.

### Quick Overview

**Including Macros:**

```asm
@include <arithmetic16.asm>; Include 16-bit arithmetic
@include <jumps.asm>       ; Include conditional jumps
```

**Common Macros:**

**16-bit Arithmetic** (`arithmetic16.asm`):

```asm
INC16 h, l                 ; HL++
DEC16 h, l                 ; HL--
ADD16 h, l, 100            ; HL += 100
SUB16 h, l, b, c           ; HL -= BC
```

**Conditional Jumps** (`jumps.asm`):

```asm
JEQ a, 42                  ; Jump if A == 42
JNE b, 10                  ; Jump if B != 10
JLT c, 50                  ; Jump if C < 50
JGT d, 25                  ; Jump if D > 25
```

**Stack Operations** (`stack.asm`):

```asm
PUSHM a, b, c, d           ; Push 4 registers
POPM a, b, c, d            ; Pop 4 registers (reversed)
```

**Functions** (`functions.asm`):

```asm
CALL [my_function]         ; Call function (saves return address)
RET                        ; Return from function
SET_FP                     ; Set frame pointer for locals
```

**Math Operations** (`multiply.asm`, `remainder.asm`, etc.):

```asm
MULTIPLY a, b, c, d        ; Z = A × B
push divisor
push dividend
CALL [remainder]           ; Z = dividend % divisor
```

---

## Documentation

Complete documentation is provided in several files:

| File                                                           | Purpose                              |
| -------------------------------------------------------------- | ------------------------------------ |
| [ASM_LANGUAGE_GUIDE.md](ASM_LANGUAGE_GUIDE.md)                 | Complete assembly language reference |
| [ASM_INSTRUCTIONS_REFERENCE.md](ASM_INSTRUCTIONS_REFERENCE.md) | Detailed instruction documentation   |
| [GRAPHICS_GUIDE.md](GRAPHICS_GUIDE.md)                         | Graphics system and modes            |
| [MACRO_SYSTEM_GUIDE.md](MACRO_SYSTEM_GUIDE.md)                 | How to create and use macros         |

### Learning Path

1. **Start with basics**: [ASM_LANGUAGE_GUIDE.md](ASM_LANGUAGE_GUIDE.md) - Overview and getting started
2. **Learn instructions**: [ASM_INSTRUCTIONS_REFERENCE.md](ASM_INSTRUCTIONS_REFERENCE.md) - All 16 instructions explained
3. **Use macros**: [MACRO_SYSTEM_GUIDE.md](MACRO_SYSTEM_GUIDE.md) - Understand the standard library
4. **Add graphics**: [GRAPHICS_GUIDE.md](GRAPHICS_GUIDE.md) - Graphics modes and examples

---

## Examples

### Example 1: Simple Counter

```asm
@include <always.asm>

main:
  mov a, 0              ; Start at 0
  mov b, 10             ; Count to 10

count_loop:
  add a, 1              ; Increment
  cmp a, b              ; Compare with 10
  lda [count_loop]      ; Load loop address
  jnz a                 ; Jump if not done

  HALT
```

Run it:

```bash
mini8 run examples/counter.asm --print-regs
```

### Example 2: 16-bit Addition

```asm
@include <always.asm>

main:
  ; HL = 0x1000
  lda [0x1000]

  ; HL += 0x0100
  ADD16 h, l, 0x0100

  ; HL now = 0x1100
  HALT
```

### Example 3: Drawing Pixels (Graphics)

```asm
@include <always.asm>

@define MB_REGISTER 0xFFFA
@define GRAPHICS_MODE 0xFFF8
@define VRAM_START 0x8000
@define SCREEN_WIDTH 128

main:
  ; Switch to VRAM
  mov a, 1
  str a, [MB_REGISTER]

  ; Set immediate mode
  mov a, 0
  str a, [GRAPHICS_MODE]

  ; Draw three pixels
  mov a, 0xE0           ; Red
  lda [(VRAM_START + 10 + SCREEN_WIDTH * 5)]
  str a, [hl]

  mov a, 0x1C           ; Green
  lda [(VRAM_START + 20 + SCREEN_WIDTH * 10)]
  str a, [hl]

  mov a, 0x03           ; Blue
  lda [(VRAM_START + 30 + SCREEN_WIDTH * 15)]
  str a, [hl]

  ; Back to RAM
  mov a, 0
  str a, [MB_REGISTER]

  HALT
```

Run with graphics:

```bash
mini8 run examples/graphics.asm --graphics
```

---

## Resources

- **[ASM_LANGUAGE_GUIDE.md](ASM_LANGUAGE_GUIDE.md)** - Complete language reference
- **[ASM_INSTRUCTIONS_REFERENCE.md](ASM_INSTRUCTIONS_REFERENCE.md)** - All instructions detailed
- **[GRAPHICS_GUIDE.md](GRAPHICS_GUIDE.md)** - Graphics system documentation
- **[MACRO_SYSTEM_GUIDE.md](MACRO_SYSTEM_GUIDE.md)** - Creating custom macros
- **outline.txt** - Hardware specification
- **asm/tests/** - Example programs and test cases

---

**Ready to start?** Read [ASM_LANGUAGE_GUIDE.md](ASM_LANGUAGE_GUIDE.md) and try the examples above!
