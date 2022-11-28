use crate::help::help;
use urlencoding::encode;
use std::{env::args, process::exit};

pub struct Args {
    pub provider: String,
    pub search_term: String,
    pub url_safe_term: String,
}

pub fn parse_args() -> Args {
    let provider = args().nth(1).unwrap_or_else(|| {
        help();
        exit(1);
    });

    let search_term = args().skip(2).collect::<Vec<String>>();

    if search_term.is_empty() {
        help();
        exit(1);
    }

    let search_term = search_term.join(" ");
    let url_safe_term = encode(&search_term).to_string();

    return Args {
        provider,
        search_term,
        url_safe_term,
    };
}
