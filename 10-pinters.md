# Pointers

pointers are variables which store addresses of other variables

```rust
fn main() {
    let mut a=4;
    unsafe{
        let p = &a as *const i32;
        println!("pinter value is: {:x?}",p);
    }
}
```

here we get address of variable `p` but if we use `*p` it will fetch the value of it

we can also use this code snippet to get the pointer value:

```rust
println!("address of a is: {:x?}", std::ptr::addr_of!(a));
```

making changes to value of a pointed address:

```rust
fn main() {
    let mut a=4;
    unsafe{
        let p = &mut a as *mut i32;
        println!("pinter value is: {:x?}",*p);
        *p = *p + 10;
    }
    println!("{}",a);
}
```

another way to read data of pointed addresses:

```rust
fn main() {
    let mut a=4;
    unsafe{
        let p = &mut a as *mut i32;
        println!("pinter value is: {:x?}",*p);
        *p = *p + 10;
        let temp = std::ptr::read(p as *const i32);
        println!("{}", temp);
    }
    println!("{}",a);
}
```

make changes to the pointed address:

```rust
std::ptr::write(p as *mut i32,100);
let temp = std::ptr::read(p as *const i32);
println!("{}", temp);
```

### pointers with arrays:

read arrays with pointers:

```rust
let mut b:[i32; 4] = [9,5,6,7];
let p = &mut b as *mut i32; //same as &mut b.as_mut_ptr()
let temp = std::ptr::read(p);
println!("{}", temp);
```

to read next element we can use such code:

we add `4` because each element here gets `4bytes`

```rust
let temp = std::ptr::read((p as usize+4) as *mut i32);
```

raw pointers:

```rust
use std::ffi::c_void;
fn main() {
    let mut a=4;
    let mut b:[i32; 4] = [9,5,6,7];

    unsafe{
        let p = &mut b as *mut i32; //same as &mut b.as_mut_ptr()
        let temp = std::ptr::read(p);
        println!("{}", temp);
        let q = b.as_mut_ptr() as *mut c_void;
```

### pointers with vectors:
