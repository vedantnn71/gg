use std::{env::consts, io::Error, process::{Command, Output}};

pub fn search(search_path: &String, term: &String) -> Result<Output, Error> {
    let url = search_path.to_owned() + term;

    match consts::OS {
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

fn open(shell: &str, command: &str) -> Result<Output, Error> {
    return Command::new(shell)
        .args([if shell == "cmd" { "/C" } else { "-c" }, command])
        .output()
}
