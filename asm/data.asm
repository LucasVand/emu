@macro 
HALT:
  orr f, 1
@end


mov a, 2

HALT

and a, 5
