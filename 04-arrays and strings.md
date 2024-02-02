# Arrays and String

array is also a datatype and stores same data types 

```rust
let l:[i32;5] = [5,6,7,8,9]
```

when defining arrays the first `[i32]` means type of data and the part`;5` means number of elements it includes

```rust
fn main() {
    let l:[i32;5] = [5,6,7,8,9];
    println!("{:#?}"",l.as_ptr());
}
```

by saying `{:#?}` it means we want to print out in debug format and get address of `l` where it lays in memory.

now to read first value of this address we proceed like this:

```rust
fn main() {
    let l:[u8;7] = [4,5,6,7,8,9,1];
    println!("{:#?}",l.as_ptr());
    unsafe{
    let temp: u8 = std::ptr::read(l.as_ptr() as *const u8);
    print!("{}",temp);
    }
}
```

to read second element:

```ruby
fn main() {
    let l:[u8;7] = [4,5,6,7,8,9,1];
    println!("{:#?}",l.as_ptr());
    unsafe{
    let temp: u8 = std::ptr::read((l.as_ptr() as isize + 1 as isize) as *const u8);
    print!("{}",temp);
    }
}
```

now if we change type to `i32`, we would have to move `4bytes`for example: to see second element:

```rust
fn main() {
    let l:[i32;7] = [4,5,6,7,8,9,1];
    println!("{:#?}",l.as_ptr());
    unsafe{
    let temp: u8 = std::ptr::read((l.as_ptr() as isize + 4 as isize) as *const u8);
    print!("{}",temp);
    }
}
```

to get third elemnet just like this`4*2`

```rust
fn main() {
    let l:[i32;7] = [4,5,6,7,8,9,1];
    println!("{:#?}",l.as_ptr());
    unsafe{
    let temp: u8 = std::ptr::read((l.as_ptr() as isize + 4*2 as isize) as *const u8);
    print!("{}",temp);
    }
}
```

- another way to refer to fourth element:
  
  ```rust
  fn main() {
      let l:[i32;7] = [4,5,6,7,8,9,1];
      println!("{:#?}",l.as_ptr());
      unsafe{
      let temp: u8 = std::ptr::read((l.as_ptr() as isize + (std::mem::size_of::<i32>()*3) as isize) as *const u8);
      print!("{}",temp);
      }
  }
  ```

## accessing and modifying array

as arrays are fixed in size and not remoeable, we make use of `vector`

- get data in the array:
  
  ```rust
  fn main() {
      let l:[i32;7] = [4,5,6,7,8,9,1];
      
      for i in 0..l.len(){
          println!("{}",l[i]);
      }
  }
  ```

- another way (safe) to get data from the array:
  
  in this criteria `option` has two values, if ther is a value, it will be stored in the `Some` and if not nothing will be in `None`; `match` here is like switch in other languages
  
  ```rust
  fn main() {
      let l:[i32;7] = [4,5,6,7,8,9,1];
      
      for i in 0..l.len(){
          let res= l.get(i);
        
         match res{
              Some(value) => println!("{}", value),
              None => {}
          };
      }
  }
  ```

- using `iter` to access elements:
  
  ```rust
  fn main() {
      let l:[i32;7] = [4,5,6,7,8,9,1];
      
      for i in l.iter(){
          println!("{}", i);
      }
  }
  ```

- to be able to change values of an array we use `mut` in declaring our variable:
  
  - for example here we want to add 100 to each element:
    
    ```rust
    fn main() {
        let mut l:[i32;7] = [4,5,6,7,8,9,1];
        
        for i in l.iter_mut(){
            *i = *i + 100;
            println!("{}", i);
        }
    }
    ```
    
    here `*i` stores reference to numbers

- to change directly the values of our array we use `.into_iter` method on our array like this:
  
  in case of `arrays` we can access outside of the `into_iter` but in case of `vector` we cannot access the values of the arrays
  
  ```rust
  fn main() {
      let mut l:[i32;7] = [4,5,6,7,8,9,1];
      
      for i in l.into_iter(){
          println!("{}", i);
      }
      println!("{:?}", l);
  }
  ```

- to check if some data is inside of an array we proceed like this:
  
  here we check wether is there `8` inside the array or not:
  
  ```rust
  fn main() {
      let mut l:[i32;7] = [4,5,6,7,8,9,1];
      
      for i in l.into_iter(){
          println!("{}", i);
      }
      println!("{:?}", l.contains(&8));
  }
  ```

- to change an array's values and return a vector we do like this:
  
  ```rust
  fn main() {
      let mut l:[i32;7] = [4,5,6,7,8,9,1];
      
      let t = l.iter().map(|x| x + 200).collect::<Vec<i32>>();
      println!("{:#?}",t);
  
      for i in l.into_iter(){
          println!("{}", i);
      }
      println!("{:?}", l.contains(&8));
  }
  ```

## Vectors

vectors also store the same data types, only difference is the can grow in size and shrink in size

**no need to declare the size for vector, the size will be calculated and stored in `len` elemnet of the vector**

- creating a vector with `100` elements and all with value of `0`:
  
  ```rust
  fn main() {
      let mut l:[i32;100] = [0;100];
      let v1:Vec<i32> = vec![2,3,6,8,9];
      let v2:Vec<i32> = Vec::new();
  
      println!("{:#?}", v1);
  }
  ```

- alter data inside a vector with the following criteria:
  
  ```rust
  fn main() {
      let mut l:[i32;100] = [0;100];
      let mut v1:Vec<i32> = vec![2,3,6,8,9];
      let v2:Vec<i32> = Vec::new();
      v1.push(100);
      println!("{:#?}", v1);
  }
  ```

- pop out an element from the vector:
  
  ```rust
  fn main() {
      let mut l:[i32;100] = [0;100];
      let mut v1:Vec<i32> = vec![2,3,6,8,9];
      let v2:Vec<i32> = Vec::new();
      v1.push(100);
      let temp = v1.pop().unwrap();
      println!("{}", temp);
      println!("===============");
      println!("{:#?}", v1);
  }
  ```

- to iterate and change values of our vector:
  
  needless to say that if we want to only iterate through the vector we only need to use `v1.iter`
  
  ```rust
  fn main() {
      let mut l:[i32;100] = [0;100];
      let mut v1:Vec<i32> = vec![2,3,6,8,9];
      v1.push(100);
      let temp = v1.pop().unwrap();
      println!("{}", temp);
      println!("===============");
      println!("{:#?}", v1);
      println!("===============");
      let v2:Vec<i32> = Vec::new();
      
      for i in v1.iter_mut(){
          *i +=100;
          println!("{}", i);
      }
  }
  ```

- while we cannot use the vector after iterating with `.into_iter` method, we can clone into a temporary vector like this:
  
  ```rust
  fn main() {
      let mut l:[i32;100] = [0;100];
      let mut v1:Vec<i32> = vec![2,3,6,8,9];
      v1.push(100);
      let temp = v1.pop().unwrap();
      println!("{}", temp);
      println!("===============");
      println!("{:#?}", v1);
      println!("===============");
      let v2:Vec<i32> = Vec::new();
      
      for i in v1.clone().into_iter(){
          //*i +=100;
          println!("{}", i);
      }
      println!("=======after iterate========");
      println!("{:?}", v1);
  }
  ```
