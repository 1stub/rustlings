// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let mut answer = 3;
    println!("The square of 3 is {}", square(answer));
}

fn square(num: i32) -> i32 {
    num * num
}
