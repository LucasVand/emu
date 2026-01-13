@define test_value 100

@macro
HALT:
  orr f, 1
@end

@macro
THING %r0:
  mov %r0, 56
@end
