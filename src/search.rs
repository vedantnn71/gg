use std::env::consts::OS;
use std::process::{Command, Output};

pub fn search(search_path: &String, term: &String) -> Output {
    let url = search_path.to_owned() + term;

    match OS {
        "windows" => {
            return open("cmd", &format!("explorer \"{}\"", url));
        }
        "macos" => {
            return open("sh", &format!("open -n \"{}\"", url));
        }
        "linux" => {
            return open("sh", &format!("xdg-open \"{}\"", url));
        }
        &_ => {
            todo!();
        }
    }
}

fn open(shell: &str, command: &str) -> Output {
    return Command::new(shell)
        .args([if shell == "cmd" { "/C" } else { "-c" }, command])
        .output()
        .expect("Oops something went wrong :((");
}
