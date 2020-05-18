use std::process::Command;

fn main() {
    let mut npm_run = Command::new("npm");
    npm_run.arg("run").arg("build");

    npm_run.current_dir("./frontend");

    npm_run.output().expect("Could not build React frontend");
}