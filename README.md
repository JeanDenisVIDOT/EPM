# EPM
: Epitech Project Manager
Written in Rust

## Installation
Firsly, you have to install [rust]:
```bash
git clone https://github.com/rust-lang/rust
cd rust
./configure
make
sudo make install
```

Now, you can clone this project:
```bash
git clone https://github.com/JeanDenisVIDOT/epm
cd epm
cargo build --release
```

And configure it with your favourite text editor, for me it's emacs:
```bash
cp user.conf.skel target/release/deps/user.conf
emacs target/release/deps/user.conf
```

You can run the program like this:
```bash
target/release/epm
```

And if you want to use it everywhere you are:
```bash
sudo ln -s target/release/epm /usr/bin/epm
```

## How to use it
### Options

* '-c' or '--config_file': set a specifil configuration file (default path: /path/to/executable/deps/user.conf)
* '-v' or '--verbose': print everything about commands

### Actions
* create: create a repository on blih
* delete: delete a repository on blih
* list: display repositories on blih
* upload: upload the default sshkey on your computer to the blih server
* setacl: set a new right for an user to your current project
* getacl: get actual rights on the project
* init: a git init like for blih + git remote add origin [user]@git.epitech.eu:/[user]/repository
* status: git status
* log: git log
* pull: git pull
* add: add --all (if no arguments) or git add file1 .. fileN
* commit: git commit -m
* push: git push [remote] [branch]
* allin: git pull, git add --all, git commit -m, git push [remote] [branch]
* clone: git clone [user]@git.epitech.eu:/(user)/[repository_name]

## Examples
### New project
```bash
mkdir new_project
cd new_project
/usr/bin/epm
epm $> init
epm $> setacl
Give rights to a colleague? [y/N] y
Colleague name: sendra_a
Rights to give to 'sendra_a': rw
ACL correctly applied
Give rights to a colleague? [y/N] n
emp $> getacl
sendra_a:rw
epm $> ^C (Control + C)
echo "# Example" > README.md
/usr/bin/epm
epm $> status
epm $> allin origin master
Commit message: toto
epm $> ^C (Control + C)
```

[rust]: https://github.com/rust-lang/rust
