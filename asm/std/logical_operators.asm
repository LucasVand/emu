; logical not
@macro
NOT %r0:
  nor %r0, %r0
@end


; logical xor
; trashes z
@macro
XOR %r0, %x1:
  mov z, %r0 ; copy op1
  orr %r0, %x1 ; orr the 2 ops
  and z, %x1 ; and op1 and op2
  not z ; not (op1 or op2)
  and %r0, z ; (op1 or op2) orr not (op1 or op2)
@end

; logical nand
@macro
NAND %r0, %x1:
  and %r0, %x1
  not %r0 
@end
 




  
