# ruapi
A webscraper and api for retrieving data from [ratingupdate.info](https://ratingupdate.info)

### Player Data
```rust
// Lookup player data given a player query.
let player_lookup_all = player_lookup("sagan")
    .await
    .unwrap();

// Lookup player data excluding players who do not have data of the given character
let player_lookup_character = player_lookup_character("sagan", &character::GOLDLEWIS)
    .await
    .unwrap();

// Load recent data based on a player id from a prior player_lookup call
let recent_games = load_match_history_id(&player_lookup_character.id, &character::GOLDLEWIS)
    .await
    .unwrap();

// Runs the same as recent_games but calls player_lookup_character first for the player id
let recent_games = load_match_history_name("sagan", &character::GOLDLEWIS)
.await
.unwrap();


// A recommended approach would look something like:
let player_data = player_lookup_character("sagan", &character::GOLDLEWIS).await?;
let player_id = player_data.id;
let recent_games = load_match_history_id(&player_id, &character::GOLDLEWIS).await?;
```

### Matchup Data (top 1000)
```rust
// load the matchup data
let matchup = matchup::load_matchups().await.unwrap();
// winrate of Goldlewis vs Potemkin as a percentage
let winrate = matchup.get_matchup(&character::GOLDLEWIS, &character::POTEMKIN).unwrap();
```