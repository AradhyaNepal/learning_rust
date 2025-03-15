

fn main(){
    let mut my_string=String::from("Hello World");
    let word=slice_the_value(&my_string[..6]);
    let word=slice_the_value(&my_string[..]);
    let word=slice_the_value(&my_string);
     let my_string_literal = "hello world";
 let word=slice_the_value(&my_string_literal[..6]);
    let word=slice_the_value(&my_string_literal[..]);
    let word=slice_the_value(&my_string_literal);
}


fn slice_the_value(value:&str)->&str{
   let value={let binaries=value.as_bytes();
   for (i,&e) in binaries.iter().enumerate(){
       if e==b' '{
           return &value[0..i];
       }
   }
   &value[..]
};
println!("value is {value}");
value
}
