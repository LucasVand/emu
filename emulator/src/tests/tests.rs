#[cfg(test)]
mod emulator_tests {
    use std::io;

    use crate::tests::test_utils::EmulatorTestUtils;

    //---- Python Start ----
#[test]
fn test_arithmetic_test() -> Result<(), io::Error> {
  EmulatorTestUtils::test_file("/Users/lucasvanderwielen/Desktop/Programming/Rust/emu/asm/tests/arithmetic_test.asm");
  return Ok(());
}
#[test]
fn test_expression_test() -> Result<(), io::Error> {
  EmulatorTestUtils::test_file("/Users/lucasvanderwielen/Desktop/Programming/Rust/emu/asm/tests/expression_test.asm");
  return Ok(());
}
#[test]
fn test_stack_test() -> Result<(), io::Error> {
  EmulatorTestUtils::test_file("/Users/lucasvanderwielen/Desktop/Programming/Rust/emu/asm/tests/stack_test.asm");
  return Ok(());
}
#[test]
fn test_define_test() -> Result<(), io::Error> {
  EmulatorTestUtils::test_file("/Users/lucasvanderwielen/Desktop/Programming/Rust/emu/asm/tests/define_test.asm");
  return Ok(());
}
#[test]
fn test_data_test() -> Result<(), io::Error> {
  EmulatorTestUtils::test_file("/Users/lucasvanderwielen/Desktop/Programming/Rust/emu/asm/tests/data_test.asm");
  return Ok(());
}
#[test]
fn test_macro_test() -> Result<(), io::Error> {
  EmulatorTestUtils::test_file("/Users/lucasvanderwielen/Desktop/Programming/Rust/emu/asm/tests/macro_test.asm");
  return Ok(());
}
#[test]
fn test_multiply16_test() -> Result<(), io::Error> {
  EmulatorTestUtils::test_file("/Users/lucasvanderwielen/Desktop/Programming/Rust/emu/asm/tests/std_tests/multiply16_test.asm");
  return Ok(());
}
#[test]
fn test_logical_operators_test() -> Result<(), io::Error> {
  EmulatorTestUtils::test_file("/Users/lucasvanderwielen/Desktop/Programming/Rust/emu/asm/tests/std_tests/logical_operators_test.asm");
  return Ok(());
}
#[test]
fn test_16bit_arithmetic_test() -> Result<(), io::Error> {
  EmulatorTestUtils::test_file("/Users/lucasvanderwielen/Desktop/Programming/Rust/emu/asm/tests/std_tests/16bit_arithmetic_test.asm");
  return Ok(());
}
#[test]
fn test_remainder_test() -> Result<(), io::Error> {
  EmulatorTestUtils::test_file("/Users/lucasvanderwielen/Desktop/Programming/Rust/emu/asm/tests/std_tests/remainder_test.asm");
  return Ok(());
}
#[test]
fn test_multiply_test() -> Result<(), io::Error> {
  EmulatorTestUtils::test_file("/Users/lucasvanderwielen/Desktop/Programming/Rust/emu/asm/tests/std_tests/multiply_test.asm");
  return Ok(());
}
#[test]
fn test_jumps_test() -> Result<(), io::Error> {
  EmulatorTestUtils::test_file("/Users/lucasvanderwielen/Desktop/Programming/Rust/emu/asm/tests/std_tests/jumps_test.asm");
  return Ok(());
}
}
