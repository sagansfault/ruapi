pub mod rating;
pub mod character;
pub mod parser;
pub mod error;

#[tokio::test]
async fn test() {
    let player_data = rating::player_lookup_character("sagan", &character::GOLDLEWIS).await.unwrap();
    let match_history = rating::load_match_history_id(&player_data.id, &character::GOLDLEWIS).await.unwrap();
    println!("{:?}", match_history.first().unwrap());
}