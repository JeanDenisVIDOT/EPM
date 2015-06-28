extern crate getopts;
extern crate regex;
extern crate crypto;

use std::io;
use git::Git;
use blih::Blih;

mod configuration;
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

    let git = Git::new(configuration.verbose());
    let blih = Blih::new(configuration.verbose());

    // start command prompt
    let mut stdin = io::stdin();
    loop {
        printfl!("epm $> ");

        let mut user_input = String::new();
        let _ = stdin.read_line(&mut user_input);
        let command: Vec<&str> = (user_input.trim()).split(' ').collect();

        let nb_args = command[1..].len();

        match command[0] {
            "status"        => git.status(),
            "log"           => git.log(),
            "pull"          => git.pull(),
            "add"           => git.add(nb_args, &command[1..]),
            "commit"        => git.commit(),
            "push"          => git.push(nb_args, &command[1..]),
            "allin"         => git.allin(nb_args, &command[1..]),
            "clone"         => git.clone(nb_args, &command[1..]),
            "create"        => blih.create(nb_args, &command[1..]),
            "new"           => blih.new_project(nb_args, &command[1..]),
            "delete"        => blih.delete(nb_args, &command[1..]),
            "list"          => blih.list (),
            "init"          => blih.init (),
            "upload"        => blih.upload(),
            "setacl"        => blih.setacl(),
            "getacl"        => blih.getacl(),
            _               => println!("'{}': unknown command", &command[0]),
        }
    }
}
