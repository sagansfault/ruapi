use std::collections::HashMap;
use once_cell::sync::Lazy;
use scraper::Selector;
use crate::character;
use crate::character::Character;

const DOMAIN: &str = "http://ratingupdate.info/matchups";

pub static TABLE_SELECTOR: Lazy<Selector> = Lazy::new(|| {
    Selector::parse("body > section.section > div > div > div > table").unwrap()
});
pub static ROW_SELECTOR: Lazy<Selector> = Lazy::new(|| {
    Selector::parse("tbody > tr").unwrap()
});
pub static TH_SELECTOR: Lazy<Selector> = Lazy::new(|| Selector::parse("th").unwrap());
pub static TD_SELECTOR: Lazy<Selector> = Lazy::new(|| Selector::parse("td").unwrap());
pub static SPAN_SELECTOR: Lazy<Selector> = Lazy::new(|| Selector::parse("span").unwrap());


pub async fn load_matchups<'a>() -> Result<MatchupData<'a>, Box<dyn std::error::Error>> {
    let mut matchups: HashMap<&'a Character, HashMap<&'a Character, f64>> = HashMap::new();

    let res = reqwest::get(DOMAIN).await?.text().await?;
    let document = scraper::Html::parse_document(&res);

    let mut tables = document.select(&TABLE_SELECTOR);
    tables.next(); // top 1000 is second table
    let table = tables.next().unwrap();
    let mut rows = table.select(&ROW_SELECTOR);
    rows.next(); // first row is titles
    for row in rows {
        let Some(character) = row.select(&TH_SELECTOR).next()
            .map(|v| v.inner_html())
            .and_then(character::get_character_regex) else {
            continue;
        };
        for matchup in row.select(&TD_SELECTOR) {
            let Some(versus) = matchup
                .attr("title")
                .map(|s| s.split(" vs "))
                .and_then(|mut s| s.nth(1)) // looks like: "SO vs KY junk"
                .map(|s| &s[..2])
                .and_then(|s| character::get_character_short(String::from(s))) else {
                continue;
            };
            let Some(fraction) = matchup.select(&SPAN_SELECTOR).next()
                .map(|v| v.inner_html())
                .map(|v| String::from(v.trim()))
                .map(|v| v.replace('%', ""))
                .and_then(|v| v.parse::<f64>().ok()) else {
                continue;
            };
            matchups.entry(character).or_default().insert(versus, fraction);
        }
    }

    Ok(MatchupData{ matchups })
}

#[derive(Debug, Clone)]
pub struct MatchupData<'a> {
    pub matchups: HashMap<&'a Character, HashMap<&'a Character, f64>>
}

impl<'a> MatchupData<'a> {
    /// Returns the win-rate percentage of `character` against `versus`
    pub fn get_matchup(&self, character: &'a Character, versus: &'a Character) -> Option<f64> {
        self.matchups.get(character).and_then(|e| e.get(versus)).copied()
    }
}