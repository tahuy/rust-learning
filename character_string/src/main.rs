#[allow(dead_code)]
#[allow(unused_imports)]
#[allow(unused_must_use)]
#[allow(unused_variables)]
#[allow(unused_assignments)]

use rand::Rng;
use std::io::stdin;

fn strings()
{
    // &str = string slice
    let s: &'static str = "hello there!";

    // cannot do
    // s = "abc";
    // println!("{}", s[0]};

    for c in s.chars().rev()
    {
        println!("{}", c);
    }

    if let Some(char) = s.chars().nth(0)
    {
        println!("first char: {}", char);
    }

    // Heap - String
    let mut letters = String::new();
    let mut a = 'a' as u8;
    while a <= ('z' as u8)
    {
        letters.push(a as char);
        letters.push_str(" ");
        a = a + 1;
    }
    println!("letters = {}", letters);

    // &str <> String
    let u: &str = &letters;
    println!("u = {}", u);

    // concatenation
    // String + str
    let v = String::from("abc");
    let t = "def".to_string();
    let z = v + &t + &u;
    println!("z = {}", z);

    let mut abc = "hello world".to_string();
    abc.remove(0);
    abc.push_str("!!!");
    println!("{}", abc.replace("ello", "goodbye"));
}

fn string_format()
{
    let name = "John";
    let greeting = format!("Hi, I am {}, nice to meet you", name);
    println!("{}", greeting);

    let run = "run";
    let forest = "forest";
    let rfr = format!("{0}, {1}, {0}", run, forest);
    println!("{}", rfr);

    let info = format!("the name's {last}. {first} {last}",
                              first = "James", last = "Bond");
    println!("{}", info);

    let mixed = format!("{1} {} {0} {} {} {data}",
                               "alpha", "beta",
                                data = "delta");
    println!("{}", mixed);
}

fn number_guessing_game()
{
    let number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Enter you guess: ");
        let mut buffer = String::new();
        match stdin().read_line(&mut buffer) {
            Ok(_) => {
                let parsed = buffer.trim_end().parse::<i64>();
                match parsed {
                    Ok(guess) => {
                        if guess < 1 || guess > 100 {
                            println!("Your guess is out of range");
                        }
                        else {
                            if guess < number {
                                println!("Your guess is too low");
                            }
                            else {
                                if guess > number {
                                    println!("Your het is too high");
                                }
                                else {
                                    println!("Correct!!!!");
                                    break;
                                }
                            }
                        }
                    },
                    Err(e) =>{
                        println!("Could not read you input. {}. Try again", e);
                    }
                }
            },
            Err(_) => continue
        }
    }
}

fn main() {
    strings();
    string_format();
    number_guessing_game();
}
