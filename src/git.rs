use std::process::Command;
use configuration::get;
use std::env;
use std::io;
use blih::Blih;

macro_rules! printfl {
     ($($tt:tt)*) => {{
             use std::io::Write;
             print!($($tt)*);
         ::std::io::stdout().flush().ok().expect("flush() fail");
     }}
}

pub struct Git {
    username: String,
    host: String,
    verbose: bool,
}

impl Git {
    pub fn new(verbose: bool) -> Git {
        let configuration = get().unwrap();

        Git {
            username: configuration.username().clone(),
            host: "git.epitech.eu".to_string(),
            verbose: verbose,
        }
    }

    pub fn init(&self) {
        if self.verbose { println!("--verbose: git init") }
        let command = Command::new("git")
            .arg("init")
            .output()
            .unwrap_or_else(|e| {
                panic!("Unable to execute git: {}", e)
            });

        print!("{}", String::from_utf8_lossy(&command.stdout));
        print!("{}", String::from_utf8_lossy(&command.stderr));

        let dir = env::current_dir().unwrap()
            .file_name().unwrap()
            .to_str().unwrap()
            .to_string();

        let handler = Blih::new(self.verbose);
        let request = "repository create ".to_string() + &dir;

        handler.request(request);

        let url = self.username.clone() + "@" + &self.host.clone() + ":/" +
            &self.username.clone() + "/" + &dir;

        if self.verbose { println!("--verbose: git remote add origin {}", &url) }
        let command = Command::new("git")
            .arg("remote")
            .arg("add")
            .arg("origin")
            .arg(url)
            .output()
            .unwrap_or_else(|e| {
                panic!("Unable to execute git: {}", e)
            });

        print!("{}", String::from_utf8_lossy(&command.stdout));
        print!("{}", String::from_utf8_lossy(&command.stderr));
    }

    fn status(&self) {
        if self.verbose { println!("--verbose: git status") }
        let command = Command::new("git")
            .arg("status")
            .output()
            .unwrap_or_else(|e| {
                panic!("Unable to execute git: {}", e)
            });

        print!("{}", String::from_utf8_lossy(&command.stdout));
        print!("{}", String::from_utf8_lossy(&command.stderr));
    }

    fn log(&self) {
        if self.verbose { println!("--verbose: git log") }
        let command = Command::new("git")
            .arg("log")
            .output()
            .unwrap_or_else(|e| {
                panic!("Unable to execute git: {}", e)
            });

        print!("{}", String::from_utf8_lossy(&command.stdout));
        print!("{}", String::from_utf8_lossy(&command.stderr));
    }

    fn pull(&self) {
        if self.verbose { println!("--verbose: git pull") }
        let command = Command::new("git")
            .arg("pull")
            .output()
            .unwrap_or_else(|e| {
                panic!("Unable to execute git: {}", e)
            });

        print!("{}", String::from_utf8_lossy(&command.stdout));
        print!("{}", String::from_utf8_lossy(&command.stderr));
    }

    fn commit(&self) {
        if self.verbose { println!("--verbose: git commit") }
        printfl!("Commit message: ");
        let mut commit_message = String::new();
        let mut stdin = io::stdin();
        let _ = stdin.read_line(&mut commit_message);

        let command = Command::new("git")
            .arg("commit")
            .arg("-m")
            .arg(commit_message.trim())
            .output()
            .unwrap_or_else(|e| {
                panic!("Unable to execute git: {}", e)
            });

        print!("{}", String::from_utf8_lossy(&command.stdout));
        print!("{}", String::from_utf8_lossy(&command.stderr));
    }

    fn add(&self, nb_args: usize, args: &[&str]) {
        if nb_args == 0 {
            if self.verbose { println!("--verbose: git add --all") }
            let command = Command::new("git")
                .arg("add")
                .arg("--all")
                .output()
                .unwrap_or_else(|e| {
                    panic!("Unable to execute git: {}", e)
                });

            print!("{}", String::from_utf8_lossy(&command.stdout));
            print!("{}", String::from_utf8_lossy(&command.stderr));
            return;
        }
        for elem in args.iter() {
            if self.verbose { println!("--verbose: git add {}", elem) }
            let command = Command::new("git")
                .arg("add")
                .arg(elem)
                .output()
                .unwrap_or_else(|e| {
                    panic!("Unable to execute git: {}", e)
                });

            print!("{}", String::from_utf8_lossy(&command.stdout));
            print!("{}", String::from_utf8_lossy(&command.stderr));
        }
    }

    fn push(&self, nb_args: usize, args: &[&str]) {
        if nb_args != 2 {
            println!("Usage: push [remote] [branch]");
            return;
        }

        if self.verbose { println!("--verbose: git push {} {}",
                                   &args[0], &args[1]) }

        let command = Command::new("git")
            .arg("push")
            .arg(args[0])
            .arg(args[1])
            .output()
            .unwrap_or_else(|e| {
                panic!("Unable to execute git: {}", e)
            });

        print!("{}", String::from_utf8_lossy(&command.stdout));
        print!("{}", String::from_utf8_lossy(&command.stderr));
    }

    fn allin(&self, nb_args: usize, args: &[&str]) {
        if nb_args != 2 && nb_args != 0 {
            println!("Usage: allin [remote] [branch]");
            return;
        }

        self.add(0, &args);
        self.commit();
        if nb_args == 0 {
            self.push(2, &vec!["origin", "master"][0..]);
        } else {
            self.push(nb_args, args);
        }
    }

    fn clone(&self, nb_args: usize, args: &[&str]) {
        if nb_args < 1 && nb_args > 2 {
            println!("Usage: clone (user) [repository_name]");
            return;
        }

        let url = if nb_args == 1 {
            self.username.clone() + "@" + &self.host.clone() + ":/" +
                &self.username.clone() + "/" + &args[0]
        } else {
            self.username.clone() + "@" + &self.host.clone() + ":/" + &args[0] +
                "/" + &args[1]
        };

        if self.verbose { println!("--verbose: git clone {}", &url) }

        let command = Command::new("git")
            .arg("clone")
            .arg(url)
            .output()
            .unwrap_or_else(|e| {
                panic!("Unable to execute git: {}", e)
            });

        print!("{}", String::from_utf8_lossy(&command.stdout));
        print!("{}", String::from_utf8_lossy(&command.stderr));
    }

}

pub fn actions(verbose: bool, nb_args: usize, args: &Vec<&str>) -> bool {
    let handler = Git::new(verbose);

    match args[0] {
        "status"        => { handler.status(); true },
        "log"           => { handler.log(); true },
        "pull"          => { handler.pull(); true },
        "add"           => { handler.add(nb_args, &args[1..]); true },
        "commit"        => { handler.commit(); true },
        "push"          => { handler.push(nb_args, &args[1..]); true },
        "allin"         => { handler.allin(nb_args, &args[1..]); true },
        "clone"         => { handler.clone(nb_args, &args[1..]); true },
        _               => false,
    }
}
