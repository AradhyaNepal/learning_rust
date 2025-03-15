fn main() {
//I am calling print_full_details
    print_full_details("Aaradhya","Nepal",22);
   let a={ //Try to find output of this
    let b=plus_one(5);//We are calling expression named plus_one which plus 5 by 1 and returns 6
    b //We are returning 6
  };
   println!("The value of a is {a}"); //We are printing the value of a
}


//We are priting full details
fn print_full_details(first_name:&str,second_name:&str, age:u8){
 println!("Your full  name is {first_name} {second_name}. And you are {age} years old.");
}

//We are adding the value with 1
fn plus_one(value:i32)->i32{
   value+1
}
