use std::ffi::c_void;
fn main() {
    let mut a=4;
    let mut b:[i32; 4] = [9,5,6,7];
    let mut c:Vec<i32> = vec![5,6,7,8];

    unsafe{
        //pointers with vectors
        let p = &c as *const Vec<i32>;
        println!("address of c: {:x?}", std::ptr::addr_of!(c));
        println!("{:x?}",p);
        let q = c.as_mut_ptr();
        println!("q pointer: {:x?}", q);
        let temp = std::ptr::read(q as *mut i32);
        println!("{:x?}", temp);
        let temp2 = std::ptr::read((p as usize + 8) as *mut i32); //to get length of our vector
        println!("{:x?}", temp2);





        /*//pointers with arrays 
        let p = &mut b as *mut i32; //same as &mut b.as_mut_ptr()
        let temp = std::ptr::read(p);
        println!("{}", temp);
        let q = b.as_mut_ptr() as *mut c_void;
        */



        /*
        //let p = &a as *const i32;
        //println!("pinter value is: {:x?}",*p);
        
        let p = &mut a as *mut i32;
        println!("pinter value is: {:x?}",*p);
        
        std::ptr::write(p as *mut i32,100);
        let temp = std::ptr::read(p as *const i32);
        println!("{}", temp);

        /*
        *p = *p + 10;
        let temp = std::ptr::read(p as *const i32);
        println!("{}", temp);
        */

        */
    }
    //println!("{}",a);
}
