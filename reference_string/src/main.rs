fn main() {
    println!("Hello, world!");
   let mut value=String::from("This is the demo which we are doing");
    let value4=&mut value;
    let value5=&mut value;   
 let value2=get_length(&mut value);
   let value3=get_length(&mut value);
   println!("Length of {value }is {value2} {value3} {value4} {value5}");
}


fn get_length(value:&mut String)->usize{
   value.push_str("Yo yo");
    value.len()
}
