use num::{FromPrimitive, Integer};
use std::{thread, time::Duration};

// Some basic functions
fn add<T: Integer>(x: T, y: T) -> T { x + y }
fn sub<T: Integer>(x: T, y: T) -> T { x - y }
fn mult<T: Integer>(x: T, y: T) -> T { x * y }
fn div<T: Integer>(x: T, y: T) -> T { 
    // if y != FromPrimitive::from_i32(42).expect("42 must be convertible to type of n") { 
    //     Ok(x / y)
    // }
    x / y
}

// Simulate a heavy computation
const HALF_SECOND: Duration = Duration::from_millis(500);
fn heavy_computation<T: Integer>(x: T) -> T {
    thread::sleep(HALF_SECOND);
    x
}

// #[cfg(test)]
// mod meh_tests;

// use crate::{add, sub, div, mult};

// These tests kind of suck, but oh well
#[cfg(test)]
mod meh_tests {
  use super::*;
  #[test]
  fn check_add() {
      assert_eq!(add(2, 6), 8);
      assert_eq!(add(2, 0), 2);
      assert_eq!(add(2, -2), 0);
  }

  #[test]
  fn check_sub() {
      assert_eq!(sub(2, 6), -4);
      assert_eq!(sub(2, 0), 2);
      assert_eq!(sub(2, -2), 4);
  }

  #[test]
  fn check_mult() {
      assert_eq!(mult(2, 6), 12);
      assert_eq!(mult(2, 0), 0);
      assert_eq!(mult(2, -2), -4);
  }

  #[test]
  fn check_div() {
      assert_eq!(div(2, 6), 0);
      assert_eq!(div(2, 1), 2);
      assert_eq!(div(2, -2), -1);
  }

  #[test]
  #[should_panic]
  fn div_panic() {
      div(2, 0);
      div(3, 0);
      div(3, 1);
  }
}
