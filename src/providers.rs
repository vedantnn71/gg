#[derive(Debug)]
pub struct Provider {
    pub name: String,
    pub short: String,
    pub search_path: String,
}

pub fn get_providers() -> Vec<Provider> {
    let providers: Vec<Provider> = vec![
        Provider {
            name: "google".to_string(),
            short: "gg".to_string(),
            search_path: "https://google.com/search?q=".to_string(),
        },
        Provider {
            name: "duckduckgo".to_string(),
            short: "ddg".to_string(),
            search_path: "https://duckduckgo.com/?q=".to_string(),
        }
    ];

    return providers;
}


