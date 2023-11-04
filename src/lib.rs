pub mod guess {
    use rand::Rng;
    use std::cmp::Ordering;
    pub struct Secret {
        value: u32,
        pub counter: u32,
    }
    impl Secret {
        pub fn new(rng: u32) -> Secret {
            let range = 10u32.pow(rng);
            println!("Selected Range: [0,{}]",range);
            let max_count = range.ilog2() as u32;
            let secret = rand::thread_rng().gen_range(1..=range);
            Secret {
                value: secret,
                counter: max_count + 1,
            }
        }
        pub fn check(&mut self, guess: u32) -> bool {
            self.counter -= 1;
            let b: bool = match self.value.cmp(&guess) {
                Ordering::Greater => {
                    print!("It's Smaller.\n");
                    false
                }
                Ordering::Less => {
                    print!("It's bigger.\n");
                    false
                }
                Ordering::Equal => {
                    print!("YOU WIN!!!\n");
                    true
                }
            };
            if self.counter == 0 && b == false {
                println!("YOU LOOSE; secret number was {}.", self.value);
                return true;
            } else {
                return b;
            }
        }
    }
}

pub mod get {
    use std::io;
    pub fn input(msg:&str) -> u32 {
        loop {
            let mut value = String::new();
            println!("{}",msg);
            io::stdin()
                .read_line(&mut value)
                .expect("Can't Read Number");
            let value: u32 = match value.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            return value;
        }
    }
}
