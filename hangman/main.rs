use std::io;
//i'm overcomplictating this to get used to crates, structs and other stuff
mod Hangman;

fn main() {
    // println!();
    println!("input some word: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    println!("the input was: {}", input);
    // //build the hangman struct
    input.pop();
    let mut game = Hangman::Hangman::new(input);
    let mut count:i32 = 0;
    while count < 6 && !game.empty() {
        let mut letter = String::new();
        println!("input a new letter: ");
        io::stdin().read_line(&mut letter).expect("Failed to read line");
        println!("the input was: {}", letter);
        if game.is_correct(letter){
            println!("Correct!");
        }
        else{
            count+=1;
        }
    }
    if count == 6{
        println!("Game Over");
    }
    else{
        println!("You Won!")
    }
    // println!("type in a number for testing user input: ");
    // 
}
