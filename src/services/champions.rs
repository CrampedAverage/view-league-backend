use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct ChampionImage {
    full: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct ChampionData {
    version: String,
    name: String,
    id: String,
    title: String,
    image: ChampionImage,
}

#[derive(Serialize, Deserialize, Debug)]
struct ResponseObject {
    data: HashMap<String, ChampionData>,
    version: String,
    format: String,
}

async fn dto_champions(champion_api_response: &str) -> Result<String, String> {
    let full_data: ResponseObject = serde_json::from_str(champion_api_response).unwrap();
    let json_data = json!(&full_data);
    Ok(json_data.to_string())
}
pub async fn get_champions() -> Result<String, String> {
    let body =
        reqwest::get("http://ddragon.leagueoflegends.com/cdn/11.8.1/data/en_US/champion.json")
            .await
            .unwrap()
            .text()
            .await;

    let data = body.unwrap();
    return dto_champions(&*data).await;
}
