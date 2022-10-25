use std::fmt;

use serde::{Deserialize, Serialize};

use crate::{
    types::{FetchResult, Status},
    SummonerGetDataQuery,
};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct SummonerInfoResponse {
    pub id: String,
    accountId: String,
    pub puuid: String,
    name: String,
    profileIconId: u32,
    revisionDate: u64,
    summonerLevel: u32,
}

// struct SummonerRanks {}
// type StringOrInteger = String | U32;
#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
pub struct SummonerRankInfo {
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

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum SummonerRanksResponse {
    Ranks(Vec<SummonerRankInfo>),
    Error(Status),
}

#[derive(Debug, Clone)]
struct GetSummonerRanksError {}

impl fmt::Display for GetSummonerRanksError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid something broke")
    }
}

pub async fn get_summoner_info(
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

pub async fn get_summoner_ranks(
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
