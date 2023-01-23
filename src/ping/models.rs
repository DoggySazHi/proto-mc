use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Version {
    pub name: String,
    pub protocol: i32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Players {
    pub max: u32,
    pub online: u32,
    pub sample: Option<Vec<Sample>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sample {
    pub name: String,
    pub id: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Description {
    pub text: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ping {
    pub version: Version,
    pub players: Players,
    pub description: Option<Description>,
    pub favicon: Option<String>,
    #[serde(rename = "previewsChat")]
    pub previews_chat: Option<bool>,
    #[serde(rename = "enforcesSecureChat")]
    pub enforces_secure_chat: Option<bool>,
    pub ping: Option<u128>
}