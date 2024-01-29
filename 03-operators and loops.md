# Operators and Loops

```rust
fn main() {
    let a: i32 = 100;
    let b: i32 = 50;
    let c: i32 = 25;
    let mut d: i32 = 0;

    d = a+b;
    d = a-b;
    d = a*b;
    d = a^b;
    d = !b;
//some other conditions to check out:
a>b
a<b
a==b
}
```

`a|b`: meaning if both are true the result will be `0` if any of the corresponding is valid then the return value will be `1`

`a&b`: same as other meanings of a&b simply

---

### for loop

printing out from 1 to 99

```rust
for i in 1..100{
        println!("{}",i);
    }
```

if we want to include `100` itself, we do like this

```rust
for i in 1..100+1{
        println!("{}",i);
    }
OR
for i in 1..=100{
        println!("{}",i);
    }
```

---

### while loop

a simple while loop:

```rust
let mut i: i32 = 0;
    while i<101{
        println!("{}",i);
        i+=1;
    }
```

---

### if loop

```rust
if a>b{
    println!("a is greater than b");
}
else if a==b{
    println!("a is not greater than b, they're the same");
}
else{
    println!("a is lesser than b");
}
```

---

### loop infinitely

**this will loop infinitely and we need to explicitly specify the break condition**


