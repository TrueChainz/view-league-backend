use crate::lib::{matches, summoner};
use crate::types::{SummonerData, SummonerGetDataQuery};

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

    let summoner_match_ids_result =
        matches::get_summoner_matches(&query, &summoner_info.puuid, api_key).await;
    if summoner_match_ids_result.is_err() {
        let error_response = format!("Error: {}", summoner_match_ids_result.unwrap_err());
        return Err(error_response);
    }
    let summoner_match_ids = summoner_match_ids_result.unwrap();

    let summoner_data = SummonerData {
        info: summoner_info,
        ranks: summoner_ranks,
        match_ids: summoner_match_ids,
    };
    return Ok(summoner_data);
}
