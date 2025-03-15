fn main() {
    println!("Hello, world!");
    let mut value:&str="Hello";
    value="World";
    println!("{value}");

    let mut value:String=String::from("Aradhya");
   println!("{value}");
   value.push_str(" Is Genious");

   println!("{value}");
 let value:u8=2;
   let (value,result)=calculate_square(value);
println!("Square of {value} is {result}");

}


fn calculate_square(value:u8)->(u8,u8){
  (value,value*value)
}
