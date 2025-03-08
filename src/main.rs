use std::fs::read_to_string;

fn main() {
    let contents = read_to_string("invite.txt").expect("Cannot read from txt file");
    println!("{}", contents);
}
