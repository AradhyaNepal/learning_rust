use std::io;
fn main() {
    println!("Enter an value");
    let mut input=String::new();
    io::stdin().read_line(&mut input).expect("Input");
    let index_of_space=first_word(&input);
    println!("Index of word is {index_of_space}");
    input.clear();
  
}

fn first_word(value: &String)->usize{
 let bytes=value.as_bytes();
 for (i,&e) in bytes.iter().enumerate(){
     if e == b' '{
         return i;
     }
 }
 value.len()
}
