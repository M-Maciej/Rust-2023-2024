mod decodings_my_solution;
mod num_decodings_not_mine;
fn main() {
    println!("{}", decodings_my_solution::num_decodings("2102922231"));
    println!(
        "{}",
        num_decodings_not_mine::num_decodings_not_mine("2102922231".to_string())
    );
}
