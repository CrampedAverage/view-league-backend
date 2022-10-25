use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    error,
    fmt::{self},
};

use crate::SummonerGetDataQuery;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct SummonerInfoResponse {
    id: String,
    accountId: String,
    puuid: String,
    name: String,
    profileIconId: u32,
    revisionDate: u64,
    summonerLevel: u32,
}

#[derive(Deserialize, Serialize, Debug)]
struct StatusBody {
    message: String,
    status_code: u16,
}

#[derive(Deserialize, Serialize, Debug)]
struct Status {
    status: StatusBody,
}

// struct SummonerRanks {}
// type StringOrInteger = String | U32;
#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
struct SummonerRankInfo {
    leagueId: String,
    queueType: String,
    tier: String,
    rank: String,
    summonerId: String,
    summonerName: String,
    leaguePoints: u8,
    wins: u32,
    losses: u32,
    veteran: bool,
    inactive: bool,
    freshBlood: bool,
    hotStreak: bool,
}

#[derive(Debug, Clone)]
struct GetSummonerRanksError {}

impl fmt::Display for GetSummonerRanksError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid something broke")
    }
}

type FetchResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
enum SummonerRanksResponse {
    Ranks(Vec<SummonerRankInfo>),
    Error(Status),
}

async fn get_summoner_info(
    query: &SummonerGetDataQuery,
    api_key: &String,
) -> Result<SummonerInfoResponse, reqwest::Error> {
    let url = format!(
        "https://{0}.api.riotgames.com/lol/summoner/v4/summoners/by-name/{1}?api_key={2}",
        query.region, query.summoner_name, api_key
    );
    let response = reqwest::get(url)
        .await?
        .json::<SummonerInfoResponse>()
        .await?;
    Ok(response)
}

async fn get_summoner_ranks(
    query: &SummonerGetDataQuery,
    id: &String,
    api_key: &String,
) -> FetchResult<SummonerRanksResponse> {
    let url = format!(
        "https://{0}.api.riotgames.com/lol/league/v4/entries/by-summoner/{1}?api_key={2}",
        query.region, id, api_key
    );
    let raw_response = reqwest::get(url).await?.text().await?;
    let response = serde_json::from_str::<SummonerRanksResponse>(&raw_response)?;

    Ok(response)
}

async fn get_summoner_matches(
    query: &SummonerGetDataQuery,
    puuid: &String,
    api_key: &String,
) -> FetchResult<Value> {
    println!("hello");
    let url = format!(
        "https://{0}.api.riotgames.com/lol/match/v5/matches/by-puuid/{1}/ids?start=0&count={2}&api_key={3}",
        query.continent, puuid, 5,api_key
    );
    let raw_response = reqwest::get(url).await?.text().await?;
    let response = serde_json::from_str::<Value>(&raw_response)?;

    Ok(response)
}

pub async fn get_summoner_data(
    query: SummonerGetDataQuery,
    api_key: &String,
) -> Result<SummonerInfoResponse, String> {
    let summoner_info_result = get_summoner_info(&query, api_key).await;
    if summoner_info_result.is_err() {
        let error_response = format!("Error: {}", summoner_info_result.unwrap_err());
        return Err(error_response);
    }
    let summoner_info = summoner_info_result.unwrap();

    let summoner_ranks_result = get_summoner_ranks(&query, &summoner_info.id, api_key).await;
    if summoner_ranks_result.is_err() {
        let error_response = format!("Error: {}", summoner_ranks_result.unwrap_err());
        return Err(error_response);
    }
    let summoner_ranks = summoner_ranks_result.unwrap();

    let summoner_matches_result = get_summoner_matches(&query, &summoner_info.puuid, api_key).await;
    if summoner_matches_result.is_err() {
        let error_response = format!("Error: {}", summoner_matches_result.unwrap_err());
        return Err(error_response);
    }

    return Ok(summoner_info);
}
