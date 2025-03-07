use std::io;

fn main() {
    println!("Hello, iaAradhya From NP!");
    println!("Guess an random number");
    let mut guess=String::new();
    io::stdin().read_line(&mut guess).expect("No valid input passed");
    println!("You guessed {} {}",guess,guess);
    print!("Printing again {}",guess)
}
