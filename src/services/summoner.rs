use serde::{Deserialize, Serialize};

use crate::lib::{matches, summoner};
use crate::SummonerGetDataQuery;

#[derive(Serialize, Deserialize, Debug)]
pub struct SummonerData {
    info: summoner::SummonerInfoResponse,
    ranks: summoner::SummonerRanksResponse,
    matches: Vec<matches::MatchInfoResponse>,
}

#[allow(unused_variables)]
pub async fn get_summoner_data(
    query: SummonerGetDataQuery,
    api_key: &String,
) -> Result<SummonerData, String> {
    let summoner_info_result = summoner::get_summoner_info(&query, api_key).await;
    if summoner_info_result.is_err() {
        let error_response = format!("Error: {}", summoner_info_result.unwrap_err());
        return Err(error_response);
    }
    let summoner_info = summoner_info_result.unwrap();

    let summoner_ranks_result =
        summoner::get_summoner_ranks(&query, &summoner_info.id, api_key).await;
    if summoner_ranks_result.is_err() {
        let error_response = format!("Error: {}", summoner_ranks_result.unwrap_err());
        return Err(error_response);
    }
    let summoner_ranks = summoner_ranks_result.unwrap();

    let summoner_matches_result =
        matches::get_summoner_matches(&query, &summoner_info.puuid, api_key).await;
    if summoner_matches_result.is_err() {
        let error_response = format!("Error: {}", summoner_matches_result.unwrap_err());
        return Err(error_response);
    }
    let summoner_matches = summoner_matches_result.unwrap();

    let summoner_data = SummonerData {
        info: summoner_info,
        ranks: summoner_ranks,
        matches: summoner_matches,
    };
    return Ok(summoner_data);
}
