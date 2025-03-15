use std::io;

fn main() {
  
   let value:i8=-1;
   let mut user_input=String::new();
   println!("Enter some value");
   match io::stdin().read_line(&mut user_input){
       Ok(_)=>println!("Converting to number"),
      Err(error)=>println!("Error occured {error}"),
   }
   let user_input:u32=match user_input.parse(){
       Ok(value)=>value,
       Err(error)=>{
          println!("The error is {error}"); 
          0
       },
   };

   println!("Parse value is {user_input}");
}
