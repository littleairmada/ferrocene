// -Zpanic_abort_tests makes this test work on panic=abort targets and
// it's a no-op on panic=unwind targets
//@ compile-flags: --test -Zpanic_abort_tests
// ignore-tidy-linelength
//@ run-pass

#![feature(core_intrinsics, generic_assert)]

use std::fmt::{Debug, Formatter};

#[derive(Clone, Copy, PartialEq)]
struct CopyDebug(i32);

impl Debug for CopyDebug {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
    f.write_str("With great power comes great electricity bills")
  }
}

#[should_panic(expected = "Assertion failed: copy_debug == CopyDebug(3)\nWith captures:\n  copy_debug = With great power comes great electricity bills\n")]
#[test]
fn test() {
  let copy_debug = CopyDebug(1);
  assert!(copy_debug == CopyDebug(3));
}

fn main() {
}

// ferrocene-annotations: um_rustc_test
