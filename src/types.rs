use std::error;

use actix_web::{Either, HttpResponse};
use serde::{Deserialize, Serialize};

pub type FetchResult<T> = std::result::Result<T, Box<dyn error::Error>>;
#[derive(Deserialize, Serialize, Debug)]
struct StatusBody {
    message: String,
    status_code: u16,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Status {
    status: StatusBody,
}

pub type Responder = Either<HttpResponse, HttpResponse>;

#[derive(Deserialize, Debug)]
pub struct SummonerGetDataQuery {
    pub summoner_name: String,
    pub region: String,
    pub continent: String,
}

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

#[derive(Serialize, Deserialize, Debug)]
pub struct SummonerData {
    pub info: SummonerInfoResponse,
    pub ranks: SummonerRanksResponse,
    pub match_ids: Vec<String>,
}
