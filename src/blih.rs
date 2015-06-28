use std::process::Command;
use git::Git;
use configuration::get;
use std::env;
use std::ascii::AsciiExt;
use std::io;
use std::path::Path;

macro_rules! printfl {
     ($($tt:tt)*) => {{
             use std::io::Write;
             print!($($tt)*);
         ::std::io::stdout().flush().ok().expect("flush() fail");
     }}
}

pub struct Blih {
    username: String,
    password: String,
    verbose: bool,
}

impl Blih {
    pub fn new(verbose: bool) -> Blih {
        let configuration = get().unwrap();

        Blih {
            username: configuration.username().clone(),
            password: configuration.password().clone(),
            verbose: verbose,
        }
    }

    pub fn request(&self, request: String) {
        if self.verbose {
            println!("--verbose: blih {}", request)
        }

        let prepare = "-u ".to_string() + &self.username + " -t " +
            &self.password + " " + &request;

        let data: Vec<&str> = prepare.split(' ').collect();

        let command = Command::new("blih")
            .args(&data)
            .output()
            .unwrap_or_else(|e| {
                panic!("Unable to execute blih: {}", e)
            });

        print!("{}", String::from_utf8_lossy(&command.stdout));
        print!("{}", String::from_utf8_lossy(&command.stderr));
    }

    pub fn create(&self, nb_args: usize, args: &[&str]) {
        if nb_args != 1 {
            println!("Usage: create [repository_name]");
            return;
        }

        let request = "repository create ".to_string() + &args[0];
        self.request(request);
    }

    pub fn delete(&self, nb_args: usize, args: &[&str]) {
        if nb_args != 1 {
            println!("Usage: delete [repository_name]");
            return;
        }

        let request = "repository delete ".to_string() + &args[0];
        self.request(request);
    }

    pub fn list(&self) {
        let request = "repository list".to_string();
        self.request(request);
    }

    pub fn upload(&self) {
        let request = "sshkey upload".to_string();
        self.request(request);
    }

    pub fn init(&self) {
        let handler = Git::new(self.verbose);
        handler.init();
    }

    pub fn setacl(&self) {
        let dir = env::current_dir().unwrap()
            .file_name().unwrap()
            .to_str().unwrap()
            .to_string();

        let mut stdin = io::stdin();
        loop {
            printfl!("Give rights to a colleague? [y/N] ");
            let mut response = String::new();
            let _ = stdin.read_line(&mut response);

            match &*(response.trim()).to_ascii_lowercase() {
                "y" | "o" | "yes" => (),
                "n" | "no" | "non" | "" => break,
                value => {
                    println!("{}: Unknown value.", value);
                    continue;
                },
            }

            printfl!("Colleague name: ");
            let mut name = String::new();
            let _ = stdin.read_line(&mut name);

            print!("Rights to give to '{}'", &name.trim());
            printfl!(": ");
            let mut rights = String::new();
            let _ = stdin.read_line(&mut rights);

            let request = "repository setacl ".to_string() + &dir + " " +
                &name.trim() + " " + &rights.trim();
            self.request(request);
        }
    }

    pub fn getacl(&self) {
        let dir = env::current_dir().unwrap()
            .file_name().unwrap()
            .to_str().unwrap()
            .to_string();

        let request = "repository getacl ".to_string() + &dir;
        self.request(request);
    }
}
