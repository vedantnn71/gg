use gg::{
    args::parse_args,
    config::{create_config, get_providers},
    help::help,
    search::search,
};
use std::process::exit;
use spinach::Spinach;

fn main() {
    let config = create_config();
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
    let spinner = Spinach::new(format!("Searching for {} with {}...", args.search_term, provider.name));

    if config.is_err() {
        spinner.fail("Failed to create config file");
        exit(1);
    }

    let search_output = search(&provider.search_path, &args.url_safe_term);

    if search_output.is_err() {
        spinner.fail("Failed to open browser");
        exit(1);
    }

    spinner.succeed("Done!");
}
