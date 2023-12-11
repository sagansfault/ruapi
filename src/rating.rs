use std::error;

use serde::{Deserialize, Serialize};

use crate::parser;
use crate::character::Character;
use crate::error::Error::NoPlayerFound;

const URL: &str = "http://ratingupdate.info";

pub async fn player_lookup(name: &str) -> Result<Vec<PlayerLookupDataAll>, Box<dyn error::Error>> {
    let url = format!("{}/api/player_lookup?name={}", URL, name);
    let body = reqwest::get(url).await?.text().await?;
    let res: Vec<PlayerLookupDataAll> = serde_json::from_str(&body)?;
    Ok(res)
}

pub async fn player_lookup_character(name: &str, character: &Character) -> Result<PlayerLookupDataSingle, Box<dyn error::Error>> {
    let url = format!("{}/api/player_lookup?name={}", URL, name);
    let body = reqwest::get(url).await?.text().await?;
    let res: Vec<PlayerLookupDataAll> = serde_json::from_str(&body)?;
    for player in res {
        for character_data in player.characters {
            if character_data.shortname.eq_ignore_ascii_case(&character.shortname) {
                return Ok(PlayerLookupDataSingle {
                    id: player.id,
                    name: player.name,
                    character: character_data,
                });
            }
        }
    }
    None.ok_or(NoPlayerFound)?
}

pub async fn load_match_history_id(id: &str, character: &Character) -> Result<Vec<RecentGame>, Box<dyn error::Error>> {
    let url = format!("{}/player/{}/{}/history?offset=0", URL, id, character.shortname);
    let body = reqwest::get(url).await?.text().await?;
    let parse = parser::parse_search_recent_games(body);
    Ok(parse)
}

pub async fn load_match_history_name(name: &str, character: &Character) -> Result<Vec<RecentGame>, Box<dyn error::Error>> {
    let player_lookup = player_lookup_character(name, character).await?;
    load_match_history_id(&player_lookup.id, character).await
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerLookupDataAll {
    pub id: String,
    pub name: String,
    pub characters: Vec<CharacterData>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerLookupDataSingle {
    pub id: String,
    pub name: String,
    pub character: CharacterData
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CharacterData {
    pub shortname: String,
    pub rating: usize,
    pub deviation: usize,
    pub game_count: usize
}

#[derive(Debug, Clone)]
pub struct RecentGame {
    pub date: String,
    pub rating: String,
    pub floor: String,
    pub opponent: String,
    pub opponent_character: String,
    pub opponent_rating: String,
    pub odds: String,
    pub result: String,
    pub rating_change: String
}