fn main() {
    let l:[u8;7] = [4,5,6,7,8,9,1];
    println!("{:#?}",l.as_ptr());
    unsafe{
    let temp: u8 = std::ptr::read((l.as_ptr() as isize + 1 as isize) as *const u8);
    print!("{}",temp);
    }
}