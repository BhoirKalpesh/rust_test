use std::io;
fn is_palindrome(s: &str) -> bool {
    let s_lower = s.to_lowercase();

    s_lower.chars().eq(s_lower.chars().rev())
}

fn main() {
    println!("Enter a string: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let input = input.trim();

    if is_palindrome(input) {
        println!("'{}' is a palindrome.", input);
    } else {
        println!("'{}' is not a palindrome.", input);
    }
}
