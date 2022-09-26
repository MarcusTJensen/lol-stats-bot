use std::{collections::HashMap, error::Error};
use serde::{Serialize, Deserialize};

pub async fn get_champs() ->  Result<Data, Box<dyn Error>> {
    let res = reqwest::get("https://ddragon.leagueoflegends.com/cdn/12.18.1/data/en_US/champion.json")
        .await?
        .text()
        .await?;
    let champ_list: Data = serde_json::from_str(&res).unwrap();
    Ok(champ_list)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    data: HashMap<String, ChampId>
}

#[derive(Debug, Serialize, Deserialize)]
struct ChampId {
    key: String
}

