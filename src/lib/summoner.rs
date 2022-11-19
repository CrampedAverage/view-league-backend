use crate::types::{
    FetchResult, SummonerGetDataQuery, SummonerInfoResponse, SummonerRanksResponse,
};

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
