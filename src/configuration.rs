use getopts::Options;
use std::env;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use regex::Regex;
use crypto::digest::Digest;
use crypto::sha2::Sha512;

pub struct Configuration {
    username: String,
    password: String,
    verbose: bool,
}

impl Configuration {
    pub fn new(username: String, password: String, verbose: bool) -> Configuration {
        Configuration {
            username: username,
            password: password,
            verbose: verbose,
        }
    }

    pub fn username(&self) -> &String {
        &self.username
    }

    pub fn password(&self) -> &String {
        &self.password
    }

    pub fn verbose(&self) -> bool {
        self.verbose
    }
}

fn parse_configuration_file(file: String) -> Option<(String, String)> {
    let file = File::open(&file)
        .unwrap_or_else(|e| {
            panic!("{}: {}", file, e)
        });

    let reader = BufReader::new(file);
    let mut len = 0;
    let regex =
        Regex::new("(?P<var>(username|password))=\"(?P<val>(.+))\"")
        .unwrap_or_else(|e| {
            panic!("Wrong regex: {}", e)
        });

    let mut username = String::new();
    let mut password = String::new();

    for line in reader.lines() {
        if len > 1 {
            panic!("Wrong configuration file")
        }

        let unwrap = line.unwrap_or_else(|e| {
            panic!("{}", e)
        });

        for cap in regex.captures_iter(&unwrap) {
            let var = cap.name("var").unwrap_or("");
            let val = cap.name("val").unwrap_or("");

            if var == "username" { username = val.to_string() }
            if var == "password" {
                let mut hasher = Sha512::new();
                hasher.input_str(val);
                password = hasher.result_str();
            }
            if username != "" && password != "" {
                return Some((username, password));
            }
        }

        len += 1;
    }

    None
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

pub fn get() -> Option<Configuration> {
    // Get arguments from command line
    let args: Vec<String> = env::args().collect();

    // Parse arguments
    let mut opts = Options::new();
    opts.optopt("c", "config_file", "set the configuration file which has to \
                                     be read", "CONFIGURATION FILE");
    opts.optflag("v", "verbose", "print everything about commands");
    opts.optflag("h", "help", "print this help menu");

    // Catch missing arguments
    let matches = opts.parse(&args[1..])
        .unwrap_or_else(|e| {
            panic!("{}", e)
        });

    // Display usage if help flag was set
    if matches.opt_present("h") {
        print_usage(&args[0], opts);
        return None;
    }

    // Set the configuration file to use
    let configuration_file = if matches.opt_present("c") {
        matches.opt_str("c").unwrap()
    } else {
        let path = env::current_exe().unwrap().parent().unwrap().to_str().unwrap().to_string();
        path.to_string() + "/deps/user.conf"
    };

    // Set verbose to display or not everything about commands
    let verbose = if matches.opt_present("v") { true } else { false };

    let user_info = parse_configuration_file(configuration_file).unwrap();

    match user_info {
        (username, password) => {
            if username == "" || password == "" {
                panic!("Wrong configuration file.")
            }
            Some(Configuration::new(username, password, verbose))
        },
    }
}
