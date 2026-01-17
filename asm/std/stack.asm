@define Stack_Addr_High 0xFFFC
@define Stack_Addr_Low 0xFFFD

@macro
PUSHM %r0, %r1, %r2, %r3:
  push %r0
  push %r1 
  push %r2 
  push %r3
@end

@macro
PUSHM %r0, %r1, %r2:
  push %r0
  push %r1 
  push %r2 
@end

@macro
PUSHM %r0, %r1:
  push %r0
  push %r1 
@end

@macro
POPM %r0, %r1, %r2, %r3:
  pop %r3 
  pop %r2
  pop %r1
  pop %r0
@end

@macro
POPM %r0, %r1, %r2:
  pop %r2 
  pop %r1
  pop %r0
@end

@macro
POPM %r0, %r1:
  pop %r1 
  pop %r0
@end

@macro ; decreiments the stack pointer, trashes hl
DEC_SP %x0:
  ldr h, [Stack_Addr_High] 
  ldr l, [Stack_Addr_Low]
  SUB16 h, l, %x0
  str h, [Stack_Addr_High]
  str l, [Stack_Addr_Low]
@end

@macro ; incriments the stack pointer, trashes hl
INC_SP %x0:
  ldr h, [Stack_Addr_High] 
  ldr l, [Stack_Addr_Low]
  ADD16 h, l, %x0
  str h, [Stack_Addr_High]
  str l, [Stack_Addr_Low]
@end

