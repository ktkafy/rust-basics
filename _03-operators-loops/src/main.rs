fn main() {
   /* 
    let a: i32 = 100;
    let b: i32 = 50;
    let c: i32 = 25;
    let mut d: i32 = 0;

    d = a+b;
    d= a-b;
    d= a*b;

    println!("{}",a+b);
    println!("{}",a-b);
    println!("{}",c);
    println!("{}",!c);
    println!("{}",a/b);
    println!("{}",a%b);
    */
    
    /* 
    for i in 1..100{
        println!("{}",i);
    }
    */
    /*
    let mut i: i32 = 0;
    while i<101{
        println!("{}",i);
        i+=1;
    }
    */
    let mut i:i32 = 22;
    loop{
        if i==1000{
            break;
        }
        print!("the number is: {}\n",i);
        i+=1;
    }
}

