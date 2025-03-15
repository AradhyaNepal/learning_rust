use std::io;

fn main() {
    println!("Hello, world!");
    let tup = ('a',5,5.5);
    let (a,b,c)=tup;
    println!("A is {a} B is {b} C is {c}");
    let a=tup.0;
    let  b= tup.1;
    let c=tup.2;
   
println!("A is {a} B is {b} C is {c}");

  let arr=['a','b','c'];
  let mut value=String::new();
  io::stdin().read_line(&mut value).expect("No value found");
let index:usize=value.trim().parse().expect("Cannot parse");
let value=arr[index];
 println!("Value at given index is {value}");
}
