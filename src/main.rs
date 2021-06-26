use num::Integer;
use std::{thread, time::Duration};

// Some basic functions
fn add<T: Integer>(x: T, y: T) -> T { x + y }
fn sub<T: Integer>(x: T, y: T) -> T { x - y }
fn mult<T: Integer>(x: T, y: T) -> T { x * y }
fn div<T: Integer>(x: T, y: T) -> T { x / y }

// Simulate a heavy computation
const HALF_SECOND: Duration = Duration::from_millis(500);
fn heavy_computation<T: Integer>(x: T) -> T {
    thread::sleep(HALF_SECOND);
    x
}

fn main() {
    println!("{}", add(2, 3));
}

// These tests kind of suck, but oh well
#[cfg(test)]
mod meh_tests {
    use crate::*;

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
}