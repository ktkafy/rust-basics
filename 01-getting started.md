# Getting started

our very first simple program

```rust
fn main(){
    println!("hello world");
}
```

running our first program:

```cmd
rustc helloworld.rs
```

it simply generates exe file for us

---

#### creating new project

```cmd
cargo new helloworld
cd halloworld
code . #run vscode for us
```

**to build program without executing we proceed as follow**

```cmd
cargo build
cargo build --release #to have release version
```

#### to format our program we do this in command line

```powershell
rustfmt main.rs
```

#### compiling to 32bit and 64bit program

```powershell
rustup show
rustup target list
rustup install stable-i686-pc-windows-msvc
```

now to change to 32bit we do like this:

```powershell
rustup default stable-i686-pc-windows-msvc
```

now if we go to a project and try to run them it will generate 32bit version out of it

```powershell
cargo run
```

#### formatting arguments in rust

```rust
fn main(){
    println!("{}",5);
}
```

if we have 64bit set, the output will be 8 and if it is 32bit it would be 4

```rust
fn main(){
    println!("{}",std::mem::size_of::<unsize>());
}
```
