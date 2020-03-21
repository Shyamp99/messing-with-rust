// #![crate_type="lib"]

pub mod Hangman{
    // use std::io;
    
    pub struct Hangman{
        word: String,
    }
    
    pub fn new(word: String) -> Hangman{
        Hangman{
            word:word,
        }
    }

    // pub fn get_input() -> String{
    //     let mut input = String::new();
    //     println!("input a new letter: ");
    //     io::stdin().read_line(&mut input).expect("Failed to read line");
    //     println!("the input was: {}", input);
    //     return input;
    // }

    impl Hangman{

        pub fn is_correct(&mut self, mut guess:String) -> bool{
            println!("in is_correct {}", self.word);
            guess.pop();
            if self.word.contains(&guess){
                self.word = self.word.replace(&guess, "");
                return true;
            }
            return false;
        }

        pub fn empty(&mut self) -> bool{
            return self.word.is_empty();
        }
    }

}