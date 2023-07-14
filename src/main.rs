use rand::{Rng, thread_rng};

fn main () {
    init();
}

fn init() {
    let rng: u8 = thread_rng().gen_range(0..=100);
    let mut gcount: u8 = 1;
    println!("{}", &rng);
    println!("Enter a number from 0-100, you will lose after 5 times");
    loop {
        let mut line = String::from("");
        std::io::stdin().read_line(&mut line).unwrap();
        let line = line.trim().parse::<u8>().unwrap();
        if line == rng {
            println!("Great you win!");
            break;
        }
        else if gcount > 4 { 
            println!("Too many guesses! you lost");
            break;
        }
        else if line > rng {
            println!("Too high try again!");
            println!("You have guessed {} times", &gcount);
        }
        else if line < rng {
            println!("Too low try again!");
            println!("You have guessed {} times", &gcount);
        }
        gcount = gcount + 1;
    }
}