
@define MB_REGISTER 0xFFFA
@define GRAPHICS_MODE 0xFFF8
@define VRAM_START 0x8000
@define SCREEN_WIDTH 128

main:
  ; Switch to VRAM bank
  mov a, 1
  str a, [MB_REGISTER]

  ; Set graphics mode to immediate (Mode 0) (default 0)
  mov a, 0
  str a, [GRAPHICS_MODE]

  ; Draw red pixel at (10, 5)
  ; Address = 0x8000 + (5 * 125) + 10
  mov a, 0xE0         ; Red color (111|000|00)
  lda [(VRAM_START + 127 + SCREEN_WIDTH * 5 )]
  str a, [hl]

  ; Draw green pixel at (20, 10)
  mov a, 0x1C         ; Green color (000|111|00)
  lda [(VRAM_START + 0 + SCREEN_WIDTH * 10)]
  str a, [hl]

  ; Draw blue pixel at (30, 15)
  mov a, 0x03         ; Blue color (000|000|11)
  lda [(VRAM_START + 0 + SCREEN_WIDTH * 15)]
  str a, [hl]

  ; Switch back to RAM bank
  mov a, 0
  str a, [MB_REGISTER]

  ; Done
  lda [main]
  jnz 1

