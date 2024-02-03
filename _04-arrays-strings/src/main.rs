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
    
    let t:i32 = v1.iter().sum();
    println!("sum is: {}", t);

    for i in v1.clone().into_iter(){
        //*i +=100;
        println!("{}", i);
    }
    println!("=======after iterate========");
    println!("{:?}", v1);

    //collecting data into vector in a range:
    let fa: Vec<i32> = (0..100).collect::<Vec<i32>>();
    //println!("{:#?}", fa);

    let mut v3:Vec<Vec<i32>> = Vec::new();
    v3.push(v1);
    println!("v3 is now {:#?}", v3);

    let mut name:Vec<char> = vec!['T', 'e', 'c', 'h'];
    
    
    let name = "string";
    let myname:String = String::from(name);
    
    for i in name2.chars(){
        println!("{}", i);
    }


    /* 
    let t = l.iter().map(|x| x + 200).collect::<Vec<i32>>();
    println!("{:#?}",t);

    for i in l.into_iter(){
        println!("{}", i);
    }
    println!("{:?}", l.contains(&8));
    */
}

