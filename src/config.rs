use std::{path::PathBuf, io::Write, process::exit, fs::{read_to_string, File}, env::{consts, var_os}};

pub struct Provider {
    pub name: String,
    pub short: String,
    pub search_path: String,
}

pub fn create_config() {
    let default_providers: Vec<Provider> = vec![
        Provider {
            name: "google".to_string(),
            short: "go".to_string(),
            search_path: "https://google.com/search?q=".to_string(),
        },
        Provider {
            name: "duckduckgo".to_string(),
            short: "ddg".to_string(),
            search_path: "https://duckdcukgo.com/search?q=".to_string(),
        },
        Provider {
            name: "youtube".to_string(),
            short: "yt".to_string(),
            search_path: "https://youtube.com/results?search_query=".to_string(),
        },
        Provider {
            name: "reddit".to_string(),
            short: "rd".to_string(),
            search_path: "https://reddit.com/search?q=".to_string(),
        },
        Provider {
            name: "wikipedia".to_string(),
            short: "wiki".to_string(),
            search_path: "https://wikipedia.org/wiki/Special:Search?search=".to_string(),
        },
        Provider {
            name: "github".to_string(),
            short: "gh".to_string(),
            search_path: "https://github.com/search?q=".to_string(),
        },
        Provider {
            name: "stackoverflow".to_string(),
            short: "so".to_string(),
            search_path: "https://stackoverflow.com/search?q=".to_string(),
        },
    ];

    let config_path = get_config_path();

    if config_path.exists() {
        return;
    }

    let mut config_file = File::create(config_path).unwrap();

    for provider in default_providers {
        let provider_string = format!(
            "\n[{}]\nshort = '{}'\nsearch_path = '{}'\n", 
            provider.name, provider.short, provider.search_path);

        let result = config_file.write_all(provider_string.as_bytes());

        if result.is_err() {
            println!("Unable to write to config file");
            exit(1);
        }
    }
}

pub fn get_providers() -> Vec<Provider> {
    let config_path = get_config_path();

    if !config_path.exists() {
        panic!("No config file found");
    }

    let file = read_to_string(config_path) .expect("Whoops unable to read the config file");
    let parse = file.parse::<toml::Value>().unwrap();
    let providers = parse.as_table().unwrap();
    let mut provider_list: Vec<Provider> = Vec::new();

    for (key, value) in providers {
        let short = value.get("short").unwrap().as_str().unwrap();
        let search_path = value.get("search_path").unwrap().as_str().unwrap();

        provider_list.push(Provider {
            name: key.to_string(),
            short: short.to_string(),
            search_path: search_path.to_string(),
        });
    }

    return provider_list;
}

fn get_config_path() -> PathBuf {
    let mut config_path: PathBuf;

    match consts::OS {
        "windows" => {
            config_path = var_os("APP_DATA").map(PathBuf::from).expect("No APP_DATA directory found");
        }
        &_ => {
            config_path = var_os("HOME").map(PathBuf::from).expect("No HOME directory found");
        }
    }
    
    config_path.push(".gg");
    config_path.set_extension("toml");

    return config_path;
}

