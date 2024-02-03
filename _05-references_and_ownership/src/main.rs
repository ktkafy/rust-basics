fn testing(name:&mut String){
    name.push_str("hello");
}
fn main() {
/* 
    let a: i32 = 6;
    let b: i32 = a;

    println!("{}", a);
    println!("{}", b);
*/
/*
    let a: i32 = 6;
    let b: &i32 = &a;

    println!("{}", a);
    println!("{}", b);
*/
/*
let a: i32 = 6;
let b: &i32 = &a;

println!("{}", a);
println!("{}", *b);
*/

/*
let mut a: i32 = 6;
let b: &i32 = &a;

a+=10;

println!("{}", a);
println!("{}", *b);
*/
/*
let mut s: String = String::from("technonlgy");
let s2: String = s;
println!("{}", s); //throws an error cause it has been owned by s2
*/
/*
let mut s: String = String::from("technonlgy");
let s2: &String = &s;
println!("{}", s);
*/
/*throws error because of more than one refence to mutable variable
let mut s: String = String::from("technonlgy");
let s2: &String = &s;
let s3: &String = &s;
s.push_str("hello");
println!("{}", s);
println!("{}", s2);
*/

//needing function testing to be declared

let mut s: String = String::from("technonlgy");
let s2: &String = &s;
let s3: &String = &s;

testing(&mut s);
println!("{}", s);
//println!("{}", s2);
}
