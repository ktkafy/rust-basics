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
