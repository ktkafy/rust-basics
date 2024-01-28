fn main() {
    let v1 = 100; //i meaning integer and 32 means 32bits
    let mut v2: u32 = 100;
    v2 = v2 + 30;
    println!("{}", v1);
    println!("{}", v2);
    let float_var: f32 = 3.123;
    let initial: char = 't';
    println!("value of v1 is: {}", std::mem::size_of_val(&v1));
    println!("value of i16 max {}", i16::MAX);
    let flag: bool = true;
    println!("the boolean variable is {}", flag)
}
