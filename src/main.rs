//use std::{thread, time};
use rand::Rng;

fn main() {
    let option_one = std::env::args().nth(1).expect("Option one arguement is missing");
    let option_two = std::env::args().nth(2).expect("Option two arguement is missing");
    let highest_random_num: i32 = 666;
    let mut wanting_num: i32 = rand::thread_rng().gen_range(0..highest_random_num);

    let mut random_num: i32 = rand::thread_rng().gen_range(1..highest_random_num);

    println!("rand number test: {}", random_num);

    let mut loop_count: i64 = 0;
    loop {
        if wanting_num == 0 {
            wanting_num += 1;
        }
        random_num = rand::thread_rng().gen_range(1..highest_random_num);
        println!("\nKey: {}", loop_count);
        println!("alpha: {}\nbeta: {}\n", random_num, wanting_num);

        if random_num == wanting_num {
            println!("Alpha matches with Beta... \ncalculating the key for anaswer...");
            println!("done!");
            if is_even(loop_count.into()) == true {
                println!("\n\ngo and {}", option_one);
            }
            else {
                println!("\n\ngo and {}", option_two);
            }
            println!("\ngoodbye!");
            break;
        }
        else {
            println!("no.. continue");
        }
        loop_count += 1;

        //thread::sleep(time::Duration::from_millis(10));
        print!("\x1B[2J\x1B[1;1H");
    }
}

fn is_even(mut num: i64) -> bool {
    num = num % 2;
    if num == 0 {
        return true;
    }
    else {
        return false;
    }
    
}
