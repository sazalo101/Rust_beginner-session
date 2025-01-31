struct Person {
       name:String,
       age:u8

}
fn main(){
   let user=Person{name:String::from("Alice"),age:25};
   println!("{} is {} years old ", user.name,user.age);
}

