use std::io;

fn main(){
    let s=String::from("Hello World");
    let hello=&s[..5];
    let world=&s[6..];
   println!("{hello} {world}");
   let mut input=String::new();
   io::stdin().read_line(&mut input).expect("Input");
   let value=slice_the_value(&input);
   input.clear();
   println!("value is {value}");
}


fn slice_the_value(value:&String)->&str{
   let binaries=value.as_bytes();
   for (i,&e) in binaries.iter().enumerate(){
       if e==b' '{
           return &value[0..i];
       }
   }
   &value[..]
}
