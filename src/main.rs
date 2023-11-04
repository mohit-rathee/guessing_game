use guessing_game::guess::Secret;
use guessing_game::get::input; 
fn main() {
    println!("Welcome to the game");
    let range = input("Enter the power of 10, as range :");
    let mut secret = Secret::new(range);
    loop {
        let guess = input(format!("Enter your guess, remaining: {}",secret.counter).as_str());
        if secret.check(guess) == true {  //EITHER WIN OR LOOSE.
            break;
        }
    }
}
