# LUC-8 Graphics Guide

Complete reference for the LUC-8 graphics system, including memory layout, memory banking, graphics modes, and pixel encoding.

## Table of Contents

1. [Overview](#overview)
2. [Memory Banking](#memory-banking)
3. [VRAM Layout](#vram-layout)
4. [Graphics Modes](#graphics-modes)
5. [Mode 0: Immediate Mode](#mode-0-immediate-mode)
6. [Mode 1: Tile Mode](#mode-1-tile-mode)

---

## Overview

The LUC-8 processor has a built-in graphics system with two rendering modes. Graphics data is stored in dedicated VRAM (Video RAM) that must be switched into the memory map using the Memory Bank register.

**Key Specifications**:

- Two graphics modes (Immediate and Tile)
- 128 × 128 pixel screen
- 8-bit color encoding (RRRGGGBB format)
- Memory-mapped graphics control via special registers
- Separate memory bank for graphics data

---

## Memory Banking

Graphics data is located in a separate memory bank that must be explicitly activated before accessing VRAM.

### Memory Bank Register

**Address**: `0xFFFA` (Memory Bank / MB register)

The MB register selects which bank of memory is accessible at addresses `0x8000..0xBFFF`:

- **MB = 0**: General Purpose RAM (default)
- **MB = 1**: Video RAM (VRAM) for graphics

### Switching to VRAM

To access graphics data, you must first set MB to 1:

```asm
@define MB_REGISTER 0xFFFA

; Switch to VRAM bank
mov a, 1            ; Set memory bank to 1
str a, [MB_REGISTER]

; Now 0x8000..0xBFFF accesses VRAM
; ... your graphics code here ...

; Switch back to RAM when done
mov a, 0
str a, [MB_REGISTER]
```

**Important**: Always remember to switch the memory bank before reading/writing graphics data, and switch back when done to access regular RAM.

---

## VRAM Layout

When MB = 1, the address space `0x8000..0xBFFF` maps to VRAM with the following layout:

---

## Graphics Modes

### Graphics Mode Register

**Address**: `0xFFF8`

Controls which rendering mode is active:

- **0** = Mode 0 (Immediate/Raw Pixel mode) (defualt)
- **1** = Mode 1 (Tile Map mode)

**Setting graphics mode**:

```asm
@define GRAPHICS_MODE 0xFFF8

; Switch to Mode 0 (Immediate)
mov a, 0
str a, [GRAPHICS_MODE]

; Switch to Mode 1 (Tile)
mov a, 1
str a, [GRAPHICS_MODE]
```

---

## Mode 0: Immediate Mode

**Raw Pixel Mode** - Each pixel is individually addressable and color-controlled.

### How Immediate Mode Works

In Mode 0, the entire VRAM bank (starting at 0x8000) contains raw pixel data:

- Each byte represents exactly one pixel on screen
- 128 × 128 pixels = 16KB total
- Pixels are stored in row-major order (left-to-right, top-to-bottom)
- Memory layout: `0x8000 = pixel[0,0]`, `0x8001 = pixel[1,0]`, `0x8079 = pixel[0,1]` (128 bytes per row)

### Color Encoding

Each pixel byte encodes RGB color as **RRRGGGBB**:

```
Bit 7-5:  Red channel   (0-7, intensity 0-255)
Bit 4-2:  Green channel (0-7, intensity 0-255)
Bit 1-0:  Blue channel  (0-3, intensity 0-255)
```

**Why different bit widths?**

- This encoding maximizes perceived color range with 8 bits
- Provides good balance of color quality and memory efficiency

### Memory Layout

Each byte in memory corresponds to a single pixel starting at 0x8000

```
Mode 0 (Immediate mode) uses 0x8000 - 0xBFFF:
  - Direct pixel data: 128 × 128 = 16KB total
  - Each byte = one pixel on screen
  - Linear layout, row-major order
```

### Pixel Address Calculation

To write a pixel at position (x, y):

```
address = 0x8000 + (y × 128) + x
```

Where:

- `0x8000` = Start of VRAM
- `y` = Row (0-124)
- `x` = Column (0-124)
- Each row takes 128 bytes

### Common Colors

```
0x00 = Black           (000|000|00)
0xE0 = Red             (111|000|00)
0x1C = Green           (000|111|00)
0x03 = Blue            (000|000|11)
0xFC = Yellow          (111|111|00)
0xFF = White           (111|111|11)
```

### Mode 0 Characteristics

**Advantages**:

- Direct, pixel-level control
- Simple linear memory layout
- Good for smooth animations and effects
- Custom graphics and anti-aliasing possible

**Disadvantages**:

- Higher memory bandwidth needed for screen updates
- Less efficient for tile-based content

---

## Mode 1: Tile Mode

**Tile Map Mode** - Efficient graphics using a tile map and tile definition table.

### How Tile Mode Works

Tile mode divides the screen into 8×8 pixel tiles and uses two data structures:

1. **Tile Map** (0x8000-0x8FFF):
   - Grid of 64 × 64 entries
   - Each byte specifies which tile (0-191) to display at that grid position

2. **Tile Table** (0x9000-0xBFFF):
   - Stores up to 192 unique 8×8 pixel tile patterns
   - Each tile is 64 bytes (8 rows × 8 columns, 1 byte per pixel)
   - Tiles 0-191 are available
   - Each pixel in a tile uses the same RRRGGGBB encoding as Mode 0

### Memory Layout

The first section is where the tile map lives so it is any array of indices into the tile lookup table.
The second section is where the tiles are stored. There is a 64 x 64 tile screen size and can store 192 different
tiles

```
0x8000 - 0x9FFF:  TILE MAP (8,192 bytes)
  - Grid of tile indices (100 x 40 = 4,000 entries)
  - Used in Mode 1 (Tile mode)
  - Each byte specifies which tile (0-191) to display

0xA000 - 0xBFFF:  TILE TABLE (8,192 bytes)
  - 192 tile slots, each 64 bytes (8x8 pixels)
  - Stores tile patterns for Mode 1
  - Each tile is 8×8 pixels, 1 byte per pixel

```

### Tile Map Layout

The tile map is a 2D grid indexed by (row, col):

```
Address of tile at grid[row][col] = 0x8000 + (row × 64) + col

Where:
  row = 0-63 (vertical position, tiles from top)
  col = 0-63 (horizontal position, tiles from left)
```

Each byte at that address is a tile ID (0-191) that specifies which tile pattern to display.

### Tile Table Layout

Each tile is 8×8 pixels, stored as 64 consecutive bytes in row-major order:

```
Tile start address = 0xA000 + (tile_id × 64)

Within each tile:
  Bytes 0-7:   Row 0 (8 pixels)
  Bytes 8-15:  Row 1 (8 pixels)
  ...
  Bytes 56-63: Row 7 (8 pixels)
```

Each byte is a pixel color (RRRGGGBB format).

### Tile Definition

Each tile is an 8×8 pattern of colors. Tiles are pre-defined and reused throughout the tile map.

**Tile characteristics**:

- Fixed 8×8 pixel size
- Maximum 192 unique tiles
- Each pixel uses full RRRGGGBB color
- Tiles can be duplicated across the tile map
- All tiles are always available simultaneously

**Tile Memory Requirements**:

- One tile: 64 bytes
- 192 tiles: 12, 288 bytes
- Tile map: 4,096 bytes
- Total for Mode 1: Up to 16,384 bytes

### Mode 1 Characteristics

**Advantages**:

- Memory efficient: Tiles are reused
- Good for level-based games and static content
- Tile map changes are fast (smaller data)
- 192 unique tiles provides reasonable variety
- Screen-level graphics updates are quick

**Disadvantages**:

- Limited to 192 unique tiles
- Graphics must fit 8×8 grid alignment
- Tile-level granularity only (no sub-tile pixel access)
- Less flexibility than immediate mode

**Typical Use Cases**:

- Game levels and backgrounds
- UI elements
- Sprite-based game graphics
- Static or slowly-changing backgrounds

---

## Pixel Color Encoding

All pixel colors use the same **RRRGGGBB** 8-bit format:

```
Bit 7: R7 (High-order red)
Bit 6: R6
Bit 5: R5 (Low-order red)
Bit 4: G7 (High-order green)
Bit 3: G6
Bit 2: G2 (Low-order green)
Bit 1: B1 (High-order blue)
Bit 0: B0 (Low-order blue)
```

### Color Examples

```
0x00 = 000|000|00 = Black
0x09 = 001|001|01 = Dark gray
0x55 = 010|101|01 = Mid-tone
0xB6 = 101|101|10 = Light gray
0xFF = 111|111|11 = White

0xE0 = 111|000|00 = Bright red
0x1C = 000|111|00 = Bright green
0x03 = 000|000|11 = Bright blue

0xFC = 111|111|00 = Yellow (red + green)
0x1F = 000|111|11 = Cyan (green + blue)
0xE3 = 111|000|11 = Magenta (red + blue)
```

---

## Switching Between Modes

When switching between graphics modes, existing graphics data in VRAM remains unchanged. However, screen output will interpret the data differently:

- **Switching from Mode 0 to Mode 1**: Previous pixel data becomes tile map entries
- **Switching from Mode 1 to Mode 0**: Tile data becomes raw pixel values on screen

If you want to preserve graphics, use separate memory areas for each mode or reload the appropriate data after switching.

---

## Simple Example: Draw Pixels

A simple example that switches to immediate mode and colors some pixels:

```asm
@define MB_REGISTER 0xFFFA
@define GRAPHICS_MODE 0xFFF8
@define VRAM_START 0x8000
@define SCREEN_WIDTH 128

main:
  ; Switch to VRAM bank
  mov a, 1
  str a, [MB_REGISTER]

  ; Set graphics mode to immediate (Mode 0)
  mov a, 0
  str a, [GRAPHICS_MODE]

  ; Draw red pixel at (10, 5)
  ; Address = 0x8000 + (5 * 128) + 10
  mov a, 0xE0         ; Red color (111|000|00)
  lda [(VRAM_START + 10 + SCREEN_WIDTH * 5)]
  str a, [hl]

  ; Draw green pixel at (20, 10)
  mov a, 0x1C         ; Green color (000|111|00)
  lda [(VRAM_START + 20 + SCREEN_WIDTH * 10)]
  str a, [hl]

  ; Draw blue pixel at (30, 15)
  mov a, 0x03         ; Blue color (000|000|11)
  lda [(VRAM_START + 30 + SCREEN_WIDTH * 15)]
  str a, [hl]

  ; Switch back to RAM bank
  mov a, 0
  str a, [MB_REGISTER]

  ; Done
  lda [main]
  jnz 1
```

This example:

1. Switches to the VRAM bank (MB=1)
2. Sets graphics mode to immediate (Mode 0)
3. Draws three colored pixels at different positions
4. Switches back to RAM bank (MB=0)
