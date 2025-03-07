use std::cmp::Ordering;
use rand::Rng;
use std::io;
fn main() {
    println!("Hello, world!");
   let a=5;
   let b=6;
   println!("Sum of {a} and {b} is {}",a+b);

   let mut rng = rand::rng();
   let random_value=rng.random_range(1..=100);  
  loop{
    println!("Enter you guess:");
    let mut guess=String::new();
    io::stdin().read_line(&mut guess).expect("Cannot read guess");
    println!("My Guess is {guess}");
    let parsed:u32=match guess.trim().parse(){
      Ok(data)=>data,
      Err(error)=>{
println!("Error is ${error}");
println!("Try again!");
continue;}
    };


    match parsed.cmp(&random_value){
        Ordering::Greater=>println!("{parsed} is Greater than random number. Try again!"),
        Ordering::Less=>println!("{parsed} is Lower than random number. Try again!"),
        Ordering::Equal=>{
print!("Congratulations!! {parsed} is the random number.");
break;
}
,
    }
}
}
