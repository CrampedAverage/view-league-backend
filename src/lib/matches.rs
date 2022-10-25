use serde::{Deserialize, Serialize};

use crate::{types::FetchResult, SummonerGetDataQuery};

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
struct MatchInfo {
    gameCreation: u128,
    gameDuration: u64,
}

#[derive(Deserialize, Serialize, Debug)]
struct MatchInfoResponse {
    info: MatchInfo,
}

async fn get_match_info(
    query: &SummonerGetDataQuery,
    match_id: &String,
    api_key: &String,
) -> FetchResult<MatchInfoResponse> {
    let url = format!(
        "https://{0}.api.riotgames.com/lol/match/v5/matches/{1}?api_key={2}",
        query.continent, match_id, api_key
    );
    println!("{}", url);
    let raw_response = reqwest::get(url).await?.text().await?;
    let response = serde_json::from_str::<MatchInfoResponse>(&raw_response)?;
    return Ok(response);
}

pub async fn get_summoner_matches(
    query: &SummonerGetDataQuery,
    puuid: &String,
    api_key: &String,
) -> FetchResult<Vec<String>> {
    println!("hello");
    let url = format!(
        "https://{0}.api.riotgames.com/lol/match/v5/matches/by-puuid/{1}/ids?start=0&count={2}&api_key={3}",
        query.continent, puuid, 1,api_key
    );
    let raw_response = reqwest::get(url).await?.text().await?;
    let response = serde_json::from_str::<Vec<String>>(&raw_response)?;
    for match_id in &response {
        let match_info_result = get_match_info(query, match_id, api_key).await;

        println!("{:#?}", match_info_result.unwrap())
    }
    Ok(response)
}
