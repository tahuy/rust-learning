#[allow(dead_code)]
#[allow(unused_imports)]
#[allow(unused_must_use)]
#[allow(unused_variables)]
#[allow(unused_assignments)]

extern crate structopt;
use structopt::StructOpt;

extern crate colored;
use colored::*;

extern crate failure;
use failure::ResultExt;

extern crate exitfailure;
use exitfailure::ExitFailure;

use std::io::{self, Read};

#[derive(StructOpt)]
struct Options {
    #[structopt(default_value = "Meow!")]
    /// What does the cat say?
    message: String,

    #[structopt(short = "d", long = "dead")]
    /// Make the cat appear dead
    dead: bool,

    // By default, StructOpt use from_str: fn(&str) -> T
    // In this case, we parse path that be represented from different OS
    // So we need to parse from an &OsStr
    #[structopt(short = "f", long = "file", parse(from_os_str))]
    /// Load the cat picture from specified file
    // catfile is wrapped in an Option<T> -> Optional
    catfile: Option<std::path::PathBuf>,

    #[structopt(short = "i", long = "stdin")]
    /// Read the message from STDIN instead of the argument
    stdin: bool,
}


// fn main() -> Result<(), Box<dyn std::error::Error>>
// fn main() -> Result<(), failure::Error>
fn main() -> Result<(), ExitFailure>
{
    let options = Options::from_args();
    // let message = options.message;
    let mut message = String::new();
    if options.stdin {
        io::stdin().read_to_string(&mut message)?;
    } else {
        message = options.message;
    }
    let eye = if options.dead { "x".red().bold() } else { "o".green().bold() };
    if message.to_lowercase() == "woof" {
        eprintln!("A cat shouldn't bark like a dog.");
    }
    else {
        println!("{}", message.bright_yellow().underline().on_purple());
        match &options.catfile {
            Some(path) => {
                // Using unwrap() or expect() for Error Handling:
                //     -> cause the program to crash with panic!()
                //     -> lose the ability to recover from failure
                /* let cat_template = std::fs::read_to_string(path)
                     .expect(&format!("Could not read file {:?}", path)); */

                /*let cat_template = match std::fs::read_to_string(path)
                {
                    Ok(file_content) => file_content,
                    Err(_) => return,
                };*/
                // ? operator equivalent to match
                // let cat_template =  std::fs::read_to_string(path)?;
                let cat_template =  std::fs::read_to_string(path)
                    .with_context(|_| format!("Could not read file {:?}", path))?;

                /*
                 We can't use format!() in this case, because it needs to know
                 the formatting string at compile time -> use replace()
                */
                let cat_picture = cat_template.replace("{eye}", &eye);
                println!("{}", &cat_picture);
            }
            None => {
                println!(" \\");
                println!("  \\");
                println!("     /\\_/\\");
                println!("    ( {eye} {eye} )", eye=eye);
                println!("    =( I )=");
            }
        }
    }
    Ok(())
}
