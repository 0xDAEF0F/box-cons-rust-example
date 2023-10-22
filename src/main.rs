#[derive(Debug, Clone)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

#[allow(dead_code)]
impl<T> List<T> {
    fn iter(&self) -> ListIterator<T> {
        ListIterator { next: Some(self) }
    }
}

struct ListIterator<'a, T> {
    next: Option<&'a List<T>>,
}

impl<'a, T> Iterator for ListIterator<'a, T>
where
    T: Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next {
            Some(list) => match list {
                List::Cons(value, next) => {
                    self.next = Some(next);
                    Some(value.clone())
                }
                List::Nil => None,
            },
            None => None,
        }
    }
}

use std::io;

use List::{Cons, Nil};

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

                let number = number.unwrap();

                my_list = Cons(number, Box::new(my_list));
            }
            Err(_) => {
                continue;
            }
        }
    }

    println!("The list is: {:?}", my_list);

    Ok(())
}
