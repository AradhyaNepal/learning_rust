use std::io;

fn main() {
   loop{

        println!("Choose the options to perform:\n Press 1 for Fehrenheit to Celcius\n Press 2 for Celcius to Fehrenheit");
   
        let user_input=valid_user_input();
       if user_input==1{
         fehrenheit_to_celcius();
      }else{
        celcius_to_fehrenheit();  

      }
   let mut user_input=String::new();

   println!("Type Capital 'Y' if you want to try again");
  io::stdin().read_line(&mut user_input).expect("Something went wrong");
    if user_input != "Y"{
      break;
    }
  } 
  println!("Hope to see you again");
}

fn fehrenheit_to_celcius(){ 
  println!("F");
}
fn celcius_to_fehrenheit(){
 println!("C");
}

fn valid_user_input()->u8{
  let user_input=loop{ 
        let mut user_input=String::new();
         match  io::stdin().read_line(&mut user_input){
           Ok(_)=>{
              println!("Processing input ...");
            },
           Err(e)=>{
             println!("Something went wrong. Try again:\n {e}");
             continue;
            }
        }
       
       let user_input:u8=match user_input.trim().parse(){
         Ok(value)=>{
            if value!=1 && value!=2{
              println!("Invalid input. Enter either 1 or 2. Please try again");
              continue;
            }
             break value;
         },
         Err(error)=>{
            println!("Invalid input. Please try again. {error}");
continue;         
}
       }
   break user_input
   };
  user_input
  

}
