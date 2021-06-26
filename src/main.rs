use num::Float;

fn add<T: Float>(x: T, y: T) -> T { x + y }
fn sub<T: Float>(x: T, y: T) -> T { x - y }
fn mult<T: Float>(x: T, y: T) -> T { x * y }
fn div<T: Float>(x: T, y: T) -> T { x / y }

fn main() {
    println!("{}", add(4.0, 2.0));
}