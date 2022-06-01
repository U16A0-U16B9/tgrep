// 
// TODO: temp solutions 
// 
#[derive(Clone)]
pub enum TriggerType {
    None,
    Positive,
    Negative
}

pub const POSITIVE_TRIGGERS: [&'static str; 7] = [
    "plus",
    "kek",
    "krk",
    "kwk",
    "kekw",
    "+",
    "gg"
];

pub const NEGATIVE_TRIGGERS: [&'static str; 2] = [
    "minus",
    "-"
];