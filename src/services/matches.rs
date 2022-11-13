use crate::lib::matches::{self, MatchInfoResponse};

pub async fn get_match_info_by_id(
    api_key: String,
    match_id: String,
    continent: String,
) -> Result<MatchInfoResponse, String> {
    let match_info_result = matches::get_match_info(&continent, match_id, &api_key).await;
    if match_info_result.is_err() {
        let error_response = format!("Error: {}", match_info_result.unwrap_err());
        return Err(error_response);
    }
    let match_info = match_info_result.unwrap();
    return Ok(match_info);
}
