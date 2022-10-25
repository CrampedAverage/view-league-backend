pub fn get_api_key() -> String {
    let api_key = std::env::var("RIOT_API_KEY").expect("Failed to load riot api key in env");
    return api_key;
}
