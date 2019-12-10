fn main() {
    //-------------------------------------------Project 1
    let mut shopping_cart: Vec<&str> = Vec::new();
    shopping_cart.push("orange");
    shopping_cart.push("banana");
    shopping_cart.push("apple");
    println!("{} Items Are Available",shopping_cart.len());
    println!("{:?}",shopping_cart);
    let mut price = vec![20, 30, 40];
    println!("Price: {:?}", price);
    for i in &mut price{
        *i += 5;
    }
    for i in &price{
        println!("{}", i);
    }
// let first = shopping_cart.get(0);
// println!("{:?}", first.unwrap());
    println!("{:?}", shopping_cart.get(0).unwrap());

}


//---------------------------------------Project 2

fn p2(){
    let mut japanese = String::new();
    japanese.push_str("안녕하세요");
    let data = "السالم علیكم";
    let arabic = data.to_string();
    let mut jap_ara = japanese + &arabic;
    jap_ara.push('!');
    println!("{}",jap_ara);
    for i in arabic.chars(){
        println!("{}",i)
    }
    for i in arabic.bytes() {
    println!("{}", i);
    }
}


//--------------Project 3
fn p3(){
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Pakistan"), 250);
scores.insert(String::from("New Zealand"), 309);
scores.insert(String::from("England"), 224);
scores.entry(String::from("Pakistan")).or_insert(300);
let mut map = HashMap::new();
for word in scores {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}

println!("{:#?}", map);

}
