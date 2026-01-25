@define additional_regs_addr 0xFFF9

@macro
PRINT:
  ldr z, [additional_regs_addr]
  orr z, 2 ; set the second bit
  str z, [additional_regs_addr]
@end

@macro
PRINT %r0:
  ldr %r0, [additional_regs_addr]
  orr %r0, 2 ; set the second bit
  str %r0, [additional_regs_addr]
@end

