use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct SummonerInfo {
    id: String,
    accountId: String,
    puuid: String,
    name: String,
    profileIconId: u32,
    revisionDate: u64,
    summonerLevel: u32,
}

pub async fn get_summoner_info(
    region: &String,
    name: &String,
    api_key: &String,
) -> Result<SummonerInfo, reqwest::Error> {
    let url = format!(
        "https://{0}.api.riotgames.com/lol/summoner/v4/summoners/by-name/{1}?api_key={2}",
        region, name, api_key
    );
    let response = reqwest::get(url).await?.json::<SummonerInfo>().await?;
    Ok(response)
}

async fn get_summoner_ranks(
    region: &String,
    name: &String,
    api_key: &String,
) -> Result<HashMap<String, String>, reqwest::Error> {
    let url = format!(
        "https://{0}.api.riotgames.com/lol/league/v4/entries/by-summoner/{1}?api_key={2}",
        region, name, api_key
    );
    let response = reqwest::get(url)
        .await?
        .json::<HashMap<String, String>>()
        .await?;

    Ok(response)
}

pub async fn get_summoner_data(
    region: String,
    name: String,
    api_key: &String,
) -> Result<SummonerInfo, String> {
    let summoner_info_result = get_summoner_info(&region, &name, api_key).await;
    if summoner_info_result.is_err() {
        let error_response = format!("{}", summoner_info_result.unwrap_err());
        return Err(error_response);
    }
    let summoner_ranks_result = get_summoner_ranks(&region, &name, api_key).await;
    if summoner_ranks_result.is_err() {
        let error_response = format!("{}", summoner_ranks_result.unwrap_err());
        return Err(error_response);
    }
    if summoner_info_result.is_err() {}
    return Ok(summoner_info_result.unwrap());
}
