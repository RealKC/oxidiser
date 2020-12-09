use serde::{Deserialize, Serialize};

// PONDER: move the packets out?
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Packet {
    // System packets
    #[serde(rename = "auth-result")]
    AuthResult { result: u64, error: String },

    #[serde(rename = "connection-result")]
    ConnectionResult { result: u64, error: String },

    #[serde(rename = "disconnected")]
    Disconnected { reason: String },

    #[serde(rename = "ping")]
    Ping,

    // General packets
    #[serde(rename = "character-link")]
    #[serde(rename_all = "camelCase")]
    CharacterLink { character_info: CharacterInfo },

    #[serde(rename = "clients-info")]
    #[serde(rename_all = "camelCase")]
    ClientsInfo { client_infos: Vec<ClientInfo> },

    #[serde(rename = "enter-world")]
    EnterWorld,

    #[serde(rename = "world-info")]
    #[serde(rename_all = "camelCase")]
    WorldInfo { world_info: WorldInfo },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CharacterInfo {
    id: u64,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClientInfo {
    id: u64,
    username: String,
    character_name: Option<String>,
    character_id: u64,
    playing: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorldInfo {}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_result_packets() {
        let packet: Packet = serde_json::from_str(
            r#"{
            "type": "auth-result",
            "result": 1,
            "error": "fun"
        }"#,
        )
        .unwrap();

        if let Packet::AuthResult { result, error } = packet {
            println!("res={} err={}", result, error);
        } else {
            panic!("Expected AuthResult, got something else")
        }

        let packet: Packet = serde_json::from_str(
            r#"{
            "type": "connection-result",
            "result": 0,
            "error": ""
        }
        "#,
        )
        .unwrap();

        if let Packet::ConnectionResult { result, error } = packet {
            println!("res={} err={}", result, error);
        } else {
            panic!("Expected AuthResult, got something else")
        }
    }

    #[test]
    fn test_character_link_packet() {
        let packet: Packet = serde_json::from_str(
            r#"{
            "type": "character-link",
            "characterInfo": {
              "id": 1234,
              "name": "KC"
            }
          }"#,
        )
        .unwrap();

        if let Packet::CharacterLink { character_info } = packet {
            println!("{:?}", character_info);
        } else {
            panic!("Expected CharacterLink, got something else")
        }
    }

    #[test]
    fn test_clients_info() {
        let client_info: ClientInfo = serde_json::from_str(
            r#"{
                "id": 1234,
                "username": "KC",
                "characterName": "hackerman",
                "characterId": 6942,
                "playing": true
            }
        "#,
        )
        .unwrap();

        println!("{:?}", client_info);

        let client_info: Packet = serde_json::from_str(
            r#"{ 
                "clientInfos": [
                    {
                        "id": 0,
                        "username": "bob",
                        "characterName": null,
                        "characterId": 0,
                        "playing": false
                    }
                ],
                "type": "clients-info"
          }"#,
        )
        .unwrap();

        println!("{:?}", client_info);
    }

    #[test]
    fn test_empty_packets() {
        let _ping: Packet = serde_json::from_str(r#"{"type": "ping"}"#).unwrap();
        let _enter_world: Packet = serde_json::from_str(r#"{"type": "enter-world"}"#).unwrap();

        let ping = serde_json::from_str::<Packet>(r#"{"type": "ping", "result": "bar"}"#);
        assert!(ping.is_err());
    }

    #[test]
    fn test_world_info() {
        let world_info: Packet = serde_json::from_str(
            r#"{ 
            "worldInfo": {
              "name": "Sample",
              "characters": [ { "id": 1, "name": "Moebius" }, { "id": 2, "name": "Ares" } ]
            },
            "type": "world-info"
          }"#,
        )
        .unwrap();

        println!("{:?}", world_info);
    }
}
