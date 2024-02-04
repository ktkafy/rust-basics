# References & Ownership

## giving other variables same data and refeting them:

- ```rust
  let a: i32 = 6;
  let b: i32 = a;
  
  println!("{}", a);
  println!("{}", b);
  ```

- ```rust
      let a: i32 = 6;
      let b: &i32 = &a;
  
      println!("{}", a);
      println!("{}", b);
  ```

- ```rust
  let a: i32 = 6;
  let b: &i32 = &a;
  
  println!("{}", a);
  println!("{}", *b);
  ```

- ```rust
  let mut a: i32 = 6;
  let b: &i32 = &a;
  
  a+=10;
  
  println!("{}", a);
  println!("{}", *b);
  ```
  
  here we get error because a is getting used in last line meaning it is accessed by referencing to it and number `6` will not be added to `10`

- ```rust
  let mut s: String = String::from("technonlgy");
  let s2: String = s;
  println!("{}", s); //throws an error cause it has been owned by s2
  ```
  
  ```rust
  let mut s: String = String::from("technonlgy");
  let s2: &String = &s;
  println!("{}", s);
  ```
  
  but here as we used reference to `s` we can still use `s` individually too.

- in rust we cannot have more than one mutable references like the following and we cannot have mutable operations whenever there is a readable reference in the scope
  
  ```rust
  let mut s: String = String::from("technonlgy");
  let s2: &String = &s;
  let s3: &String = &s;
  s.push_str("hello");
  println!("{}", s);
  println!("{}", s2);
  ```

- consuming a value: taking a parameter and not returning anything if we do not `return` a value
  
  ```rust
  fn testing(name:String) -> String{
      return name;
  }
  fn main(){
  let mut s: String = String::from("technonlgy");
  let s2: &String = &s;
  let s3: &String = &s;
  
  testing(s);
  println!("{}", s);
  }
  ```
  
  we can also pass reference to this one:
  
  ```rust
  fn testing(name:&String){
      println!("{}", name.len());
  }
  fn main() {
  let mut s: String = String::from("technonlgy");
  let s2: &String = &s;
  let s3: &String = &s;
  
  testing(&s);
  println!("{}", s);
  }
  ```

- pass mutable reference to make changes in the function:
  
  we pass `mut` in referencing and `mut` when passing argument in the function.
  
  ```rust
  fn testing(name:&mut String){
      name.push_str("hello");
  }
  fn main() {
  let mut s: String = String::from("technonlgy");
  let s2: &String = &s;
  let s3: &String = &s;
  
  testing(&mut s);
  println!("{}", s);
  }
  ```
