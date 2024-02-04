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
