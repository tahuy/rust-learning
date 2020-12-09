#[allow(dead_code)]
#[allow(unused_imports)]
#[allow(unused_must_use)]
#[allow(unused_variables)]
#[allow(unused_assignments)]
extern crate rand;
// use rand::Rng;
use std::io::stdin;

enum State {
    Locked,
    Failed,
    Unlocked
}

fn if_statement()
{
    println!("\n\nFunction if statement!");
    let temp = 35;
    if temp > 30
    {
        println!("Really hot!");
    }
    else if temp < 10
    {
        println!("Really cold");
    }
    else
    {
        println!("Temperature is OK");
    }

    let day: &str = if temp > 20 {"Sunny day"} else {"Cloudy day"};
    println!("{}", day);
    println!("It is {}",
             if temp > 20 {"hot"} else if temp < 10 {"cold"} else {"OK"});
    println!("it is {}",
             if temp > 20 {
                 if temp > 30 {"very hot"} else {"hot"}
             }
             else {
                 if temp < 10 { "cold" } else { "OK" }
             });
}

fn while_and_loop()
{
    let mut x = 1;
    while x < 1000 {
        x *= 2;
        if x == 64
        {
            continue;
        }
        println!("x = {}", x);
    }

    let mut y = 1;
    loop {
        y *= 2;
        println!("y = {}", y);
        if y == 1 << 10
        {
            break;
        }
    }
}

fn for_loop()
{
    for x in 1..11
    {
        if x == 5
        {
            continue;
        }
        if x == 8
        {
            break;
        }
        println!("x = {}", x);
    }
    for (pos, y) in (30..41).enumerate()
    {
        println!("{}: {}", pos, y);
    }
}

fn match_statement()
{
    let country_code = 100;
    let country = match country_code {
        44       => "UK",
        46       => "Sweden",
        7        => "Russia",
        1..=1000 => "unknown",
        _        => "invalid"
    };

    println!("The country with code {} is {}", country_code, country);

    let x = true;
    let s = match x {
        true  => "yes",
        false => "no"
    };
    println!("s = {}", s);
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit  => println!("Quit"),
            Message::Write(_) => println!("Write"),
            Message::Move{x, y} => println!("Move {} and {}", x, y),
            Message::ChangeColor(_, _, _) => println!("ChangeColor")
        };
    }
}

fn match_statement_complex() {
    let m = Message::Write(String::from("hello"));
    m.call();

    let n = Message::ChangeColor(2, 5, 8);
    n.call();

    let u = Message::Quit;
    u.call();

    let v = Message::Move{x: 3, y: 7};
    v.call();
}

fn main() {
    if_statement();
    while_and_loop();
    for_loop();
    match_statement_complex();
    match_statement();

    let code: String      = String::from("1234");
    let mut state         = State::Locked;
    let mut entry: String = String::new();

    loop {
        match state {
            State::Locked => {
                let mut input: String = String::new();
                match stdin().read_line(&mut input) {
                    Ok(_) => {
                        entry.push_str(&input.trim_end());
                    }
                    Err(_) => continue
                }
                if entry == code {
                    state = State::Unlocked;
                    continue;
                }
                if !code.starts_with(&entry) {
                    state = State::Failed;
                }
            },
            State::Failed => {
                println!("FAILED");
                entry.clear();
                state = State::Locked;
                continue;
            },
            State::Unlocked => {
                println!("UNLOCKED");
                return;
            }
        }
    }
}
