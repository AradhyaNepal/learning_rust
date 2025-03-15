fn main() {
    println!("Hello, world!");
   let mut value="Hello World";
  let mut loop_times=3;
   while loop_times>0{
    println!("Is looping");
    value="{value} {value}";
    loop_times=loop_times-1;
   }

println!("{value}");


let a=["Aradhya","GOpal","Nepal"];

for e in a{
 println!("{e}");
}

for e in (1..5).rev(){//[1,5)
  println!("{e}");

}

}
