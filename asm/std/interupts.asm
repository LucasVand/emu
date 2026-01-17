@define Additional_Flags_Address 0xFFF9

@macro
HALT:
  ldr z, [Additional_Flags_Address]
  orr z, 1
  str z, [Additional_Flags_Address]
@end

