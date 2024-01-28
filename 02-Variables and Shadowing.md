# Variables and Shadowing

```rust
fn main() {
    let v1:i32 = 100; //i meaning integer and 32 means 32bits
    let v2:u32 = 100;
}
```

- here when we use `i32` it is signed and it can be either negative or positive number

- when we use `u32` it means that it cannot use negative numbers

**variables are immutable by default**

so in order to that we do like this:

```rust
let mut v1:i32 = 100;
v1 = v1 + 100;
```

so these values for data types can be set:

```
integer data types: u8,i8    u16,i16    u32,i32    u64,i64
float: f16    f32    f64
char
```

#### scope

if we want to have a variable without dependent to other variables outside of that section, we do so:

```rust
fn main() {
    let v1 = 100; //i meaning integer and 32 means 32bits
    let mut v2: u32 = 100;
    v2 = v2 + 30;
    {
        let var_inside:i32 = 50
        println!("value of var_inside is only defined in this scope {}",var_inside)
    }
 }
```

**if we have made our variable as mutable and we change it inside the scope, it will be changed in outer scope too.**

---

to show the maximum number in a condition:

```rust
println!("value of i16 max {}", i16::MAX)
```

#### boolean data type

```rust
let flag: bool = true;
```
