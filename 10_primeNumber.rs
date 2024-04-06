use std::io;

fn is_prime(num: u32) -> bool {
    if num <= 1 {
      return false; 
    }
    for i in 2..=(num as f64).sqrt() as u32 { 
        if num % i == 0 {
          return false;
        }
      }
    true 
  }

fn main() {
    println!("Enter the number: ");
    let mut input=String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let number=input.trim().parse().expect("Invalid number");
    if is_prime(number){
        println!("{} is a prime number",number); 
    }
    else{
        println!("{} is not a prime number",number);
    }
}
