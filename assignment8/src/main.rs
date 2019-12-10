/*
fn main(){
    #[derive(Debug)]
    enum Direction{
        Forward,
        Backward,
        Left,
        Right,
    }

    fn check_direction(check: Direction){
        match check{
            Direction::Forward =>  {
                println!("Vehicle is moved forward!");
            },
            Direction::Backward => {
                println!("Vehicle is moved backward!");
            },
            Direction::Left => {
                println!("Vehicle is moved Left!");
            },
            Direction::Right => {
                println!("Vehicle is moved Right!");
            },
        }
    }


    check_direction(Direction::Forward);
    check_direction(Direction::Backward);
    check_direction(Direction::Right);
    check_direction(Direction::Left);
}
*/

fn main(){


let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);


println!("{:?}",six);
println!("{:?}",none);

}
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
