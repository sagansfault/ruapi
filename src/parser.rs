use std::sync::OnceLock;

use scraper::Selector;

use crate::rating::RecentGame;

fn get_search_games_table_row_selector() -> &'static Selector {
    static TABLE_ROW_SELECTOR: OnceLock<Selector> = OnceLock::new();
    TABLE_ROW_SELECTOR.get_or_init(|| Selector::parse("body > div > table > tbody > tr").unwrap())
}
fn get_td_selector() -> &'static Selector {
    static TD_SELECTOR: OnceLock<Selector> = OnceLock::new();
    TD_SELECTOR.get_or_init(|| Selector::parse("td").unwrap())
}

fn get_a_selector() -> &'static Selector {
    static A_SELECTOR: OnceLock<Selector> = OnceLock::new();
    A_SELECTOR.get_or_init(|| Selector::parse("a").unwrap())
}

fn get_span_selector() -> &'static Selector {
    static SPAN_SELECTOR: OnceLock<Selector> = OnceLock::new();
    SPAN_SELECTOR.get_or_init(|| Selector::parse("span").unwrap())
}

pub(crate) fn parse_search_recent_games(body: String) -> Vec<RecentGame> {
    let mut result_vec: Vec<RecentGame> = vec![];
    let document = scraper::Html::parse_document(&body);
    let mut table_rows = document.select(get_search_games_table_row_selector());
    table_rows.next(); // first one is empty
    for next in table_rows {
        let mut element = next.select(get_td_selector());
        let Some(date) = element.next()
            .map(|e| e.inner_html())
            .map(sanitize) else {
            continue;
        };
        let Some(_rating) = element.next()
            .map(|e| e.inner_html())
            .map(sanitize) else {
            continue;
        };
        let Some(floor) = element.next()
            .map(|e| e.inner_html())
            .map(sanitize) else {
            continue;
        };
        let Some(opponent) = element.next()
            .and_then(|e| e.select(get_a_selector()).next())
            .map(|e| e.inner_html())
            .map(sanitize) else {
            continue;
        };
        let Some(opponent_character) = element.next()
            .map(|e| e.inner_html())
            .map(sanitize) else {
            continue;
        };
        let Some(opponent_rating) = element.next()
            .map(|e| e.inner_html())
            .map(sanitize) else {
            continue;
        };
        let Some(odds) = element.next()
            .map(|e| e.inner_html())
            .map(sanitize) else {
            continue;
        };
        let Some(result) = element.next()
            .and_then(|e| e.select(get_span_selector()).next())
            .map(|e| e.inner_html())
            .map(sanitize) else {
            continue;
        };
        let Some(rating_change) = element.next()
            .and_then(|e| e.select(get_span_selector()).next())
            .map(|e| e.inner_html())
            .map(sanitize) else {
            continue;
        };
        result_vec.push(RecentGame {
            date,
            floor,
            opponent,
            opponent_character,
            opponent_rating,
            odds,
            result,
            rating_change,
        });
    }
    result_vec
}

fn sanitize(st: String) -> String {
    st.replace('\n', "").trim().to_string()
}