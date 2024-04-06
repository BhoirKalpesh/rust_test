fn reverse_string(input: &str) -> String {
    let mut reversed = String::new();
    
    for c in input.chars().rev() {
        reversed.push(c);
    }
    
    reversed
}

fn main() {
    let original_string = "Hello, Rust!";
    let reversed_string = reverse_string(original_string);
    
    println!("Original String: {}", original_string);
    println!("Reversed String: {}", reversed_string);
}