mov a, b
mov b, c
mov d, 10
add a, b 
sub a, b
lda [label]
lda [ab]

ldr z, [0xFFFF]
ldr h, [0]
ldr a, [hl]
ldr b, [dc]
str a, [label]
jnz a
label:
  jnz 0

mov a, (1 << 4)
