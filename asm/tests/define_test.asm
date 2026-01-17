@include <testing.asm>

@define num_test 10
@define hex_test 0x10
@define bin_test 0b01010101
@define char_test 'c'

main:
mov a, num_test ; a
ASSERT a, num_test ; b

mov a, hex_test ; c
ASSERT a, hex_test ; d

mov a, bin_test ; e
ASSERT a, bin_test ; f

mov a, char_test ; h
ASSERT a, char_test ; i




COMPLETE_TESTS
