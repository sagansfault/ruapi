pub mod rating;
pub mod character;
pub mod parser;
pub mod error;
pub mod matchup;

#[tokio::test]
async fn test() {
    let player_data = rating::player_lookup_character("sagan", &character::GOLDLEWIS).await.unwrap();
    let match_history = rating::load_match_history_id(&player_data.id, &character::GOLDLEWIS).await.unwrap();
    println!("Match history test: {:?}", match_history.first().unwrap());

    let matchup = matchup::load_matchups(matchup::MatchupChart::TopHundred).await.unwrap();
    println!("Matchup test: GO vs PO = {}", matchup.get_matchup(&character::GOLDLEWIS, &character::POTEMKIN).unwrap());
}