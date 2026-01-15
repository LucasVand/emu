@macro 
HALT:
  orr f, 1
@end


mov b, 100

MOV_HALT


@macro
MOV_HALT:
  mov a, 10
  MOV_HALT
  HALT
@end
