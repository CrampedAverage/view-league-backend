use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::{json, to_string};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct ChampionImage {
    full: String,
    w: u8,
    h: u8,
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

pub async fn get_champions() -> Result<String, reqwest::Error> {
    let url = "http://ddragon.leagueoflegends.com/cdn/11.8.1/data/en_US/champion.json";
    let response = reqwest::get(url).await?.json::<ResponseObject>().await?;
    let serialized_response = json!(response).to_string();
    return Ok(serialized_response);
}
