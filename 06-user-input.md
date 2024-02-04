# Reading user input

- ```rust
  use std::io;
  fn main() {
      let mut n = String::new();
      println!("enter a number");
      io::stdin().read_line(&mut n).expect("expected user input");
      println!("you have entered: {}", n.trim().parse::<i32>().unwrap());
  }
  ```
  
  when using default use input, it will get as string but if we want to convert it to integer, we need to do like the above code: we used `.trim()` to remove new line character and then the rest of the code.

- ```rust
  use std::io;
  fn main() {
      let mut n = String::new();
      println!("enter a number");
      io::stdin().read_line(&mut n).expect("expected user input");
      let count = n.trim().parse::<i32>().unwrap();
      let mut v1:Vec<i32> = Vec::new();
  
      for i in 0..count{
          let mut temp = String::new();
          io::stdin().read_line(&mut temp).expect("msg");
          let t1 = temp.trim().parse::<i32>().expect("integer expected.");
          v1.push(t1);
      }
      println!("all numbers: {:?}",v1);
  }
  ```
  
  in this example we used `push()` to insert all data got from user into a variable and print them all
