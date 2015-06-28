use blih;
use git;

pub fn manage(verbose: bool, command: Vec<&str>) {
    let nb_args = command[1..].len();

    let blih = blih::actions(verbose, nb_args, &command);
    if !blih {
        let git = git::actions(verbose, nb_args, &command);
        if !git {
            println!("{}: Unknown command", command[0])
        }
    }
}
