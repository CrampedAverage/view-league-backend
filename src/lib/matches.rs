use serde::{Deserialize, Serialize};

use crate::types::{FetchResult, SummonerGetDataQuery};

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
struct ParticipantInfo {
    summonerName: String,
    teamId: u8,
    win: bool,
    kills: u8,
    item0: u32,
    item1: u32,
    item2: u32,
    item3: u32,
    item4: u32,
    item5: u32,
    item6: u32,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
struct MatchInfo {
    gameCreation: u64,
    gameDuration: u64,
    gameId: u64,
    gameVersion: String,
    participants: Vec<ParticipantInfo>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MatchInfoResponse {
    info: MatchInfo,
}

pub async fn get_match_info(
    continent: &String,
    match_id: String,
    api_key: &String,
) -> FetchResult<MatchInfoResponse> {
    let url = format!(
        "https://{0}.api.riotgames.com/lol/match/v5/matches/{1}?api_key={2}",
        continent, match_id, api_key
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
    let url = format!(
        "https://{0}.api.riotgames.com/lol/match/v5/matches/by-puuid/{1}/ids?start=0&count={2}&api_key={3}",
        query.continent, puuid, 10,api_key
    );
    let raw_response = reqwest::get(url).await?.text().await?;
    let response = serde_json::from_str::<Vec<String>>(&raw_response)?;
    let mut match_ids: Vec<String> = vec![];
    for match_id in &response {
        match_ids.push(match_id.to_string());
    }
    Ok(match_ids)
}
