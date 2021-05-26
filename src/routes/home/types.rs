use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Flags {
    pub nsfw: bool,
    pub religious: bool,
    pub political: bool,
    pub racist: bool,
    pub sexist: bool,
    pub explicit: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Data {
    pub error: bool,
    pub category: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub setup: String,
    pub delivery: String,
    pub flags: Flags,
    pub id: i64,
    pub safe: bool,
    pub lang: String,
}
