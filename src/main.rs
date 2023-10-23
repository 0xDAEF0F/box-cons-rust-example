use scratch_pad::List::{self, Cons, Nil};
use std::io;

fn main() -> io::Result<()> {
    let mut my_list: List<i32> = Nil;

    loop {
        let mut input = String::new();

        println!("Please enter a number (\"cancel\" or \"exit\" to exit).");

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();

                if input == "cancel" || input == "exit" {
                    break;
                }

                let number = input.parse::<i32>();

                if number.is_err() {
                    eprintln!("Could not parse number.");
                    continue;
                }

                my_list = Cons(number.unwrap(), Box::new(my_list));
            }
            Err(_) => {
                continue;
            }
        }
    }

    println!("The list is: {:?}", my_list);

    let doubled_list_vec = my_list.iter().map(|x| x + x).collect::<Vec<_>>();

    println!("The list doubled into a vector is: {:?}", doubled_list_vec);

    Ok(())
}
