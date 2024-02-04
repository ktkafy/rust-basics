# Functions

- defining a function:
  
  ```rust
  fn countupto(x:i32){
      let mut counter = 0;
      for i in 1..x+1{
          counter += i;
      }
      println!("{}",counter);
  }
  fn main() {
      countupto(4);
  ```

- defining a function and returning output of it with this criteria:
  
  ```rust
  fn countupto(x:i32) -> i32 {
      let mut counter = 0;
      for i in 1..x+1{
          counter += i;
      }
      return counter;
  }
  fn main() {
      let res = countupto(4);
      println!("{}", res);
  }
  ```
  
  another way to returning a variable is to use it without semicolon and bare at the middle of the page like this:
  
  ```rust
  fn countupto(x:i32) -> i32 {
      let mut counter = 0;
      for i in 1..x+1{
          counter += i;
      }
      counter
  }
  fn main() {
      let res = countupto(4);
      println!("{}", res);
  }
  ```

- declared functions are `private` by default so if we want to we need to specify `pub` to make them public
  
  ```rust
  pub fn countupto(x:i32) -> i32 {
      let mut counter = 0;
      for i in 1..x+1{
          counter += i;
      }
      counter
  }
  fn main() {
      let res = countupto(4);
      println!("{}", res);
  }
  ```

- pass array to the function:
  
  ```rust
  pub fn adder(a:&[i32]){
      println!("{:x?}", a);
  }
  fn main() {
      let arr: [i32;5] = [2,3,6,7,0];
      adder(&arr);
  }
  ```

- pass vector to a function:
  
  ```rust
  pub fn address(a:Vec<i32>){
      println!("{:x?}", a);
  }
  fn main() {
      let v1: Vec<i32> = (0..50).collect::<Vec<i32>>();
      address(v1);
  }
  ```
  
  to access the values from the point where we call the function, so we reference that to the desired vector:
  
  ```rust
  pub fn address(a:&Vec<i32>){
      println!("{:x?}", a);
  }
  fn main() {
      let v1: Vec<i32> = (0..50).collect::<Vec<i32>>();
      address(v1);
      println!("{:?}", v1);
  }
  ```
  
  return from a function as string:
  
  ```rust
  fn randomgererator() -> String{
      let name = "technology";
      return name.to_string();
  }
  fn main() {
      println!("{}",randomgererator());
  }
  ```
