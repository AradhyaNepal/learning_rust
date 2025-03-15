


fn main() {
    println!("Hello, world!");
    let mut user= User{
      active:true,
      user_name:String::from("Aradhya"),
      email:String::from("aradhya.1221@gmail.com"),
      sign_in_count:5,
    };
  user.email=String::from("aradhya.1441@gmail.com");
 println!("Email is {0}",user.email);
   let user=build_user(String::from("Gopal"),String::from("aradhya@yopmail.com"));
  println!("Email is {0}",user.email);
println!("Name is {0}",user.user_name);

   let user=User{
email:String::from("a@b.c"),
..user
};

   println!("Email is {0}",user.email);
println!("Name is {0}",user.user_name);

 let value= AlwaysTrue;
}

fn build_user(user_name:String, email:String)->User{
 User{
      active:true,
      user_name,
      email:email,
      sign_in_count:5,
    }
}

struct User{
 active:bool,
 user_name:String,
 email:String,
 sign_in_count:i8,
}


struct AlwaysTrue
