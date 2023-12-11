use once_cell::sync::Lazy;
use regex::Regex;

pub static SOL: Lazy<Character> = Lazy::new(|| Character::new("SO", 0, r"(?i)(so)"));
pub static KY: Lazy<Character> = Lazy::new(|| Character::new("KY", 1, r"(?i)(ky)"));
pub static MAY: Lazy<Character> = Lazy::new(|| Character::new("MA", 2, r"(?i)(ma)"));
pub static AXL: Lazy<Character> = Lazy::new(|| Character::new("AX", 3, r"(?i)(ax)"));
pub static CHIPP: Lazy<Character> = Lazy::new(|| Character::new("CH", 4, r"(?i)(chip)"));
pub static POTEMKIN: Lazy<Character> = Lazy::new(|| Character::new("PO", 5, r"(?i)(pot)"));
pub static FAUST: Lazy<Character> = Lazy::new(|| Character::new("FA", 6, r"(?i)(faust)"));
pub static MILLIA: Lazy<Character> = Lazy::new(|| Character::new("MI", 7, r"(?i)(mill?ia)"));
pub static ZATO: Lazy<Character> = Lazy::new(|| Character::new("ZA", 8, r"(?i)(zato)"));
pub static RAMLETHAL: Lazy<Character> = Lazy::new(|| Character::new("RA", 9, r"(?i)(ram)"));
pub static LEO: Lazy<Character> = Lazy::new(|| Character::new("LE", 10, r"(?i)(leo)"));
pub static NAGORIYUKI: Lazy<Character> = Lazy::new(|| Character::new("NA", 11, r"(?i)(nago)"));
pub static GIOVANNA: Lazy<Character> = Lazy::new(|| Character::new("GI", 12, r"(?i)(gio)"));
pub static ANJI: Lazy<Character> = Lazy::new(|| Character::new("AN", 13, r"(?i)(anji)"));
pub static INO: Lazy<Character> = Lazy::new(|| Character::new("IN", 14, r"(?i)(i-?no)"));
pub static GOLDLEWIS: Lazy<Character> = Lazy::new(|| Character::new("GO", 15, r"(?i)(gl|gold|lewis)"));
pub static JACKO: Lazy<Character> = Lazy::new(|| Character::new("JC", 16, r"(?i)(jack)"));
pub static HAPPYCHAOS: Lazy<Character> = Lazy::new(|| Character::new("HA", 17, r"(?i)(hc|happy|chaos)"));
pub static BAIKEN: Lazy<Character> = Lazy::new(|| Character::new("BA", 18, r"(?i)(baiken)"));
pub static TESTAMENT: Lazy<Character> = Lazy::new(|| Character::new("TE", 19, r"(?i)(test)"));
pub static BRIDGET: Lazy<Character> = Lazy::new(|| Character::new("BI", 20, r"(?i)(bridget)"));
pub static SIN: Lazy<Character> = Lazy::new(|| Character::new("SI", 21, r"(?i)(sin)"));
pub static BEDMAN: Lazy<Character> = Lazy::new(|| Character::new("BE", 22, r"(?i)(bed)"));
pub static ASUKA: Lazy<Character> = Lazy::new(|| Character::new("AS", 23, r"(?i)(asuka)"));
pub static JOHNNY: Lazy<Character> = Lazy::new(|| Character::new("JN", 24, r"(?i)(jo)"));
pub static ELPHELT: Lazy<Character> = Lazy::new(|| Character::new("EL", 25, r"(?i)(el)"));

pub static CHARACTERS: Lazy<[&Character; 26]> = Lazy::new(|| [
    &SOL, &KY, &AXL, &MAY, &CHIPP, &POTEMKIN, &FAUST, &MILLIA, &ZATO, &RAMLETHAL, &LEO,
    &NAGORIYUKI, &GIOVANNA, &ANJI, &INO, &GOLDLEWIS, &JACKO, &HAPPYCHAOS, &BAIKEN, &TESTAMENT,
    &BRIDGET, &SIN, &BEDMAN, &ASUKA, &JOHNNY, &ELPHELT
]);

pub fn get_character<'a>(name: String) -> Option<&'a Character> {
    (*CHARACTERS).into_iter().find(|&character| character.matcher.is_match(&name))
}

#[derive(Clone, Debug)]
pub struct Character {
    pub shortname: String,
    pub id: usize,
    pub matcher: Regex,
}

impl Character {
    fn new(short: &str, id: usize, matcher: &str) -> Self {
        Character {
            shortname: short.to_string(), id, matcher: Regex::new(&format!(r"(?i)({})", matcher)).unwrap(),
        }
    }
}