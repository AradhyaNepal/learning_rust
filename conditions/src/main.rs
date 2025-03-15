use std::io;

fn main() {
loop{
    println!("Enter an number");
   let mut input_value=String::new();
   io::stdin().read_line(&mut input_value).expect("Cannot get input");
   let input_value:i32=input_value.trim().parse().expect("Cannot parse");
  if input_value<0{
   println!("{input_value} is negative");
 }
 else if input_value>0{
  println!("{input_value} is positive");
}else{
  println!("{input_value} is zero");
 }
  println!("Do you want to try again. Y for yes");
  let mut input_value=String::new();
  io::stdin().read_line(&mut input_value).expect("Cannot read");
  let user_input:bool=if input_value.trim()=="Y" {true}else {false};
 if user_input==false{
  break;
  }
}

}
