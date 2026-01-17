

@macro ; jump if equal
JEQ %r0, %x1:
  cmp %r0, %x1
  and f, 0b01000000 ; mask the zero flag
  jnz f
@end

@macro ; jump if zero
JZE %r0:
  JEQ %r0, 0 
@end

@macro ; jump not equal
JNE %r0, %x1:
  cmp %r0, %x1 
  not f
  and f, 0b01000000
  jnz f
@end

@macro ; jump less then
JLT %r0, %x1:
  cmp %r0, %x1
  and f, 0b10000000
  jnz f
@end

@macro ; jump less then or equal
JLE %r0, %x1:
  cmp %r0, %x1
  and f, 0b11000000
  jnz f
@end

@macro ; jump less then
JGT %r0, %x1:
  cmp %r0, %x1
  and f, 0b10000000
  jnz f
@end

@macro ; jump less then or equal
JGE %r0, %x1:
  cmp %r0, %x1
  not f
  and f, 0b10000000
  jnz f
@end

