use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    let frontend = Path::new("./frontend");
    env::set_current_dir(&frontend).unwrap();
    Command::new("npm run build");
}