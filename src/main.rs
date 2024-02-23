use rand::Rng;
use std::io;
fn main() {
    println!("Welcome to Caleb's Calculating Fractions!");
    loop {
        let (x, y, z) = generate_terms();
        // leave the nth term out to be solved
        let nth_term = rand::thread_rng().gen_range(1..=4);
        match nth_term {
            1 => {
                println!("Find the missing term: ? / {} * {} = {} ", x, x*z, y*z);
                let input = handle_input();
                if input == y {
                    println!("Correct!");
                } else {
                    println!("Incorrect, The answer is {}", y);
                }
            },
            2 => {
                println!("Find the missing term: {} / ? * {} = {} ", y, x*z, y*z);
                let input = handle_input();
                if input == x {
                    println!("Correct!");
                } else {
                    println!("Incorrect, The answer is {}", x);
                }
            },
            3 => {
                println!("Find the missing term: {} / {} * ? = {} ", y, x, y*z);
                let input = handle_input();
                if input == x*z {
                    println!("Correct!");
                } else {
                    println!("Incorrect, The answer is {}", x*z);
                }
            },
            4 => {
                println!("Find the missing term: {} / {} * {} = ? ", y, x, x*z);
                let input = handle_input();
                if input == y*z {
                    println!("Correct!");
                } else {
                    println!("Incorrect, The answer is {}", y*z);
                }
            },
            _ => println!("Error: nth_term out of range")   
    }
}
}

fn generate_terms () -> (u32, u32, u32){
    let x = rand::thread_rng().gen_range(2..=12);
    let y = rand::thread_rng().gen_range(2..=12);
    let z = rand::thread_rng().gen_range(2..=12);
    (x, y, z)
}

fn handle_input() -> u32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    let input: u32 = input.trim().parse()
        .expect("Please type a number!");
    input
}

// refactor to make random number generation more flexible (modularize out into function)
// 