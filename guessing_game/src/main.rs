use std::io;
use std::process;
use std::cmp::Ordering;
use rand::Rng;

fn p(text: &str){
    println!();
    println!("{}", text);
    println!();
}

fn main() {
    let mut first = true;
    let secret_number = rand::thread_rng().gen_range(1..101); 
    p("Welcome to Guess the number!");
    loop{
        if !first {
            p("😡 Again!");
        }

    
        println!("📯 Please input your guess");
    
        let mut guess = String::new();
    
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
            
        let guess = guess.trim();

        if guess.eq("quit") {
            p("👋 Bye, bye!");
            process::exit(0);
        }

        if guess.eq("cheat") {
            println!("🤫 The secret number is: {}", secret_number);
            continue;
        }
        
        let guess: u32 = match guess.parse() {
            Ok(num) => num,
            Err(_) => {
                p("💩 FFS! A NUMBER! How hard can it be?");
                continue;
            },
        };

        println!("You guessed: {}", guess);
    
        match guess.cmp(&secret_number){
            Ordering::Less => println!("☝️  Too Small"),
            Ordering::Greater => println!("👇  Too Big!"),
            Ordering::Equal => {
                p("🎉 You Win!");
                break;
            },
        }
        first = false;
    }
}
