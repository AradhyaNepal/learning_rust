use std:io;

fn main() {
    println!("Enter an number");
   let mut inputValue=String::new();
   io::stdin().read_line(&mut inputValue).expect("Cannot get input");
   let inputValue:i32=inputValue.parse().trim().except("Cannot parse");
  if inputValue<0{
   println!("{inputValue} is negative");
 }
 else if inputValue>0{
  println!("{inputValue} is positive");
}else{
  println!("{inputValue} is zero");
 }

}
