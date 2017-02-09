use chrono::prelude::*;
use serde_json;
use eventsource::event::Event;

#[derive(Serialize, Deserialize)]
pub struct GameFlags {
    #[serde(rename = "joinAllowed")]
    pub join_allowed: bool,
    #[serde(rename = "passwordNeeded")]
    pub password_needed: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Scenario {
    #[serde(rename = "fileSize")]
    pub file_size: u32,
    #[serde(rename = "fileCRC")]
    pub file_crc: u32,
    #[serde(rename = "contentsCRC")]
    pub contents_crc: u32,
    pub filename: String,
    pub author: String,
}

#[derive(Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub team: u32,
    pub color: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Game {
    pub id: u32,
    pub title: String,
    pub status: String,
    #[serde(rename = "type")]
    pub game_type: String,
    pub comment: String,
    #[serde(rename = "maxPlayers")]
    pub max_players: u32,
    pub host: String,
    pub created: DateTime<Local>,
    pub updated: DateTime<Local>,
    pub flags: GameFlags,
    pub scenario: Scenario,
    pub players: Vec<Player>,
}

pub enum GameEvent {
    Init(Vec<Game>),
    Create(Game),
    Update(Game),
    Delete(Game),
}

macro_rules! try_option {
    ($x:expr) => (
        match $x {
            Ok(x) => x,
            Err(_) => return None,
        }
    )
}

pub fn parse_event(event: &Event) -> Option<GameEvent> {
    use self::GameEvent::*;
    match event.event_type.as_ref().map(String::as_ref) {
        Some("init")   => Some(Init(try_option!(serde_json::from_str(&event.data)))),
        Some("create") => Some(Create(try_option!(serde_json::from_str(&event.data)))),
        Some("update") => Some(Update(try_option!(serde_json::from_str(&event.data)))),
        Some("delete") => Some(Delete(try_option!(serde_json::from_str(&event.data)))),
        _ => None
    }
}

mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn deserialize_game() {
        let data = r#"{"id":435272,"title":"Lava kommt von unten hoch","status":"lobby","type":"melee","comment":"Qualit\u00e4tshost \u00a6 Zitat HJK: lol xD \u00a6 Zitat alex: rofl  \u00a6 Gerne alerten","maxPlayers":24,"host":"Fliegkeks","created":"2017-02-09T22:14:47+0100","updated":"2017-02-09T22:18:46+0100","flags":{"joinAllowed":true,"passwordNeeded":false},"scenario":{"fileSize":162398,"fileCRC":256140861,"contentsCRC":2677459067,"filename":"lkvuh.c4s","author":"Oliver Schneider"},"players":[{"name":"Peder","team":1,"color":15990784},{"name":"[TL] alex","team":2,"color":2291228},{"name":"Funni","team":3,"color":38136}]}"#;
        let game: Game = serde_json::from_str(data).unwrap();
        assert_eq!(game.id, 435272);
    }
}
