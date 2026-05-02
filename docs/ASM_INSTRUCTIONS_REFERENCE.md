# MINI-8 Instruction and Data Definition Reference

Complete reference for all 16 core instructions and data definition directives of the MINI-8 processor.

## Table of Contents

### Instructions

1. [MOV - Move/Load Immediate](#mov---moveload-immediate)
2. [LDR - Load from Memory](#ldr---load-from-memory)
3. [STR - Store to Memory](#str---store-to-memory)
4. [PUSH - Push to Stack](#push---push-to-stack)
5. [POP - Pop from Stack](#pop---pop-from-stack)
6. [LDA - Load Address](#lda---load-address)
7. [JNZ - Jump if Not Zero](#jnz---jump-if-not-zero)
8. [LSL - Logical Shift Left](#lsl---logical-shift-left)
9. [SUB - Subtract](#sub---subtract)
10. [ADD - Add](#add---add)
11. [ADC - Add with Carry](#adc---add-with-carry)
12. [AND - Bitwise AND](#and---bitwise-and)
13. [OR - Bitwise OR](#or---bitwise-or)
14. [NOR - Bitwise NOR](#nor---bitwise-nor)
15. [CMP - Compare](#cmp---compare)
16. [SBB - Subtract with Borrow](#sbb---subtract-with-borrow)

### Data Definitions

17. [@db - Define Bytes](#db---define-bytes)
18. [@dd - Define Double Words](#dd---define-double-words)
19. [@dw - Define Words (Reserve Space)](#dw---define-words-reserve-space)
20. [@ds - Define String](#ds---define-string)

---

## MOV - Move/Load Immediate

Load an 8-bit value into a register.

**Syntax**: `mov register, imm8/register`

**Operands**:

- `register`: Destination (A, B, C, D, L, H, Z)
- `imm8/register`: Source value (8-bit immediate or register)

**Modifies**: Destination register

**Flags**: None

**Bytes**: 2

**Examples**:

```asm
mov a, 42           ; A = 42
mov b, a            ; B = A
mov c, 0xFF         ; C = 255
mov d, 'X'          ; D = 88 (ASCII code of 'X')
```

---

## LDR - Load from Memory

Load a byte from memory into a register.

**Syntax**: `ldr register, [imm16]` or `ldr register, [hl]`

**Operands**:

- `register`: Destination register
- `[imm16]`: 16-bit memory address (3 bytes total)
- `[hl]`: Use address in HL register pair (2 bytes total)

**Modifies**: Destination register

**Flags**: None

**Bytes**: 2-3

**Examples**:

```asm
ldr a, [0x1000]     ; A = memory[0x1000]
ldr b, [hl]         ; B = memory[HL]
ldr c, [0xFFFB]     ; C = controller input
ldr d, [0xFFF8]     ; D = graphics mode
```

---

## STR - Store to Memory

Store a register value into memory.

**Syntax**: `str register, [imm16]` or `str register, [hl]`

**Operands**:

- `register`: Source register (value to store)
- `[imm16]`: 16-bit memory address (3 bytes total)
- `[hl]`: Use address in HL register pair (2 bytes total)

**Modifies**: Memory at the specified address

**Flags**: None

**Bytes**: 2-3

**Examples**:

```asm
str a, [0x1000]     ; memory[0x1000] = A
str b, [hl]         ; memory[HL] = B
str c, [0xFFFA]     ; Set memory bank register
```

---

## PUSH - Push to Stack

Push a value onto the stack. Stack pointer (0xFFFC-0xFFFD) is incremented.

**Syntax**: `push imm8` or `push register`

**Operands**:

- `imm8/register`: Value to push

**Modifies**: Stack memory, Stack Pointer

**Flags**: None

**Bytes**: 1-2

**Examples**:

```asm
push a              ; Push A onto stack
push 100            ; Push value 100
push h              ; Push H
```

---

## POP - Pop from Stack

Pop a value from stack into a register. Stack pointer is decremented.

**Syntax**: `pop register`

**Operands**:

- `register`: Destination register

**Modifies**: Destination register, Stack Pointer

**Flags**: None

**Bytes**: 1

**Examples**:

```asm
pop a               ; A = stack.pop()
pop b               ; B = stack.pop()
pop z               ; Z = stack.pop()
```

---

## LDA - Load Address

Load a 16-bit address into the HL register pair.

**Syntax**: `lda [imm16]`

**Operands**:

- `imm16`: 16-bit address to load

**Modifies**: H (high byte), L (low byte)

**Flags**: None

**Bytes**: 3 (always)

**Examples**:

```asm
lda [0x1000]        ; HL = 0x1000
lda [loop]          ; HL = address of loop label
lda [my_function]   ; HL = address of function
```

**Usage**: Usually followed by `jnz 1` to jump to the address:

```asm
lda [some_address]
jnz 1               ; Jump to address in HL
```

---

## JNZ - Jump if Not Zero

Jump to address in HL if the operand is non-zero.

**Syntax**: `jnz imm8` or `jnz register`

**Operands**:

- `imm8/register`: Test value (if non-zero, jump occurs)

**Modifies**: Program Counter (if condition true)

**Flags**: None

**Bytes**: 2

**Examples**:

```asm
lda [loop]
jnz 1               ; Always jump (1 is always non-zero)

lda [next]
jnz a               ; Jump if A is non-zero

lda [skip]
jnz b               ; Jump if B is non-zero
```

---

## LSL - Logical Shift Left

Shift register bits left by N positions. Zeros enter from the right, high bit goes to carry.

**Syntax**: `lsl register, imm8` or `lsl register, register`

**Operands**:

- First arg: Register to shift
- Second arg: Shift amount

**Modifies**: First register, Carry flag

**Flags**: C (Carry) - set if bit 7 was 1

**Bytes**: 2

**Examples**:

```asm
lsl a, 1            ; A = A << 1 (multiply by 2)
lsl b, 3            ; B = B << 3 (multiply by 8)
mov c, 2
lsl d, c            ; D = D << (value in C)
```

---

## SUB - Subtract

Subtract an 8-bit value from a register.

**Syntax**: `sub register, imm8` or `sub register, register`

**Operands**:

- First arg: Register (minuend and destination)
- Second arg: Subtrahend

**Modifies**: First register, Flags

**Flags**: C (Carry/Borrow), O (Overflow)

**Bytes**: 2

**Examples**:

```asm
sub a, 5            ; A = A - 5
sub b, c            ; B = B - C
sub d, 1            ; Decrement D
```

---

## ADD - Add

Add an 8-bit value to a register.

**Syntax**: `add register, imm8` or `add register, register`

**Operands**:

- First arg: Register (augend and destination)
- Second arg: Addend

**Modifies**: First register, Flags

**Flags**: C (Carry), O (Overflow)

**Bytes**: 2

**Examples**:

```asm
add a, 10           ; A = A + 10
add b, c            ; B = B + C
add d, 1            ; Increment D
```

---

## ADC - Add with Carry

Add an 8-bit value and the carry flag to a register. Used for multi-byte addition.

**Syntax**: `adc register, imm8` or `adc register, register`

**Operands**:

- First arg: Register (augend and destination)
- Second arg: Addend

**Modifies**: First register, Flags

**Flags**: C (Carry), O (Overflow)

**Bytes**: 2

**Examples**:

```asm
add l, 1            ; L = L + 1 (low byte)
adc h, 0            ; H = H + 0 + carry (16-bit increment)

add b, c            ; B = B + C
adc a, d            ; A = A + D + carry (16-bit add)
```

---

## AND - Bitwise AND

Perform bitwise AND operation.

**Syntax**: `and register, imm8` or `and register, register`

**Operands**:

- First arg: Register
- Second arg: Value to AND with

**Modifies**: First register, Flags

**Flags**: Z (Zero)

**Bytes**: 2

**Examples**:

```asm
and a, 0x0F         ; A = A & 0x0F (keep low 4 bits)
and b, c            ; B = B & C
and f, 0b01000000   ; Keep only zero flag
```

---

## OR - Bitwise OR

Perform bitwise OR operation.

**Syntax**: `or register, imm8` or `or register, register` (also `orr`)

**Operands**:

- First arg: Register
- Second arg: Value to OR with

**Modifies**: First register, Flags

**Flags**: Z (Zero)

**Bytes**: 2

**Examples**:

```asm
or a, 0xF0          ; A = A | 0xF0 (set high 4 bits)
orr b, c            ; B = B | C
orr f, 0b00000010   ; Set bit in flags register
```

---

## NOR - Bitwise NOR

Perform bitwise NOR (NOT of OR) operation. Equivalent to NOT(A OR B).

**Syntax**: `nor register, imm8` or `nor register, register`

**Operands**:

- First arg: Register
- Second arg: Value

**Modifies**: First register, Flags

**Flags**: Z (Zero)

**Bytes**: 2

**Examples**:

```asm
nor a, 0            ; A = ~A (bitwise NOT of A)
nor b, c            ; B = ~(B | C)
```

---

## CMP - Compare

Compare a register with a value. Like SUB but doesn't store the result, only updates flags.

**Syntax**: `cmp register, imm8` or `cmp register, register`

**Operands**:

- First arg: Register to compare
- Second arg: Value to compare with

**Modifies**: Flags only

**Flags**: Z (Zero), L (Less/Negative)

**Bytes**: 2

**Examples**:

```asm
cmp a, 0            ; Check if A is zero
cmp b, 100          ; Compare B with 100
cmp c, d            ; Compare C with D
```

---

## SBB - Subtract with Borrow

Subtract an 8-bit value and the borrow flag from a register. Used for multi-byte subtraction.

**Syntax**: `sbb register, imm8` or `sbb register, register`

**Operands**:

- First arg: Register (minuend and destination)
- Second arg: Subtrahend

**Modifies**: First register, Flags

**Flags**: C (Carry/Borrow), O (Overflow)

**Bytes**: 2

**Examples**:

```asm
sub l, 1            ; L = L - 1 (low byte)
sbb h, 0            ; H = H - 0 - borrow (16-bit subtract)

sub b, c            ; B = B - C
sbb a, d            ; A = A - D - borrow
```

---

## Quick Reference Table

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

## @db - Define Bytes

Define one or more 8-bit bytes in memory. Each value takes 1 byte.

**Syntax**: `label: @db value1, value2, value3, ...`

**Values can be**:

- Decimal: `42`, `255`
- Hexadecimal: `0xFF`, `0x00`
- Binary: `0b11110000`, `0b00001111`
- Characters: `'A'`, `'Z'`
- Expressions: `(10 + 5)`, `(Snake_Length + 5)`

**Examples**:

```asm
my_value:
  @db 42

multiple:
  @db 1, 2, 3, 4, 5

flags:
  @db 0xFF, 0x00, 0b10101010

chars:
  @db 'H', 'e', 'l', 'l', 'o'

data_with_calc:
  @db (Snake_Length + 5), 20
```

---

## @dd - Define Double Words

Define one or more 16-bit values in memory. Each value takes 2 bytes.

**Syntax**: `label: @dd value1, value2, value3, ...`

**Each value is stored as two consecutive bytes**

**Examples**:

```asm
addresses:
  @dd 0x1000, 0x2000

double_word:
  @dd 300, 400, 500

large_values:
  @dd 65535, 32768
```

---

## @dw - Define Words (Reserve Space)

Reserve N bytes of space in memory. Can be used with a single size value or a list of values that sum to the total space.

**Syntax**: `label: @dw size` or `label: @dw value1, value2, ...`

**Size is in bytes**

**Examples**:

```asm
; Reserve 40 bytes of space
buffer:
  @dw 40

; Reserve space by summing a list
snake_body_data:
  @dw (Snake_Length * 2)

; With multiple values (space = 2 + 4 + 5 = 11 bytes)
workspace:
  @dw 2, 4, 5
```

---

## @ds - Define String

Define a string in memory. Can be a single string or a list of values.

**Syntax**: `label: @ds "text"` or `label: @ds value1, value2, ...`

**Examples**:

```asm
message:
  @ds "Hello, World!"

name:
  @ds "MINI-8"

; String with list
data:
  @ds "ABC", 10, 20
```

## Data Definition Quick Reference

| Directive | Purpose                | Syntax                                       | Bytes per Value |
| --------- | ---------------------- | -------------------------------------------- | --------------- |
| @db       | Define Bytes           | `label: @db val1, val2, ...`                 | 1 byte each     |
| @dd       | Define Double Words    | `label: @dd val1, val2, ...`                 | 2 bytes each    |
| @dw       | Define Words (Reserve) | `label: @dw size` or `@dw size1, size2, ...` | As specified    |
| @ds       | Define String          | `label: @ds "text"` or `@ds val1, val2, ...` | String length   |
