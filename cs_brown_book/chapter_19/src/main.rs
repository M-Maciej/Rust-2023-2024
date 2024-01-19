use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

struct Fries {}

impl HelloMacro for Fries {
    fn hello_macro() {
        println!("Whatever");
    }
}
#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Fries::hello_macro();
    Pancakes::hello_macro();
}
