use std::process::{exit, Command};
use gg::providers::get_providers;
use gg::args::parse_args;
use gg::help::help;

fn main() {
    let providers = get_providers();
    let args = parse_args();
    
    let provider = providers
        .iter()
        .find(|&p| p.name == args.provider || p.short == args.provider);

    if provider.is_none() {
        help();
        exit(1);
    }

    let provider = provider.unwrap();
    let url = format!("{}{}", provider.search_path, args.url_safe_term);

    println!("Searching for \"{}\" using {}...", args.search_term, provider.name);

    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", &format!("explorer {}", url)])
            .output()
            .expect(&format!("Whoops, failed to open {}", url));
    } else if cfg!(target_os = "macos") {
        Command::new("sh")
            .args(["-c", &format!("open -n {}", url)])
            .output()
            .expect(&format!("Whoops failed to open {}", url));
    } else {
        Command::new("sh")
            .args(["-c", &format!("xdg-open {}", url)])
            .output()
            .expect(&format!("Whoops, failed to open {}", url));
    }
}


