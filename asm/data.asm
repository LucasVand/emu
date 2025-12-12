lda [main]
jnz 1 


orr f, 1 


main: 
  mov a, 100
  lda [store]
  str a, [hl]

  orr f, 1


str:
  @ds "this", "is"

store:

