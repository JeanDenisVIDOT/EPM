extern crate getopts;
extern crate regex;
extern crate crypto;

use std::io;

mod configuration;
mod actions;
mod blih;
mod git;

macro_rules! printfl {
     ($($tt:tt)*) => {{
             use std::io::Write;
             print!($($tt)*);
         ::std::io::stdout().flush().ok().expect("flush() fail");
     }}
}

fn main() {
    // get data into the configuration file (can be set with the flag -c)
    let configuration = configuration::get().unwrap();

    if configuration.verbose() {
        println!("Verbose mode actived.");
    }

    // start command prompt
    let mut stdin = io::stdin();
    loop {
        printfl!("epm $> ");

        let mut user_input = String::new();
        let _ = stdin.read_line(&mut user_input);
        let command: Vec<&str> = (user_input.trim()).split(' ').collect();

        actions::manage(configuration.verbose(), command);
    }

}
