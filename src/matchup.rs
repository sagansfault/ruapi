use std::collections::HashMap;

use enum_ordinalize::Ordinalize;
use once_cell::sync::Lazy;
use regex::Regex;
use scraper::Selector;

use crate::character;
use crate::character::Character;

const DOMAIN: &str = "http://ratingupdate.info/matchups";

static TABLE_SELECTOR: Lazy<Selector> = Lazy::new(|| {
    Selector::parse("body > section.section > div > div > div > table").unwrap()
});
static ROW_SELECTOR: Lazy<Selector> = Lazy::new(|| {
    Selector::parse("tbody > tr").unwrap()
});
static TH_SELECTOR: Lazy<Selector> = Lazy::new(|| Selector::parse("th").unwrap());
static TD_SELECTOR: Lazy<Selector> = Lazy::new(|| Selector::parse("td").unwrap());

static CHARACTER_REGEX_MATCHER: Lazy<Regex> = Lazy::new(|| Regex::new(r"vs (\S{2})").unwrap());
static WINRATE_REGEX_MATCHER: Lazy<Regex> = Lazy::new(|| Regex::new(r"Raw: (\S{2,4})%").unwrap());

pub async fn load_matchups<'a>(matchup_chart: MatchupChart) -> Result<MatchupData<'a>, Box<dyn std::error::Error>> {
    let mut matchups: HashMap<&'a Character, HashMap<&'a Character, f64>> = HashMap::new();

    let res = reqwest::get(DOMAIN).await?.text().await?;
    let document = scraper::Html::parse_document(&res);

    let mut tables = document.select(&TABLE_SELECTOR);
    // skip tables until we get to the one we want
    for _ in 0..matchup_chart.ordinal() {
        tables.next();
    }
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
            let Some(info_str) = matchup.attr("title") else {
                continue;
            };
            let Some(versus) = CHARACTER_REGEX_MATCHER.captures(info_str)
                .and_then(|caps| caps.get(1)) // first match (0) is always the whole match
                .map(|cap| cap.as_str())
                .and_then(|s| character::get_character_short(String::from(s))) else {
                continue;
            };
            let Some(winrate) = WINRATE_REGEX_MATCHER.captures(info_str)
                .and_then(|caps| caps.get(1)) // first match (0) is always the whole match
                .map(|cap| cap.as_str())
                .and_then(|s| s.parse::<f64>().ok()) else {
                continue;
            };
            matchups.entry(character).or_default().insert(versus, winrate);
        }
    }

    Ok(MatchupData{ matchups })
}

#[derive(Ordinalize)]
pub enum MatchupChart {
    Global, TopThousand, Proportional, TopHundred
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