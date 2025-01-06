use serde::{Deserialize,Deserializer};
use sqlx::FromRow;
use time::{Date, Time};
use time::macros::format_description;

fn deserialize_tags<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    Ok(s.split(',').map(|s| s.trim().to_string()).collect())
}

fn deserialize_time<'de, D>(deserializer: D) -> Result<Time, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let format = format_description!("[hour]:[minute]");
    Time::parse(&s, &format).map_err(serde::de::Error::custom)
}

#[derive(Clone, Debug, Deserialize, FromRow)]
pub struct Protest {
    pub id: i32,
    pub title: String,
    pub description: String,
    #[serde(deserialize_with = "deserialize_tags")]
    pub tags: Vec<String>,
    pub town: Option<String>,
    pub region: Option<String>,
    pub date: Date,
    #[serde(deserialize_with = "deserialize_time")]
    pub time: Time,
    pub location: String,
}

#[derive(Clone, Debug, Deserialize, FromRow)]
pub struct ProtestSave {
    pub title: String,
    pub description: String,
    #[serde(deserialize_with = "deserialize_tags")]
    pub tags: Vec<String>,
    pub date: Date,
    #[serde(deserialize_with = "deserialize_time")]
    pub time: Time,
    pub location: String,
}
