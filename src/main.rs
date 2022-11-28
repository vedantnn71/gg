use std::process::exit;
use gg::{args::parse_args, help::help, search::search, config::{create_config, get_providers}};

fn main() {
    create_config();

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

    println!("Searching for \"{}\" using {}...", args.search_term, provider.name);

    search(&provider.search_path, &args.url_safe_term);
}


