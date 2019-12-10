fn main() {
    println!("Welcome in Gulshan");
    let age:u8;
    //println!("{}",age);

    let valuenone : Option<i32> = None;
    println!("Value variable has : {:?}",valuenone);

    let name = Some("Gulshan");
    println!("{:?}",name);
    println!("{}");

    let val1 : Option<u8> = Some(1);
    let val2 : u8 = 1;
    println!("{}",val1+val2);

}