fn countupto(x:i32) -> i32 {
    let mut counter = 0;
    for i in 1..x+1{
        counter += i;
    }
    return counter;
}

pub fn adder(a:&[i32]){
    println!("{:x?}", a);
}

pub fn address(a:&Vec<i32>){
    println!("{:x?}", a);
}
fn randomgererator() -> String{
    let name = "technology";
    return name.to_string();
}
fn main() {
    //let res = countupto(4);
    //println!("{}", res);
   
   //let arr: [i32;5] = [2,3,6,7,0];
    //adder(&arr);

    //let v1: Vec<i32> = (0..50).collect::<Vec<i32>>();
    //address(v1)

    //let v1: Vec<i32> = (0..50).collect::<Vec<i32>>();
    //address(&v1);
    //println!("{:?}",v1);

    println!("{}",randomgererator());
}
