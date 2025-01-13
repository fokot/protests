use serde::{Deserialize, Deserializer};
use sqlx::FromRow;
use time::{Date, Time};
use time::macros::format_description;
use serde_with::{serde_as, NoneAsEmptyString};

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
    pub user_id: i32,
    pub user_name: String,
    pub image_name: Option<String>,
}

#[serde_as]
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
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(default)]
    pub image_id: Option<i32>,
}


#[serde_as]
#[derive(Deserialize)]
pub struct ProtestSearch {
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(default)]
    pub town: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(default)]
    pub date_from: Option<String>,
    pub tags: Option<Vec<String>>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(default)]
    pub created_by: Option<String>,
}

#[derive(Deserialize)]
pub struct LoginForm {
    pub email: String,
}


// test method to deserialize the ProtestSearch struct
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn deserialize_protest_search() {
        let json = r#"{"town":"Bratislava","date_from":"2021-01-01","tags":["tag1","tag2"],"created_by":"user"}"#;
        let search: ProtestSearch = serde_json::from_str(json).unwrap();

        assert_eq!(search.town, Some("Bratislava".to_string()));
        assert_eq!(search.date_from, Some("2021-01-01".to_string()));
        assert_eq!(search.tags, Some(vec!["tag1".to_string(), "tag2".to_string()]));
        assert_eq!(search.created_by, Some("user".to_string()));
    }

    #[test]
    fn deserialize_protest_search_empty_strings() {
        let json = r#"{"town":"","date_from":"","tags":[],"created_by":""}"#;
        let search: ProtestSearch = serde_json::from_str(json).unwrap();

        assert_eq!(search.town, None);
        assert_eq!(search.date_from, None);
        assert_eq!(search.tags, Some(vec![]));
        assert_eq!(search.created_by, None);
    }

    #[test]
    fn deserialize_protest_search_missing_fields() {
        let json = r#"{}"#;
        let search: ProtestSearch = serde_json::from_str(json).unwrap();

        assert_eq!(search.town, None);
        assert_eq!(search.date_from, None);
        assert_eq!(search.tags, None);
        assert_eq!(search.created_by, None);
    }
}