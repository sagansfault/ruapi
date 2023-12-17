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

    let matchups = matchup::load_matchups(matchup::MatchupChart::TopHundred).await.unwrap();
    println!("Matchup test: GO vs PO = {}", matchups.get_matchup(&character::GOLDLEWIS, &character::POTEMKIN).unwrap());

    let mut tierlist: Vec<(String, f64)> = vec![];
    for (character, mus) in matchups.matchups {
        let count = mus.len() as f64;
        let sum = mus.values().sum::<f64>();
        let avg_winrate = sum / count;
        tierlist.push((character.readablename.clone(), avg_winrate));
    }
    tierlist.sort_by(|(_, b), (_, d)| b.partial_cmp(d).unwrap());
    tierlist.reverse();
    let tierlist = tierlist.into_iter()
        .enumerate()
        .map(|(ind, (c, f))| format!("{}. {} ({:.1}%)", ind + 1, c, f))
        .collect::<Vec<String>>()
        .join("\n");
    println!("Tierlist from average winrates:\n{}", tierlist);
}