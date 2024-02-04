# Command line argument

- getting command line arguments:
  
  ```rust
  use std::env;
  fn main() {
      let mut args = env::args().collect::<Vec<String>>();
      println!("{:?}", args);
  }
  ```

- limiting command line arguments:
  
  ```rust
  use std::env;
  fn main() {
      let mut args = env::args().collect::<Vec<String>>();
      if args.len() !=3{
          println!("[+] Usage: file.exe arg1 arg2 arg3");
          std::process::exit(0);
      }
      println!("{:?}", args);
  }
  ```


