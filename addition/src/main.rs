use rand::Rng;
use std::io;
fn main() {
    println!("Hello, world!");
   let a=5;
   let b=6;
   println!("Sum of {a} and {b} is {}",a+b);

   let mut rng = rand::rng();
   let random_value=rng.random_range(1..=100);
    println!("The random number is {random_value}");
    println!("Enter you guess:")
    let mut guess=String::new();
    io::stdin().read_line(&mut guess).expect("Cannot read guess");
    println!("My Guess is {guess}");
}
