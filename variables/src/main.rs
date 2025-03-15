use std::io;
fn main() {
    let mut  x=5;
    println!("Your number is {x}");
    x=6;
   println!("Your new number is {x}");
   const THREE_HOURS_IN_SECONDS: u32=60*60*3;
   println!("Value is {THREE_HOURS_IN_SECONDS}");
   println!("-------------------------------------");
   let y=10;
   let y=y+5;
   println!("Value of outer upper y is {y}");
   {
     let y=y*2;
     println!("Value of inner  y is {y}");
     
   } 
   println!("Value of outer lower y is {y}");
  let mut user_input=String::new();
  io::stdin().read_line(&mut user_input).expect("Cannot find name");
  let user_input=user_input.len();
  println!("Length of user input is {user_input}")
}
