use std::fs;

fn main(){
    let data = fs::read_to_string("./text.txt").unwrap();
    println!("{}", data);
}
