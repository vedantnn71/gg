use std::process::{exit, Command};
use std::env::args;

#[derive(Debug)]
struct Website {
    name: String,
    short: String,
    search_path: String,
}

fn main() {
    let websites = get_websites();
    let args = parse_args();
    
    let website = websites
        .iter()
        .find(|&w| w.name == args.website || w.short == args.website);

    if website.is_none() {
        help();
        exit(1);
    }

    let website = website.unwrap();
    let url = format!("{}{}", website.search_path, args.url_safe_term);

    println!("Searching for \"{}\" using {}...", url, website.name);

    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", &format!("explorer {}", url)])
            .output()
            .expect(&format!("Whoops, failed to open {}", url));
    } else {
        Command::new("sh")
            .args(["-c", &format!("xdg-open {}", url)])
            .output()
            .expect(&format!("Whoops, failed to open {}", url));
    }
}

fn get_websites() -> Vec<Website> {
    let websites: Vec<Website> = vec![
        Website {
            name: "google".to_string(),
            short: "gg".to_string(),
            search_path: "https://google.com/search?q=".to_string(),
        },
        Website {
            name: "duckduckgo".to_string(),
            short: "ddg".to_string(),
            search_path: "https://duckduckgo.com/?q=".to_string(),
        }
    ];

    return websites;
}

struct Args {
    website: String,
    search_term: String,
    url_safe_term: String,
}

fn parse_args() -> Args {
    let website = args().nth(1).unwrap();
    let search_term = args()
        .skip(2)
        .collect::<Vec<String>>();

    if search_term.is_empty() {
        help();
        exit(1);
    }

    let url_safe_term = search_term.join("%20");
    let search_term = search_term.join(" ");

    return Args {
        website,
        search_term,
        url_safe_term,
    }
}

fn help() {
    println!("Usage: <website> <search term>");
}

