@include <testing.asm>

@define num_test 10
@define hex_test 0x10
@define bin_test 0b01010101
@define char_test 'c'

main:
mov a, num_test
ASSERT a, num_test

mov a, hex_test
ASSERT a, hex_test

mob a, bin_test
ASSERT a, bin_test

mob a, char_test
ASSERT a, char_test

COMPLETE_TESTS
